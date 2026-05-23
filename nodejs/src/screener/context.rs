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
        market: String,
    ) -> Result<ScreenerRecommendStrategiesResponse> {
        Ok(self
            .ctx
            .screener_recommend_strategies(market)
            .await
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get the current user's saved screener strategies
    #[napi]
    pub async fn screener_user_strategies(
        &self,
        market: String,
    ) -> Result<ScreenerUserStrategiesResponse> {
        Ok(self
            .ctx
            .screener_user_strategies(market)
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

    /// Search / screen securities using a strategy or custom conditions.
    ///
    /// When `strategyId` is given (Mode A), the strategy is fetched from the AI
    /// endpoint and its filters drive the search.  The market is taken from the
    /// strategy response.
    ///
    /// When `strategyId` is `null` / `undefined` (Mode B), `conditions` must be
    /// `"KEY:MIN:MAX"` strings and `market` is used directly.
    ///
    /// `filter_` is stripped from every `items[].indicators[].key` in the
    /// response before it is returned.
    #[napi]
    pub async fn screener_search(
        &self,
        market: String,
        strategy_id: Option<i64>,
        #[napi(ts_arg_type = "string[]")] conditions: Vec<String>,
        #[napi(ts_arg_type = "string[]")] show: Vec<String>,
        page: u32,
        size: u32,
    ) -> Result<ScreenerSearchResponse> {
        Ok(self
            .ctx
            .screener_search(market, strategy_id, conditions, show, page, size)
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
