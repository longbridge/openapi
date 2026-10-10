//! Keeps HTTP-transport quote data aligned with the WebSocket's US overnight
//! rule.
//!
//! The quote WebSocket only returns the US overnight session when the
//! connection opted in (`need_over_night_quote`, i.e.
//! `Config::enable_overnight`), and it applies that *before* count / range
//! caps. The REST endpoints always return overnight data and offer no opt-out,
//! so with [`QuoteTransport::Http`](crate::QuoteTransport::Http) and overnight
//! disabled the SDK drops overnight bars client-side and, for capped
//! candlestick windows, pages further so both transports yield the same bars.

use longbridge_proto::quote;
use time::{Date, OffsetDateTime, PrimitiveDateTime};
use time_tz::{OffsetDateTimeExt, timezones::db::america::NEW_YORK};

use crate::{Result, quote::Candlestick};

/// Server-side cap on the `count` of a candlestick request (`count = 5000` is
/// rejected with `301607`; date-range queries observed returning up to 1440
/// bars, so a page of at least this size is treated as capped).
pub(crate) const MAX_HISTORY_CANDLESTICKS: usize = 1000;

/// Hard limit on extra requests one call may issue while topping up a window.
pub(crate) const MAX_REFILL_ROUNDS: usize = 8;

/// `true` for US equities / ETFs (`AAPL.US`), whose overnight session is what
/// the WebSocket gates. Indices (`.VIX.US`) trade on their own schedule and
/// are left alone.
pub(crate) fn is_us_equity_symbol(symbol: &str) -> bool {
    symbol.ends_with(".US") && !symbol.starts_with('.')
}

/// Whether `timestamp` falls in the US overnight session (20:00–04:00 New
/// York).
pub(crate) fn is_us_overnight(timestamp: OffsetDateTime) -> bool {
    longbridge_candlesticks::markets::US.trade_session(timestamp)
        == Some(longbridge_candlesticks::TRADE_SESSION_OVERNIGHT)
}

fn is_overnight_candlestick(candlestick: &quote::Candlestick) -> bool {
    candlestick.trade_session == quote::TradeSession::OvernightTrade as i32
}

/// New York wall-clock time of `timestamp`, as used by the offset query.
pub(crate) fn new_york_local(timestamp: OffsetDateTime) -> PrimitiveDateTime {
    let local = timestamp.to_timezone(NEW_YORK);
    PrimitiveDateTime::new(local.date(), local.time())
}

/// New York calendar date of `timestamp`.
pub(crate) fn new_york_date(timestamp: OffsetDateTime) -> Date {
    timestamp.to_timezone(NEW_YORK).date()
}

/// How many bars a candlestick window should hold once overnight bars are
/// dropped, i.e. what the WebSocket (which drops them before capping) returns.
#[derive(Debug, Clone, Copy)]
pub(crate) enum CandlestickWindow {
    /// A count-limited query (`count` requested).
    Count(usize),
    /// A date-range query, served newest-first and capped at a fixed size.
    DateRange { start: Option<Date> },
}

/// The number of bars the WebSocket would return for `window` given the raw
/// page, or `None` if the page is not capped (nothing to top up).
pub(crate) fn window_target(
    window: CandlestickWindow,
    raw: &[quote::Candlestick],
) -> Option<usize> {
    match window {
        // A page shorter than `count` is either all the data there is or the
        // server's cap; either way it is the window the WebSocket would return.
        CandlestickWindow::Count(count) => Some(count.min(raw.len())),
        // With a `start`, the window is capped only if it did not reach the
        // first bar of that day (midnight New York, where the overnight
        // session begins); without one, a page at least as long as the request
        // cap is taken as capped.
        CandlestickWindow::DateRange { start } => {
            let capped = match (start, raw_edge(raw, false)) {
                (Some(start), Some(earliest)) => new_york_local(earliest) > start.midnight(),
                (None, Some(_)) => raw.len() >= MAX_HISTORY_CANDLESTICKS,
                _ => false,
            };
            capped.then_some(raw.len())
        }
    }
}

/// Earliest (`forward == false`) or latest (`forward == true`) timestamp in a
/// raw page.
pub(crate) fn raw_edge(
    candlesticks: &[quote::Candlestick],
    forward: bool,
) -> Option<OffsetDateTime> {
    let timestamps = candlesticks.iter().map(|candlestick| candlestick.timestamp);
    if forward {
        timestamps.max()
    } else {
        timestamps.min()
    }
    .and_then(|timestamp| OffsetDateTime::from_unix_timestamp(timestamp).ok())
}

/// Convert a raw page, dropping overnight candlesticks. Returns the kept
/// candlesticks and how many were dropped.
pub(crate) fn drop_overnight(
    candlesticks: Vec<quote::Candlestick>,
) -> Result<(Vec<Candlestick>, usize)> {
    let total = candlesticks.len();
    let kept = candlesticks
        .into_iter()
        .filter(|candlestick| !is_overnight_candlestick(candlestick))
        .map(TryInto::try_into)
        .collect::<Result<Vec<_>>>()?;
    let dropped = total - kept.len();
    Ok((kept, dropped))
}

/// Top up a capped candlestick window after overnight bars were dropped.
///
/// `candlesticks` is the already-filtered first page and `cursor` the edge of
/// its *raw* window (overnight bars included). Pages are fetched beyond the
/// cursor with `fetch(anchor_in_new_york, count)`, where the server treats the
/// anchor as inclusive, until `target` bars are collected, the data runs out,
/// `bound` (a New York date, for date-range queries) is crossed, or
/// [`MAX_REFILL_ROUNDS`] is reached.
pub(crate) async fn refill<F, Fut>(
    forward: bool,
    mut candlesticks: Vec<Candlestick>,
    mut cursor: Option<OffsetDateTime>,
    target: usize,
    bound: Option<Date>,
    mut fetch: F,
) -> Result<Vec<Candlestick>>
where
    F: FnMut(PrimitiveDateTime, usize) -> Fut,
    Fut: Future<Output = Result<Vec<quote::Candlestick>>>,
{
    let mut stalled = false;
    for _ in 0..MAX_REFILL_ROUNDS {
        if candlesticks.len() >= target {
            break;
        }
        let Some(anchor) = cursor else {
            break;
        };
        let need = target - candlesticks.len();
        // Over-fetch a little for the anchor bar (returned and discarded) and
        // scattered overnight bars. The overnight session is one contiguous
        // 20:00–04:00 block, so once a page yields nothing the cursor is
        // inside it: jump with a full page rather than inching through it.
        let count = if stalled {
            MAX_HISTORY_CANDLESTICKS
        } else {
            (need + need / 2 + 1).min(MAX_HISTORY_CANDLESTICKS)
        };
        let raw = fetch(new_york_local(anchor), count).await?;
        let exhausted = raw.len() < count;
        let next_cursor = raw_edge(&raw, forward);

        let (mut more, _) = drop_overnight(raw)?;
        more.retain(|candlestick| {
            if forward {
                candlestick.timestamp > anchor
            } else {
                candlestick.timestamp < anchor
            }
        });
        let mut crossed_bound = false;
        if let Some(bound) = bound {
            more.retain(|candlestick| {
                let date = new_york_date(candlestick.timestamp);
                let inside = if forward {
                    date <= bound
                } else {
                    date >= bound
                };
                crossed_bound |= !inside;
                inside
            });
        }

        stalled = more.is_empty();
        if forward {
            more.truncate(need);
            candlesticks.extend(more);
        } else {
            let skip = more.len().saturating_sub(need);
            candlesticks.splice(0..0, more.into_iter().skip(skip));
        }

        if exhausted || crossed_bound || next_cursor.is_none() || next_cursor == cursor {
            break;
        }
        cursor = next_cursor;
    }

    Ok(candlesticks)
}

#[cfg(test)]
mod tests {
    use std::{cell::Cell, rc::Rc};

    use time::macros::datetime;
    use time_tz::PrimitiveDateTimeExt;

    use super::*;

    /// One-minute bars for New York 2026-10-09 (EDT) from `start` for `n`
    /// minutes, with the trade session derived from the clock.
    fn timeline(start: OffsetDateTime, n: usize) -> Vec<quote::Candlestick> {
        (0..n as i64)
            .map(|i| {
                let timestamp = start + time::Duration::minutes(i);
                let local = timestamp.to_timezone(NEW_YORK).time();
                let session = match local.hour() {
                    0..=3 | 20..=23 => quote::TradeSession::OvernightTrade,
                    4..=8 => quote::TradeSession::PreTrade,
                    9 if local.minute() < 30 => quote::TradeSession::PreTrade,
                    9..=15 => quote::TradeSession::NormalTrade,
                    _ => quote::TradeSession::PostTrade,
                };
                quote::Candlestick {
                    close: "1".into(),
                    open: "1".into(),
                    low: "1".into(),
                    high: "1".into(),
                    volume: 1,
                    turnover: "1".into(),
                    timestamp: timestamp.unix_timestamp(),
                    trade_session: session as i32,
                }
            })
            .collect()
    }

    /// A fake server: `count` bars from the anchor (inclusive) in `forward`
    /// direction, ascending.
    fn server(
        data: Vec<quote::Candlestick>,
        forward: bool,
        calls: Rc<Cell<usize>>,
    ) -> impl FnMut(PrimitiveDateTime, usize) -> std::future::Ready<Result<Vec<quote::Candlestick>>>
    {
        move |anchor, count| {
            calls.set(calls.get() + 1);
            let anchor = anchor
                .assume_timezone(NEW_YORK)
                .unwrap_first()
                .unix_timestamp();
            let page: Vec<_> = if forward {
                data.iter()
                    .filter(|c| c.timestamp >= anchor)
                    .take(count)
                    .cloned()
                    .collect()
            } else {
                let mut page: Vec<_> = data
                    .iter()
                    .rev()
                    .filter(|c| c.timestamp <= anchor)
                    .take(count)
                    .cloned()
                    .collect();
                page.reverse();
                page
            };
            std::future::ready(Ok(page))
        }
    }

    fn minutes(candlesticks: &[Candlestick]) -> Vec<String> {
        candlesticks
            .iter()
            .map(|c| {
                let local = c.timestamp.to_timezone(NEW_YORK);
                format!("{:02}:{:02}", local.hour(), local.minute())
            })
            .collect()
    }

    fn assert_sorted_unique(candlesticks: &[Candlestick]) {
        for pair in candlesticks.windows(2) {
            assert!(
                pair[0].timestamp < pair[1].timestamp,
                "not ascending/unique"
            );
        }
    }

    const DAY: OffsetDateTime = datetime!(2026-10-09 00:00 -04:00);

    #[test]
    fn backward_fills_after_dropping_overnight() {
        let data = timeline(DAY, 24 * 60);
        // Raw page 19:00–20:39 NY: 60 post + 40 overnight.
        let end = 20 * 60 + 40;
        let page = data[end - 100..end].to_vec();
        let cursor = raw_edge(&page, false);
        let (kept, dropped) = drop_overnight(page).unwrap();
        assert_eq!((kept.len(), dropped), (60, 40));

        let calls = Rc::new(Cell::new(0));
        let rt = tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap();
        let out = rt
            .block_on(refill(
                false,
                kept,
                cursor,
                100,
                None,
                server(data, false, calls.clone()),
            ))
            .unwrap();
        assert_eq!(out.len(), 100);
        assert_sorted_unique(&out);
        let m = minutes(&out);
        assert_eq!(
            (m.first().unwrap().as_str(), m.last().unwrap().as_str()),
            ("18:20", "19:59")
        );
        assert_eq!(calls.get(), 1);
    }

    #[test]
    fn forward_fills_after_dropping_overnight() {
        let data = timeline(DAY, 24 * 60);
        // First 100 raw bars from 03:00 NY: 60 overnight + 40 pre.
        let start = 3 * 60;
        let page = data[start..start + 100].to_vec();
        let cursor = raw_edge(&page, true);
        let (kept, dropped) = drop_overnight(page).unwrap();
        assert_eq!((kept.len(), dropped), (40, 60));

        let calls = Rc::new(Cell::new(0));
        let rt = tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap();
        let out = rt
            .block_on(refill(
                true,
                kept,
                cursor,
                100,
                None,
                server(data, true, calls.clone()),
            ))
            .unwrap();
        assert_eq!(out.len(), 100);
        assert_sorted_unique(&out);
        let m = minutes(&out);
        assert_eq!(
            (m.first().unwrap().as_str(), m.last().unwrap().as_str()),
            ("04:00", "05:39")
        );
        assert_eq!(calls.get(), 1);
    }

    #[test]
    fn page_of_only_overnight_bars_still_makes_progress() {
        let data = timeline(DAY, 24 * 60);
        // Raw page 21:00–22:39: every bar is overnight.
        let start = 21 * 60;
        let page = data[start..start + 100].to_vec();
        let cursor = raw_edge(&page, false);
        let (kept, dropped) = drop_overnight(page).unwrap();
        assert_eq!((kept.len(), dropped), (0, 100));

        let calls = Rc::new(Cell::new(0));
        let rt = tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap();
        let out = rt
            .block_on(refill(
                false,
                kept,
                cursor,
                100,
                None,
                server(data, false, calls.clone()),
            ))
            .unwrap();
        assert_eq!(out.len(), 100);
        assert_sorted_unique(&out);
        let m = minutes(&out);
        assert_eq!(
            (m.first().unwrap().as_str(), m.last().unwrap().as_str()),
            ("18:20", "19:59")
        );
        assert!(calls.get() <= 3, "took {} rounds", calls.get());
    }

    #[test]
    fn stops_at_date_bound_and_when_data_runs_out() {
        // Only 2026-10-09 exists; a range starting that day cannot reach back.
        let data = timeline(DAY, 24 * 60);
        let page = data[..100].to_vec(); // 00:00–01:39, all overnight
        let cursor = raw_edge(&page, false);
        let (kept, _) = drop_overnight(page).unwrap();

        let calls = Rc::new(Cell::new(0));
        let rt = tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap();
        let out = rt
            .block_on(refill(
                false,
                kept,
                cursor,
                100,
                Some(DAY.date()),
                server(data.clone(), false, calls.clone()),
            ))
            .unwrap();
        assert!(out.is_empty());
        assert_eq!(calls.get(), 1);

        // Without a bound the data simply runs out: terminates, returns what
        // exists.
        let page = data[..100].to_vec();
        let cursor = raw_edge(&page, false);
        let (kept, _) = drop_overnight(page).unwrap();
        let calls = Rc::new(Cell::new(0));
        let out = rt
            .block_on(refill(
                false,
                kept,
                cursor,
                100,
                None,
                server(data, false, calls.clone()),
            ))
            .unwrap();
        assert!(out.is_empty());
        assert_eq!(calls.get(), 1);
    }

    #[test]
    fn small_need_inside_overnight_block_backward() {
        // Raw page 03:50–05:29: 10 overnight + 90 pre. Only 10 bars are
        // needed, but they lie on the far side of the 8-hour overnight block.
        let data = timeline(DAY - time::Duration::days(1), 2 * 24 * 60);
        let start = 24 * 60 + 3 * 60 + 50;
        let page = data[start..start + 100].to_vec();
        let cursor = raw_edge(&page, false);
        let (kept, dropped) = drop_overnight(page).unwrap();
        assert_eq!((kept.len(), dropped), (90, 10));
        let calls = Rc::new(Cell::new(0));
        let rt = tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap();
        let out = rt
            .block_on(refill(
                false,
                kept,
                cursor,
                100,
                None,
                server(data, false, calls.clone()),
            ))
            .unwrap();
        assert_eq!(out.len(), 100);
        assert_sorted_unique(&out);
        let m = minutes(&out);
        assert_eq!(
            (m[0].as_str(), m[9].as_str(), m[10].as_str()),
            ("19:50", "19:59", "04:00")
        );
        assert!(calls.get() <= 3, "took {} rounds", calls.get());
    }

    #[test]
    fn small_need_inside_overnight_block_forward() {
        // Raw page 19:40–21:19: 20 post + 80 overnight; the next bars are at
        // 04:00 the following day.
        let data = timeline(DAY, 2 * 24 * 60);
        let start = 19 * 60 + 40;
        let page = data[start..start + 100].to_vec();
        let cursor = raw_edge(&page, true);
        let (kept, dropped) = drop_overnight(page).unwrap();
        assert_eq!((kept.len(), dropped), (20, 80));
        let calls = Rc::new(Cell::new(0));
        let rt = tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap();
        let out = rt
            .block_on(refill(
                true,
                kept,
                cursor,
                100,
                None,
                server(data, true, calls.clone()),
            ))
            .unwrap();
        assert_eq!(out.len(), 100);
        assert_sorted_unique(&out);
        let m = minutes(&out);
        assert_eq!(
            (m[19].as_str(), m[20].as_str(), m[99].as_str()),
            ("19:59", "04:00", "05:19")
        );
        assert!(calls.get() <= 3, "took {} rounds", calls.get());
    }

    #[test]
    fn window_targets() {
        let data = timeline(DAY, 24 * 60);
        assert_eq!(
            window_target(CandlestickWindow::Count(1000), &data[..300]),
            Some(300)
        );
        assert_eq!(
            window_target(CandlestickWindow::Count(100), &data[..300]),
            Some(100)
        );
        let start = Some(DAY.date());
        // Reaches midnight of `start`: not capped.
        assert_eq!(
            window_target(CandlestickWindow::DateRange { start }, &data),
            None
        );
        // Earliest bar later than midnight of `start`: capped.
        assert_eq!(
            window_target(CandlestickWindow::DateRange { start }, &data[20 * 60..]),
            Some(4 * 60)
        );
        // Earlier `start` the page did not reach: capped.
        let earlier = Some(DAY.date().previous_day().unwrap());
        assert_eq!(
            window_target(CandlestickWindow::DateRange { start: earlier }, &data),
            Some(1440)
        );
        // No start: only a full-size page counts as capped.
        assert_eq!(
            window_target(CandlestickWindow::DateRange { start: None }, &data[..999]),
            None
        );
        assert_eq!(
            window_target(CandlestickWindow::DateRange { start: None }, &data),
            Some(1440)
        );
        assert_eq!(
            window_target(CandlestickWindow::DateRange { start }, &[]),
            None
        );
    }

    #[test]
    fn round_limit_is_enforced() {
        // A pathological server that always returns a full page of overnight
        // bars.
        let calls = Rc::new(Cell::new(0));
        let c2 = calls.clone();
        let mut t = DAY.unix_timestamp();
        let fetch = move |_anchor: PrimitiveDateTime, count: usize| {
            c2.set(c2.get() + 1);
            t -= 60 * count as i64;
            let page = (0..count as i64)
                .map(|i| quote::Candlestick {
                    close: "1".into(),
                    open: "1".into(),
                    low: "1".into(),
                    high: "1".into(),
                    volume: 1,
                    turnover: "1".into(),
                    timestamp: t + i * 60,
                    trade_session: quote::TradeSession::OvernightTrade as i32,
                })
                .collect();
            std::future::ready(Ok(page))
        };
        let rt = tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap();
        let out = rt
            .block_on(refill(false, vec![], Some(DAY), 50, None, fetch))
            .unwrap();
        assert!(out.is_empty());
        assert_eq!(calls.get(), MAX_REFILL_ROUNDS);
    }

    #[test]
    fn us_overnight_window() {
        // EDT (2026-10-09) and EST (2026-01-09) boundaries.
        for day in [
            datetime!(2026-10-09 00:00 -04:00),
            datetime!(2026-01-09 00:00 -05:00),
        ] {
            let at = |h: i64, m: i64| day + time::Duration::minutes(h * 60 + m);
            assert!(is_us_overnight(at(0, 0)));
            assert!(is_us_overnight(at(3, 59)));
            assert!(!is_us_overnight(at(4, 0)));
            assert!(!is_us_overnight(at(9, 30)));
            assert!(!is_us_overnight(at(16, 0)));
            assert!(!is_us_overnight(at(19, 59)));
            assert!(is_us_overnight(at(20, 0)));
            assert!(is_us_overnight(at(23, 59)));
        }
        assert!(is_us_equity_symbol("AAPL.US"));
        assert!(is_us_equity_symbol("AAPL251017C340000.US"));
        assert!(!is_us_equity_symbol(".VIX.US"));
        assert!(!is_us_equity_symbol("700.HK"));
    }
}
