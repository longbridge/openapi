use std::{collections::HashSet, sync::OnceLock};

static US_ETF_SET: OnceLock<HashSet<&'static str>> = OnceLock::new();

fn us_etf_set() -> &'static HashSet<&'static str> {
    US_ETF_SET.get_or_init(|| {
        include_str!("US-ETF.csv")
            .lines()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .collect()
    })
}

/// Convert a symbol (e.g. `TSLA.US`) to a counter_id (e.g. `ST/US/TSLA`).
/// US ETFs are detected via embedded list.
pub(crate) fn symbol_to_counter_id(symbol: &str) -> String {
    if let Some((code, market)) = symbol.rsplit_once('.') {
        let market = market.to_uppercase();
        let etf_candidate = format!("ETF/{market}/{code}");
        if us_etf_set().contains(etf_candidate.as_str()) {
            etf_candidate
        } else {
            format!("ST/{market}/{code}")
        }
    } else {
        symbol.to_string()
    }
}

/// Convert an index symbol (e.g. `HSI.HK`) to counter_id (e.g. `IX/HK/HSI`).
pub(crate) fn index_symbol_to_counter_id(symbol: &str) -> String {
    if let Some((code, market)) = symbol.rsplit_once('.') {
        format!("IX/{}/{code}", market.to_uppercase())
    } else {
        symbol.to_string()
    }
}

/// Convert a counter_id (e.g. `ST/US/TSLA`) back to a symbol (e.g. `TSLA.US`).
pub(crate) fn counter_id_to_symbol(counter_id: &str) -> String {
    let parts: Vec<&str> = counter_id.splitn(3, '/').collect();
    if parts.len() == 3 {
        format!("{}.{}", parts[2], parts[1])
    } else {
        counter_id.to_string()
    }
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
    fn etf_us() {
        assert_eq!(symbol_to_counter_id("SPY.US"), "ETF/US/SPY");
    }

    #[test]
    fn index() {
        assert_eq!(index_symbol_to_counter_id("HSI.HK"), "IX/HK/HSI");
    }

    #[test]
    fn roundtrip() {
        let cid = symbol_to_counter_id("TSLA.US");
        assert_eq!(counter_id_to_symbol(&cid), "TSLA.US");
    }
}
