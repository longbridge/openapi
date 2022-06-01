use std::sync::Arc;

use time::Date;

use crate::{
    blocking::runtime::BlockingRuntime,
    quote::{
        AdjustType, Candlestick, IntradayLine, IssuerInfo, MarketTradingDays, MarketTradingSession,
        OptionQuote, ParticipantInfo, Period, PushEvent, RealtimeQuote, SecurityBrokers,
        SecurityDepth, SecurityQuote, SecurityStaticInfo, StrikePriceInfo, SubFlags, Trade,
        WarrantQuote,
    },
    Config, Market, QuoteContext, Result,
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

    /// Subscribe
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::{sync::Arc, thread::sleep, time::Duration};
    ///
    /// use longbridge::{
    ///     blocking::QuoteContextSync,
    ///     quote::{PushEvent, SubFlags},
    ///     Config,
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
    /// ctx.subscribe(["700.HK", "AAPL.US"], SubFlags::QUOTE, false)?;
    /// sleep(Duration::from_secs(30));
    /// # Ok(())
    /// # }
    /// ```
    pub fn subscribe<I, T, F>(&self, symbols: I, sub_types: F, is_first_push: bool) -> Result<()>
    where
        I: IntoIterator<Item = T> + Send + 'static,
        I::IntoIter: Send + 'static,
        T: Into<String> + Send + 'static,
        F: Into<SubFlags> + Send + 'static,
    {
        self.rt.call(move |ctx| async move {
            ctx.subscribe(symbols, sub_types.into(), is_first_push)
                .await
        })
    }

    /// Unsubscribe quote
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{blocking::QuoteContextSync, quote::SubFlags, Config};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Arc::new(Config::from_env()?);
    /// let ctx = QuoteContextSync::try_new(config, |_| ())?;
    ///
    /// ctx.subscribe(["700.HK", "AAPL.US"], SubFlags::QUOTE, false)?;
    /// ctx.subscribe(["AAPL.US"], SubFlags::QUOTE, false)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn unsubscribe<I, T, F>(&self, symbols: I, sub_types: F) -> Result<()>
    where
        I: IntoIterator<Item = T> + Send + 'static,
        I::IntoIter: Send + 'static,
        T: Into<String> + Send + 'static,
        F: Into<SubFlags> + Send + 'static,
    {
        self.rt
            .call(move |ctx| async move { ctx.unsubscribe(symbols, sub_types.into()).await })
    }

    /// Get basic information of securities
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{blocking::QuoteContextSync, Config};
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
    /// use longbridge::{blocking::QuoteContextSync, Config};
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
    /// use longbridge::{blocking::QuoteContextSync, Config};
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
    /// use longbridge::{blocking::QuoteContextSync, Config};
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
    /// use longbridge::{blocking::QuoteContextSync, Config};
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
    /// use longbridge::{blocking::QuoteContextSync, Config};
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
    /// use longbridge::{blocking::QuoteContextSync, Config};
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
    /// use longbridge::{blocking::QuoteContextSync, Config};
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
    /// use longbridge::{blocking::QuoteContextSync, Config};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Arc::new(Config::from_env()?);
    /// let ctx = QuoteContextSync::try_new(config, |_| ())?;
    ///
    /// let resp = ctx.intraday("700.HK")?;
    /// println!("{:?}", resp);
    /// # Ok(())
    /// # }
    /// ```
    pub fn intraday(
        &self,
        symbol: impl Into<String> + Send + 'static,
    ) -> Result<Vec<IntradayLine>> {
        self.rt
            .call(move |ctx| async move { ctx.intraday(symbol).await })
    }

    /// Get security candlesticks
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{
    ///     blocking::QuoteContextSync,
    ///     quote::{AdjustType, Period},
    ///     Config,
    /// };
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Arc::new(Config::from_env()?);
    /// let ctx = QuoteContextSync::try_new(config, |_| ())?;
    ///
    /// let resp = ctx.candlesticks("700.HK", Period::Day, 10, AdjustType::NoAdjust)?;
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
    ) -> Result<Vec<Candlestick>> {
        self.rt.call(move |ctx| async move {
            ctx.candlesticks(symbol, period, count, adjust_type).await
        })
    }

    /// Get option chain expiry date list
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{blocking::QuoteContextSync, Config};
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
    /// use longbridge::{blocking::QuoteContextSync, Config};
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
    /// use longbridge::{blocking::QuoteContextSync, Config};
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

    /// Get trading session of the day
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{blocking::QuoteContextSync, Config};
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
    /// use longbridge::{blocking::QuoteContextSync, Config, Market};
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
    /// use longbridge::{blocking::QuoteContextSync, quote::SubFlags, Config, Market};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Arc::new(Config::from_env()?);
    /// let ctx = QuoteContextSync::try_new(config, |_| ())?;
    ///
    /// ctx.subscribe(["HK.700", "AAPL.US"], SubFlags::QUOTE, true)?;
    /// sleep(Duration::from_secs(5));
    ///
    /// let resp = ctx.realtime_quote(["HK.700", "AAPL.US"])?;
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
    /// use longbridge::{blocking::QuoteContextSync, quote::SubFlags, Config, Market};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Arc::new(Config::from_env()?);
    /// let ctx = QuoteContextSync::try_new(config, |_| ())?;
    ///
    /// ctx.subscribe(["HK.700", "AAPL.US"], SubFlags::DEPTH, true)?;
    /// sleep(Duration::from_secs(5));
    ///
    /// let resp = ctx.realtime_depth("HK.700")?;
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
    /// use longbridge::{blocking::QuoteContextSync, quote::SubFlags, Config, Market};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Arc::new(Config::from_env()?);
    /// let ctx = QuoteContextSync::try_new(config, |_| ())?;
    ///
    /// ctx.subscribe(["HK.700", "AAPL.US"], SubFlags::TRADE, false)?;
    /// sleep(Duration::from_secs(5));
    ///
    /// let resp = ctx.realtime_trades("HK.700", 10)?;
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
    /// use longbridge::{blocking::QuoteContextSync, quote::SubFlags, Config, Market};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Arc::new(Config::from_env()?);
    /// let ctx = QuoteContextSync::try_new(config, |_| ())?;
    ///
    /// ctx.subscribe(["HK.700", "AAPL.US"], SubFlags::BROKER, false)?;
    /// sleep(Duration::from_secs(5));
    ///
    /// let resp = ctx.realtime_brokers("HK.700")?;
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
}
