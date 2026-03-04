use longport::{
    Decimal, Error, Market, QuoteContext, TradeContext,
    quote::{
        AdjustType, Candlestick, CapitalDistributionResponse, CapitalFlowLine,
        HistoryMarketTemperatureResponse, MarketTemperature, MarketTradingDays, OptionQuote,
        ParticipantInfo, Period, SecurityBrokers, SecurityDepth, SecurityQuote, SecurityStaticInfo,
        StrikePriceInfo, Trade, TradeSessions,
    },
    trade::{
        AccountBalance, FundPositionChannel, GetHistoryOrdersOptions, MarginRatio, Order,
        OrderDetail, OrderSide, OrderType, OutsideRTH, StockPositionChannel, SubmitOrderOptions,
        SubmitOrderResponse, TimeInForceType,
    },
};
use poem_mcpserver::{
    Tools,
    content::{Json, Text},
};
use time::{
    Date, OffsetDateTime, format_description::BorrowedFormatItem, macros::format_description,
};

const DATE_FORMAT: &[BorrowedFormatItem] = format_description!("[year]-[month]-[day]");

pub(crate) struct Longport {
    quote_context: QuoteContext,
    trade_context: TradeContext,
}

impl Longport {
    #[inline]
    pub(crate) fn new(quote_context: QuoteContext, trade_context: TradeContext) -> Self {
        Self {
            quote_context,
            trade_context,
        }
    }
}

/// LongPort OpenAPI SDK.
#[Tools]
impl Longport {
    /// Get current time.
    async fn now(&self) -> Text<String> {
        Text(
            OffsetDateTime::now_utc()
                .format(&time::format_description::well_known::Rfc3339)
                .unwrap(),
        )
    }

    /// Get basic information of the securities.
    async fn static_info(
        &self,
        /// A list of security symbols. (e.g. ["700.HK", "AAPL.US", "000001.SH",
        /// "D05.SG"])
        symbols: Vec<String>,
    ) -> Result<Vec<Json<SecurityStaticInfo>>, Error> {
        Ok(self
            .quote_context
            .static_info(symbols)
            .await?
            .into_iter()
            .map(Json)
            .collect::<Vec<_>>())
    }

    /// Get the latest price of the securities.
    async fn quote(&self, symbols: Vec<String>) -> Result<Vec<Json<SecurityQuote>>, Error> {
        Ok(self
            .quote_context
            .quote(symbols)
            .await?
            .into_iter()
            .map(Json)
            .collect::<Vec<_>>())
    }

    /// Get the latest price of option securities.
    async fn option_quote(
        &self,
        /// A list of option symbols. (e.g. ["AAPL230317P160000.US",
        /// "AAPL230317C160000.US"]) Maximum 500 symbols per request.
        symbols: Vec<String>,
    ) -> Result<Vec<Json<OptionQuote>>, Error> {
        Ok(self
            .quote_context
            .option_quote(symbols)
            .await?
            .into_iter()
            .map(Json)
            .collect::<Vec<_>>())
    }

    /// Get the latest depth of the securities.
    async fn depth(&self, symbol: String) -> Result<Json<SecurityDepth>, Error> {
        Ok(Json(self.quote_context.depth(symbol).await?))
    }

    /// Get the latest trades of the securities.
    async fn trades(
        &self,
        symbol: String,
        /// max 1000
        count: usize,
    ) -> Result<Vec<Json<Trade>>, Error> {
        Ok(self
            .quote_context
            .trades(symbol, count)
            .await?
            .into_iter()
            .map(Json)
            .collect::<Vec<_>>())
    }

    /// Get the latest n candlesticks of the security.
    async fn candlesticks(
        &self,
        symbol: String,
        /// 1m, 2m, 3m, 5m, 10m, 15m, 20m, 30m, 45m, 60m, 120m, 180m, 240m, day,
        /// week, month, quarter, year
        period: String,
        /// last n candlesticks (max: 1000)
        count: usize,
        /// whether to adjust the historical data for splits, dividends, etc.
        /// (required)
        forward_adjust: bool,
        ///  trade sessions (required)
        /// - intraday: regular trading hours
        /// - all: all trading hours (intraday, pre, post, overnight)
        trade_sessions: String,
    ) -> Result<Vec<Json<Candlestick>>, Error> {
        let period = match period.as_str() {
            "1m" => Period::OneMinute,
            "2m" => Period::TwoMinute,
            "3m" => Period::ThreeMinute,
            "5m" => Period::FiveMinute,
            "10m" => Period::TenMinute,
            "15m" => Period::FifteenMinute,
            "20m" => Period::TwentyMinute,
            "30m" => Period::ThirtyMinute,
            "45m" => Period::FortyFiveMinute,
            "60m" => Period::SixtyMinute,
            "120m" => Period::TwoHour,
            "180m" => Period::ThreeHour,
            "240m" => Period::FourHour,
            "day" => Period::Day,
            "week" => Period::Week,
            "month" => Period::Month,
            "quarter" => Period::Quarter,
            "year" => Period::Year,
            _ => {
                return Err(Error::ParseField {
                    name: "market",
                    error: "invalid period".to_string(),
                });
            }
        };
        let trade_sessions = match trade_sessions.as_str() {
            "intraday" => TradeSessions::Intraday,
            "all" => TradeSessions::All,
            _ => {
                return Err(Error::ParseField {
                    name: "market",
                    error: "invalid trade_sessions".to_string(),
                });
            }
        };

        Ok(self
            .quote_context
            .candlesticks(
                symbol,
                period,
                count,
                if forward_adjust {
                    AdjustType::ForwardAdjust
                } else {
                    AdjustType::NoAdjust
                },
                trade_sessions,
            )
            .await?
            .into_iter()
            .map(Json)
            .collect::<Vec<_>>())
    }

    /// Get the trading days between the specified dates.
    ///
    /// The results include the `start_date` and `end_date`.
    async fn trading_days(
        &self,
        /// Market code. (e.g. "HK", "US", "CN", "SG")
        market: String,
        /// Start date of the trading days. (Format: "yyyy-mm-dd")
        start_date: String,
        /// End date of the trading days. (Format: "yyyy-mm-dd")
        end_date: String,
    ) -> Result<Json<MarketTradingDays>, Error> {
        let market = market.parse::<Market>().map_err(|err| Error::ParseField {
            name: "market",
            error: err.to_string(),
        })?;
        let start_date =
            Date::parse(&start_date, DATE_FORMAT).map_err(|err| Error::ParseField {
                name: "start_date",
                error: err.to_string(),
            })?;
        let end_date = Date::parse(&end_date, DATE_FORMAT).map_err(|err| Error::ParseField {
            name: "end_date",
            error: err.to_string(),
        })?;

        Ok(Json(
            self.quote_context
                .trading_days(market, start_date, end_date)
                .await?,
        ))
    }

    /// Returns the real-time broker queue data of security.
    async fn broker_queue(&self, symbol: String) -> Result<Json<SecurityBrokers>, Error> {
        Ok(Json(self.quote_context.brokers(symbol).await?))
    }

    /// Returns the participants information.
    async fn broker_info(&self) -> Result<Vec<Json<ParticipantInfo>>, Error> {
        Ok(self
            .quote_context
            .participants()
            .await?
            .into_iter()
            .map(Json)
            .collect::<Vec<_>>())
    }

    /// Returns the option chain list of the security.
    async fn option_chain_list(&self, symbol: String) -> Result<Vec<Text<String>>, Error> {
        Ok(self
            .quote_context
            .option_chain_expiry_date_list(symbol)
            .await?
            .into_iter()
            .map(|date| {
                Text(
                    date.format(format_description!("[year]-[month]-[day]"))
                        .unwrap(),
                )
            })
            .collect::<Vec<_>>())
    }

    /// Returns the option chain information of the security.
    async fn option_chain_info(
        &self,
        symbol: String,
        /// format: "yyyy-mm-dd"
        expiry_date: String,
    ) -> Result<Vec<Json<StrikePriceInfo>>, Error> {
        let expiry_date = Date::parse(&expiry_date, format_description!("[year]-[month]-[day]"))
            .map_err(|err| Error::ParseField {
                name: "expiry_date",
                error: err.to_string(),
            })?;
        Ok(self
            .quote_context
            .option_chain_info_by_date(symbol, expiry_date)
            .await?
            .into_iter()
            .map(Json)
            .collect::<Vec<_>>())
    }

    // Returns the capital flow of the security.
    async fn capital_flow(&self, symbol: String) -> Result<Vec<Json<CapitalFlowLine>>, Error> {
        Ok(self
            .quote_context
            .capital_flow(symbol)
            .await?
            .into_iter()
            .map(Json)
            .collect::<Vec<_>>())
    }

    /// Returns the capital distribution of the security.
    async fn capital_distribution(
        &self,
        symbol: String,
    ) -> Result<Json<CapitalDistributionResponse>, Error> {
        Ok(Json(self.quote_context.capital_distribution(symbol).await?))
    }

    /// Returns the market temperature of the specified market.
    async fn current_market_temperature(
        &self,
        /// Market code. (e.g. "HK", "US", "CN", "SG")
        market: String,
    ) -> Result<Json<MarketTemperature>, Error> {
        let market = market.parse::<Market>().map_err(|err| Error::ParseField {
            name: "market",
            error: err.to_string(),
        })?;
        Ok(Json(self.quote_context.market_temperature(market).await?))
    }

    /// Returns the historical market temperature of the specified market.
    ///
    /// includes the `start` and `end` dates.
    async fn history_market_temperature(
        &self,
        /// Market code. (e.g. "HK", "US", "CN", "SG")
        market: String,
        /// format: "yyyy-mm-dd"
        start: String,
        /// format: "yyyy-mm-dd"
        end: String,
    ) -> Result<Json<HistoryMarketTemperatureResponse>, Error> {
        let market = market.parse::<Market>().map_err(|err| Error::ParseField {
            name: "market",
            error: err.to_string(),
        })?;
        let start = Date::parse(&start, DATE_FORMAT).map_err(|err| Error::ParseField {
            name: "start",
            error: err.to_string(),
        })?;
        let end = Date::parse(&end, DATE_FORMAT).map_err(|err| Error::ParseField {
            name: "end",
            error: err.to_string(),
        })?;
        Ok(Json(
            self.quote_context
                .history_market_temperature(market, start, end)
                .await?,
        ))
    }

    /// Get the account balance.
    async fn account_balance(&self) -> Result<Vec<Json<AccountBalance>>, Error> {
        Ok(self
            .trade_context
            .account_balance(None)
            .await?
            .into_iter()
            .map(Json)
            .collect::<Vec<_>>())
    }

    /// Returns the stock positions.
    async fn stock_positions(&self) -> Result<Vec<Json<StockPositionChannel>>, Error> {
        Ok(self
            .trade_context
            .stock_positions(None)
            .await?
            .channels
            .into_iter()
            .map(Json)
            .collect::<Vec<_>>())
    }

    /// Returns the fund positions.
    async fn fund_positions(&self) -> Result<Vec<Json<FundPositionChannel>>, Error> {
        Ok(self
            .trade_context
            .fund_positions(None)
            .await?
            .channels
            .into_iter()
            .map(Json)
            .collect::<Vec<_>>())
    }

    /// Returns the initial margin ratio, maintain the margin ratio and
    /// strengthen the margin ratio of stocks.
    async fn magin_ratio(&self, symbol: String) -> Result<Json<MarginRatio>, Error> {
        Ok(Json(self.trade_context.margin_ratio(symbol).await?))
    }

    /// Submit an order.
    #[allow(clippy::too_many_arguments)]
    async fn submit_order(
        &self,
        symbol: String,
        /// Order type
        /// LO:	Limit Order
        /// ELO: Enhanced Limit Order
        /// MO: Market Order
        /// AO: At-auction Order
        /// ALO: At-auction Limit Order
        /// ODD: Odd Lots Order
        /// LIT: Limit If Touched
        /// MIT: Market If Touched
        /// TSLPAMT: Trailing Limit If Touched (Trailing Amount)
        /// TSLPPCT: Trailing Limit If Touched (Trailing Percent)
        /// SLO: Special Limit Order. Not Support Replace Order.
        order_type: String,
        /// for LO, ELO, ALO, ODD, LIT
        submitted_price: Option<Decimal>,
        submitted_quantity: Decimal,
        /// for LIT, MIT
        trigger_price: Option<Decimal>,
        /// for TSLPAMT, TSLPPCT
        limit_offset: Option<Decimal>,
        /// for TSLPAMT
        trailing_amount: Option<Decimal>,
        /// for TSLPPCT (0-1)
        trailing_percent: Option<Decimal>,
        /// format: "yyyy-mm-dd"
        expire_date: Option<String>,
        /// Side of the order (Buy or Sell)
        side: String,
        /// - RTH_ONLY: regular trading hour only
        /// - ANY_TIME: any time
        /// - OVERNIGHT: overnight
        outside_rth: Option<String>,
        /// - Day: Day Order
        /// - GTC: Good Till Cancel
        /// - GTD: Good Till Date
        time_in_force: String,
        /// Limit depth level
        limit_depth_level: Option<i32>,
        /// Trigger count
        trigger_count: Option<i32>,
        /// Monitor price
        monitor_price: Option<Decimal>,
    ) -> Result<Json<SubmitOrderResponse>, Error> {
        let mut opts = SubmitOrderOptions::new(
            symbol,
            order_type
                .parse::<OrderType>()
                .map_err(|err| Error::ParseField {
                    name: "order_type",
                    error: err.to_string(),
                })?,
            side.parse::<OrderSide>().map_err(|err| Error::ParseField {
                name: "side",
                error: err.to_string(),
            })?,
            submitted_quantity,
            time_in_force
                .parse::<TimeInForceType>()
                .map_err(|err| Error::ParseField {
                    name: "time_in_force",
                    error: err.to_string(),
                })?,
        );

        if let Some(submitted_price) = submitted_price {
            opts = opts.submitted_price(submitted_price);
        }
        if let Some(trigger_price) = trigger_price {
            opts = opts.trigger_price(trigger_price);
        }
        if let Some(limit_offset) = limit_offset {
            opts = opts.limit_offset(limit_offset);
        }
        if let Some(trailing_amount) = trailing_amount {
            opts = opts.trailing_amount(trailing_amount);
        }
        if let Some(trailing_percent) = trailing_percent {
            opts = opts.trailing_percent(trailing_percent);
        }

        if let Some(expire_date) = expire_date {
            opts = opts.expire_date(
                Date::parse(&expire_date, format_description!("[year]-[month]-[day]")).map_err(
                    |err| Error::ParseField {
                        name: "expire_date",
                        error: err.to_string(),
                    },
                )?,
            );
        }

        if let Some(outside_rth) = outside_rth {
            opts = opts.outside_rth(outside_rth.parse::<OutsideRTH>().map_err(|err| {
                Error::ParseField {
                    name: "outside_rth",
                    error: err.to_string(),
                }
            })?);
        }

        if let Some(limit_depth_level) = limit_depth_level {
            opts = opts.limit_depth_level(limit_depth_level);
        }
        if let Some(trigger_count) = trigger_count {
            opts = opts.trigger_count(trigger_count);
        }
        if let Some(monitor_price) = monitor_price {
            opts = opts.monitor_price(monitor_price);
        }

        self.trade_context.submit_order(opts).await.map(Json)
    }

    async fn cancel_order(&self, order_id: String) -> Result<(), Error> {
        self.trade_context.cancel_order(order_id).await
    }

    /// Get the order detail.
    async fn order_detail(&self, order_id: String) -> Result<Json<OrderDetail>, Error> {
        Ok(Json(self.trade_context.order_detail(order_id).await?))
    }

    /// Get the current account's orders for the day.
    async fn today_orders(&self) -> Result<Vec<Json<Order>>, Error> {
        Ok(self
            .trade_context
            .today_orders(None)
            .await?
            .into_iter()
            .map(Json)
            .collect::<Vec<_>>())
    }

    /// Get the historical orders of the current account.
    ///
    /// does not include today's orders
    async fn history_orders(
        &self,
        /// if not provided, default to all symbols
        symbol: Option<String>,
        /// format: RFC3339
        start_at: String,
        /// format: RFC3339
        end_at: String,
    ) -> Result<Vec<Json<Order>>, Error> {
        let mut opts = GetHistoryOrdersOptions::new()
            .start_at(
                OffsetDateTime::parse(&start_at, &time::format_description::well_known::Rfc3339)
                    .map_err(|err| Error::ParseField {
                        name: "start_at",
                        error: err.to_string(),
                    })?,
            )
            .end_at(
                OffsetDateTime::parse(&end_at, &time::format_description::well_known::Rfc3339)
                    .map_err(|err| Error::ParseField {
                        name: "end_at",
                        error: err.to_string(),
                    })?,
            );

        if let Some(symbol) = symbol {
            opts = opts.symbol(symbol);
        }

        Ok(self
            .trade_context
            .history_orders(opts)
            .await?
            .into_iter()
            .map(Json)
            .collect::<Vec<_>>())
    }
}
