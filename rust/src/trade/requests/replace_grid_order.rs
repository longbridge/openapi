use serde::Serialize;

use crate::trade::GridTradeRule;

/// Options for replace grid trading order request
#[derive(Debug, Serialize, Clone)]
pub struct ReplaceGridOrderOptions {
    order_id: String,
    grid_trading_rule: GridTradeRule,
}

impl ReplaceGridOrderOptions {
    /// Create a new `ReplaceGridOrderOptions`
    #[inline]
    pub fn new(order_id: impl Into<String>, grid_trading_rule: GridTradeRule) -> Self {
        Self {
            order_id: order_id.into(),
            grid_trading_rule,
        }
    }
}
