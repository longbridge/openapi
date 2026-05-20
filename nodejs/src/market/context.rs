use std::sync::Arc;

use napi::Result;

use crate::{config::Config, error::ErrorNewType, market::types::*};

/// Market data context
#[napi_derive::napi]
#[derive(Clone)]
pub struct MarketContext {
    ctx: longbridge::MarketContext,
}

#[napi_derive::napi]
impl MarketContext {
    /// Create a new `MarketContext`
    #[napi]
    pub fn new(config: &Config) -> MarketContext {
        Self {
            ctx: longbridge::MarketContext::new(Arc::new(config.0.clone())),
        }
    }

    /// Get market trading status
    #[napi]
    pub async fn market_status(&self) -> Result<MarketStatusResponse> {
        Ok(self.ctx.market_status().await.map_err(ErrorNewType)?.into())
    }

    /// Get top broker holdings
    #[napi]
    pub async fn broker_holding(
        &self,
        symbol: String,
        period: BrokerHoldingPeriod,
    ) -> Result<BrokerHoldingTop> {
        Ok(self
            .ctx
            .broker_holding(symbol, period.into())
            .await
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get full broker holding details
    #[napi]
    pub async fn broker_holding_detail(&self, symbol: String) -> Result<BrokerHoldingDetail> {
        Ok(self
            .ctx
            .broker_holding_detail(symbol)
            .await
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get daily holding history for a broker
    #[napi]
    pub async fn broker_holding_daily(
        &self,
        symbol: String,
        broker_id: String,
    ) -> Result<BrokerHoldingDailyHistory> {
        Ok(self
            .ctx
            .broker_holding_daily(symbol, broker_id)
            .await
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get A/H premium K-lines
    #[napi]
    pub async fn ah_premium(
        &self,
        symbol: String,
        period: AhPremiumPeriod,
        count: u32,
    ) -> Result<AhPremiumKlines> {
        Ok(self
            .ctx
            .ah_premium(symbol, period.into(), count)
            .await
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get A/H premium intraday data
    #[napi]
    pub async fn ah_premium_intraday(&self, symbol: String) -> Result<AhPremiumIntraday> {
        Ok(self
            .ctx
            .ah_premium_intraday(symbol)
            .await
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get trade statistics
    #[napi]
    pub async fn trade_stats(&self, symbol: String) -> Result<TradeStatsResponse> {
        Ok(self
            .ctx
            .trade_stats(symbol)
            .await
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get market anomaly alerts
    #[napi]
    pub async fn anomaly(&self, market: String) -> Result<AnomalyResponse> {
        Ok(self.ctx.anomaly(market).await.map_err(ErrorNewType)?.into())
    }

    /// Get index constituent stocks
    #[napi]
    pub async fn constituent(&self, symbol: String) -> Result<IndexConstituents> {
        Ok(self
            .ctx
            .constituent(symbol)
            .await
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get stock events across one or more markets
    #[napi]
    pub async fn stock_events(
        &self,
        markets: Vec<String>,
        sort: u32,
        date: Option<String>,
        limit: u32,
    ) -> Result<StockEventsResponse> {
        Ok(self
            .ctx
            .stock_events(markets, sort, date, limit)
            .await
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get all available rank category keys and labels
    #[napi]
    pub async fn rank_categories(&self) -> Result<RankCategoriesResponse> {
        Ok(self
            .ctx
            .rank_categories()
            .await
            .map_err(ErrorNewType)?
            .into())
    }

    /// Get a ranked list of securities for the given category key
    #[napi]
    pub async fn rank_list(&self, key: String, need_article: bool) -> Result<RankListResponse> {
        Ok(self
            .ctx
            .rank_list(key, need_article)
            .await
            .map_err(ErrorNewType)?
            .into())
    }
}
