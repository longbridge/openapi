//! Symbol ↔ counter_id conversion utilities.
//!
//! A `counter_id` is the internal instrument identifier used by the
//! Longbridge backend, e.g. `ST/US/TSLA`, `ETF/US/SPY`, `IX/HK/HSI`,
//! `WT/HK/10005`. These helpers convert between user-facing symbols
//! (e.g. `TSLA.US`, `700.HK`, `.DJI.US`) and counter IDs, using an
//! embedded ETF + index + warrant directory to pick the right prefix.

use std::{collections::HashSet, sync::OnceLock};

static SPECIAL_COUNTER_IDS: OnceLock<HashSet<&'static str>> = OnceLock::new();

fn special_counter_ids() -> &'static HashSet<&'static str> {
    SPECIAL_COUNTER_IDS.get_or_init(|| {
        [
            include_str!("US-ETF.csv"),
            include_str!("US-IX.csv"),
            include_str!("US-WT.csv"),
        ]
        .iter()
        .flat_map(|s| s.lines())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect()
    })
}

/// Convert a user-supplied symbol (e.g. `TSLA.US`, `700.HK`, `.DJI.US`,
/// `HSI.HK`) to a counter_id (e.g. `ST/US/TSLA`, `ST/HK/700`, `IX/US/.DJI`,
/// `IX/HK/HSI`).
///
/// Leading-dot symbols (e.g. `.DJI.US`) are US market indexes and always map
/// to `IX/`. All other symbols are checked against the embedded
/// ETF + index + warrant set; a matching entry is returned as-is. Unmatched
/// symbols default to `ST/`.
pub fn symbol_to_counter_id(symbol: &str) -> String {
    if let Some((code, market)) = symbol.rsplit_once('.') {
        let market = market.to_uppercase();
        // Leading-dot symbols are US market indexes; the dot is part of the
        // counter_id (e.g. `.DJI.US` → `IX/US/.DJI`)
        if code.starts_with('.') {
            return format!("IX/{market}/{code}");
        }
        // Strip leading zeros from numeric HK codes (e.g. `00700` → `700`).
        // Other markets keep their codes verbatim (A-share codes such as
        // `000001.SZ` have significant leading zeros).
        let code = if market == "HK" && code.chars().all(|c| c.is_ascii_digit()) {
            code.trim_start_matches('0')
        } else {
            code
        };
        // Check special counter_ids set (ETF, IX, and WT entries)
        for prefix in &["ETF", "IX", "WT"] {
            let candidate = format!("{prefix}/{market}/{code}");
            if special_counter_ids().contains(candidate.as_str()) {
                return candidate;
            }
        }
        format!("ST/{market}/{code}")
    } else {
        symbol.to_string()
    }
}

/// Convert an index symbol (e.g. `HSI.HK`) to counter_id (e.g. `IX/HK/HSI`),
/// always using the `IX/` prefix.
pub fn index_symbol_to_counter_id(symbol: &str) -> String {
    if let Some((code, market)) = symbol.rsplit_once('.') {
        format!("IX/{}/{code}", market.to_uppercase())
    } else {
        symbol.to_string()
    }
}

/// Convert a counter_id (e.g. `ST/US/TSLA`, `ETF/US/SPY`, `IX/US/.DJI`,
/// `ST/HK/700`) back to a display symbol (e.g. `TSLA.US`, `SPY.US`,
/// `.DJI.US`, `700.HK`).
///
/// US index counter IDs (`IX/US/...`) preserve the leading dot in the code
/// part (e.g. `IX/US/.DJI` → `.DJI.US`).
pub fn counter_id_to_symbol(counter_id: &str) -> String {
    let parts: Vec<&str> = counter_id.splitn(3, '/').collect();
    if parts.len() == 3 {
        format!("{}.{}", parts[2], parts[1])
    } else {
        counter_id.to_string()
    }
}

/// Whether a user-supplied symbol resolves to an ETF (e.g. `QQQ.US`,
/// `SPY.US`).
///
/// Determined by checking the embedded special counter_id set: a symbol is an
/// ETF when [`symbol_to_counter_id`] maps it to an `ETF/...` counter_id.
pub fn is_etf(symbol: &str) -> bool {
    symbol_to_counter_id(symbol).starts_with("ETF/")
}

/// serde deserializer: reads a `counter_id` string and converts it to a symbol.
pub(crate) fn deserialize_counter_id_as_symbol<'de, D>(d: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize;
    let counter_id = String::deserialize(d)?;
    Ok(counter_id_to_symbol(&counter_id))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stock_us() {
        assert_eq!(symbol_to_counter_id("TSLA.US"), "ST/US/TSLA");
    }

    #[test]
    fn stock_hk() {
        assert_eq!(symbol_to_counter_id("700.HK"), "ST/HK/700");
    }

    #[test]
    fn stock_hk_leading_zeros() {
        assert_eq!(symbol_to_counter_id("00700.HK"), "ST/HK/700");
    }

    #[test]
    fn stock_hk_leading_zeros_short() {
        assert_eq!(symbol_to_counter_id("09988.HK"), "ST/HK/9988");
    }

    #[test]
    fn stock_sz_keeps_leading_zeros() {
        assert_eq!(symbol_to_counter_id("000001.SZ"), "ST/SZ/000001");
    }

    #[test]
    fn etf_us_spy() {
        assert_eq!(symbol_to_counter_id("SPY.US"), "ETF/US/SPY");
    }

    #[test]
    fn etf_us_qqq() {
        assert_eq!(symbol_to_counter_id("QQQ.US"), "ETF/US/QQQ");
    }

    #[test]
    fn etf_us_dram() {
        assert_eq!(symbol_to_counter_id("DRAM.US"), "ETF/US/DRAM");
    }

    #[test]
    fn market_suffix_lowercase_normalised() {
        assert_eq!(symbol_to_counter_id("SPY.us"), "ETF/US/SPY");
    }

    #[test]
    fn no_dot_passthrough() {
        assert_eq!(symbol_to_counter_id("NODOT"), "NODOT");
    }

    #[test]
    fn ix_us_dji() {
        assert_eq!(symbol_to_counter_id(".DJI.US"), "IX/US/.DJI");
    }

    #[test]
    fn ix_us_vix() {
        assert_eq!(symbol_to_counter_id(".VIX.US"), "IX/US/.VIX");
    }

    #[test]
    fn ix_us_ixic() {
        assert_eq!(symbol_to_counter_id(".IXIC.US"), "IX/US/.IXIC");
    }

    #[test]
    fn ix_us_spx() {
        assert_eq!(symbol_to_counter_id(".SPX.US"), "IX/US/.SPX");
    }

    #[test]
    fn ix_hk_hsi_via_set() {
        assert_eq!(symbol_to_counter_id("HSI.HK"), "IX/HK/HSI");
    }

    #[test]
    fn wt_hk_via_set() {
        assert_eq!(symbol_to_counter_id("10005.HK"), "WT/HK/10005");
    }

    #[test]
    fn is_etf_us() {
        assert!(is_etf("QQQ.US"));
        assert!(is_etf("SPY.US"));
        assert!(is_etf("DRAM.US"));
    }

    #[test]
    fn is_etf_non_etf() {
        assert!(!is_etf("TSLA.US"));
        assert!(!is_etf("HSI.HK"));
        assert!(!is_etf("700.HK"));
    }

    #[test]
    fn index() {
        assert_eq!(index_symbol_to_counter_id("HSI.HK"), "IX/HK/HSI");
    }

    #[test]
    fn counter_id_ix_us_to_symbol() {
        assert_eq!(counter_id_to_symbol("IX/US/.DJI"), ".DJI.US");
    }

    #[test]
    fn counter_id_ix_hk_to_symbol() {
        assert_eq!(counter_id_to_symbol("IX/HK/HSI"), "HSI.HK");
    }

    #[test]
    fn roundtrip() {
        let cid = symbol_to_counter_id("TSLA.US");
        assert_eq!(counter_id_to_symbol(&cid), "TSLA.US");
    }
}
