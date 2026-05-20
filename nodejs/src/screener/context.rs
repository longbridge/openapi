use std::sync::Arc;

use napi::Result;

use crate::{config::Config, error::ErrorNewType, screener::types::*};

/// Screener context
#[napi_derive::napi]
#[derive(Clone)]
pub struct ScreenerContext {
    ctx: longbridge::ScreenerContext,
}

#[napi_derive::napi]
impl ScreenerContext {
    /// Create a new `ScreenerContext`
    #[napi]
    pub fn new(config: &Config) -> ScreenerContext {
        Self {
            ctx: longbridge::ScreenerContext::new(Arc::new(config.0.clone())),
        }
    }

    /// Get recommended built-in screener strategies
    #[napi]
    pub async fn screener_recommend_strategies(
        &self,
    ) -> Result<ScreenerRecommendStrategiesResponse> {
        Ok(self
            .ctx
            .screener_recommend_strategies()
            .await
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get the current user's saved screener strategies
    #[napi]
    pub async fn screener_user_strategies(&self) -> Result<ScreenerUserStrategiesResponse> {
        Ok(self
            .ctx
            .screener_user_strategies()
            .await
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get detail for one screener strategy by ID
    #[napi]
    pub async fn screener_strategy(&self, id: i64) -> Result<ScreenerStrategyResponse> {
        Ok(self
            .ctx
            .screener_strategy(id)
            .await
            .map_err(ErrorNewType)?
            .into())
    }

    /// Search / screen securities using a strategy
    #[napi]
    pub async fn screener_search(
        &self,
        market: String,
        strategy_id: Option<i64>,
        page: u32,
        size: u32,
    ) -> Result<ScreenerSearchResponse> {
        Ok(self
            .ctx
            .screener_search(market, strategy_id, page, size)
            .await
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get all available screener indicator definitions
    #[napi]
    pub async fn screener_indicators(&self) -> Result<ScreenerIndicatorsResponse> {
        Ok(self
            .ctx
            .screener_indicators()
            .await
            .map_err(ErrorNewType)?
            .into())
    }
}
