mod context;
pub(crate) mod types;

use pyo3::prelude::*;

pub(crate) fn register_types(parent: &Bound<PyModule>) -> PyResult<()> {
    use types::*;
    parent.add_class::<FundNavValue>()?;
    parent.add_class::<FundPerformancePoint>()?;
    parent.add_class::<HotFund>()?;
    parent.add_class::<FundBrief>()?;
    parent.add_class::<FundFilters>()?;
    parent.add_class::<FundAssetAllocationItem>()?;
    parent.add_class::<FundAssetAllocation>()?;
    parent.add_class::<FundDetail>()?;
    parent.add_class::<FundAnalysis>()?;
    parent.add_class::<FundAnalysisDetail>()?;
    parent.add_class::<FundTrendContrast>()?;
    parent.add_class::<FundTrend>()?;
    parent.add_class::<FundNamedContrast>()?;
    parent.add_class::<FundPerformanceComparison>()?;
    parent.add_class::<FundAnnualReturn>()?;
    parent.add_class::<FundQuarterlyReturn>()?;
    parent.add_class::<FundPerformance>()?;
    parent.add_class::<FundHolding>()?;
    parent.add_class::<FundHoldings>()?;
    parent.add_class::<FundStockHolding>()?;
    parent.add_class::<FundPosition>()?;
    parent.add_class::<FundPositions>()?;
    parent.add_class::<FundDatedValue>()?;
    parent.add_class::<FundUnitValue>()?;
    parent.add_class::<FundPositionDetailValues>()?;
    parent.add_class::<FundPositionDetail>()?;
    parent.add_class::<FundPositionPerformance>()?;
    parent.add_class::<FundPositionProfits>()?;
    parent.add_class::<FundPositionNav>()?;
    parent.add_class::<FundDividend>()?;
    parent.add_class::<FundDividends>()?;
    parent.add_class::<FundOrder>()?;
    parent.add_class::<FundOrderKeyword>()?;
    parent.add_class::<FundOrderStage>()?;
    parent.add_class::<FundOrderInfo>()?;
    parent.add_class::<FundOrderDetail>()?;
    parent.add_class::<FundTransaction>()?;
    parent.add_class::<FundOrderValidation>()?;
    parent.add_class::<FundOrderSubmitResponse>()?;
    parent.add_class::<context::FundContext>()?;
    Ok(())
}
