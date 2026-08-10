use crate::trade::requests::submit_grid_order::GridTradeRule;

/// Options for replace grid trading order request
#[napi_derive::napi(object)]
pub struct ReplaceGridOrderOptions<'env> {
    /// Grid master order id
    pub order_id: String,
    /// Grid trading rule
    pub grid_trading_rule: GridTradeRule<'env>,
}

impl<'env> From<ReplaceGridOrderOptions<'env>> for longbridge::trade::ReplaceGridOrderOptions {
    #[inline]
    fn from(opts: ReplaceGridOrderOptions<'env>) -> Self {
        longbridge::trade::ReplaceGridOrderOptions::new(
            opts.order_id,
            opts.grid_trading_rule.into(),
        )
    }
}
