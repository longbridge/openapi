use std::sync::Arc;

use crate::{
    blocking::runtime::BlockingRuntime,
    trade::{
        AccountBalance, CashFlow, Execution, FundPositionsResponse, GetCashFlowOptions,
        GetFundPositionsOptions, GetHistoryExecutionsOptions, GetHistoryOrdersOptions,
        GetStockPositionsOptions, GetTodayExecutionsOptions, GetTodayOrdersOptions, MarginRatio,
        Order, PushEvent, ReplaceOrderOptions, StockPositionsResponse, SubmitOrderOptions,
        SubmitOrderResponse, TopicType, TradeContext,
    },
    Config, Result,
};

/// Trade context
pub struct TradeContextSync {
    rt: BlockingRuntime<TradeContext>,
}

impl TradeContextSync {
    /// Create a `TradeContextSync` object
    pub fn try_new<F>(config: Arc<Config>, push_callback: F) -> Result<Self>
    where
        F: FnMut(PushEvent) + Send + 'static,
    {
        let rt = BlockingRuntime::try_new(move || TradeContext::try_new(config), push_callback)?;
        Ok(Self { rt })
    }

    /// Subscribe topics
    pub fn subscribe<I>(&self, topics: I) -> Result<()>
    where
        I: IntoIterator<Item = TopicType> + Send + 'static,
    {
        self.rt
            .call(move |ctx| async move { ctx.subscribe(topics).await })
    }

    /// Unsubscribe topics
    pub fn unsubscribe<I>(&self, topics: I) -> Result<()>
    where
        I: IntoIterator<Item = TopicType> + Send + 'static,
    {
        self.rt
            .call(move |ctx| async move { ctx.unsubscribe(topics).await })
    }

    /// Get history executions
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{blocking::TradeContextSync, trade::GetHistoryExecutionsOptions, Config};
    /// use time::macros::datetime;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Arc::new(Config::from_env()?);
    /// let ctx = TradeContextSync::try_new(config, |_| ())?;
    ///
    /// let opts = GetHistoryExecutionsOptions::new().symbol("700.HK")
    ///     .start_at(datetime!(2022-05-09 0:00 UTC))
    ///     .end_at(datetime!(2022-05-12 0:00 UTC));
    /// let resp = ctx.history_executions(opts)?;
    /// println!("{:?}", resp);
    /// # Ok(())
    /// # }
    /// ```
    pub fn history_executions(
        &self,
        options: impl Into<Option<GetHistoryExecutionsOptions>> + Send + 'static,
    ) -> Result<Vec<Execution>> {
        self.rt
            .call(move |ctx| async move { ctx.history_executions(options).await })
    }

    /// Get today executions
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{blocking::TradeContextSync, trade::GetTodayExecutionsOptions, Config};
    /// use time::macros::datetime;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Arc::new(Config::from_env()?);
    /// let ctx = TradeContextSync::try_new(config, |_| ())?;
    ///
    /// let opts = GetTodayExecutionsOptions::new().symbol("700.HK");
    /// let resp = ctx.today_executions(opts)?;
    /// println!("{:?}", resp);
    /// # Ok(())
    /// # }
    /// ```
    pub fn today_executions(
        &self,
        options: impl Into<Option<GetTodayExecutionsOptions>> + Send + 'static,
    ) -> Result<Vec<Execution>> {
        self.rt
            .call(move |ctx| async move { ctx.today_executions(options).await })
    }

    /// Get history orders
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{
    ///     blocking::TradeContextSync,
    ///     trade::{GetHistoryOrdersOptions, OrderSide, OrderStatus},
    ///     Config, Market,
    /// };
    /// use time::macros::datetime;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Arc::new(Config::from_env()?);
    /// let ctx = TradeContextSync::try_new(config, |_| ())?;
    ///
    /// let opts = GetHistoryOrdersOptions::new()
    ///     .symbol("700.HK")
    ///     .status([OrderStatus::Filled, OrderStatus::New])
    ///     .side(OrderSide::Buy)
    ///     .market(Market::HK)
    ///     .start_at(datetime!(2022-05-09 0:00 UTC))
    ///     .end_at(datetime!(2022-05-12 0:00 UTC));
    /// let resp = ctx.history_orders(opts)?;
    /// println!("{:?}", resp);
    /// # Ok(())
    /// # }
    /// ```
    pub fn history_orders(
        &self,
        options: impl Into<Option<GetHistoryOrdersOptions>> + Send + 'static,
    ) -> Result<Vec<Order>> {
        self.rt
            .call(move |ctx| async move { ctx.history_orders(options).await })
    }

    /// Get today orders
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{
    ///     blocking::TradeContextSync,
    ///     trade::{GetTodayOrdersOptions, OrderSide, OrderStatus},
    ///     Config, Market,
    /// };
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Arc::new(Config::from_env()?);
    /// let ctx = TradeContextSync::try_new(config, |_| ())?;
    ///
    /// let opts = GetTodayOrdersOptions::new()
    ///     .symbol("700.HK")
    ///     .status([OrderStatus::Filled, OrderStatus::New])
    ///     .side(OrderSide::Buy)
    ///     .market(Market::HK);
    /// let resp = ctx.today_orders(opts)?;
    /// println!("{:?}", resp);
    /// # Ok(())
    /// # }
    /// ```
    pub fn today_orders(
        &self,
        options: impl Into<Option<GetTodayOrdersOptions>> + Send + 'static,
    ) -> Result<Vec<Order>> {
        self.rt
            .call(move |ctx| async move { ctx.today_orders(options).await })
    }

    /// Replace order
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{blocking::TradeContextSync, decimal, trade::ReplaceOrderOptions, Config};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Arc::new(Config::from_env()?);
    /// let ctx = TradeContextSync::try_new(config, |_| ())?;
    ///
    /// let opts = ReplaceOrderOptions::new("709043056541253632", 100).price(decimal!(300i32));
    /// let resp = ctx.replace_order(opts)?;
    /// println!("{:?}", resp);
    /// # Ok(())
    /// # }
    /// ```
    pub fn replace_order(&self, options: ReplaceOrderOptions) -> Result<()> {
        self.rt
            .call(move |ctx| async move { ctx.replace_order(options).await })
    }

    /// Submit order
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{
    ///     blocking::TradeContextSync,
    ///     decimal,
    ///     trade::{OrderSide, OrderType, SubmitOrderOptions, TimeInForceType},
    ///     Config,
    /// };
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Arc::new(Config::from_env()?);
    /// let ctx = TradeContextSync::try_new(config, |_| ())?;
    ///
    /// let opts = SubmitOrderOptions::new(
    ///     "700.HK",
    ///     OrderType::LO,
    ///     OrderSide::Buy,
    ///     200,
    ///     TimeInForceType::Day,
    /// )
    /// .submitted_price(decimal!(50i32));
    /// let resp = ctx.submit_order(opts)?;
    /// println!("{:?}", resp);
    /// # Ok(())
    /// # }
    /// ```
    pub fn submit_order(&self, options: SubmitOrderOptions) -> Result<SubmitOrderResponse> {
        self.rt
            .call(move |ctx| async move { ctx.submit_order(options).await })
    }

    /// Cancel order
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{blocking::TradeContextSync, Config};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Arc::new(Config::from_env()?);
    /// let ctx = TradeContextSync::try_new(config, |_| ())?;
    ///
    /// ctx.cancel_order("709043056541253632")?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn cancel_order(&self, order_id: impl Into<String> + Send + 'static) -> Result<()> {
        self.rt
            .call(move |ctx| async move { ctx.cancel_order(order_id).await })
    }

    /// Get account balance
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{blocking::TradeContextSync, Config};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Arc::new(Config::from_env()?);
    /// let ctx = TradeContextSync::try_new(config, |_| ())?;
    ///
    /// let resp = ctx.account_balance()?;
    /// println!("{:?}", resp);
    /// # Ok(())
    /// # }
    /// ```
    pub fn account_balance(&self) -> Result<Vec<AccountBalance>> {
        self.rt
            .call(move |ctx| async move { ctx.account_balance().await })
    }

    /// Get cash flow
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{blocking::TradeContextSync, trade::GetCashFlowOptions, Config};
    /// use time::macros::datetime;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Arc::new(Config::from_env()?);
    /// let ctx = TradeContextSync::try_new(config, |_| ())?;
    ///
    /// let opts = GetCashFlowOptions::new(datetime!(2022-05-09 0:00 UTC), datetime!(2022-05-12 0:00 UTC));
    /// let resp = ctx.cash_flow(opts)?;
    /// println!("{:?}", resp);
    /// # Ok(())
    /// # }
    /// ```
    pub fn cash_flow(&self, options: GetCashFlowOptions) -> Result<Vec<CashFlow>> {
        self.rt
            .call(move |ctx| async move { ctx.cash_flow(options).await })
    }

    /// Get fund positions
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{blocking::TradeContextSync, Config};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Arc::new(Config::from_env()?);
    /// let ctx = TradeContextSync::try_new(config, |_| ())?;
    ///
    /// let resp = ctx.fund_positions(None)?;
    /// println!("{:?}", resp);
    /// # Ok(())
    /// # }
    /// ```
    pub fn fund_positions(
        &self,
        opts: impl Into<Option<GetFundPositionsOptions>> + Send + 'static,
    ) -> Result<FundPositionsResponse> {
        self.rt
            .call(move |ctx| async move { ctx.fund_positions(opts).await })
    }

    /// Get stock positions
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{blocking::TradeContextSync, Config};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Arc::new(Config::from_env()?);
    /// let ctx = TradeContextSync::try_new(config, |_| ())?;
    ///
    /// let resp = ctx.stock_positions(None)?;
    /// println!("{:?}", resp);
    /// # Ok(())
    /// # }
    /// ```
    pub fn stock_positions(
        &self,
        opts: impl Into<Option<GetStockPositionsOptions>> + Send + 'static,
    ) -> Result<StockPositionsResponse> {
        self.rt
            .call(move |ctx| async move { ctx.stock_positions(opts).await })
    }

    /// Get margin ratio
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    ///
    /// use longbridge::{blocking::TradeContextSync, Config};
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = Arc::new(Config::from_env()?);
    /// let ctx = TradeContextSync::try_new(config, |_| ())?;
    ///
    /// let resp = ctx.margin_ratio("700.HK")?;
    /// println!("{:?}", resp);
    /// # Ok(())
    /// # }
    /// ```
    pub fn margin_ratio(&self, symbol: impl Into<String> + Send + 'static) -> Result<MarginRatio> {
        self.rt
            .call(move |ctx| async move { ctx.margin_ratio(symbol).await })
    }
}
