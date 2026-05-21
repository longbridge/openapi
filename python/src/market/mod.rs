mod context;
mod context_async;
pub(crate) mod types;

use pyo3::prelude::*;

pub(crate) fn register_types(parent: &Bound<PyModule>) -> PyResult<()> {
    use types::*;
    parent.add_class::<MarketStatusResponse>()?;
    parent.add_class::<MarketTimeItem>()?;
    parent.add_class::<BrokerHoldingTop>()?;
    parent.add_class::<BrokerHoldingEntry>()?;
    parent.add_class::<BrokerHoldingDetail>()?;
    parent.add_class::<BrokerHoldingDetailItem>()?;
    parent.add_class::<BrokerHoldingChanges>()?;
    parent.add_class::<BrokerHoldingDailyHistory>()?;
    parent.add_class::<BrokerHoldingDailyItem>()?;
    parent.add_class::<AhPremiumKlines>()?;
    parent.add_class::<AhPremiumIntraday>()?;
    parent.add_class::<AhPremiumKline>()?;
    parent.add_class::<TradeStatsResponse>()?;
    parent.add_class::<TradeStatistics>()?;
    parent.add_class::<TradePriceLevel>()?;
    parent.add_class::<AnomalyResponse>()?;
    parent.add_class::<AnomalyItem>()?;
    parent.add_class::<IndexConstituents>()?;
    parent.add_class::<ConstituentStock>()?;
    parent.add_class::<BrokerHoldingPeriod>()?;
    parent.add_class::<AhPremiumPeriod>()?;
    parent.add_class::<TopMoversResponse>()?;
    parent.add_class::<RankCategoriesResponse>()?;
    parent.add_class::<RankListResponse>()?;
    parent.add_class::<context::MarketContext>()?;
    parent.add_class::<context_async::AsyncMarketContext>()?;
    Ok(())
}
