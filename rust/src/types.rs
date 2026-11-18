use strum_macros::{Display, EnumString};

/// Market
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, EnumString, Display)]
pub enum Market {
    /// Unknown
    Unknown,
    /// US market
    US,
    /// HK market
    HK,
    /// CN market
    CN,
    /// SG market
    SG,
    /// Crypto market
    Crypto,
}

impl_default_for_enum_string!(Market);
impl_serde_for_enum_string!(Market);

/// Canonicalize a security symbol for the Longbridge gateway.
///
/// HK symbols are carried without the leading zeros the Longbridge apps
/// display, so `00700.HK` becomes `700.HK`. Symbols of every other market are
/// returned unchanged.
///
/// The padded form is not handled consistently. Measured against the gateway:
/// `/v1/asset/stock?symbol=00700.HK` answers `200` with **no** positions
/// instead of an error, while `700.HK` returns the position. The quote
/// subscription path has always trimmed the symbol itself, and
/// longbridge/developers#789 reports that the quote HTTP endpoints accept the
/// padded form — that last part is the reporter's observation, not something
/// measured here. Normalizing on the way out makes both spellings behave the
/// same in every SDK call that goes through these builders.
pub(crate) fn normalize_symbol(symbol: &str) -> &str {
    match symbol.split_once('.') {
        Some((code, market)) if market.eq_ignore_ascii_case("HK") => {
            let trimmed = code.trim_start_matches('0');
            // An all-zero code would trim away to nothing; leave such a symbol
            // alone rather than sending a bare `.HK`.
            if trimmed.is_empty() {
                symbol
            } else {
                &symbol[code.len() - trimmed.len()..]
            }
        }
        _ => symbol,
    }
}

/// [`normalize_symbol`] for an owned symbol, reusing the allocation when
/// nothing had to be trimmed.
pub(crate) fn normalize_symbol_owned(symbol: impl Into<String>) -> String {
    let symbol = symbol.into();
    let normalized = normalize_symbol(&symbol);
    if normalized.len() == symbol.len() {
        symbol
    } else {
        normalized.to_string()
    }
}

#[cfg(test)]
mod normalize_symbol_tests {
    use super::{normalize_symbol, normalize_symbol_owned};

    #[test]
    fn strips_leading_zeros_for_hk_only() {
        assert_eq!(normalize_symbol("00700.HK"), "700.HK");
        assert_eq!(normalize_symbol("700.HK"), "700.HK");
        assert_eq!(normalize_symbol("00700.hk"), "700.hk");
        // Other markets keep their symbol verbatim, zeros and all.
        assert_eq!(normalize_symbol("AAPL.US"), "AAPL.US");
        assert_eq!(normalize_symbol("000001.SZ"), "000001.SZ");
        assert_eq!(normalize_symbol("0001.SG"), "0001.SG");
        // No market suffix at all.
        assert_eq!(normalize_symbol("700"), "700");
        assert_eq!(normalize_symbol(""), "");
    }

    #[test]
    fn owned_variant_matches_the_borrowed_one() {
        for s in ["00700.HK", "700.HK", "AAPL.US", "000001.SZ", "0.HK", ""] {
            assert_eq!(normalize_symbol_owned(s), normalize_symbol(s));
        }
    }

    #[test]
    fn leaves_an_all_zero_code_alone() {
        assert_eq!(normalize_symbol("0.HK"), "0.HK");
        assert_eq!(normalize_symbol("000.HK"), "000.HK");
        assert_eq!(normalize_symbol(".HK"), ".HK");
    }
}
