//! Extension methods for [`TradeContext`] – profit analysis.

use longbridge_httpcli::{Json, Method};
use tracing::instrument::WithSubscriber;

use crate::{
    Result,
    trade::{
        TradeContext,
        extra_types::{
            ProfitAnalysisDetailOptions, ProfitAnalysisSummaryOptions,
            ProfitAnalysisSublistOptions,
        },
    },
};

impl TradeContext {
    /// Get profit analysis summary.
    ///
    /// Path: GET /v1/portfolio/profit-analysis-summary
    pub async fn profit_analysis_summary(
        &self,
        opts: ProfitAnalysisSummaryOptions,
    ) -> Result<serde_json::Value> {
        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/portfolio/profit-analysis-summary")
            .query_params(opts)
            .response::<Json<serde_json::Value>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0)
    }

    /// Get profit analysis sub-list.
    ///
    /// Path: GET /v1/portfolio/profit-analysis-sublist
    pub async fn profit_analysis_sublist(
        &self,
        opts: ProfitAnalysisSublistOptions,
    ) -> Result<serde_json::Value> {
        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/portfolio/profit-analysis-sublist")
            .query_params(opts)
            .response::<Json<serde_json::Value>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0)
    }

    /// Get profit analysis detail for a symbol.
    ///
    /// Path: GET /v1/portfolio/profit-analysis/detail
    pub async fn profit_analysis_detail(
        &self,
        opts: ProfitAnalysisDetailOptions,
    ) -> Result<serde_json::Value> {
        Ok(self
            .0
            .http_cli
            .request(Method::GET, "/v1/portfolio/profit-analysis/detail")
            .query_params(opts)
            .response::<Json<serde_json::Value>>()
            .send()
            .with_subscriber(self.0.log_subscriber.clone())
            .await?
            .0)
    }
}
