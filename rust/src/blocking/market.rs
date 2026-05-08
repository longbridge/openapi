use std::sync::Arc;

use tokio::sync::mpsc;

use crate::{
    Config, Result,
    blocking::runtime::BlockingRuntime,
    market::{MarketContext, types::*},
};

/// Blocking market data context
pub struct MarketContextSync {
    rt: BlockingRuntime<MarketContext>,
}

impl MarketContextSync {
    /// Create a [`MarketContextSync`]
    pub fn new(config: Arc<Config>) -> Result<Self> {
        let rt = BlockingRuntime::try_new(
            move || {
                let ctx = MarketContext::new(config);
                let (tx, rx) = mpsc::unbounded_channel::<std::convert::Infallible>();
                std::mem::forget(tx);
                Ok::<_, crate::Error>((ctx, rx))
            },
            |_: std::convert::Infallible| {},
        )?;
        Ok(Self { rt })
    }

    /// Get current trading status for all markets
    pub fn market_status(&self) -> Result<MarketStatusResponse> {
        self.rt.call(|ctx| async move { ctx.market_status().await })
    }

    /// Get top broker holdings for a security
    pub fn broker_holding(
        &self,
        symbol: impl Into<String> + Send + 'static,
        period: BrokerHoldingPeriod,
    ) -> Result<BrokerHoldingTop> {
        self.rt
            .call(move |ctx| async move { ctx.broker_holding(symbol, period).await })
    }

    /// Get full broker holding details
    pub fn broker_holding_detail(
        &self,
        symbol: impl Into<String> + Send + 'static,
    ) -> Result<BrokerHoldingDetail> {
        self.rt
            .call(move |ctx| async move { ctx.broker_holding_detail(symbol).await })
    }

    /// Get daily holding history for a broker
    pub fn broker_holding_daily(
        &self,
        symbol: impl Into<String> + Send + 'static,
        broker_id: impl Into<String> + Send + 'static,
    ) -> Result<BrokerHoldingDailyHistory> {
        self.rt
            .call(move |ctx| async move { ctx.broker_holding_daily(symbol, broker_id).await })
    }

    /// Get A/H premium K-lines
    pub fn ah_premium(
        &self,
        symbol: impl Into<String> + Send + 'static,
        period: AhPremiumPeriod,
        count: u32,
    ) -> Result<AhPremiumKlines> {
        self.rt
            .call(move |ctx| async move { ctx.ah_premium(symbol, period, count).await })
    }

    /// Get A/H premium intraday data
    pub fn ah_premium_intraday(
        &self,
        symbol: impl Into<String> + Send + 'static,
    ) -> Result<AhPremiumIntraday> {
        self.rt
            .call(move |ctx| async move { ctx.ah_premium_intraday(symbol).await })
    }

    /// Get trade statistics
    pub fn trade_stats(
        &self,
        symbol: impl Into<String> + Send + 'static,
    ) -> Result<TradeStatsResponse> {
        self.rt
            .call(move |ctx| async move { ctx.trade_stats(symbol).await })
    }

    /// Get market anomaly alerts
    pub fn anomaly(&self, market: impl Into<String> + Send + 'static) -> Result<AnomalyResponse> {
        self.rt
            .call(move |ctx| async move { ctx.anomaly(market).await })
    }

    /// Get index constituent stocks
    pub fn constituent(
        &self,
        symbol: impl Into<String> + Send + 'static,
    ) -> Result<IndexConstituents> {
        self.rt
            .call(move |ctx| async move { ctx.constituent(symbol).await })
    }
}
