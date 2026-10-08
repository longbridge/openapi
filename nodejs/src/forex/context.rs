use std::sync::Arc;

use napi::{Error, Result, Status};

use crate::{
    config::Config,
    error::ErrorNewType,
    forex::types::{ForexOrderDetail, ForexQuote},
};

/// Forex (currency exchange) channel context.
#[napi_derive::napi]
#[derive(Clone)]
pub struct ForexContext {
    ctx: longbridge::forex::ForexContext,
}

#[napi_derive::napi]
impl ForexContext {
    /// Create a new `ForexContext`.
    #[napi]
    pub fn new(config: &Config) -> ForexContext {
        Self {
            ctx: longbridge::forex::ForexContext::new(Arc::new(config.0.clone())),
        }
    }

    /// Get a forex quote.
    ///
    /// `from` / `to` are ISO 4217 currency codes. Provide at most one of
    /// `amount` / `target_amount` (decimal strings, e.g. `"1000.00"`).
    #[napi]
    pub async fn quote(
        &self,
        from: String,
        to: String,
        amount: Option<String>,
        target_amount: Option<String>,
    ) -> Result<ForexQuote> {
        let mut options = longbridge::forex::GetForexQuoteOptions::new();
        if let Some(amount) = amount {
            options = options.amount(
                amount
                    .parse::<rust_decimal::Decimal>()
                    .map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?,
            );
        }
        if let Some(target_amount) = target_amount {
            options = options.target_amount(
                target_amount
                    .parse::<rust_decimal::Decimal>()
                    .map_err(|e| Error::new(Status::InvalidArg, e.to_string()))?,
            );
        }
        self.ctx
            .quote(from, to, options)
            .await
            .map_err(ErrorNewType)?
            .try_into()
    }

    /// Submit a forex order.
    ///
    /// Success only means the order was accepted; conversion is asynchronous —
    /// poll `order` with the same `client_order_id` for the final state.
    #[napi]
    pub async fn submit_order(&self, quote_id: String, client_order_id: String) -> Result<()> {
        let options = longbridge::forex::SubmitForexOrderOptions::new(quote_id, client_order_id);
        self.ctx.submit_order(options).await.map_err(ErrorNewType)?;
        Ok(())
    }

    /// Query a forex order by `client_order_id`.
    #[napi]
    pub async fn order(&self, client_order_id: String) -> Result<ForexOrderDetail> {
        self.ctx
            .order(client_order_id)
            .await
            .map_err(ErrorNewType)?
            .try_into()
    }
}
