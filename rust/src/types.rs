use strum_macros::{Display, EnumString};

/// Market
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, EnumString, Display)]
pub enum Market {
    /// Unknown
    #[strum(disabled)]
    Unknown,
    /// US market
    US,
    /// HK market
    HK,
    /// CN market
    CN,
    /// SG market
    SG,
}

impl_default_for_enum_string!(Market);
impl_serde_for_enum_string!(Market);

impl From<longport_candlesticks::Market> for Market {
    fn from(market: longport_candlesticks::Market) -> Self {
        match market {
            longport_candlesticks::Market::HK => Market::HK,
            longport_candlesticks::Market::US => Market::US,
            longport_candlesticks::Market::SH | longport_candlesticks::Market::SZ => Market::CN,
            longport_candlesticks::Market::SG => Market::SG,
        }
    }
}
