use std::sync::Arc;

use tokio::sync::mpsc;

use crate::{
    Config, Result,
    blocking::runtime::BlockingRuntime,
    screener::{
        ScreenerContext,
        types::{
            ScreenerIndicatorsResponse, ScreenerRecommendStrategiesResponse,
            ScreenerSearchResponse, ScreenerStrategyResponse, ScreenerUserStrategiesResponse,
        },
    },
};

/// Blocking screener context
pub struct ScreenerContextSync {
    rt: BlockingRuntime<ScreenerContext>,
}

impl ScreenerContextSync {
    /// Create a [`ScreenerContextSync`]
    pub fn new(config: Arc<Config>) -> Result<Self> {
        let rt = BlockingRuntime::try_new(
            move || {
                let ctx = ScreenerContext::new(config);
                let (tx, rx) = mpsc::unbounded_channel::<std::convert::Infallible>();
                std::mem::forget(tx);
                Ok::<_, crate::Error>((ctx, rx))
            },
            |_: std::convert::Infallible| {},
        )?;
        Ok(Self { rt })
    }

    /// Get recommended built-in screener strategies
    pub fn screener_recommend_strategies(
        &self,
        market: impl Into<String> + Send + 'static,
    ) -> Result<ScreenerRecommendStrategiesResponse> {
        self.rt
            .call(|ctx| async move { ctx.screener_recommend_strategies(market).await })
    }

    /// Get the current user's saved screener strategies
    pub fn screener_user_strategies(
        &self,
        market: impl Into<String> + Send + 'static,
    ) -> Result<ScreenerUserStrategiesResponse> {
        self.rt
            .call(|ctx| async move { ctx.screener_user_strategies(market).await })
    }

    /// Get detail for one screener strategy by ID
    pub fn screener_strategy(&self, id: i64) -> Result<ScreenerStrategyResponse> {
        self.rt
            .call(move |ctx| async move { ctx.screener_strategy(id).await })
    }

    /// Search / screen securities using a strategy
    pub fn screener_search(
        &self,
        market: impl Into<String> + Send + 'static,
        strategy_id: Option<i64>,
        conditions: Vec<crate::screener::ScreenerCondition>,
        show: Vec<String>,
        page: u32,
        size: u32,
    ) -> Result<ScreenerSearchResponse> {
        self.rt.call(move |ctx| async move {
            ctx.screener_search(market, strategy_id, conditions, show, page, size)
                .await
        })
    }

    /// Get all available screener indicator definitions
    pub fn screener_indicators(&self) -> Result<ScreenerIndicatorsResponse> {
        self.rt
            .call(|ctx| async move { ctx.screener_indicators().await })
    }
}
