use std::sync::Arc;

use longbridge::blocking::MarketContextSync;
use pyo3::prelude::*;

use crate::{config::Config, error::ErrorNewType, market::types::*};

/// Market data context (synchronous).
#[pyclass]
pub(crate) struct MarketContext {
    ctx: MarketContextSync,
}

#[pymethods]
impl MarketContext {
    #[new]
    fn new(config: &Config) -> PyResult<Self> {
        Ok(Self {
            ctx: MarketContextSync::new(Arc::new(config.0.clone())).map_err(ErrorNewType)?,
        })
    }

    /// Get current trading status for all markets.
    fn market_status(&self) -> PyResult<MarketStatusResponse> {
        Ok(self.ctx.market_status().map_err(ErrorNewType)?.into())
    }

    /// Get top broker holdings for a security.
    #[pyo3(signature = (symbol, period = BrokerHoldingPeriod::Rct1))]
    fn broker_holding(
        &self,
        symbol: String,
        period: BrokerHoldingPeriod,
    ) -> PyResult<BrokerHoldingTop> {
        Ok(self
            .ctx
            .broker_holding(symbol, period.into())
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get full broker holding details.
    fn broker_holding_detail(&self, symbol: String) -> PyResult<BrokerHoldingDetail> {
        Ok(self
            .ctx
            .broker_holding_detail(symbol)
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get daily holding history for a specific broker.
    fn broker_holding_daily(
        &self,
        symbol: String,
        broker_id: String,
    ) -> PyResult<BrokerHoldingDailyHistory> {
        Ok(self
            .ctx
            .broker_holding_daily(symbol, broker_id)
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get A/H premium K-line data.
    #[pyo3(signature = (symbol, period = AhPremiumPeriod::Day, count = 100))]
    fn ah_premium(
        &self,
        symbol: String,
        period: AhPremiumPeriod,
        count: u32,
    ) -> PyResult<AhPremiumKlines> {
        Ok(self
            .ctx
            .ah_premium(symbol, period.into(), count)
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get A/H premium intraday data.
    fn ah_premium_intraday(&self, symbol: String) -> PyResult<AhPremiumIntraday> {
        Ok(self
            .ctx
            .ah_premium_intraday(symbol)
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get buy/sell/neutral trade statistics.
    fn trade_stats(&self, symbol: String) -> PyResult<TradeStatsResponse> {
        Ok(self.ctx.trade_stats(symbol).map_err(ErrorNewType)?.into())
    }

    /// Get market anomaly alerts.
    fn anomaly(&self, market: String) -> PyResult<AnomalyResponse> {
        Ok(self.ctx.anomaly(market).map_err(ErrorNewType)?.into())
    }

    /// Get index constituent stocks.
    fn constituent(&self, symbol: String) -> PyResult<IndexConstituents> {
        Ok(self.ctx.constituent(symbol).map_err(ErrorNewType)?.into())
    }
}
