use std::sync::Arc;

use napi::Result;

use crate::{config::Config, error::ErrorNewType, portfolio::types::*};

/// Portfolio analytics context — exchange rates and P&L analysis.
#[napi_derive::napi]
#[derive(Clone)]
pub struct PortfolioContext {
    ctx: longbridge::PortfolioContext,
}

#[napi_derive::napi]
impl PortfolioContext {
    /// Create a new PortfolioContext.
    #[napi]
    pub fn new(config: &Config) -> PortfolioContext {
        Self {
            ctx: longbridge::PortfolioContext::new(Arc::new(config.0.clone())),
        }
    }

    /// Get exchange rates for supported currencies.
    #[napi]
    pub async fn exchange_rate(&self) -> Result<ExchangeRates> {
        Ok(self.ctx.exchange_rate().await.map_err(ErrorNewType)?.into())
    }

    /// Get portfolio P&L analysis (summary + per-security breakdown).
    ///
    /// `start` and `end` are optional date strings in `YYYY-MM-DD` format.
    #[napi]
    pub async fn profit_analysis(
        &self,
        start: Option<String>,
        end: Option<String>,
    ) -> Result<ProfitAnalysis> {
        Ok(self
            .ctx
            .profit_analysis(start, end)
            .await
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get P&L detail for a specific security.
    ///
    /// `start` and `end` are optional date strings in `YYYY-MM-DD` format.
    #[napi]
    pub async fn profit_analysis_detail(
        &self,
        symbol: String,
        start: Option<String>,
        end: Option<String>,
    ) -> Result<ProfitAnalysisDetail> {
        Ok(self
            .ctx
            .profit_analysis_detail(symbol, start, end)
            .await
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get paginated P&L analysis grouped by market.
    ///
    /// All filter parameters are optional. `page` is 1-based (default 1);
    /// `size` controls the page size (default 20).
    /// `start` and `end` are optional date strings in `YYYY-MM-DD` format.
    #[napi]
    pub async fn profit_analysis_by_market(
        &self,
        market: Option<String>,
        start: Option<String>,
        end: Option<String>,
        currency: Option<String>,
        page: u32,
        size: u32,
    ) -> Result<ProfitAnalysisByMarket> {
        Ok(self
            .ctx
            .profit_analysis_by_market(market, start, end, currency, page, size)
            .await
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get paginated P&L flow records for a security.
    ///
    /// `start` and `end` are optional date strings in `YYYY-MM-DD` format.
    #[napi]
    pub async fn profit_analysis_flows(
        &self,
        symbol: String,
        page: u32,
        size: u32,
        derivative: bool,
        start: Option<String>,
        end: Option<String>,
    ) -> Result<ProfitAnalysisFlows> {
        Ok(self
            .ctx
            .profit_analysis_flows(symbol, page, size, derivative, start, end)
            .await
            .map_err(ErrorNewType)?
            .into())
    }
}
