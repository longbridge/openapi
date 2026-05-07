//! Extension methods for [`AssetContext`].

use longbridge_httpcli::{Json, Method};
use serde::{Deserialize, Serialize};
use tracing::instrument::WithSubscriber;

use crate::{Result, asset::AssetContext};

/// A single exchange-rate entry returned by `/v1/asset/exchange_rates`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeRate {
    /// Source currency code.
    pub from_currency: String,
    /// Target currency code.
    pub to_currency: String,
    /// Exchange rate value.
    pub rate: String,
}

impl AssetContext {
    /// Get all exchange rates.
    ///
    /// Path: GET /v1/asset/exchange_rates
    pub async fn exchange_rates(&self) -> Result<serde_json::Value> {
        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/asset/exchange_rates")
            .response::<Json<serde_json::Value>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0)
    }
}
