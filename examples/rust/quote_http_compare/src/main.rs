//! Compare every quote pull API over the WebSocket vs HTTP transports.
//!
//! Reads credentials from `LONGBRIDGE_*` env (or `.env`). Point
//! `LONGBRIDGE_HTTP_URL` / `LONGBRIDGE_QUOTE_WS_URL` at an environment that
//! serves the `/quote/*` routes (canary). For each call prints the arguments,
//! both results (`Debug`), latency and whether WS == HTTP.
use std::{sync::Arc, time::Instant};

use longbridge::{
    quote::{
        AdjustType, CalcIndex, Period, QuoteContext, SortOrderType, TradeSessions, WarrantSortBy,
    },
    Config, Market, QuoteTransport,
};
use time::macros::{date, datetime};

macro_rules! compare {
    ($ws:ident, $http:ident, $label:expr, $args:expr, |$ctx:ident| $call:expr) => {{
        let t = Instant::now();
        let a = {
            let $ctx = &$ws;
            $call.await.map_err(|e| e.into_simple_error())
        };
        let ws_ms = t.elapsed().as_millis();
        let t = Instant::now();
        let b = {
            let $ctx = &$http;
            $call.await.map_err(|e| e.into_simple_error())
        };
        let http_ms = t.elapsed().as_millis();
        // Errors are compared by business code only: WS and HTTP differ in
        // the error variant / trace_id by construction.
        let (a, b) = match (&a, &b) {
            (Err(x), Err(y)) => (
                format!("Err(code={:?}, message={:?})", x.code(), x.message()),
                format!("Err(code={:?}, message={:?})", y.code(), y.message()),
            ),
            _ => (format!("{a:#?}"), format!("{b:#?}")),
        };
        println!("=== {}", $label);
        println!("REQ {}", $args);
        println!("WS_MS {ws_ms}\nHTTP_MS {http_ms}");
        println!("EQUAL {}", a == b);
        println!("WS<<\n{a}\n>>WS\nHTTP<<\n{b}\n>>HTTP");
    }};
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base = Config::from_apikey_env()?;
    let (ws, _) = QuoteContext::new(Arc::new(
        base.clone().quote_transport(QuoteTransport::WebSocket),
    ));
    let (http, _) = QuoteContext::new(Arc::new(base.quote_transport(QuoteTransport::Http)));

    // Resolve an option and a warrant symbol to use below.
    let expiries = ws.option_chain_expiry_date_list("AAPL.US").await?;
    let Some(&first_expiry) = expiries.first() else {
        return Err("no option expiry dates for AAPL.US".into());
    };
    // `option_chain_info_by_date` is not deployed on every environment; fall
    // back to building an at-the-money-ish contract symbol from the expiry.
    let option_symbol = match ws
        .option_chain_info_by_date("AAPL.US", first_expiry, false)
        .await
    {
        Ok(chain) if !chain.is_empty() => chain[chain.len() / 2].symbol.clone(),
        _ => {
            let d = first_expiry;
            format!(
                "AAPL{:02}{:02}{:02}C340000.US",
                d.year() % 100,
                d.month() as u8,
                d.day()
            )
        }
    };
    let warrants = ws
        .warrant_list(
            "700.HK",
            WarrantSortBy::LastDone,
            SortOrderType::Descending,
            None,
            None,
            None,
            None,
            None,
        )
        .await?;
    let Some(warrant_symbol) = warrants.first().map(|w| w.symbol.clone()) else {
        return Err("no warrants for 700.HK".into());
    };

    compare!(ws, http, "static_info", r#"["700.HK","AAPL.US"]"#, |c| c
        .static_info(["700.HK", "AAPL.US"]));
    compare!(ws, http, "quote", r#"["700.HK","AAPL.US"]"#, |c| c
        .quote(["700.HK", "AAPL.US"]));
    compare!(
        ws,
        http,
        "option_quote",
        format!("[{option_symbol:?}]"),
        |c| c.option_quote([option_symbol.clone()])
    );
    compare!(
        ws,
        http,
        "warrant_quote",
        format!("[{warrant_symbol:?}]"),
        |c| c.warrant_quote([warrant_symbol.clone()])
    );
    for symbol in ["700.HK", "AAPL.US"] {
        compare!(ws, http, format!("depth {symbol}"), symbol, |c| c
            .depth(symbol));
    }
    compare!(ws, http, "brokers 700.HK", "700.HK", |c| c
        .brokers("700.HK"));
    compare!(ws, http, "participants", "-", |c| c.participants());
    compare!(ws, http, "trades 700.HK", "700.HK, count=20", |c| c
        .trades("700.HK", 20));
    compare!(ws, http, "intraday AAPL.US", "AAPL.US, All", |c| c
        .intraday("AAPL.US", TradeSessions::All));
    compare!(
        ws,
        http,
        "candlesticks AAPL.US",
        "AAPL.US, Day, 20, NoAdjust, All",
        |c| c.candlesticks(
            "AAPL.US",
            Period::Day,
            20,
            AdjustType::NoAdjust,
            TradeSessions::All
        )
    );
    compare!(
        ws,
        http,
        "candlesticks 700.HK 1m",
        "700.HK, OneMinute, 30, ForwardAdjust, Intraday",
        |c| c.candlesticks(
            "700.HK",
            Period::OneMinute,
            30,
            AdjustType::ForwardAdjust,
            TradeSessions::Intraday
        )
    );
    compare!(
        ws,
        http,
        "candlesticks AAPL.US 1m All",
        "AAPL.US, OneMinute, 1000, NoAdjust, All",
        |c| c.candlesticks(
            "AAPL.US",
            Period::OneMinute,
            1000,
            AdjustType::NoAdjust,
            TradeSessions::All
        )
    );
    compare!(
        ws,
        http,
        "history_candlesticks_by_date AAPL.US 1m All",
        "AAPL.US, OneMinute, NoAdjust, 2026-10-08..2026-10-09, All",
        |c| c.history_candlesticks_by_date(
            "AAPL.US",
            Period::OneMinute,
            AdjustType::NoAdjust,
            Some(date!(2026 - 10 - 08)),
            Some(date!(2026 - 10 - 09)),
            TradeSessions::All
        )
    );
    for forward in [false, true] {
        compare!(
            ws,
            http,
            format!("history_candlesticks_by_offset AAPL.US 1m All forward={forward}"),
            format!("AAPL.US, OneMinute, NoAdjust, forward={forward}, 2026-10-08 12:00, 600, All"),
            |c| c.history_candlesticks_by_offset(
                "AAPL.US",
                Period::OneMinute,
                AdjustType::NoAdjust,
                forward,
                Some(datetime!(2026-10-08 12:00)),
                600,
                TradeSessions::All
            )
        );
    }
    compare!(
        ws,
        http,
        "history_candlesticks_by_date AAPL.US 1m All (uncapped)",
        "AAPL.US, OneMinute, NoAdjust, 2026-10-09..2026-10-09, All",
        |c| c.history_candlesticks_by_date(
            "AAPL.US",
            Period::OneMinute,
            AdjustType::NoAdjust,
            Some(date!(2026 - 10 - 09)),
            Some(date!(2026 - 10 - 09)),
            TradeSessions::All
        )
    );
    compare!(ws, http, "intraday .VIX.US (index)", ".VIX.US, All", |c| c
        .intraday(".VIX.US", TradeSessions::All));
    compare!(ws, http, "intraday 700.HK", "700.HK, Intraday", |c| c
        .intraday("700.HK", TradeSessions::Intraday));
    compare!(ws, http, "trades AAPL.US", "AAPL.US, count=50", |c| c
        .trades("AAPL.US", 50));
    compare!(
        ws,
        http,
        "history_candlesticks_by_offset 700.HK",
        "700.HK, Day, NoAdjust, forward=false, 2026-06-01 00:00, 10, Intraday",
        |c| c.history_candlesticks_by_offset(
            "700.HK",
            Period::Day,
            AdjustType::NoAdjust,
            false,
            Some(datetime!(2026-06-01 00:00)),
            10,
            TradeSessions::Intraday
        )
    );
    compare!(
        ws,
        http,
        "history_candlesticks_by_date AAPL.US",
        "AAPL.US, Week, ForwardAdjust, 2026-01-01..2026-03-01, All",
        |c| c.history_candlesticks_by_date(
            "AAPL.US",
            Period::Week,
            AdjustType::ForwardAdjust,
            Some(date!(2026 - 01 - 01)),
            Some(date!(2026 - 03 - 01)),
            TradeSessions::All
        )
    );
    compare!(
        ws,
        http,
        "option_chain_expiry_date_list AAPL.US",
        "AAPL.US",
        |c| c.option_chain_expiry_date_list("AAPL.US")
    );
    compare!(ws, http, "warrant_issuers", "-", |c| c.warrant_issuers());
    compare!(
        ws,
        http,
        "warrant_list 700.HK",
        "700.HK, ChangeRate, Ascending, no filters",
        |c| c.warrant_list(
            "700.HK",
            WarrantSortBy::ChangeRate,
            SortOrderType::Ascending,
            None,
            None,
            None,
            None,
            None
        )
    );
    compare!(ws, http, "trading_session", "-", |c| c.trading_session());
    compare!(
        ws,
        http,
        "trading_days HK",
        "HK, 2026-10-01..2026-10-31",
        |c| c.trading_days(Market::HK, date!(2026 - 10 - 01), date!(2026 - 10 - 31))
    );
    compare!(ws, http, "capital_flow 700.HK", "700.HK", |c| c
        .capital_flow("700.HK"));
    compare!(ws, http, "capital_distribution 700.HK", "700.HK", |c| c
        .capital_distribution("700.HK"));
    compare!(
        ws,
        http,
        "calc_indexes",
        "[700.HK, AAPL.US, option, warrant], all indexes",
        |c| c.calc_indexes(
            [
                "700.HK".to_string(),
                "AAPL.US".to_string(),
                option_symbol.clone(),
                warrant_symbol.clone()
            ],
            [
                CalcIndex::LastDone,
                CalcIndex::ChangeValue,
                CalcIndex::ChangeRate,
                CalcIndex::Volume,
                CalcIndex::Turnover,
                CalcIndex::YtdChangeRate,
                CalcIndex::TurnoverRate,
                CalcIndex::TotalMarketValue,
                CalcIndex::CapitalFlow,
                CalcIndex::Amplitude,
                CalcIndex::VolumeRatio,
                CalcIndex::PeTtmRatio,
                CalcIndex::PbRatio,
                CalcIndex::DividendRatioTtm,
                CalcIndex::FiveDayChangeRate,
                CalcIndex::TenDayChangeRate,
                CalcIndex::HalfYearChangeRate,
                CalcIndex::FiveMinutesChangeRate,
                CalcIndex::ExpiryDate,
                CalcIndex::StrikePrice,
                CalcIndex::UpperStrikePrice,
                CalcIndex::LowerStrikePrice,
                CalcIndex::OutstandingQty,
                CalcIndex::OutstandingRatio,
                CalcIndex::Premium,
                CalcIndex::ItmOtm,
                CalcIndex::ImpliedVolatility,
                CalcIndex::WarrantDelta,
                CalcIndex::CallPrice,
                CalcIndex::ToCallPrice,
                CalcIndex::EffectiveLeverage,
                CalcIndex::LeverageRatio,
                CalcIndex::ConversionRatio,
                CalcIndex::BalancePoint,
                CalcIndex::OpenInterest,
                CalcIndex::Delta,
                CalcIndex::Gamma,
                CalcIndex::Theta,
                CalcIndex::Vega,
                CalcIndex::Rho,
            ]
        )
    );

    // Error paths: the business error code must match across transports.
    compare!(ws, http, "ERR depth NOTEXIST.XX", "NOTEXIST.XX", |c| c
        .depth("NOTEXIST.XX"));
    compare!(ws, http, "ERR quote []", "[]", |c| c
        .quote(Vec::<String>::new()));
    compare!(
        ws,
        http,
        "ERR candlesticks count=5000",
        "700.HK, Day, 5000",
        |c| c.candlesticks(
            "700.HK",
            Period::Day,
            5000,
            AdjustType::NoAdjust,
            TradeSessions::Intraday
        )
    );
    Ok(())
}
