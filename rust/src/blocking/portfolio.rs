use std::sync::Arc;

use tokio::sync::mpsc;

use crate::{
    Config, Result,
    blocking::runtime::BlockingRuntime,
    portfolio::{PortfolioContext, types::*},
};

/// Blocking portfolio analytics context
pub struct PortfolioContextSync {
    rt: BlockingRuntime<PortfolioContext>,
}

impl PortfolioContextSync {
    /// Create a [`PortfolioContextSync`]
    pub fn new(config: Arc<Config>) -> Result<Self> {
        let rt = BlockingRuntime::try_new(
            move || {
                let ctx = PortfolioContext::new(config);
                let (tx, rx) = mpsc::unbounded_channel::<std::convert::Infallible>();
                std::mem::forget(tx);
                Ok::<_, crate::Error>((ctx, rx))
            },
            |_: std::convert::Infallible| {},
        )?;
        Ok(Self { rt })
    }

    /// Get exchange rates
    pub fn exchange_rate(&self) -> Result<ExchangeRates> {
        self.rt.call(|ctx| async move { ctx.exchange_rate().await })
    }

    /// Get portfolio P&L analysis
    pub fn profit_analysis(
        &self,
        start: Option<String>,
        end: Option<String>,
    ) -> Result<ProfitAnalysis> {
        self.rt
            .call(move |ctx| async move { ctx.profit_analysis(start, end).await })
    }

    /// Get P&L detail for a specific security
    pub fn profit_analysis_detail(
        &self,
        symbol: impl Into<String> + Send + 'static,
        start: Option<String>,
        end: Option<String>,
    ) -> Result<ProfitAnalysisDetail> {
        self.rt
            .call(move |ctx| async move { ctx.profit_analysis_detail(symbol, start, end).await })
    }

    /// Get paginated P&L analysis filtered by market
    pub fn profit_analysis_by_market(
        &self,
        market: Option<String>,
        start: Option<String>,
        end: Option<String>,
        currency: Option<String>,
        page: u32,
        size: u32,
    ) -> Result<ProfitAnalysisByMarket> {
        self.rt.call(move |ctx| async move {
            ctx.profit_analysis_by_market(market, start, end, currency, page, size)
                .await
        })
    }

    /// Get paginated P&L flow records for a security
    #[allow(clippy::too_many_arguments)]
    pub fn profit_analysis_flows(
        &self,
        symbol: impl Into<String> + Send + 'static,
        page: u32,
        size: u32,
        derivative: bool,
        start: Option<String>,
        end: Option<String>,
    ) -> Result<ProfitAnalysisFlows> {
        self.rt.call(move |ctx| async move {
            ctx.profit_analysis_flows(symbol, page, size, derivative, start, end)
                .await
        })
    }
}
