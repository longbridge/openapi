//! Symbol ↔ counter_id conversion utilities.
//!
//! A `counter_id` is the internal instrument identifier used by the
//! Longbridge backend, e.g. `ST/US/TSLA`, `ETF/US/SPY`, `IX/HK/HSI`,
//! `WT/HK/10005`. These helpers convert between user-facing symbols
//! (e.g. `TSLA.US`, `700.HK`, `.DJI.US`) and counter IDs, using an
//! embedded ETF + index + warrant directory to pick the right prefix.
//!
//! The embedded directory may lag behind newly listed instruments. Entries
//! resolved remotely (see `QuoteContext::resolve_counter_ids`) are persisted
//! to a local cache file and consulted on subsequent lookups.

use std::{
    collections::HashSet,
    path::PathBuf,
    sync::{OnceLock, RwLock},
};

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

// ── remote-resolved counter_id cache ──────────────────────────────

static CACHED_COUNTER_IDS: OnceLock<RwLock<HashSet<String>>> = OnceLock::new();

#[cfg(test)]
static TEST_CACHE_DIR: OnceLock<PathBuf> = OnceLock::new();

/// Cache file path: `$LONGBRIDGE_CACHE_DIR/counter-ids.csv`, defaulting to
/// `~/.longbridge/cache/counter-ids.csv` (one counter_id per line, same
/// format as the embedded directory files).
fn cache_file_path() -> Option<PathBuf> {
    #[cfg(test)]
    if let Some(dir) = TEST_CACHE_DIR.get() {
        return Some(dir.join("counter-ids.csv"));
    }
    let dir = match std::env::var_os("LONGBRIDGE_CACHE_DIR") {
        Some(dir) => PathBuf::from(dir),
        None => {
            #[cfg(windows)]
            let home = std::env::var_os("USERPROFILE")?;
            #[cfg(not(windows))]
            let home = std::env::var_os("HOME")?;
            PathBuf::from(home).join(".longbridge").join("cache")
        }
    };
    Some(dir.join("counter-ids.csv"))
}

fn cached_counter_ids() -> &'static RwLock<HashSet<String>> {
    CACHED_COUNTER_IDS.get_or_init(|| {
        let set = cache_file_path()
            .and_then(|path| std::fs::read_to_string(path).ok())
            .map(|s| {
                s.lines()
                    .map(str::trim)
                    .filter(|line| !line.is_empty())
                    .map(ToString::to_string)
                    .collect()
            })
            .unwrap_or_default();
        RwLock::new(set)
    })
}

/// Merge remotely resolved counter IDs into the local cache (in memory and
/// on disk), so subsequent [`symbol_to_counter_id`] / [`lookup_counter_id`]
/// calls resolve them without another network round trip.
pub fn cache_counter_ids<'a>(counter_ids: impl IntoIterator<Item = &'a str>) {
    let mut set = match cached_counter_ids().write() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    let before = set.len();
    set.extend(
        counter_ids
            .into_iter()
            .map(str::trim)
            .filter(|id| !id.is_empty())
            .map(ToString::to_string),
    );
    if set.len() == before {
        return;
    }
    if let Some(path) = cache_file_path() {
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let mut lines: Vec<&str> = set.iter().map(String::as_str).collect();
        lines.sort_unstable();
        let _ = std::fs::write(path, lines.join("\n") + "\n");
    }
}

/// Look up a symbol in the local directory only (embedded special set, the
/// remote-resolved cache, and leading-dot index notation). Returns `None`
/// when the symbol is unknown locally — i.e. [`symbol_to_counter_id`] would
/// fall back to the default `ST/` prefix, which may be wrong for newly
/// listed ETFs / indexes / warrants.
pub fn lookup_counter_id(symbol: &str) -> Option<String> {
    let (code, market) = symbol.rsplit_once('.')?;
    let market = market.to_uppercase();
    if code.starts_with('.') {
        return Some(format!("IX/{market}/{code}"));
    }
    let code = if market == "HK" && code.chars().all(|c| c.is_ascii_digit()) {
        code.trim_start_matches('0')
    } else {
        code
    };
    for prefix in &["ETF", "IX", "WT"] {
        let candidate = format!("{prefix}/{market}/{code}");
        if special_counter_ids().contains(candidate.as_str()) {
            return Some(candidate);
        }
    }
    let cached = match cached_counter_ids().read() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    for prefix in &["ETF", "IX", "WT", "ST"] {
        let candidate = format!("{prefix}/{market}/{code}");
        if cached.contains(candidate.as_str()) {
            return Some(candidate);
        }
    }
    None
}

/// Convert a user-supplied symbol (e.g. `TSLA.US`, `700.HK`, `.DJI.US`,
/// `HSI.HK`) to a counter_id (e.g. `ST/US/TSLA`, `ST/HK/700`, `IX/US/.DJI`,
/// `IX/HK/HSI`).
///
/// Leading-dot symbols (e.g. `.DJI.US`) are US market indexes and always map
/// to `IX/`. All other symbols are checked against the embedded
/// ETF + index + warrant set and the remote-resolved cache; a matching entry
/// is returned as-is. Crypto symbols in `PAIR.EXCHANGE` format (non-standard
/// market suffixes such as `HAS`) are converted to `VA/{EXCHANGE}/{PAIR}`.
/// Unmatched symbols default to `ST/`.
pub fn symbol_to_counter_id(symbol: &str) -> String {
    if let Some((code, market)) = symbol.rsplit_once('.') {
        if let Some(counter_id) = lookup_counter_id(symbol) {
            return counter_id;
        }
        let market_upper = market.to_uppercase();
        // Non-standard market suffixes are crypto exchange identifiers.
        match market_upper.as_str() {
            "US" | "HK" | "CN" | "SH" | "SZ" | "A" | "B" => {}
            _ => return format!("VA/{market_upper}/{code}"),
        }
        // Strip leading zeros from numeric HK codes (e.g. `00700` → `700`).
        let code = if market_upper == "HK" && code.chars().all(|c| c.is_ascii_digit()) {
            code.trim_start_matches('0')
        } else {
            code
        };
        format!("ST/{market_upper}/{code}")
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

/// Convert a counter_id back to a display symbol.
///
/// - `ST/US/TSLA`    → `TSLA.US`
/// - `ETF/US/SPY`    → `SPY.US`
/// - `IX/US/.DJI`    → `.DJI.US`
/// - `VA/HAS/BTCUSD` → `BTCUSD.HAS`  (crypto: `PAIR.EXCHANGE`)
pub fn counter_id_to_symbol(counter_id: &str) -> String {
    let parts: Vec<&str> = counter_id.splitn(3, '/').collect();
    if parts.len() == 3 {
        let (prefix, market, code) = (parts[0], parts[1], parts[2]);
        if prefix.eq_ignore_ascii_case("VA") {
            // Crypto: return PAIR.EXCHANGE format
            return format!("{code}.{market}");
        }
        format!("{code}.{market}")
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

    #[test]
    fn cached_counter_ids_roundtrip() {
        let dir = std::env::temp_dir().join("lb-counter-cache-test");
        // Redirect the cache file away from the real user cache directory.
        let dir = TEST_CACHE_DIR.get_or_init(|| dir).clone();

        // Unknown symbol falls back to ST/ before caching
        assert_eq!(lookup_counter_id("FAKE9.US"), None);
        assert_eq!(symbol_to_counter_id("FAKE9.US"), "ST/US/FAKE9");

        // After caching remote-resolved entries, lookups return them —
        // including backend-confirmed ST/ entries
        cache_counter_ids(["ETF/US/FAKE9", "ST/US/FAKE8"]);
        assert_eq!(
            lookup_counter_id("FAKE9.US").as_deref(),
            Some("ETF/US/FAKE9")
        );
        assert_eq!(symbol_to_counter_id("FAKE9.US"), "ETF/US/FAKE9");
        assert_eq!(
            lookup_counter_id("FAKE8.US").as_deref(),
            Some("ST/US/FAKE8")
        );

        // Persisted to disk as one counter_id per line
        let saved = std::fs::read_to_string(dir.join("counter-ids.csv")).unwrap();
        assert_eq!(saved, "ETF/US/FAKE9\nST/US/FAKE8\n");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn lookup_known_special() {
        assert_eq!(lookup_counter_id("QQQ.US").as_deref(), Some("ETF/US/QQQ"));
        assert_eq!(lookup_counter_id("HSI.HK").as_deref(), Some("IX/HK/HSI"));
        assert_eq!(lookup_counter_id(".DJI.US").as_deref(), Some("IX/US/.DJI"));
        assert_eq!(lookup_counter_id("TSLA.US"), None);
        assert_eq!(lookup_counter_id("NODOT"), None);
    }
}
