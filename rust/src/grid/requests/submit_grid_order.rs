use serde::Serialize;

use crate::grid::GridTradeRule;

/// Options for submit grid trading order request
#[derive(Debug, Serialize, Clone)]
pub struct SubmitGridOrderOptions {
    symbol: String,
    settlement_currency: String,
    grid_trading_rule: GridTradeRule,
}

impl SubmitGridOrderOptions {
    /// Create a new `SubmitGridOrderOptions`
    #[inline]
    pub fn new(
        symbol: impl Into<String>,
        settlement_currency: impl Into<String>,
        grid_trading_rule: GridTradeRule,
    ) -> Self {
        Self {
            symbol: symbol.into(),
            settlement_currency: settlement_currency.into(),
            grid_trading_rule,
        }
    }
}
