use std::sync::Arc;

use longbridge::{
    blocking::ForexContextSync,
    forex::{GetForexQuoteOptions, SubmitForexOrderOptions},
};
use pyo3::{PyResult, exceptions::PyValueError, pyclass, pymethods};

use crate::{
    config::Config,
    error::ErrorNewType,
    forex::types::{ForexOrderDetail, ForexQuote},
};

/// Forex (currency exchange) channel context (REST-only).
#[pyclass]
pub(crate) struct ForexContext {
    ctx: ForexContextSync,
}

#[pymethods]
impl ForexContext {
    #[new]
    fn new(config: &Config) -> PyResult<Self> {
        Ok(Self {
            ctx: ForexContextSync::new(Arc::new(config.0.clone())).map_err(ErrorNewType)?,
        })
    }

    /// Get a forex quote.
    ///
    /// `from_` / `to` are ISO 4217 currency codes. Provide at most one of
    /// `amount` / `target_amount` (decimal strings, e.g. ``"1000.00"``).
    #[pyo3(signature = (from_, to, amount = None, target_amount = None))]
    fn quote(
        &self,
        from_: String,
        to: String,
        amount: Option<String>,
        target_amount: Option<String>,
    ) -> PyResult<ForexQuote> {
        let mut opts = GetForexQuoteOptions::new();
        if let Some(amount) = amount {
            opts = opts.amount(
                amount
                    .parse::<rust_decimal::Decimal>()
                    .map_err(|e| PyValueError::new_err(e.to_string()))?,
            );
        }
        if let Some(target_amount) = target_amount {
            opts = opts.target_amount(
                target_amount
                    .parse::<rust_decimal::Decimal>()
                    .map_err(|e| PyValueError::new_err(e.to_string()))?,
            );
        }
        self.ctx
            .quote(from_, to, opts)
            .map_err(ErrorNewType)?
            .try_into()
    }

    /// Submit a forex order.
    ///
    /// Success only means the order was accepted; conversion is asynchronous —
    /// poll `order` with the same `client_order_id` for the final state.
    fn submit_order(&self, quote_id: String, client_order_id: String) -> PyResult<()> {
        let opts = SubmitForexOrderOptions::new(quote_id, client_order_id);
        self.ctx.submit_order(opts).map_err(ErrorNewType)?;
        Ok(())
    }

    /// Query a forex order by `client_order_id`.
    fn order(&self, client_order_id: String) -> PyResult<ForexOrderDetail> {
        self.ctx
            .order(client_order_id)
            .map_err(ErrorNewType)?
            .try_into()
    }
}
