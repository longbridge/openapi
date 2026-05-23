use std::sync::Arc;

use longbridge::blocking::ScreenerContextSync;
use pyo3::prelude::*;

use crate::{config::Config, error::ErrorNewType, screener::types::*};

/// Screener context (synchronous).
#[pyclass]
pub(crate) struct ScreenerContext {
    ctx: ScreenerContextSync,
}

#[pymethods]
impl ScreenerContext {
    #[new]
    fn new(config: &Config) -> PyResult<Self> {
        Ok(Self {
            ctx: ScreenerContextSync::new(Arc::new(config.0.clone())).map_err(ErrorNewType)?,
        })
    }

    /// Get recommended built-in screener strategies.
    fn screener_recommend_strategies(
        &self,
        market: String,
    ) -> PyResult<ScreenerRecommendStrategiesResponse> {
        Ok(self
            .ctx
            .screener_recommend_strategies(market)
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get the current user's saved screener strategies.
    fn screener_user_strategies(&self, market: String) -> PyResult<ScreenerUserStrategiesResponse> {
        Ok(self
            .ctx
            .screener_user_strategies(market)
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get detail for one screener strategy by ID.
    fn screener_strategy(&self, id: i64) -> PyResult<ScreenerStrategyResponse> {
        Ok(self.ctx.screener_strategy(id).map_err(ErrorNewType)?.into())
    }

    /// Search / screen securities using a strategy.
    #[pyo3(signature = (market, strategy_id = None, conditions = vec![], show = vec![], page = 0, size = 20))]
    fn screener_search(
        &self,
        market: String,
        strategy_id: Option<i64>,
        conditions: Vec<ScreenerCondition>,
        show: Vec<String>,
        page: u32,
        size: u32,
    ) -> PyResult<ScreenerSearchResponse> {
        let lb_conditions: Vec<longbridge::screener::ScreenerCondition> =
            conditions.into_iter().map(Into::into).collect();
        Ok(self
            .ctx
            .screener_search(market, strategy_id, lb_conditions, show, page, size)
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get all available screener indicator definitions.
    fn screener_indicators(&self) -> PyResult<ScreenerIndicatorsResponse> {
        Ok(self.ctx.screener_indicators().map_err(ErrorNewType)?.into())
    }
}
