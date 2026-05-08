mod context;
mod context_async;
pub(crate) mod types;
use pyo3::prelude::*;
pub(crate) fn register_types(parent: &Bound<PyModule>) -> PyResult<()> {
    use types::*;
    parent.add_class::<ExchangeRate>()?;
    parent.add_class::<ExchangeRates>()?;
    parent.add_class::<ProfitSummaryInfo>()?;
    parent.add_class::<ProfitSummaryBreakdown>()?;
    parent.add_class::<ProfitAnalysisSummary>()?;
    parent.add_class::<ProfitAnalysisItem>()?;
    parent.add_class::<ProfitAnalysisSublist>()?;
    parent.add_class::<ProfitAnalysis>()?;
    parent.add_class::<ProfitDetailEntry>()?;
    parent.add_class::<ProfitDetails>()?;
    parent.add_class::<ProfitAnalysisDetail>()?;
    parent.add_class::<ProfitAnalysisByMarketItem>()?;
    parent.add_class::<ProfitAnalysisByMarket>()?;
    parent.add_class::<FlowItem>()?;
    parent.add_class::<ProfitAnalysisFlows>()?;
    parent.add_class::<context::PortfolioContext>()?;
    parent.add_class::<context_async::AsyncPortfolioContext>()?;
    Ok(())
}
