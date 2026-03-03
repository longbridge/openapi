use std::sync::Arc;

use time::{Date, PrimitiveDateTime};

use crate::{
    Config, Market, QuoteContext, Result,
    blocking::runtime::BlockingRuntime,
    quote::{
        AdjustType, CalcIndex, Candlestick, CapitalDistributionResponse, CapitalFlowLine,
        FilterWarrantExpiryDate, FilterWarrantInOutBoundsType, HistoryMarketTemperatureResponse,
        IntradayLine, IssuerInfo, MarketTemperature, MarketTradingDays, MarketTradingSession,
        OptionQuote, ParticipantInfo, Period, PushEvent, QuotePackageDetail, RealtimeQuote,
        RequestCreateWatchlistGroup, RequestUpdateWatchlistGroup, Security, SecurityBrokers,
        SecurityCalcIndex, SecurityDepth, SecurityListCategory, SecurityQuote, SecurityStaticInfo,
        SortOrderType, StrikePriceInfo, SubFlags, Subscription, Trade, TradeSessions, WarrantInfo,
        WarrantQuote, WarrantSortBy, WarrantStatus, WarrantType, WatchlistGroup,
    },
};

/// Quote context
pub struct QuoteContextSync {
    rt: BlockingRuntime<QuoteContext>,
}

impl QuoteContextSync {
    /// Create a `QuoteContextSync` object
    pub fn try_new<F>(config: Arc<Config>, push_callback: F) -> Result<Self>
    where
        F: FnMut(PushEvent) + Send + 'static,
    {
        let rt = BlockingRuntime::try_new(move || QuoteContext::try_new(config), push_callback)?;
        Ok(Self { rt })
    }

    /// Returns the member ID
    pub fn member_id(&self) -> Result<i64> {
        self.rt.call(|ctx| async move { Ok(ctx.member_id()) })
    }

    /// Returns the quote level
    pub fn quote_level(&self) -> Result<String> {
        self.rt
            .call(|ctx| async move { Ok(ctx.quote_level().to_string()) })
    }

    /// Returns the quote package details
    pub fn quote_package_details(&self) -> Result<Vec<QuotePackageDetail>> {
        self.rt
            .call(|ctx| async move { Ok(ctx.quote_package_details().to_vec()) })
    }

    /// Subscribe
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::{sync::Arc, thread::sleep, time::Duration};
    ///
    /// use longport::{
    ///     Config,
    ///     blocking::QuoteContextSync,
    ///     quote::{PushEvent, SubFlags},
    /// };
    ///
    /// fn event_handler(event: PushEvent) {
    ///     println!("{:?}", event);
    /// }
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Arc::new(Config::from_env()?);
    /// let ctx = QuoteContextSync::try_new(config, event_handler)?;
    ///
    /// ctx.subscribe(["700.HK", "AAPL.US"], SubFlags::QUOTE)?;
    /// sleep(Duration::from_secs(30));
    /// # Ok(())
    /// # }
    /// ```
    pub fn subscribe<I, T, F>(&self, symbols: I, sub_types: F) -> Result<()>
    where
        I: IntoIterator<Item = T> + Send + 'static,
        I::IntoIter: Send + 'static,
        T: AsRef<str> + Send + 'static,
        F: Into<SubFlags> + Send + 'static,
    {
        self.rt
            .call(move |ctx| async move { ctx.subscribe(symbols, sub_types.into()).await })
    }

    /// Unsubscribe quote
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longport::{Config, blocking::QuoteContextSync, quote::SubFlags};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Arc::new(Config::from_env()?);
    /// let ctx = QuoteContextSync::try_new(config, |_| ())?;
    ///
    /// ctx.subscribe(["700.HK", "AAPL.US"], SubFlags::QUOTE)?;
    /// ctx.subscribe(["AAPL.US"], SubFlags::QUOTE)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn unsubscribe<I, T, F>(&self, symbols: I, sub_types: F) -> Result<()>
    where
        I: IntoIterator<Item = T> + Send + 'static,
        I::IntoIter: Send + 'static,
        T: AsRef<str> + Send + 'static,
        F: Into<SubFlags> + Send + 'static,
    {
        self.rt
            .call(move |ctx| async move { ctx.unsubscribe(symbols, sub_types.into()).await })
    }

    /// Subscribe security candlesticks
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::{sync::Arc, thread::sleep, time::Duration};
    ///
    /// use longport::{
    ///     Config,
    ///     blocking::QuoteContextSync,
    ///     quote::{Period, PushEvent, TradeSessions},
    /// };
    ///
    /// fn event_handler(event: PushEvent) {
    ///     println!("{:?}", event);
    /// }
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Arc::new(Config::from_env()?);
    /// let ctx = QuoteContextSync::try_new(config, event_handler)?;
    ///
    /// ctx.subscribe_candlesticks("AAPL.US", Period::OneMinute, TradeSessions::Intraday)?;
    /// sleep(Duration::from_secs(30));
    /// # Ok(())
    /// # }
    /// ```
    pub fn subscribe_candlesticks<T>(
        &self,
        symbol: T,
        period: Period,
        trade_sessions: TradeSessions,
    ) -> Result<Vec<Candlestick>>
    where
        T: AsRef<str> + Send + 'static,
    {
        self.rt.call(move |ctx| async move {
            ctx.subscribe_candlesticks(symbol, period, trade_sessions)
                .await
        })
    }

    /// Unsubscribe security candlesticks
    pub fn unsubscribe_candlesticks<T>(&self, symbol: T, period: Period) -> Result<()>
    where
        T: AsRef<str> + Send + 'static,
    {
        self.rt
            .call(move |ctx| async move { ctx.unsubscribe_candlesticks(symbol, period).await })
    }

    /// Get subscription information
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longport::{Config, blocking::QuoteContextSync, quote::SubFlags};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Arc::new(Config::from_env()?);
    /// let ctx = QuoteContextSync::try_new(config, |_| ())?;
    ///
    /// ctx.subscribe(["700.HK", "AAPL.US"], SubFlags::QUOTE)?;
    /// let resp = ctx.subscriptions();
    /// println!("{:?}", resp);
    /// # Ok(())
    /// # }
    /// ```
    pub fn subscriptions(&self) -> Result<Vec<Subscription>> {
        self.rt
            .call(move |ctx| async move { ctx.subscriptions().await })
    }

    /// Get basic information of securities
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longport::{Config, blocking::QuoteContextSync};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Arc::new(Config::from_env()?);
    /// let ctx = QuoteContextSync::try_new(config, |_| ())?;
    ///
    /// let resp = ctx.static_info(["700.HK", "AAPL.US", "TSLA.US", "NFLX.US"])?;
    /// println!("{:?}", resp);
    /// # Ok(())
    /// # }
    /// ```
    pub fn static_info<I, T>(&self, symbols: I) -> Result<Vec<SecurityStaticInfo>>
    where
        I: IntoIterator<Item = T> + Send + 'static,
        I::IntoIter: Send + 'static,
        T: Into<String> + Send + 'static,
    {
        self.rt
            .call(move |ctx| async move { ctx.static_info(symbols).await })
    }

    /// Get quote of securities
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longport::{Config, blocking::QuoteContextSync};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Arc::new(Config::from_env()?);
    /// let ctx = QuoteContextSync::try_new(config, |_| ())?;
    ///
    /// let resp = ctx.quote(["700.HK", "AAPL.US", "TSLA.US", "NFLX.US"])?;
    /// println!("{:?}", resp);
    /// # Ok(())
    /// # }
    /// ```
    pub fn quote<I, T>(&self, symbols: I) -> Result<Vec<SecurityQuote>>
    where
        I: IntoIterator<Item = T> + Send + 'static,
        I::IntoIter: Send + 'static,
        T: Into<String> + Send + 'static,
    {
        self.rt
            .call(move |ctx| async move { ctx.quote(symbols).await })
    }

    /// Get quote of option securities
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longport::{Config, blocking::QuoteContextSync};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Arc::new(Config::from_env()?);
    /// let ctx = QuoteContextSync::try_new(config, |_| ())?;
    ///
    /// let resp = ctx.option_quote(["AAPL230317P160000.US"])?;
    /// println!("{:?}", resp);
    /// # Ok(())
    /// # }
    /// ```
    pub fn option_quote<I, T>(&self, symbols: I) -> Result<Vec<OptionQuote>>
    where
        I: IntoIterator<Item = T> + Send + 'static,
        I::IntoIter: Send + 'static,
        T: Into<String> + Send + 'static,
    {
        self.rt
            .call(move |ctx| async move { ctx.option_quote(symbols).await })
    }

    /// Get quote of warrant securities
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longport::{Config, blocking::QuoteContextSync};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Arc::new(Config::from_env()?);
    /// let ctx = QuoteContextSync::try_new(config, |_| ())?;
    ///
    /// let resp = ctx.warrant_quote(["21125.HK"])?;
    /// println!("{:?}", resp);
    /// # Ok(())
    /// # }
    /// ```
    pub fn warrant_quote<I, T>(&self, symbols: I) -> Result<Vec<WarrantQuote>>
    where
        I: IntoIterator<Item = T> + Send + 'static,
        I::IntoIter: Send + 'static,
        T: Into<String> + Send + 'static,
    {
        self.rt
            .call(move |ctx| async move { ctx.warrant_quote(symbols).await })
    }

    /// Get security depth
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longport::{Config, blocking::QuoteContextSync};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Arc::new(Config::from_env()?);
    /// let ctx = QuoteContextSync::try_new(config, |_| ())?;
    ///
    /// let resp = ctx.depth("700.HK")?;
    /// println!("{:?}", resp);
    /// # Ok(())
    /// # }
    /// ```
    pub fn depth(&self, symbol: impl Into<String> + Send + 'static) -> Result<SecurityDepth> {
        self.rt
            .call(move |ctx| async move { ctx.depth(symbol).await })
    }

    /// Get security brokers
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longport::{Config, blocking::QuoteContextSync};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Arc::new(Config::from_env()?);
    /// let ctx = QuoteContextSync::try_new(config, |_| ())?;
    ///
    /// let resp = ctx.brokers("700.HK")?;
    /// println!("{:?}", resp);
    /// # Ok(())
    /// # }
    /// ```
    pub fn brokers(&self, symbol: impl Into<String> + Send + 'static) -> Result<SecurityBrokers> {
        self.rt
            .call(move |ctx| async move { ctx.brokers(symbol).await })
    }

    /// Get participants
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longport::{Config, blocking::QuoteContextSync};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Arc::new(Config::from_env()?);
    /// let ctx = QuoteContextSync::try_new(config, |_| ())?;
    ///
    /// let resp = ctx.participants()?;
    /// println!("{:?}", resp);
    /// # Ok(())
    /// # }
    /// ```
    pub fn participants(&self) -> Result<Vec<ParticipantInfo>> {
        self.rt
            .call(move |ctx| async move { ctx.participants().await })
    }

    /// Get security trades
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longport::{Config, blocking::QuoteContextSync};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Arc::new(Config::from_env()?);
    /// let ctx = QuoteContextSync::try_new(config, |_| ())?;
    ///
    /// let resp = ctx.trades("700.HK", 10)?;
    /// println!("{:?}", resp);
    /// # Ok(())
    /// # }
    /// ```
    pub fn trades(
        &self,
        symbol: impl Into<String> + Send + 'static,
        count: usize,
    ) -> Result<Vec<Trade>> {
        self.rt
            .call(move |ctx| async move { ctx.trades(symbol, count).await })
    }

    /// Get security intraday lines
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longport::{Config, blocking::QuoteContextSync, quote::TradeSessions};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Arc::new(Config::from_env()?);
    /// let ctx = QuoteContextSync::try_new(config, |_| ())?;
    ///
    /// let resp = ctx.intraday("700.HK", TradeSessions::Intraday)?;
    /// println!("{:?}", resp);
    /// # Ok(())
    /// # }
    /// ```
    pub fn intraday(
        &self,
        symbol: impl Into<String> + Send + 'static,
        trade_sessions: TradeSessions,
    ) -> Result<Vec<IntradayLine>> {
        self.rt
            .call(move |ctx| async move { ctx.intraday(symbol, trade_sessions).await })
    }

    /// Get security candlesticks
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longport::{
    ///     Config,
    ///     blocking::QuoteContextSync,
    ///     quote::{AdjustType, Period, TradeSessions},
    /// };
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Arc::new(Config::from_env()?);
    /// let ctx = QuoteContextSync::try_new(config, |_| ())?;
    ///
    /// let resp = ctx.candlesticks(
    ///     "700.HK",
    ///     Period::Day,
    ///     10,
    ///     AdjustType::NoAdjust,
    ///     TradeSessions::Intraday,
    /// )?;
    /// println!("{:?}", resp);
    /// # Ok(())
    /// # }
    /// ```
    pub fn candlesticks(
        &self,
        symbol: impl Into<String> + Send + 'static,
        period: Period,
        count: usize,
        adjust_type: AdjustType,
        trade_sessions: TradeSessions,
    ) -> Result<Vec<Candlestick>> {
        self.rt.call(move |ctx| async move {
            ctx.candlesticks(symbol, period, count, adjust_type, trade_sessions)
                .await
        })
    }

    /// Get security history candlesticks by offset
    #[allow(clippy::too_many_arguments)]
    pub fn history_candlesticks_by_offset(
        &self,
        symbol: impl Into<String> + Send + 'static,
        period: Period,
        adjust_type: AdjustType,
        forward: bool,
        time: Option<PrimitiveDateTime>,
        count: usize,
        trade_sessions: TradeSessions,
    ) -> Result<Vec<Candlestick>> {
        self.rt.call(move |ctx| async move {
            ctx.history_candlesticks_by_offset(
                symbol,
                period,
                adjust_type,
                forward,
                time,
                count,
                trade_sessions,
            )
            .await
        })
    }

    /// Get security history candlesticks by date
    pub fn history_candlesticks_by_date(
        &self,
        symbol: impl Into<String> + Send + 'static,
        period: Period,
        adjust_type: AdjustType,
        start: Option<Date>,
        end: Option<Date>,
        trade_sessions: TradeSessions,
    ) -> Result<Vec<Candlestick>> {
        self.rt.call(move |ctx| async move {
            ctx.history_candlesticks_by_date(
                symbol,
                period,
                adjust_type,
                start,
                end,
                trade_sessions,
            )
            .await
        })
    }

    /// Get option chain expiry date list
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longport::{Config, blocking::QuoteContextSync};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Arc::new(Config::from_env()?);
    /// let ctx = QuoteContextSync::try_new(config, |_| ())?;
    ///
    /// let resp = ctx.option_chain_expiry_date_list("AAPL.US")?;
    /// println!("{:?}", resp);
    /// # Ok(())
    /// # }
    /// ```
    pub fn option_chain_expiry_date_list(
        &self,
        symbol: impl Into<String> + Send + 'static,
    ) -> Result<Vec<Date>> {
        self.rt
            .call(move |ctx| async move { ctx.option_chain_expiry_date_list(symbol).await })
    }

    /// Get option chain info by date
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longport::{Config, blocking::QuoteContextSync};
    /// use time::macros::date;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Arc::new(Config::from_env()?);
    /// let ctx = QuoteContextSync::try_new(config, |_| ())?;
    ///
    /// let resp = ctx.option_chain_info_by_date("AAPL.US", date!(2023 - 01 - 20))?;
    /// println!("{:?}", resp);
    /// # Ok(())
    /// # }
    /// ```
    pub fn option_chain_info_by_date(
        &self,
        symbol: impl Into<String> + Send + 'static,
        expiry_date: Date,
    ) -> Result<Vec<StrikePriceInfo>> {
        self.rt.call(
            move |ctx| async move { ctx.option_chain_info_by_date(symbol, expiry_date).await },
        )
    }

    /// Get warrant issuers
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longport::{Config, blocking::QuoteContextSync};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Arc::new(Config::from_env()?);
    /// let ctx = QuoteContextSync::try_new(config, |_| ())?;
    ///
    /// let resp = ctx.warrant_issuers()?;
    /// println!("{:?}", resp);
    /// # Ok(())
    /// # }
    /// ```
    pub fn warrant_issuers(&self) -> Result<Vec<IssuerInfo>> {
        self.rt
            .call(move |ctx| async move { ctx.warrant_issuers().await })
    }

    /// Query warrant list
    #[allow(clippy::too_many_arguments)]
    pub fn warrant_list(
        &self,
        symbol: impl Into<String> + Send + 'static,
        sort_by: WarrantSortBy,
        sort_order: SortOrderType,
        warrant_type: Option<&[WarrantType]>,
        issuer: Option<&[i32]>,
        expiry_date: Option<&[FilterWarrantExpiryDate]>,
        price_type: Option<&[FilterWarrantInOutBoundsType]>,
        status: Option<&[WarrantStatus]>,
    ) -> Result<Vec<WarrantInfo>> {
        let warrant_type = warrant_type.map(|v| v.to_vec());
        let issuer = issuer.map(|v| v.to_vec());
        let expiry_date = expiry_date.map(|v| v.to_vec());
        let price_type = price_type.map(|v| v.to_vec());
        let status = status.map(|v| v.to_vec());
        self.rt.call(move |ctx| async move {
            ctx.warrant_list(
                symbol,
                sort_by,
                sort_order,
                warrant_type.as_deref(),
                issuer.as_deref(),
                expiry_date.as_deref(),
                price_type.as_deref(),
                status.as_deref(),
            )
            .await
        })
    }

    /// Get trading session of the day
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longport::{Config, blocking::QuoteContextSync};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Arc::new(Config::from_env()?);
    /// let ctx = QuoteContextSync::try_new(config, |_| ())?;
    ///
    /// let resp = ctx.trading_session()?;
    /// println!("{:?}", resp);
    /// # Ok(())
    /// # }
    /// ```
    pub fn trading_session(&self) -> Result<Vec<MarketTradingSession>> {
        self.rt
            .call(move |ctx| async move { ctx.trading_session().await })
    }

    /// Get market trading days
    ///
    /// The interval must be less than one month, and only the most recent year
    /// is supported.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longport::{Config, Market, blocking::QuoteContextSync};
    /// use time::macros::date;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Arc::new(Config::from_env()?);
    /// let ctx = QuoteContextSync::try_new(config, |_| ())?;
    ///
    /// let resp = ctx.trading_days(Market::HK, date!(2022 - 01 - 20), date!(2022 - 02 - 20))?;
    /// println!("{:?}", resp);
    /// # Ok(())
    /// # }
    /// ```
    pub fn trading_days(
        &self,
        market: Market,
        begin: Date,
        end: Date,
    ) -> Result<MarketTradingDays> {
        self.rt
            .call(move |ctx| async move { ctx.trading_days(market, begin, end).await })
    }

    /// Get capital flow intraday
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longport::{Config, blocking::QuoteContextSync};
    /// use time::macros::date;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Arc::new(Config::from_env()?);
    /// let ctx = QuoteContextSync::try_new(config, |_| ())?;
    ///
    /// let resp = ctx.capital_flow("700.HK")?;
    /// println!("{:?}", resp);
    /// # Ok(())
    /// # }
    /// ```
    pub fn capital_flow(
        &self,
        symbol: impl Into<String> + Send + 'static,
    ) -> Result<Vec<CapitalFlowLine>> {
        self.rt
            .call(move |ctx| async move { ctx.capital_flow(symbol).await })
    }

    /// Get capital distribution
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longport::{Config, blocking::QuoteContextSync};
    /// use time::macros::date;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Arc::new(Config::from_env()?);
    /// let ctx = QuoteContextSync::try_new(config, |_| ())?;
    ///
    /// let resp = ctx.capital_distribution("700.HK")?;
    /// println!("{:?}", resp);
    /// # Ok(())
    /// # }
    /// ```
    pub fn capital_distribution(
        &self,
        symbol: impl Into<String> + Send + 'static,
    ) -> Result<CapitalDistributionResponse> {
        self.rt
            .call(move |ctx| async move { ctx.capital_distribution(symbol).await })
    }

    /// Get calc indexes
    pub fn calc_indexes<I, T, J>(&self, symbols: I, indexes: J) -> Result<Vec<SecurityCalcIndex>>
    where
        I: IntoIterator<Item = T> + Send + 'static,
        I::IntoIter: Send + 'static,
        T: Into<String> + Send + 'static,
        J: IntoIterator<Item = CalcIndex> + Send + 'static,
        J::IntoIter: Send + 'static,
    {
        self.rt
            .call(move |ctx| async move { ctx.calc_indexes(symbols, indexes).await })
    }

    /// Get watchlist
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longport::{Config, blocking::QuoteContextSync};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Arc::new(Config::from_env()?);
    /// let ctx = QuoteContextSync::try_new(config, |_| ())?;
    ///
    /// let resp = ctx.watchlist()?;
    /// println!("{:?}", resp);
    /// # Ok(())
    /// # }
    /// ```
    pub fn watchlist(&self) -> Result<Vec<WatchlistGroup>> {
        self.rt
            .call(move |ctx| async move { ctx.watchlist().await })
    }

    /// Create watchlist group
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longport::{Config, blocking::QuoteContextSync, quote::RequestCreateWatchlistGroup};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Arc::new(Config::from_env()?);
    /// let ctx = QuoteContextSync::try_new(config, |_| ())?;
    ///
    /// let req = RequestCreateWatchlistGroup::new("Watchlist1").securities(["700.HK", "BABA.US"]);
    /// let group_id = ctx.create_watchlist_group(req)?;
    /// println!("{}", group_id);
    /// # Ok(())
    /// # }
    /// ```
    pub fn create_watchlist_group(&self, req: RequestCreateWatchlistGroup) -> Result<i64> {
        self.rt
            .call(move |ctx| async move { ctx.create_watchlist_group(req).await })
    }

    /// Delete watchlist group
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longport::{Config, blocking::QuoteContextSync};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Arc::new(Config::from_env()?);
    /// let ctx = QuoteContextSync::try_new(config, |_| ())?;
    ///
    /// ctx.delete_watchlist_group(10086, true)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn delete_watchlist_group(&self, id: i64, purge: bool) -> Result<()> {
        self.rt
            .call(move |ctx| async move { ctx.delete_watchlist_group(id, purge).await })
    }

    /// Update watchlist group
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longport::{Config, blocking::QuoteContextSync, quote::RequestUpdateWatchlistGroup};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Arc::new(Config::from_env()?);
    /// let ctx = QuoteContextSync::try_new(config, |_| ())?;
    ///
    /// let req = RequestUpdateWatchlistGroup::new(10086)
    ///     .name("Watchlist2")
    ///     .securities(["700.HK", "BABA.US"]);
    /// ctx.update_watchlist_group(req)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn update_watchlist_group(&self, req: RequestUpdateWatchlistGroup) -> Result<()> {
        self.rt
            .call(move |ctx| async move { ctx.update_watchlist_group(req).await })
    }

    /// Get security list
    pub fn security_list(
        &self,
        market: Market,
        category: impl Into<Option<SecurityListCategory>>,
    ) -> Result<Vec<Security>> {
        let category = category.into();
        self.rt
            .call(move |ctx| async move { ctx.security_list(market, category).await })
    }

    /// Get current market temperature
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longport::{Config, Market, blocking::QuoteContextSync};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Arc::new(Config::from_env()?);
    /// let ctx = QuoteContextSync::try_new(config, |_| ())?;
    ///
    /// let resp = ctx.market_temperature(Market::HK)?;
    /// println!("{:?}", resp);
    /// # Ok(())
    /// # }
    /// ```
    pub fn market_temperature(&self, market: Market) -> Result<MarketTemperature> {
        self.rt
            .call(move |ctx| async move { ctx.market_temperature(market).await })
    }

    /// Get historical market temperature
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longport::{Config, Market, blocking::QuoteContextSync};
    /// use time::macros::date;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Arc::new(Config::from_env()?);
    /// let ctx = QuoteContextSync::try_new(config, |_| ())?;
    ///
    /// let resp =
    ///     ctx.history_market_temperature(Market::HK, date!(2023 - 01 - 01), date!(2023 - 01 - 31))?;
    /// println!("{:?}", resp);
    /// # Ok(())
    /// # }
    /// ```
    pub fn history_market_temperature(
        &self,
        market: Market,
        start_date: Date,
        end: Date,
    ) -> Result<HistoryMarketTemperatureResponse> {
        self.rt.call(move |ctx| async move {
            ctx.history_market_temperature(market, start_date, end)
                .await
        })
    }

    /// Get real-time quotes
    ///
    /// Get real-time quotes of the subscribed symbols, it always returns the
    /// data in the local storage.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::{sync::Arc, thread::sleep, time::Duration};
    ///
    /// use longport::{Config, Market, blocking::QuoteContextSync, quote::SubFlags};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Arc::new(Config::from_env()?);
    /// let ctx = QuoteContextSync::try_new(config, |_| ())?;
    ///
    /// ctx.subscribe(["700.HK", "AAPL.US"], SubFlags::QUOTE)?;
    /// sleep(Duration::from_secs(5));
    ///
    /// let resp = ctx.realtime_quote(["700.HK", "AAPL.US"])?;
    /// println!("{:?}", resp);
    /// # Ok(())
    /// # }
    /// ```
    pub fn realtime_quote<I, T>(&self, symbols: I) -> Result<Vec<RealtimeQuote>>
    where
        I: IntoIterator<Item = T> + Send + 'static,
        I::IntoIter: Send + 'static,
        T: Into<String> + Send + 'static,
    {
        self.rt
            .call(move |ctx| async move { ctx.realtime_quote(symbols).await })
    }

    /// Get real-time depth
    ///
    /// Get real-time depth of the subscribed symbols, it always returns the
    /// data in the local storage.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::{sync::Arc, thread::sleep, time::Duration};
    ///
    /// use longport::{Config, Market, blocking::QuoteContextSync, quote::SubFlags};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Arc::new(Config::from_env()?);
    /// let ctx = QuoteContextSync::try_new(config, |_| ())?;
    ///
    /// ctx.subscribe(["700.HK", "AAPL.US"], SubFlags::DEPTH)?;
    /// sleep(Duration::from_secs(5));
    ///
    /// let resp = ctx.realtime_depth("700.HK")?;
    /// println!("{:?}", resp);
    /// # Ok(())
    /// # }
    /// ```
    pub fn realtime_depth(
        &self,
        symbol: impl Into<String> + Send + 'static,
    ) -> Result<SecurityDepth> {
        self.rt
            .call(move |ctx| async move { ctx.realtime_depth(symbol).await })
    }

    /// Get real-time trades
    ///
    /// Get real-time trades of the subscribed symbols, it always returns the
    /// data in the local storage.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::{sync::Arc, thread::sleep, time::Duration};
    ///
    /// use longport::{Config, Market, blocking::QuoteContextSync, quote::SubFlags};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Arc::new(Config::from_env()?);
    /// let ctx = QuoteContextSync::try_new(config, |_| ())?;
    ///
    /// ctx.subscribe(["700.HK", "AAPL.US"], SubFlags::TRADE)?;
    /// sleep(Duration::from_secs(5));
    ///
    /// let resp = ctx.realtime_trades("700.HK", 10)?;
    /// println!("{:?}", resp);
    /// # Ok(())
    /// # }
    /// ```
    pub fn realtime_trades(
        &self,
        symbol: impl Into<String> + Send + 'static,
        count: usize,
    ) -> Result<Vec<Trade>> {
        self.rt
            .call(move |ctx| async move { ctx.realtime_trades(symbol, count).await })
    }

    /// Get real-time broker queue
    ///
    /// Get real-time brokers of the subscribed symbols, it always returns the
    /// data in the local storage.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::{sync::Arc, thread::sleep, time::Duration};
    ///
    /// use longport::{Config, Market, blocking::QuoteContextSync, quote::SubFlags};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Arc::new(Config::from_env()?);
    /// let ctx = QuoteContextSync::try_new(config, |_| ())?;
    ///
    /// ctx.subscribe(["700.HK", "AAPL.US"], SubFlags::BROKER)?;
    /// sleep(Duration::from_secs(5));
    ///
    /// let resp = ctx.realtime_brokers("700.HK")?;
    /// println!("{:?}", resp);
    /// # Ok(())
    /// # }
    /// ```
    pub fn realtime_brokers(
        &self,
        symbol: impl Into<String> + Send + 'static,
    ) -> Result<SecurityBrokers> {
        self.rt
            .call(move |ctx| async move { ctx.realtime_brokers(symbol).await })
    }

    /// Get real-time candlesticks
    ///
    /// Get real-time candlesticks of the subscribed symbols, it always returns
    /// the data in the local storage.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::{sync::Arc, thread::sleep, time::Duration};
    ///
    /// use longport::{
    ///     Config, Market,
    ///     blocking::QuoteContextSync,
    ///     quote::{Period, TradeSessions},
    /// };
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Arc::new(Config::from_env()?);
    /// let ctx = QuoteContextSync::try_new(config, |_| ())?;
    ///
    /// ctx.subscribe_candlesticks("AAPL.US", Period::OneMinute, TradeSessions::Intraday)?;
    /// sleep(Duration::from_secs(5));
    ///
    /// let resp = ctx.realtime_candlesticks("AAPL.US", Period::OneMinute, 10)?;
    /// println!("{:?}", resp);
    /// # Ok(())
    /// # }
    /// ```
    pub fn realtime_candlesticks(
        &self,
        symbol: impl Into<String> + Send + 'static,
        period: Period,
        count: usize,
    ) -> Result<Vec<Candlestick>> {
        self.rt
            .call(move |ctx| async move { ctx.realtime_candlesticks(symbol, period, count).await })
    }
}
