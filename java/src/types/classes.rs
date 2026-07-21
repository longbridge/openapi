use std::borrow::Borrow;

use longbridge::{Decimal, Market};
use longbridge_java_macros::impl_java_class;
use time::Date;

impl_java_class!(
    "com/longbridge/quote/Trade",
    longbridge::quote::Trade,
    [
        price,
        volume,
        timestamp,
        trade_type,
        direction,
        trade_session
    ]
);

impl_java_class!(
    "com/longbridge/quote/Brokers",
    longbridge::quote::Brokers,
    [
        position,
        #[java(priarray)]
        broker_ids
    ]
);

impl_java_class!(
    "com/longbridge/quote/Depth",
    longbridge::quote::Depth,
    [position, price, volume, order_num]
);

impl_java_class!(
    "com/longbridge/quote/Subscription",
    longbridge::quote::Subscription,
    [
        symbol,
        sub_types,
        #[java(objarray)]
        candlesticks
    ]
);

impl_java_class!(
    "com/longbridge/quote/PushQuote",
    longbridge::quote::PushQuote,
    [
        last_done,
        open,
        high,
        low,
        timestamp,
        volume,
        turnover,
        trade_status,
        trade_session,
        current_volume,
        current_turnover
    ]
);

impl_java_class!(
    "com/longbridge/quote/PushDepth",
    longbridge::quote::PushDepth,
    [
        #[java(objarray)]
        asks,
        #[java(objarray)]
        bids
    ]
);

impl_java_class!(
    "com/longbridge/quote/PushBrokers",
    longbridge::quote::PushBrokers,
    [
        #[java(objarray)]
        ask_brokers,
        #[java(objarray)]
        bid_brokers
    ]
);

impl_java_class!(
    "com/longbridge/quote/PushTrades",
    longbridge::quote::PushTrades,
    [
        #[java(objarray)]
        trades,
    ]
);

impl_java_class!(
    "com/longbridge/quote/PushCandlestick",
    longbridge::quote::PushCandlestick,
    [period, candlestick, is_confirmed]
);

impl_java_class!(
    "com/longbridge/quote/Security",
    longbridge::quote::Security,
    [symbol, name_cn, name_en, name_hk,]
);

impl_java_class!(
    "com/longbridge/quote/SecurityStaticInfo",
    longbridge::quote::SecurityStaticInfo,
    [
        symbol,
        name_cn,
        name_en,
        name_hk,
        exchange,
        currency,
        lot_size,
        total_shares,
        circulating_shares,
        hk_shares,
        eps,
        eps_ttm,
        bps,
        dividend_yield,
        #[java(set_as = crate::types::enum_types::DerivativeTypes)]
        stock_derivatives,
        board,
    ]
);

impl_java_class!(
    "com/longbridge/quote/PrePostQuote",
    longbridge::quote::PrePostQuote,
    [
        last_done, timestamp, volume, turnover, high, low, prev_close
    ]
);

impl_java_class!(
    "com/longbridge/quote/SecurityQuote",
    longbridge::quote::SecurityQuote,
    [
        symbol,
        last_done,
        prev_close,
        open,
        high,
        low,
        timestamp,
        volume,
        turnover,
        trade_status,
        pre_market_quote,
        post_market_quote,
        overnight_quote
    ]
);

impl_java_class!(
    "com/longbridge/quote/OptionQuote",
    longbridge::quote::OptionQuote,
    [
        symbol,
        last_done,
        prev_close,
        open,
        high,
        low,
        timestamp,
        volume,
        turnover,
        trade_status,
        implied_volatility,
        open_interest,
        expiry_date,
        strike_price,
        contract_multiplier,
        contract_type,
        contract_size,
        direction,
        historical_volatility,
        underlying_symbol,
    ]
);

impl_java_class!(
    "com/longbridge/quote/WarrantQuote",
    longbridge::quote::WarrantQuote,
    [
        symbol,
        last_done,
        prev_close,
        open,
        high,
        low,
        timestamp,
        volume,
        turnover,
        trade_status,
        implied_volatility,
        expiry_date,
        last_trade_date,
        outstanding_ratio,
        outstanding_quantity,
        conversion_ratio,
        category,
        strike_price,
        upper_strike_price,
        lower_strike_price,
        call_price,
        underlying_symbol
    ]
);

impl_java_class!(
    "com/longbridge/quote/SecurityDepth",
    longbridge::quote::SecurityDepth,
    [
        #[java(objarray)]
        asks,
        #[java(objarray)]
        bids
    ]
);

impl_java_class!(
    "com/longbridge/quote/SecurityBrokers",
    longbridge::quote::SecurityBrokers,
    [
        #[java(objarray)]
        ask_brokers,
        #[java(objarray)]
        bid_brokers
    ]
);

impl_java_class!(
    "com/longbridge/quote/ParticipantInfo",
    longbridge::quote::ParticipantInfo,
    [
        #[java(priarray)]
        broker_ids,
        name_cn,
        name_en,
        name_hk
    ]
);

impl_java_class!(
    "com/longbridge/quote/IntradayLine",
    longbridge::quote::IntradayLine,
    [price, timestamp, volume, turnover, avg_price]
);

impl_java_class!(
    "com/longbridge/quote/Candlestick",
    longbridge::quote::Candlestick,
    [
        close,
        open,
        low,
        high,
        volume,
        turnover,
        timestamp,
        trade_session
    ],
    non_exhaustive
);

impl_java_class!(
    "com/longbridge/quote/StrikePriceInfo",
    longbridge::quote::StrikePriceInfo,
    [price, call_symbol, put_symbol, standard]
);

impl_java_class!(
    "com/longbridge/quote/IssuerInfo",
    longbridge::quote::IssuerInfo,
    [issuer_id, name_cn, name_en, name_hk]
);

impl_java_class!(
    "com/longbridge/quote/MarketTradingSession",
    longbridge::quote::MarketTradingSession,
    [
        market,
        #[java(objarray)]
        trade_sessions
    ]
);

impl_java_class!(
    "com/longbridge/quote/TradingSessionInfo",
    longbridge::quote::TradingSessionInfo,
    [begin_time, end_time, trade_session]
);

impl_java_class!(
    "com/longbridge/quote/MarketTradingDays",
    longbridge::quote::MarketTradingDays,
    [
        #[java(objarray)]
        trading_days,
        #[java(objarray)]
        half_trading_days
    ]
);

impl_java_class!(
    "com/longbridge/quote/CapitalFlowLine",
    longbridge::quote::CapitalFlowLine,
    [inflow, timestamp]
);

impl_java_class!(
    "com/longbridge/quote/CapitalDistribution",
    longbridge::quote::CapitalDistribution,
    [large, medium, small]
);

pub(crate) struct SecurityCalcIndex {
    pub(crate) symbol: String,
    pub(crate) last_done: Option<Decimal>,
    pub(crate) change_value: Option<Decimal>,
    pub(crate) change_rate: Option<Decimal>,
    pub(crate) volume: i64,
    pub(crate) turnover: Option<Decimal>,
    pub(crate) ytd_change_rate: Option<Decimal>,
    pub(crate) turnover_rate: Option<Decimal>,
    pub(crate) total_market_value: Option<Decimal>,
    pub(crate) capital_flow: Option<Decimal>,
    pub(crate) amplitude: Option<Decimal>,
    pub(crate) volume_ratio: Option<Decimal>,
    pub(crate) pe_ttm_ratio: Option<Decimal>,
    pub(crate) pb_ratio: Option<Decimal>,
    pub(crate) dividend_ratio_ttm: Option<Decimal>,
    pub(crate) five_day_change_rate: Option<Decimal>,
    pub(crate) ten_day_change_rate: Option<Decimal>,
    pub(crate) half_year_change_rate: Option<Decimal>,
    pub(crate) five_minutes_change_rate: Option<Decimal>,
    pub(crate) expiry_date: Option<Date>,
    pub(crate) strike_price: Option<Decimal>,
    pub(crate) upper_strike_price: Option<Decimal>,
    pub(crate) lower_strike_price: Option<Decimal>,
    pub(crate) outstanding_qty: i64,
    pub(crate) outstanding_ratio: Option<Decimal>,
    pub(crate) premium: Option<Decimal>,
    pub(crate) itm_otm: Option<Decimal>,
    pub(crate) implied_volatility: Option<Decimal>,
    pub(crate) warrant_delta: Option<Decimal>,
    pub(crate) call_price: Option<Decimal>,
    pub(crate) to_call_price: Option<Decimal>,
    pub(crate) effective_leverage: Option<Decimal>,
    pub(crate) leverage_ratio: Option<Decimal>,
    pub(crate) conversion_ratio: Option<Decimal>,
    pub(crate) balance_point: Option<Decimal>,
    pub(crate) open_interest: i64,
    pub(crate) delta: Option<Decimal>,
    pub(crate) gamma: Option<Decimal>,
    pub(crate) theta: Option<Decimal>,
    pub(crate) vega: Option<Decimal>,
    pub(crate) rho: Option<Decimal>,
}

impl From<longbridge::quote::SecurityCalcIndex> for SecurityCalcIndex {
    fn from(
        longbridge::quote::SecurityCalcIndex {
            symbol,
            last_done,
            change_value,
            change_rate,
            volume,
            turnover,
            ytd_change_rate,
            turnover_rate,
            total_market_value,
            capital_flow,
            amplitude,
            volume_ratio,
            pe_ttm_ratio,
            pb_ratio,
            dividend_ratio_ttm,
            five_day_change_rate,
            ten_day_change_rate,
            half_year_change_rate,
            five_minutes_change_rate,
            expiry_date,
            strike_price,
            upper_strike_price,
            lower_strike_price,
            outstanding_qty,
            outstanding_ratio,
            premium,
            itm_otm,
            implied_volatility,
            warrant_delta,
            call_price,
            to_call_price,
            effective_leverage,
            leverage_ratio,
            conversion_ratio,
            balance_point,
            open_interest,
            delta,
            gamma,
            theta,
            vega,
            rho,
        }: longbridge::quote::SecurityCalcIndex,
    ) -> Self {
        Self {
            symbol,
            last_done,
            change_value,
            change_rate,
            volume: volume.unwrap_or_default(),
            turnover,
            ytd_change_rate,
            turnover_rate,
            total_market_value,
            capital_flow,
            amplitude,
            volume_ratio,
            pe_ttm_ratio,
            pb_ratio,
            dividend_ratio_ttm,
            five_day_change_rate,
            ten_day_change_rate,
            half_year_change_rate,
            five_minutes_change_rate,
            expiry_date,
            strike_price,
            upper_strike_price,
            lower_strike_price,
            outstanding_qty: outstanding_qty.unwrap_or_default(),
            outstanding_ratio,
            premium,
            itm_otm,
            implied_volatility,
            warrant_delta,
            call_price,
            to_call_price,
            effective_leverage,
            leverage_ratio,
            conversion_ratio,
            balance_point,
            open_interest: open_interest.unwrap_or_default(),
            delta,
            gamma,
            theta,
            vega,
            rho,
        }
    }
}

impl_java_class!(
    "com/longbridge/quote/SecurityCalcIndex",
    SecurityCalcIndex,
    [
        symbol,
        last_done,
        change_value,
        change_rate,
        volume,
        turnover,
        ytd_change_rate,
        turnover_rate,
        total_market_value,
        capital_flow,
        amplitude,
        volume_ratio,
        pe_ttm_ratio,
        pb_ratio,
        dividend_ratio_ttm,
        five_day_change_rate,
        ten_day_change_rate,
        half_year_change_rate,
        five_minutes_change_rate,
        expiry_date,
        strike_price,
        upper_strike_price,
        lower_strike_price,
        outstanding_qty,
        outstanding_ratio,
        premium,
        itm_otm,
        implied_volatility,
        warrant_delta,
        call_price,
        to_call_price,
        effective_leverage,
        leverage_ratio,
        conversion_ratio,
        balance_point,
        open_interest,
        delta,
        gamma,
        theta,
        vega,
        rho
    ]
);

impl_java_class!(
    "com/longbridge/quote/WatchlistGroup",
    longbridge::quote::WatchlistGroup,
    [
        id,
        name,
        #[java(objarray)]
        securities
    ]
);

impl_java_class!(
    "com/longbridge/quote/WatchlistSecurity",
    longbridge::quote::WatchlistSecurity,
    [symbol, market, name, watched_price, watched_at, is_pinned]
);

pub(crate) struct CreateWatchlistGroupResponse {
    pub(crate) id: i64,
}

impl_java_class!(
    "com/longbridge/quote/CreateWatchlistGroupResponse",
    CreateWatchlistGroupResponse,
    [id]
);

impl_java_class!(
    "com/longbridge/quote/CapitalDistributionResponse",
    longbridge::quote::CapitalDistributionResponse,
    [timestamp, capital_in, capital_out]
);

impl_java_class!(
    "com/longbridge/quote/RealtimeQuote",
    longbridge::quote::RealtimeQuote,
    [
        symbol,
        last_done,
        open,
        high,
        low,
        timestamp,
        volume,
        turnover,
        trade_status
    ]
);

impl_java_class!(
    "com/longbridge/quote/WarrantInfo",
    longbridge::quote::WarrantInfo,
    [
        symbol,
        warrant_type,
        name,
        last_done,
        change_rate,
        change_value,
        volume,
        turnover,
        expiry_date,
        strike_price,
        upper_strike_price,
        lower_strike_price,
        outstanding_qty,
        outstanding_ratio,
        premium,
        itm_otm,
        implied_volatility,
        delta,
        call_price,
        to_call_price,
        effective_leverage,
        leverage_ratio,
        conversion_ratio,
        balance_point,
        status,
    ]
);

impl_java_class!(
    "com/longbridge/trade/PushOrderChanged",
    longbridge::trade::PushOrderChanged,
    [
        side,
        stock_name,
        submitted_quantity,
        symbol,
        order_type,
        submitted_price,
        executed_quantity,
        executed_price,
        order_id,
        currency,
        status,
        submitted_at,
        updated_at,
        trigger_price,
        msg,
        tag,
        trigger_status,
        trigger_at,
        trailing_amount,
        trailing_percent,
        limit_offset,
        account_no,
        last_share,
        last_price,
        remark
    ]
);

impl_java_class!(
    "com/longbridge/trade/Execution",
    longbridge::trade::Execution,
    [order_id, trade_id, symbol, trade_done_at, quantity, price]
);

impl_java_class!(
    "com/longbridge/trade/AllExecutionsResponse",
    longbridge::trade::AllExecutionsResponse,
    [
        has_more,
        #[java(objarray)]
        trades
    ]
);

impl_java_class!(
    "com/longbridge/trade/Order",
    longbridge::trade::Order,
    [
        order_id,
        status,
        stock_name,
        quantity,
        executed_quantity,
        price,
        executed_price,
        submitted_at,
        side,
        symbol,
        order_type,
        last_done,
        trigger_price,
        msg,
        tag,
        time_in_force,
        expire_date,
        updated_at,
        trigger_at,
        trailing_amount,
        trailing_percent,
        limit_offset,
        trigger_status,
        currency,
        outside_rth,
        #[java(set_as_opt = crate::types::JavaInteger)]
        limit_depth_level,
        #[java(set_as_opt = crate::types::JavaInteger)]
        trigger_count,
        monitor_price,
        remark
    ]
);

impl_java_class!(
    "com/longbridge/trade/SubmitOrderResponse",
    longbridge::trade::SubmitOrderResponse,
    [order_id]
);

impl_java_class!(
    "com/longbridge/trade/CashInfo",
    longbridge::trade::CashInfo,
    [
        withdraw_cash,
        available_cash,
        frozen_cash,
        settling_cash,
        currency
    ]
);

impl_java_class!(
    "com/longbridge/trade/FrozenTransactionFee",
    longbridge::trade::FrozenTransactionFee,
    [currency, frozen_transaction_fee]
);

impl_java_class!(
    "com/longbridge/trade/AccountBalance",
    longbridge::trade::AccountBalance,
    [
        total_cash,
        max_finance_amount,
        remaining_finance_amount,
        risk_level,
        margin_call,
        currency,
        #[java(objarray)]
        cash_infos,
        net_assets,
        init_margin,
        maintenance_margin,
        buy_power,
        #[java(objarray)]
        frozen_transaction_fees
    ]
);

impl_java_class!(
    "com/longbridge/trade/CashFlow",
    longbridge::trade::CashFlow,
    [
        transaction_flow_name,
        direction,
        business_type,
        balance,
        currency,
        business_time,
        symbol,
        description,
    ]
);

impl_java_class!(
    "com/longbridge/trade/FundPositionsResponse",
    longbridge::trade::FundPositionsResponse,
    [
        #[java(objarray)]
        channels
    ]
);

impl_java_class!(
    "com/longbridge/trade/FundPositionChannel",
    longbridge::trade::FundPositionChannel,
    [
        account_channel,
        #[java(objarray)]
        positions
    ]
);

impl_java_class!(
    "com/longbridge/trade/FundPosition",
    longbridge::trade::FundPosition,
    [
        symbol,
        current_net_asset_value,
        net_asset_value_day,
        symbol_name,
        currency,
        cost_net_asset_value,
        holding_units
    ]
);

pub(crate) struct StockPositionsResponse {
    channels: Vec<StockPositionChannel>,
}

impl From<longbridge::trade::StockPositionsResponse> for StockPositionsResponse {
    fn from(value: longbridge::trade::StockPositionsResponse) -> Self {
        Self {
            channels: value
                .channels
                .into_iter()
                .map(StockPositionChannel::from)
                .collect(),
        }
    }
}

impl_java_class!(
    "com/longbridge/trade/StockPositionsResponse",
    StockPositionsResponse,
    [
        #[java(objarray)]
        channels
    ]
);

pub(crate) struct StockPositionChannel {
    account_channel: String,
    positions: Vec<StockPosition>,
}

impl From<longbridge::trade::StockPositionChannel> for StockPositionChannel {
    fn from(value: longbridge::trade::StockPositionChannel) -> Self {
        Self {
            account_channel: value.account_channel,
            positions: value
                .positions
                .into_iter()
                .map(StockPosition::from)
                .collect(),
        }
    }
}

impl_java_class!(
    "com/longbridge/trade/StockPositionChannel",
    StockPositionChannel,
    [
        account_channel,
        #[java(objarray)]
        positions
    ]
);

pub(crate) struct StockPosition {
    symbol: String,
    symbol_name: String,
    quantity: Decimal,
    available_quantity: Decimal,
    currency: String,
    cost_price: Decimal,
    market: Market,
    init_quantity: Decimal,
}

impl From<longbridge::trade::StockPosition> for StockPosition {
    fn from(value: longbridge::trade::StockPosition) -> Self {
        Self {
            symbol: value.symbol,
            symbol_name: value.symbol_name,
            quantity: value.quantity,
            available_quantity: value.available_quantity,
            currency: value.currency,
            cost_price: value.cost_price,
            market: value.market,
            init_quantity: value.init_quantity.unwrap_or_default(),
        }
    }
}

impl_java_class!(
    "com/longbridge/trade/StockPosition",
    StockPosition,
    [
        symbol,
        symbol_name,
        quantity,
        available_quantity,
        currency,
        cost_price,
        market,
        init_quantity
    ]
);

impl_java_class!(
    "com/longbridge/trade/MarginRatio",
    longbridge::trade::MarginRatio,
    [im_factor, mm_factor, fm_factor]
);

impl_java_class!(
    "com/longbridge/trade/OrderHistoryDetail",
    longbridge::trade::OrderHistoryDetail,
    [price, quantity, status, msg, time]
);

impl_java_class!(
    "com/longbridge/trade/OrderChargeFee",
    longbridge::trade::OrderChargeFee,
    [code, name, amount, currency]
);

impl_java_class!(
    "com/longbridge/trade/OrderChargeItem",
    longbridge::trade::OrderChargeItem,
    [
        code,
        name,
        #[java(objarray)]
        fees
    ]
);

impl_java_class!(
    "com/longbridge/trade/OrderChargeDetail",
    longbridge::trade::OrderChargeDetail,
    [
        total_amount,
        currency,
        #[java(objarray)]
        items
    ]
);

impl_java_class!(
    "com/longbridge/trade/OrderDetail",
    longbridge::trade::OrderDetail,
    [
        order_id,
        status,
        stock_name,
        quantity,
        executed_quantity,
        price,
        executed_price,
        submitted_at,
        side,
        symbol,
        order_type,
        last_done,
        trigger_price,
        msg,
        tag,
        time_in_force,
        expire_date,
        updated_at,
        trigger_at,
        trailing_amount,
        trailing_percent,
        limit_offset,
        trigger_status,
        currency,
        outside_rth,
        #[java(set_as_opt = crate::types::JavaInteger)]
        limit_depth_level,
        #[java(set_as_opt = crate::types::JavaInteger)]
        trigger_count,
        monitor_price,
        remark,
        free_status,
        free_amount,
        free_currency,
        deductions_status,
        deductions_amount,
        deductions_currency,
        platform_deducted_status,
        platform_deducted_amount,
        platform_deducted_currency,
        #[java(objarray)]
        history,
        charge_detail
    ]
);

impl_java_class!(
    "com/longbridge/trade/EstimateMaxPurchaseQuantityResponse",
    longbridge::trade::EstimateMaxPurchaseQuantityResponse,
    [cash_max_qty, margin_max_qty]
);

impl_java_class!(
    "com/longbridge/quote/QuotePackageDetail",
    longbridge::quote::QuotePackageDetail,
    [key, name, description, start_at, end_at]
);

impl_java_class!(
    "com/longbridge/quote/MarketTemperature",
    longbridge::quote::MarketTemperature,
    [temperature, description, valuation, sentiment, timestamp]
);

impl_java_class!(
    "com/longbridge/quote/HistoryMarketTemperatureResponse",
    longbridge::quote::HistoryMarketTemperatureResponse,
    [
        granularity,
        #[java(objarray)]
        records
    ]
);

impl_java_class!(
    "com/longbridge/quote/FilingItem",
    longbridge::quote::FilingItem,
    [
        id,
        title,
        description,
        file_name,
        #[java(objarray)]
        file_urls,
        published_at
    ]
);

impl_java_class!(
    "com/longbridge/content/TopicItem",
    longbridge::content::TopicItem,
    [
        id,
        title,
        description,
        url,
        published_at,
        comments_count,
        likes_count,
        shares_count
    ]
);

impl_java_class!(
    "com/longbridge/content/NewsItem",
    longbridge::content::NewsItem,
    [
        id,
        title,
        description,
        url,
        published_at,
        comments_count,
        likes_count,
        shares_count
    ]
);

impl_java_class!(
    "com/longbridge/content/TopicAuthor",
    longbridge::content::TopicAuthor,
    [member_id, name, avatar]
);

impl_java_class!(
    "com/longbridge/content/TopicImage",
    longbridge::content::TopicImage,
    [url, sm, lg]
);

impl_java_class!(
    "com/longbridge/content/OwnedTopic",
    longbridge::content::OwnedTopic,
    [
        id,
        title,
        description,
        body,
        author,
        #[java(objarray)]
        tickers,
        #[java(objarray)]
        hashtags,
        #[java(objarray)]
        images,
        likes_count,
        comments_count,
        views_count,
        shares_count,
        topic_type,
        detail_url,
        created_at,
        updated_at
    ]
);

// ── MarketContext types ───────────────────────────────────────────

impl_java_class!(
    "com/longbridge/market/MarketStatusResponse",
    longbridge::market::MarketStatusResponse,
    [
        #[java(objarray)]
        market_time
    ]
);

impl_java_class!(
    "com/longbridge/market/MarketTimeItem",
    longbridge::market::MarketTimeItem,
    [
        market,
        trade_status,
        timestamp,
        delay_trade_status,
        delay_timestamp,
        sub_status,
        delay_sub_status
    ]
);

impl_java_class!(
    "com/longbridge/market/BrokerHoldingTop",
    longbridge::market::BrokerHoldingTop,
    [
        #[java(objarray)]
        buy,
        #[java(objarray)]
        sell,
        updated_at
    ]
);

impl_java_class!(
    "com/longbridge/market/BrokerHoldingEntry",
    longbridge::market::BrokerHoldingEntry,
    [name, parti_number, chg, strong]
);

impl_java_class!(
    "com/longbridge/market/BrokerHoldingDetail",
    longbridge::market::BrokerHoldingDetail,
    [
        #[java(objarray)]
        list,
        updated_at
    ]
);

impl_java_class!(
    "com/longbridge/market/BrokerHoldingDetailItem",
    longbridge::market::BrokerHoldingDetailItem,
    [name, parti_number, ratio, shares, strong]
);

impl_java_class!(
    "com/longbridge/market/BrokerHoldingChanges",
    longbridge::market::BrokerHoldingChanges,
    [value, chg_1, chg_5, chg_20, chg_60]
);

impl_java_class!(
    "com/longbridge/market/BrokerHoldingDailyHistory",
    longbridge::market::BrokerHoldingDailyHistory,
    [
        #[java(objarray)]
        list
    ]
);

impl_java_class!(
    "com/longbridge/market/BrokerHoldingDailyItem",
    longbridge::market::BrokerHoldingDailyItem,
    [date, holding, ratio, chg]
);

impl_java_class!(
    "com/longbridge/market/AhPremiumKlines",
    longbridge::market::AhPremiumKlines,
    [
        #[java(objarray)]
        klines
    ]
);

impl_java_class!(
    "com/longbridge/market/AhPremiumIntraday",
    longbridge::market::AhPremiumIntraday,
    [
        #[java(objarray)]
        klines
    ]
);

impl_java_class!(
    "com/longbridge/market/AhPremiumKline",
    longbridge::market::AhPremiumKline,
    [
        aprice,
        apreclose,
        hprice,
        hpreclose,
        currency_rate,
        ahpremium_rate,
        price_spread,
        timestamp
    ]
);

impl_java_class!(
    "com/longbridge/market/TradeStatsResponse",
    longbridge::market::TradeStatsResponse,
    [
        statistics,
        #[java(objarray)]
        trades
    ]
);

impl_java_class!(
    "com/longbridge/market/TradeStatistics",
    longbridge::market::TradeStatistics,
    [
        avgprice,
        buy,
        neutral,
        preclose,
        sell,
        timestamp,
        total_amount,
        #[java(objarray)]
        trade_date,
        trades_count
    ]
);

impl_java_class!(
    "com/longbridge/market/TradePriceLevel",
    longbridge::market::TradePriceLevel,
    [buy_amount, neutral_amount, price, sell_amount]
);

impl_java_class!(
    "com/longbridge/market/AnomalyResponse",
    longbridge::market::AnomalyResponse,
    [
        all_off,
        #[java(objarray)]
        changes
    ]
);

impl_java_class!(
    "com/longbridge/market/AnomalyItem",
    longbridge::market::AnomalyItem,
    [
        symbol,
        name,
        alert_name,
        alert_time,
        #[java(objarray)]
        change_values,
        emotion
    ]
);

impl_java_class!(
    "com/longbridge/market/IndexConstituents",
    longbridge::market::IndexConstituents,
    [
        fall_num,
        flat_num,
        rise_num,
        #[java(objarray)]
        stocks
    ]
);

impl_java_class!(
    "com/longbridge/market/ConstituentStock",
    longbridge::market::ConstituentStock,
    [
        symbol,
        name,
        last_done,
        prev_close,
        inflow,
        balance,
        amount,
        total_shares,
        #[java(objarray)]
        tags,
        intro,
        market,
        circulating_shares,
        delay,
        chg,
        trade_status
    ]
);

// ── CalendarContext types ─────────────────────────────────────────

impl_java_class!(
    "com/longbridge/calendar/CalendarEventsResponse",
    longbridge::calendar::CalendarEventsResponse,
    [
        date,
        #[java(objarray)]
        list,
        next_date
    ]
);

impl_java_class!(
    "com/longbridge/calendar/CalendarDateGroup",
    longbridge::calendar::CalendarDateGroup,
    [
        date,
        count,
        #[java(objarray)]
        infos
    ]
);

impl_java_class!(
    "com/longbridge/calendar/CalendarEventInfo",
    longbridge::calendar::CalendarEventInfo,
    [
        symbol,
        market,
        content,
        counter_name,
        date_type,
        date,
        chart_uid,
        #[java(objarray)]
        data_kv,
        event_type,
        datetime,
        icon,
        star,
        live,
        id,
        financial_market_time,
        currency,
        ext,
        activity_type
    ]
);

impl_java_class!(
    "com/longbridge/calendar/CalendarDataKv",
    longbridge::calendar::CalendarDataKv,
    [key, value, value_type, value_raw]
);

// ── PortfolioContext types ────────────────────────────────────────

impl_java_class!(
    "com/longbridge/portfolio/ExchangeRates",
    longbridge::portfolio::ExchangeRates,
    [
        #[java(objarray)]
        exchanges
    ]
);

impl_java_class!(
    "com/longbridge/portfolio/ExchangeRate",
    longbridge::portfolio::ExchangeRate,
    [
        average_rate,
        base_currency,
        bid_rate,
        offer_rate,
        other_currency
    ]
);

impl_java_class!(
    "com/longbridge/portfolio/ProfitAnalysis",
    longbridge::portfolio::ProfitAnalysis,
    [summary, sublist]
);

impl_java_class!(
    "com/longbridge/portfolio/ProfitAnalysisSummary",
    longbridge::portfolio::ProfitAnalysisSummary,
    [
        currency,
        current_total_asset,
        start_date,
        end_date,
        start_time,
        end_time,
        ending_asset_value,
        initial_asset_value,
        invest_amount,
        is_traded,
        sum_profit,
        sum_profit_rate,
        profits
    ]
);

impl_java_class!(
    "com/longbridge/portfolio/ProfitSummaryBreakdown",
    longbridge::portfolio::ProfitSummaryBreakdown,
    [
        stock,
        fund,
        crypto,
        mmf,
        other,
        cumulative_transaction_amount,
        trade_order_num,
        trade_stock_num,
        ipo,
        ipo_hit,
        ipo_subscription,
        #[java(objarray)]
        summary_info
    ]
);

impl_java_class!(
    "com/longbridge/portfolio/ProfitSummaryInfo",
    longbridge::portfolio::ProfitSummaryInfo,
    [
        asset_type,
        profit_max,
        profit_max_name,
        loss_max,
        loss_max_name
    ]
);

impl_java_class!(
    "com/longbridge/portfolio/ProfitAnalysisSublist",
    longbridge::portfolio::ProfitAnalysisSublist,
    [
        start,
        end,
        start_date,
        end_date,
        updated_at,
        updated_date,
        #[java(objarray)]
        items
    ]
);

impl_java_class!(
    "com/longbridge/portfolio/ProfitAnalysisItem",
    longbridge::portfolio::ProfitAnalysisItem,
    [
        name,
        market,
        is_holding,
        profit,
        profit_rate,
        clearance_times,
        item_type,
        currency,
        symbol,
        holding_period,
        security_code,
        isin,
        underlying_profit,
        derivatives_profit,
        order_profit
    ]
);

impl_java_class!(
    "com/longbridge/portfolio/ProfitAnalysisDetail",
    longbridge::portfolio::ProfitAnalysisDetail,
    [
        profit,
        underlying_details,
        derivative_pnl_details,
        name,
        updated_at,
        updated_date,
        currency,
        default_tag,
        start,
        end,
        start_date,
        end_date
    ]
);

impl_java_class!(
    "com/longbridge/portfolio/ProfitDetails",
    longbridge::portfolio::ProfitDetails,
    [
        holding_value,
        profit,
        cumulative_credited_amount,
        #[java(objarray)]
        credited_details,
        cumulative_debited_amount,
        #[java(objarray)]
        debited_details,
        cumulative_fee_amount,
        #[java(objarray)]
        fee_details,
        short_holding_value,
        long_holding_value,
        holding_value_at_beginning,
        holding_value_at_ending
    ]
);

impl_java_class!(
    "com/longbridge/portfolio/ProfitDetailEntry",
    longbridge::portfolio::ProfitDetailEntry,
    [describe, amount]
);

impl_java_class!(
    "com/longbridge/portfolio/ProfitAnalysisByMarketItem",
    longbridge::portfolio::ProfitAnalysisByMarketItem,
    [code, name, market, profit]
);

impl_java_class!(
    "com/longbridge/portfolio/ProfitAnalysisByMarket",
    longbridge::portfolio::ProfitAnalysisByMarket,
    [
        profit,
        has_more,
        #[java(objarray)]
        stock_items
    ]
);

// ── DcaPlan and friends ───────────────────────────────────────────

impl_java_class!(
    "com/longbridge/dca/DcaPlan",
    longbridge::dca::DcaPlan,
    [
        plan_id,
        status,
        symbol,
        member_id,
        aaid,
        account_channel,
        display_account,
        market,
        per_invest_amount,
        invest_frequency,
        invest_day_of_week,
        invest_day_of_month,
        allow_margin_finance,
        alter_hours,
        created_at,
        updated_at,
        next_trd_date,
        stock_name,
        cum_amount,
        issue_number,
        average_cost,
        cum_profit
    ]
);

impl_java_class!(
    "com/longbridge/dca/DcaList",
    longbridge::dca::DcaList,
    [
        #[java(objarray)]
        plans
    ]
);

impl_java_class!(
    "com/longbridge/dca/DcaStats",
    longbridge::dca::DcaStats,
    [
        active_count,
        finished_count,
        suspended_count,
        #[java(objarray)]
        nearest_plans,
        rest_days,
        total_amount,
        total_profit
    ]
);

impl_java_class!(
    "com/longbridge/dca/DcaCreateResult",
    longbridge::dca::DcaCreateResult,
    [plan_id]
);

// ── SharelistContext types ────────────────────────────────────────

impl_java_class!(
    "com/longbridge/sharelist/SharelistStock",
    longbridge::sharelist::SharelistStock,
    [
        symbol,
        name,
        market,
        code,
        intro,
        unread_change_log_category,
        change,
        last_done,
        trade_status,
        latency
    ]
);

impl_java_class!(
    "com/longbridge/sharelist/SharelistScopes",
    longbridge::sharelist::SharelistScopes,
    [subscription, is_self]
);

impl_java_class!(
    "com/longbridge/sharelist/SharelistInfo",
    longbridge::sharelist::SharelistInfo,
    [
        id,
        name,
        description,
        cover,
        subscribers_count,
        created_at,
        edited_at,
        this_year_chg,
        creator,
        #[java(objarray)]
        stocks,
        subscribed,
        chg,
        sharelist_type,
        industry_code
    ]
);

impl_java_class!(
    "com/longbridge/sharelist/SharelistList",
    longbridge::sharelist::SharelistList,
    [
        #[java(objarray)]
        sharelists,
        #[java(objarray)]
        subscribed_sharelists,
        tail_mark
    ]
);

impl_java_class!(
    "com/longbridge/sharelist/SharelistDetail",
    longbridge::sharelist::SharelistDetail,
    [sharelist, scopes]
);
// ── DCAContext types ──────────────────────────────────────────────

impl_java_class!(
    "com/longbridge/dca/DcaHistoryRecord",
    longbridge::dca::DcaHistoryRecord,
    [
        created_at,
        order_id,
        status,
        action,
        order_type,
        executed_qty,
        executed_price,
        executed_amount,
        rejected_reason,
        symbol
    ]
);

impl_java_class!(
    "com/longbridge/dca/DcaHistoryResponse",
    longbridge::dca::DcaHistoryResponse,
    [
        #[java(objarray)]
        records,
        has_more
    ]
);

impl_java_class!(
    "com/longbridge/dca/DcaSupportInfo",
    longbridge::dca::DcaSupportInfo,
    [symbol, support_regular_saving]
);

impl_java_class!(
    "com/longbridge/dca/DcaSupportList",
    longbridge::dca::DcaSupportList,
    [
        #[java(objarray)]
        infos
    ]
);

impl_java_class!(
    "com/longbridge/dca/DcaCalcDateResult",
    longbridge::dca::DcaCalcDateResult,
    [trade_date]
);

// DcaPlan has serde_json::Value creator field - use JSON for DcaList
// ── AlertContext types ────────────────────────────────────────────

impl_java_class!(
    "com/longbridge/alert/AlertItem",
    longbridge::alert::AlertItem,
    [
        id,
        indicator_id,
        enabled,
        frequency,
        scope,
        text,
        #[java(priarray)]
        state,
        value_map
    ]
);

impl_java_class!(
    "com/longbridge/alert/AlertSymbolGroup",
    longbridge::alert::AlertSymbolGroup,
    [
        symbol,
        code,
        market,
        name,
        price,
        chg,
        p_chg,
        product,
        #[java(objarray)]
        indicators
    ]
);

impl_java_class!(
    "com/longbridge/alert/AlertList",
    longbridge::alert::AlertList,
    [
        #[java(objarray)]
        lists
    ]
);
// ── FundamentalContext types ──────────────────────────────────────

impl_java_class!(
    "com/longbridge/fundamental/FinancialReports",
    longbridge::fundamental::FinancialReports,
    [list]
);

impl_java_class!(
    "com/longbridge/fundamental/DividendList",
    longbridge::fundamental::DividendList,
    [
        #[java(objarray)]
        list
    ]
);

impl_java_class!(
    "com/longbridge/fundamental/DividendItem",
    longbridge::fundamental::DividendItem,
    [symbol, id, desc, record_date, ex_date, payment_date]
);

impl_java_class!(
    "com/longbridge/fundamental/InstitutionRating",
    longbridge::fundamental::InstitutionRating,
    [latest, summary]
);

impl_java_class!(
    "com/longbridge/fundamental/InstitutionRatingLatest",
    longbridge::fundamental::InstitutionRatingLatest,
    [
        evaluate,
        target,
        industry_id,
        industry_name,
        industry_rank,
        industry_total,
        industry_mean,
        industry_median
    ]
);

impl_java_class!(
    "com/longbridge/fundamental/RatingEvaluate",
    longbridge::fundamental::RatingEvaluate,
    [
        buy, over, hold, under, sell, no_opinion, total, start_date, end_date
    ]
);

impl_java_class!(
    "com/longbridge/fundamental/RatingTarget",
    longbridge::fundamental::RatingTarget,
    [
        highest_price,
        lowest_price,
        prev_close,
        start_date,
        end_date
    ]
);

impl_java_class!(
    "com/longbridge/fundamental/InstitutionRatingSummary",
    longbridge::fundamental::InstitutionRatingSummary,
    [ccy_symbol, change, evaluate, recommend, target, updated_at]
);

impl_java_class!(
    "com/longbridge/fundamental/RatingSummaryEvaluate",
    longbridge::fundamental::RatingSummaryEvaluate,
    [buy, date, hold, sell, strong_buy, under]
);

impl_java_class!(
    "com/longbridge/fundamental/InstitutionRatingDetail",
    longbridge::fundamental::InstitutionRatingDetail,
    [ccy_symbol, evaluate, target]
);

impl_java_class!(
    "com/longbridge/fundamental/InstitutionRatingDetailEvaluate",
    longbridge::fundamental::InstitutionRatingDetailEvaluate,
    [
        #[java(objarray)]
        list
    ]
);

impl_java_class!(
    "com/longbridge/fundamental/InstitutionRatingDetailEvaluateItem",
    longbridge::fundamental::InstitutionRatingDetailEvaluateItem,
    [buy, date, hold, sell, strong_buy, no_opinion, under]
);

impl_java_class!(
    "com/longbridge/fundamental/InstitutionRatingDetailTarget",
    longbridge::fundamental::InstitutionRatingDetailTarget,
    [
        data_percent,
        prediction_accuracy,
        updated_at,
        #[java(objarray)]
        list
    ]
);

impl_java_class!(
    "com/longbridge/fundamental/InstitutionRatingDetailTargetItem",
    longbridge::fundamental::InstitutionRatingDetailTargetItem,
    [
        avg_target, date, max_target, min_target, meet, price, timestamp
    ]
);

impl_java_class!(
    "com/longbridge/fundamental/ForecastEps",
    longbridge::fundamental::ForecastEps,
    [
        #[java(objarray)]
        items
    ]
);

impl_java_class!(
    "com/longbridge/fundamental/ForecastEpsItem",
    longbridge::fundamental::ForecastEpsItem,
    [
        forecast_eps_median,
        forecast_eps_mean,
        forecast_eps_lowest,
        forecast_eps_highest,
        institution_total,
        institution_up,
        institution_down,
        forecast_start_date,
        forecast_end_date
    ]
);

impl_java_class!(
    "com/longbridge/fundamental/FinancialConsensus",
    longbridge::fundamental::FinancialConsensus,
    [
        #[java(objarray)]
        list,
        current_index,
        currency,
        #[java(objarray)]
        opt_periods,
        current_period
    ]
);

impl_java_class!(
    "com/longbridge/fundamental/ConsensusReport",
    longbridge::fundamental::ConsensusReport,
    [
        fiscal_year,
        fiscal_period,
        period_text,
        #[java(objarray)]
        details
    ]
);

impl_java_class!(
    "com/longbridge/fundamental/ConsensusDetail",
    longbridge::fundamental::ConsensusDetail,
    [
        key,
        name,
        description,
        actual,
        estimate,
        comp_value,
        comp_desc,
        comp,
        is_released
    ]
);

impl_java_class!(
    "com/longbridge/fundamental/ValuationData",
    longbridge::fundamental::ValuationData,
    [metrics]
);

impl_java_class!(
    "com/longbridge/fundamental/ValuationMetricsData",
    longbridge::fundamental::ValuationMetricsData,
    [pe, pb, ps, dvd_yld]
);

impl_java_class!(
    "com/longbridge/fundamental/ValuationMetricData",
    longbridge::fundamental::ValuationMetricData,
    [
        desc,
        high,
        low,
        median,
        #[java(objarray)]
        list
    ]
);

impl_java_class!(
    "com/longbridge/fundamental/ValuationPoint",
    longbridge::fundamental::ValuationPoint,
    [timestamp, value]
);

impl_java_class!(
    "com/longbridge/fundamental/ValuationHistoryResponse",
    longbridge::fundamental::ValuationHistoryResponse,
    [history]
);

impl_java_class!(
    "com/longbridge/fundamental/ValuationHistoryData",
    longbridge::fundamental::ValuationHistoryData,
    [metrics]
);

impl_java_class!(
    "com/longbridge/fundamental/ValuationHistoryMetrics",
    longbridge::fundamental::ValuationHistoryMetrics,
    [pe, pb, ps]
);

impl_java_class!(
    "com/longbridge/fundamental/ValuationHistoryMetric",
    longbridge::fundamental::ValuationHistoryMetric,
    [
        desc,
        high,
        low,
        median,
        #[java(objarray)]
        list
    ]
);

impl_java_class!(
    "com/longbridge/fundamental/IndustryValuationList",
    longbridge::fundamental::IndustryValuationList,
    [
        #[java(objarray)]
        list
    ]
);

impl_java_class!(
    "com/longbridge/fundamental/IndustryValuationItem",
    longbridge::fundamental::IndustryValuationItem,
    [
        symbol,
        name,
        currency,
        assets,
        bps,
        eps,
        dps,
        div_yld,
        div_payout_ratio,
        five_y_avg_dps,
        pe,
        #[java(objarray)]
        history
    ]
);

impl_java_class!(
    "com/longbridge/fundamental/IndustryValuationHistory",
    longbridge::fundamental::IndustryValuationHistory,
    [date, pe, pb, ps]
);

impl_java_class!(
    "com/longbridge/fundamental/IndustryValuationDist",
    longbridge::fundamental::IndustryValuationDist,
    [pe, pb, ps]
);

impl_java_class!(
    "com/longbridge/fundamental/ValuationDist",
    longbridge::fundamental::ValuationDist,
    [low, high, median, value, ranking, rank_index, rank_total]
);

impl_java_class!(
    "com/longbridge/fundamental/CompanyOverview",
    longbridge::fundamental::CompanyOverview,
    [
        name,
        company_name,
        founded,
        listing_date,
        market,
        region,
        address,
        office_address,
        website,
        issue_price,
        shares_offered,
        chairman,
        secretary,
        audit_inst,
        category,
        year_end,
        employees,
        phone,
        fax,
        email,
        legal_repr,
        manager,
        bus_license,
        accounting_firm,
        securities_rep,
        legal_counsel,
        zip_code,
        ticker,
        icon,
        profile,
        ads_ratio,
        sector
    ]
);

impl_java_class!(
    "com/longbridge/fundamental/ExecutiveList",
    longbridge::fundamental::ExecutiveList,
    [
        #[java(objarray)]
        professional_list
    ]
);

impl_java_class!(
    "com/longbridge/fundamental/ExecutiveGroup",
    longbridge::fundamental::ExecutiveGroup,
    [
        symbol,
        forward_url,
        total,
        #[java(objarray)]
        professionals
    ]
);

impl_java_class!(
    "com/longbridge/fundamental/Professional",
    longbridge::fundamental::Professional,
    [
        id, name, name_zhcn, name_en, title, biography, photo, wiki_url
    ]
);

impl_java_class!(
    "com/longbridge/fundamental/ShareholderList",
    longbridge::fundamental::ShareholderList,
    [
        #[java(objarray)]
        shareholder_list,
        forward_url,
        total
    ]
);

impl_java_class!(
    "com/longbridge/fundamental/Shareholder",
    longbridge::fundamental::Shareholder,
    [
        shareholder_id,
        shareholder_name,
        institution_type,
        percent_of_shares,
        shares_changed,
        report_date,
        #[java(objarray)]
        stocks
    ]
);

impl_java_class!(
    "com/longbridge/fundamental/ShareholderStock",
    longbridge::fundamental::ShareholderStock,
    [symbol, code, market, chg]
);

impl_java_class!(
    "com/longbridge/fundamental/FundHolders",
    longbridge::fundamental::FundHolders,
    [
        #[java(objarray)]
        lists
    ]
);

impl_java_class!(
    "com/longbridge/fundamental/FundHolder",
    longbridge::fundamental::FundHolder,
    [code, symbol, currency, name, position_ratio, report_date]
);

impl_java_class!(
    "com/longbridge/fundamental/CorpActionLive",
    longbridge::fundamental::CorpActionLive,
    [id, status, started_at, name, icon]
);

impl_java_class!(
    "com/longbridge/fundamental/CorpActions",
    longbridge::fundamental::CorpActions,
    [
        #[java(objarray)]
        items
    ]
);

impl_java_class!(
    "com/longbridge/fundamental/CorpActionItem",
    longbridge::fundamental::CorpActionItem,
    [
        id,
        date,
        date_str,
        date_type,
        date_zone,
        act_type,
        act_desc,
        action,
        recent,
        is_delay,
        delay_content,
        live,
        security
    ]
);

impl_java_class!(
    "com/longbridge/fundamental/InvestRelations",
    longbridge::fundamental::InvestRelations,
    [
        forward_url,
        #[java(objarray)]
        invest_securities
    ]
);

impl_java_class!(
    "com/longbridge/fundamental/InvestSecurity",
    longbridge::fundamental::InvestSecurity,
    [
        company_id,
        company_name,
        company_name_en,
        company_name_zhcn,
        symbol,
        currency,
        percent_of_shares,
        shares_rank,
        shares_value
    ]
);

impl_java_class!(
    "com/longbridge/fundamental/OperatingList",
    longbridge::fundamental::OperatingList,
    [
        #[java(objarray)]
        list
    ]
);

impl_java_class!(
    "com/longbridge/fundamental/OperatingItem",
    longbridge::fundamental::OperatingItem,
    [
        id,
        report,
        title,
        txt,
        latest,
        web_url,
        financial,
        #[java(objarray)]
        keywords
    ]
);

impl_java_class!(
    "com/longbridge/fundamental/OperatingFinancial",
    longbridge::fundamental::OperatingFinancial,
    [
        code,
        symbol,
        currency,
        name,
        region,
        report,
        report_txt,
        #[java(objarray)]
        indicators
    ]
);

impl_java_class!(
    "com/longbridge/fundamental/OperatingIndicator",
    longbridge::fundamental::OperatingIndicator,
    [field_name, indicator_name, indicator_value, yoy]
);

// ── QuoteContext extensions ───────────────────────────────────────

impl_java_class!(
    "com/longbridge/quote/ShortPositionsItem",
    longbridge::quote::ShortPositionsItem,
    [
        timestamp,
        rate,
        close,
        current_shares_short,
        avg_daily_share_volume,
        days_to_cover,
        amount,
        balance,
        cost
    ]
);

impl_java_class!(
    "com/longbridge/quote/ShortPositionsResponse",
    longbridge::quote::ShortPositionsResponse,
    [
        #[java(objarray)]
        data
    ]
);

impl_java_class!(
    "com/longbridge/quote/ShortTradesItem",
    longbridge::quote::ShortTradesItem,
    [
        timestamp,
        rate,
        close,
        nus_amount,
        ny_amount,
        total_amount,
        amount,
        balance
    ]
);

impl_java_class!(
    "com/longbridge/quote/ShortTradesResponse",
    longbridge::quote::ShortTradesResponse,
    [
        #[java(objarray)]
        data
    ]
);

impl_java_class!(
    "com/longbridge/quote/OptionVolumeStats",
    longbridge::quote::OptionVolumeStats,
    [c, p]
);

impl_java_class!(
    "com/longbridge/quote/OptionVolumeDaily",
    longbridge::quote::OptionVolumeDaily,
    [
        #[java(objarray)]
        stats
    ]
);

impl_java_class!(
    "com/longbridge/quote/OptionVolumeDailyStat",
    longbridge::quote::OptionVolumeDailyStat,
    [
        symbol,
        timestamp,
        total_volume,
        total_put_volume,
        total_call_volume,
        put_call_volume_ratio,
        total_open_interest,
        total_put_open_interest,
        total_call_open_interest,
        put_call_open_interest_ratio
    ]
);

// ── FundamentalContext: BuybackData and related ───────────────────

impl_java_class!(
    "com/longbridge/fundamental/RecentBuybacks",
    longbridge::fundamental::RecentBuybacks,
    [currency, net_buyback_ttm, net_buyback_yield_ttm]
);

impl_java_class!(
    "com/longbridge/fundamental/BuybackHistoryItem",
    longbridge::fundamental::BuybackHistoryItem,
    [
        fiscal_year,
        fiscal_year_range,
        net_buyback,
        net_buyback_yield,
        net_buyback_growth_rate,
        currency
    ]
);

impl_java_class!(
    "com/longbridge/fundamental/BuybackRatios",
    longbridge::fundamental::BuybackRatios,
    [net_buyback_payout_ratio, net_buyback_to_cashflow_ratio]
);

impl_java_class!(
    "com/longbridge/fundamental/BuybackData",
    longbridge::fundamental::BuybackData,
    [
        recent_buybacks,
        #[java(objarray)]
        buyback_history,
        #[java(objarray)]
        buyback_ratios
    ]
);

// ── FundamentalContext: StockRatings and related ──────────────────

impl_java_class!(
    "com/longbridge/fundamental/RatingIndicator",
    longbridge::fundamental::RatingIndicator,
    [name, score, letter]
);

impl_java_class!(
    "com/longbridge/fundamental/RatingLeafIndicator",
    longbridge::fundamental::RatingLeafIndicator,
    [name, value, value_type, score, letter]
);

impl_java_class!(
    "com/longbridge/fundamental/RatingSubIndicatorGroup",
    longbridge::fundamental::RatingSubIndicatorGroup,
    [
        indicator,
        #[java(objarray)]
        sub_indicators
    ]
);

impl_java_class!(
    "com/longbridge/fundamental/RatingCategory",
    longbridge::fundamental::RatingCategory,
    [
        kind,
        #[java(objarray)]
        sub_indicators
    ]
);

impl_java_class!(
    "com/longbridge/fundamental/StockRatings",
    longbridge::fundamental::StockRatings,
    [
        style_txt_name,
        scale_txt_name,
        report_period_txt,
        multi_score,
        multi_letter,
        multi_score_change,
        industry_name,
        industry_rank,
        industry_total,
        industry_mean_score,
        industry_median_score,
        #[java(objarray)]
        ratings
    ]
);

// ── FundamentalContext: new APIs ──────────────────────────────────

impl_java_class!(
    "com/longbridge/fundamental/BusinessSegmentItem",
    longbridge::fundamental::BusinessSegmentItem,
    [name, percent]
);

impl_java_class!(
    "com/longbridge/fundamental/BusinessSegments",
    longbridge::fundamental::BusinessSegments,
    [
        date,
        total,
        currency,
        #[java(objarray)]
        business
    ]
);

impl_java_class!(
    "com/longbridge/fundamental/BusinessSegmentHistoryItem",
    longbridge::fundamental::BusinessSegmentHistoryItem,
    [name, percent, value]
);

impl_java_class!(
    "com/longbridge/fundamental/BusinessSegmentsHistoricalItem",
    longbridge::fundamental::BusinessSegmentsHistoricalItem,
    [
        date,
        total,
        currency,
        #[java(objarray)]
        business,
        #[java(objarray)]
        regionals
    ]
);

impl_java_class!(
    "com/longbridge/fundamental/BusinessSegmentsHistory",
    longbridge::fundamental::BusinessSegmentsHistory,
    [
        #[java(objarray)]
        historical
    ]
);

impl_java_class!(
    "com/longbridge/fundamental/InstitutionRatingViewItem",
    longbridge::fundamental::InstitutionRatingViewItem,
    [date, buy, over, hold, under, sell, total]
);

impl_java_class!(
    "com/longbridge/fundamental/InstitutionRatingViews",
    longbridge::fundamental::InstitutionRatingViews,
    [
        #[java(objarray)]
        elist
    ]
);

impl_java_class!(
    "com/longbridge/fundamental/IndustryRankItem",
    longbridge::fundamental::IndustryRankItem,
    [
        name,
        counter_id,
        chg,
        leading_name,
        leading_ticker,
        leading_chg,
        value_name,
        value_data
    ]
);

impl_java_class!(
    "com/longbridge/fundamental/IndustryRankGroup",
    longbridge::fundamental::IndustryRankGroup,
    [
        #[java(objarray)]
        lists
    ]
);

impl_java_class!(
    "com/longbridge/fundamental/IndustryRankResponse",
    longbridge::fundamental::IndustryRankResponse,
    [
        #[java(objarray)]
        items
    ]
);

impl_java_class!(
    "com/longbridge/fundamental/IndustryPeersTop",
    longbridge::fundamental::IndustryPeersTop,
    [name, market]
);

// IndustryPeerNode has a recursive `next` field; we serialize it as nextJson.
// Manual impl (macro can't rename fields).
#[allow(non_upper_case_globals)]
static com_longbridge_fundamental_IndustryPeerNode: std::sync::OnceLock<jni::objects::GlobalRef> =
    std::sync::OnceLock::new();

impl crate::types::ClassLoader for longbridge::fundamental::IndustryPeerNode {
    fn init(env: &mut jni::JNIEnv) {
        let cls = jni::descriptors::Desc::<jni::objects::JClass>::lookup(
            "com/longbridge/fundamental/IndustryPeerNode",
            env,
        )
        .expect("com/longbridge/fundamental/IndustryPeerNode");
        let _ = com_longbridge_fundamental_IndustryPeerNode.set(env.new_global_ref(&*cls).unwrap());
    }

    fn class_ref() -> jni::objects::GlobalRef {
        com_longbridge_fundamental_IndustryPeerNode
            .get()
            .cloned()
            .unwrap()
    }
}

impl crate::types::JSignature for longbridge::fundamental::IndustryPeerNode {
    #[inline]
    fn signature() -> ::std::borrow::Cow<'static, str> {
        "Lcom/longbridge/fundamental/IndustryPeerNode;".into()
    }
}

impl crate::types::IntoJValue for longbridge::fundamental::IndustryPeerNode {
    fn into_jvalue<'a>(
        self,
        env: &mut jni::JNIEnv<'a>,
    ) -> jni::errors::Result<jni::objects::JValueOwned<'a>> {
        let longbridge::fundamental::IndustryPeerNode {
            name,
            counter_id,
            stock_num,
            chg,
            ytd_chg,
            next,
        } = self;
        let next_json = serde_json::to_string(&next).unwrap_or_default();
        let cls = <Self as crate::types::ClassLoader>::class_ref();
        let obj = env.new_object(cls.borrow(), "()V", &[])?;
        crate::types::set_field(env, &obj, "name", name)?;
        crate::types::set_field(env, &obj, "counterId", counter_id)?;
        crate::types::set_field(env, &obj, "stockNum", stock_num)?;
        crate::types::set_field(env, &obj, "chg", chg)?;
        crate::types::set_field(env, &obj, "ytdChg", ytd_chg)?;
        crate::types::set_field(env, &obj, "nextJson", next_json)?;
        Ok(obj.into())
    }
}

impl_java_class!(
    "com/longbridge/fundamental/IndustryPeersResponse",
    longbridge::fundamental::IndustryPeersResponse,
    [top, chain]
);

impl_java_class!(
    "com/longbridge/fundamental/SnapshotForecastMetric",
    longbridge::fundamental::SnapshotForecastMetric,
    [value, yoy, cmp_desc, est_value]
);

impl_java_class!(
    "com/longbridge/fundamental/SnapshotReportedMetric",
    longbridge::fundamental::SnapshotReportedMetric,
    [value, yoy]
);

impl_java_class!(
    "com/longbridge/fundamental/FinancialReportSnapshot",
    longbridge::fundamental::FinancialReportSnapshot,
    [
        name,
        ticker,
        fp_start,
        fp_end,
        currency,
        report_desc,
        #[java(nullable)]
        fo_revenue,
        #[java(nullable)]
        fo_ebit,
        #[java(nullable)]
        fo_eps,
        #[java(nullable)]
        fr_revenue,
        #[java(nullable)]
        fr_profit,
        #[java(nullable)]
        fr_operate_cash,
        #[java(nullable)]
        fr_invest_cash,
        #[java(nullable)]
        fr_finance_cash,
        #[java(nullable)]
        fr_total_assets,
        #[java(nullable)]
        fr_total_liability,
        fr_roe_ttm,
        fr_profit_margin,
        fr_profit_margin_ttm,
        fr_asset_turn_ttm,
        fr_leverage_ttm,
        fr_debt_assets_ratio
    ]
);

// ── PortfolioContext: ProfitAnalysisFlows and related ─────────────

impl_java_class!(
    "com/longbridge/portfolio/FlowItem",
    longbridge::portfolio::FlowItem,
    [
        executed_date,
        executed_timestamp,
        code,
        direction,
        executed_quantity,
        executed_price,
        executed_cost,
        describe
    ]
);

impl_java_class!(
    "com/longbridge/portfolio/ProfitAnalysisFlowsResponse",
    longbridge::portfolio::ProfitAnalysisFlows,
    [
        #[java(objarray)]
        flows_list,
        has_more
    ]
);

// ── FundamentalContext: shareholders / valuation comparison ────────

impl_java_class!(
    "com/longbridge/fundamental/ShareholderTopResponse",
    longbridge::fundamental::ShareholderTopResponse,
    [data]
);

impl_java_class!(
    "com/longbridge/fundamental/ShareholderDetailResponse",
    longbridge::fundamental::ShareholderDetailResponse,
    [data]
);

impl_java_class!(
    "com/longbridge/fundamental/ValuationHistoryPoint",
    longbridge::fundamental::ValuationHistoryPoint,
    [date, pe, pb, ps]
);

impl_java_class!(
    "com/longbridge/fundamental/ValuationComparisonItem",
    longbridge::fundamental::ValuationComparisonItem,
    [
        symbol,
        name,
        currency,
        market_value,
        price_close,
        pe,
        pb,
        ps,
        roe,
        eps,
        bps,
        dps,
        div_yld,
        assets,
        #[java(objarray)]
        history
    ]
);

impl_java_class!(
    "com/longbridge/fundamental/ValuationComparisonResponse",
    longbridge::fundamental::ValuationComparisonResponse,
    [
        #[java(objarray)]
        list
    ]
);

impl_java_class!(
    "com/longbridge/fundamental/MultiLanguageText",
    longbridge::fundamental::MultiLanguageText,
    [english, simplified_chinese, traditional_chinese]
);

impl_java_class!(
    "com/longbridge/fundamental/MacroeconomicIndicator",
    longbridge::fundamental::MacroeconomicIndicator,
    [
        indicator_code,
        source_org,
        country,
        name,
        adjustment_factor,
        periodicity,
        category,
        describe,
        importance,
        start_date
    ]
);

impl_java_class!(
    "com/longbridge/fundamental/Macroeconomic",
    longbridge::fundamental::Macroeconomic,
    [
        period,
        release_at,
        actual_value,
        previous_value,
        forecast_value,
        revised_value,
        next_release_at,
        unit,
        unit_prefix
    ]
);

impl_java_class!(
    "com/longbridge/fundamental/MacroeconomicIndicatorListResponse",
    longbridge::fundamental::MacroeconomicIndicatorListResponse,
    [
        #[java(objarray)]
        data,
        count
    ]
);

impl_java_class!(
    "com/longbridge/fundamental/MacroeconomicResponse",
    longbridge::fundamental::MacroeconomicResponse,
    [
        info,
        #[java(objarray)]
        data,
        count
    ]
);

// ── MarketContext: top movers / rank ──────────────────────────────

impl_java_class!(
    "com/longbridge/market/TopMoversStock",
    longbridge::market::TopMoversStock,
    [
        symbol,
        code,
        name,
        full_name,
        change,
        last_done,
        market,
        #[java(objarray)]
        labels,
        logo
    ]
);

impl_java_class!(
    "com/longbridge/market/TopMoversEvent",
    longbridge::market::TopMoversEvent,
    [timestamp, alert_reason, alert_type, stock, post]
);

impl_java_class!(
    "com/longbridge/market/TopMoversResponse",
    longbridge::market::TopMoversResponse,
    [
        #[java(objarray)]
        events,
        next_params
    ]
);

impl_java_class!(
    "com/longbridge/market/RankCategoriesResponse",
    longbridge::market::RankCategoriesResponse,
    [data]
);

impl_java_class!(
    "com/longbridge/market/RankListItem",
    longbridge::market::RankListItem,
    [
        symbol,
        code,
        name,
        last_done,
        chg,
        change,
        inflow,
        market_cap,
        industry,
        pre_post_price,
        pre_post_chg,
        amplitude,
        five_day_chg,
        turnover_rate,
        volume_rate,
        pb_ttm
    ]
);

impl_java_class!(
    "com/longbridge/market/RankListResponse",
    longbridge::market::RankListResponse,
    [
        bmp,
        #[java(objarray)]
        lists
    ]
);

// ── ScreenerContext ───────────────────────────────────────────────

impl_java_class!(
    "com/longbridge/screener/ScreenerRecommendStrategiesResponse",
    longbridge::screener::ScreenerRecommendStrategiesResponse,
    [data]
);

impl_java_class!(
    "com/longbridge/screener/ScreenerUserStrategiesResponse",
    longbridge::screener::ScreenerUserStrategiesResponse,
    [data]
);

impl_java_class!(
    "com/longbridge/screener/ScreenerStrategyResponse",
    longbridge::screener::ScreenerStrategyResponse,
    [data]
);

impl_java_class!(
    "com/longbridge/screener/ScreenerSearchResponse",
    longbridge::screener::ScreenerSearchResponse,
    [data]
);

impl_java_class!(
    "com/longbridge/screener/ScreenerIndicatorsResponse",
    longbridge::screener::ScreenerIndicatorsResponse,
    [data]
);

// ── AgentContext types ─────────────────────────────────────────────

impl_java_class!(
    "com/longbridge/agent/Workspace",
    longbridge::agent::Workspace,
    [id, name, created_at, updated_at]
);

impl_java_class!(
    "com/longbridge/agent/WorkspacesResponse",
    longbridge::agent::WorkspacesResponse,
    [
        #[java(objarray)]
        workspaces
    ]
);

impl_java_class!(
    "com/longbridge/agent/Agent",
    longbridge::agent::Agent,
    [
        uid,
        name,
        description,
        mode,
        icon,
        is_published,
        published_at,
        created_at,
        updated_at
    ]
);

impl_java_class!(
    "com/longbridge/agent/AgentsResponse",
    longbridge::agent::AgentsResponse,
    [
        #[java(objarray)]
        agents,
        total
    ]
);

impl_java_class!(
    "com/longbridge/agent/Reference",
    longbridge::agent::Reference,
    [index, title, url]
);

impl_java_class!(
    "com/longbridge/agent/QuestionOption",
    longbridge::agent::QuestionOption,
    [description]
);

impl_java_class!(
    "com/longbridge/agent/Question",
    longbridge::agent::Question,
    [
        question,
        #[java(objarray)]
        options,
        multi_select
    ]
);

impl_java_class!(
    "com/longbridge/agent/Interrupt",
    longbridge::agent::Interrupt,
    [
        node_id,
        tool_call_id,
        #[java(objarray)]
        questions,
        message_id,
        chat_id
    ]
);

// The Java class is named `ConversationError` (not `AgentError`) since it
// describes a failed conversation *run*, not a JNI/transport-level failure —
// `OpenApiException` already owns that role for this SDK. `impl_java_class!`
// only cares that the two names are given independently, so no wrapper type
// is needed just to rename it.
impl_java_class!(
    "com/longbridge/agent/ConversationError",
    longbridge::agent::AgentError,
    [code, message]
);

impl_java_class!(
    "com/longbridge/agent/ChatStartedEvent",
    longbridge::agent::ChatStartedPayload,
    [chat_uid, message_id]
);

// JNI-side view of `longbridge::agent::WorkflowStartedInputs`, the `inputs`
// sub-object of a `workflow_started` event — mapped to its own Java class
// the same way `Interrupt`/`Question` nest inside `ConversationResponse`
// above.
impl_java_class!(
    "com/longbridge/agent/WorkflowStartedInputs",
    longbridge::agent::WorkflowStartedInputs,
    [chat_id, chat_uid, message_id, query]
);

impl_java_class!(
    "com/longbridge/agent/WorkflowStartedEvent",
    longbridge::agent::WorkflowStartedPayload,
    [hit_cache, inputs, started_at, workflow_id]
);

impl_java_class!(
    "com/longbridge/agent/MessageEvent",
    longbridge::agent::MessagePayload,
    [
        text,
        message_type,
        key,
        started_at,
        stage,
        stage_title,
        stage_finished_title,
        outputs
    ]
);

/// JNI-side marker for
/// [`longbridge::agent::ConversationStreamEvent::Ping`], which — unlike
/// every other variant — carries no payload at all. `impl_java_class!` works
/// fine against a zero-field struct, so this is used instead of hand-writing
/// a bespoke JNI object constructor for the one variant with nothing to
/// carry. See the manual `IntoJValue` for `ConversationStreamEvent` below,
/// the only place this type is used.
pub(crate) struct PingEvent {}

impl_java_class!("com/longbridge/agent/PingEvent", PingEvent, []);

impl_java_class!(
    "com/longbridge/agent/ThinkingStartedEvent",
    longbridge::agent::ThinkingStartedPayload,
    [started_at]
);

impl_java_class!(
    "com/longbridge/agent/ThinkingFinishedEvent",
    longbridge::agent::ThinkingFinishedPayload,
    [finished_at, elapsed_time]
);

impl_java_class!(
    "com/longbridge/agent/NodeToolUseStartedEvent",
    longbridge::agent::NodeToolUseStartedPayload,
    [
        tool_use_id,
        tool_name,
        tool_func_name,
        tool_args,
        tips,
        #[java(objarray)]
        tip_chips,
        iteration,
        started_at
    ]
);

/// JNI-side view of [`longbridge::agent::NodeToolUseOutputs`], with
/// `references`/`reference_domains` normalized from `Option<Vec<_>>` down to a
/// plain `Vec` (empty when absent) — same convention as
/// [`ConversationResponse`]'s `references` field above.
pub(crate) struct NodeToolUseOutputs {
    pub(crate) references: Vec<longbridge::agent::Reference>,
    pub(crate) reference_domains: Vec<String>,
    pub(crate) query: Option<String>,
    pub(crate) text: Option<String>,
    pub(crate) tool_args: Option<serde_json::Value>,
    pub(crate) data: Option<serde_json::Value>,
}

impl From<longbridge::agent::NodeToolUseOutputs> for NodeToolUseOutputs {
    fn from(value: longbridge::agent::NodeToolUseOutputs) -> Self {
        Self {
            references: value.references.unwrap_or_default(),
            reference_domains: value.reference_domains.unwrap_or_default(),
            query: value.query,
            text: value.text,
            tool_args: value.tool_args,
            data: value.data,
        }
    }
}

impl_java_class!(
    "com/longbridge/agent/NodeToolUseOutputs",
    NodeToolUseOutputs,
    [
        #[java(objarray)]
        references,
        #[java(objarray)]
        reference_domains,
        query,
        text,
        tool_args,
        data
    ]
);

impl_java_class!(
    "com/longbridge/agent/NodeToolUseFinishedEvent",
    longbridge::agent::NodeToolUseFinishedPayload,
    [
        tool_use_id,
        status,
        error,
        elapsed_time,
        started_at,
        tool_name,
        tool_func_name,
        tool_args,
        tool_type,
        tips,
        #[java(objarray)]
        tip_chips,
        iteration,
        is_thinking,
        #[java(set_as = crate::types::NodeToolUseOutputs)]
        outputs
    ]
);

impl_java_class!(
    "com/longbridge/agent/SubagentStartedEvent",
    longbridge::agent::SubagentStartedPayload,
    [
        node_id,
        tool_use_id,
        started_at,
        goal,
        prompt,
        subagent_id,
        #[java(objarray)]
        tools
    ]
);

impl_java_class!(
    "com/longbridge/agent/SubagentProgressEvent",
    longbridge::agent::SubagentProgressPayload,
    [
        node_id,
        parent_tool_call_id,
        subagent_tool_name,
        subagent_tool_args,
        subagent_status,
        subagent_duration_ms,
        subagent_iteration,
        started_at
    ]
);

/// JNI-side view of [`longbridge::agent::SubagentOutputs`], with
/// `subagent_tools` normalized from `Option<Vec<_>>` down to a plain `Vec`
/// (empty when absent) — same convention as [`NodeToolUseOutputs`] above.
pub(crate) struct SubagentOutputs {
    pub(crate) goal: Option<String>,
    pub(crate) result: Option<String>,
    pub(crate) subagent_tools: Vec<serde_json::Value>,
}

impl From<longbridge::agent::SubagentOutputs> for SubagentOutputs {
    fn from(value: longbridge::agent::SubagentOutputs) -> Self {
        Self {
            goal: value.goal,
            result: value.result,
            subagent_tools: value.subagent_tools.unwrap_or_default(),
        }
    }
}

impl_java_class!(
    "com/longbridge/agent/SubagentOutputs",
    SubagentOutputs,
    [
        goal,
        result,
        #[java(objarray)]
        subagent_tools
    ]
);

impl_java_class!(
    "com/longbridge/agent/SubagentFinishedEvent",
    longbridge::agent::SubagentFinishedPayload,
    [
        node_id,
        tool_use_id,
        status,
        started_at,
        elapsed_time,
        error,
        #[java(set_as = crate::types::SubagentOutputs)]
        outputs
    ]
);

impl_java_class!(
    "com/longbridge/agent/AgentToolStartedEvent",
    longbridge::agent::AgentToolStartedPayload,
    [
        node_id,
        tool_use_id,
        agent_tool_name,
        title,
        started_at,
        tool_args,
        tool_name,
        tips,
        #[java(objarray)]
        tip_chips,
        is_thinking
    ]
);

impl_java_class!(
    "com/longbridge/agent/AgentToolProgressEvent",
    longbridge::agent::AgentToolProgressPayload,
    [
        node_id,
        parent_tool_call_id,
        agent_tool_name,
        inner_tool_name,
        inner_tool_args,
        status,
        duration_ms,
        started_at,
        is_thinking
    ]
);

impl_java_class!(
    "com/longbridge/agent/AgentToolFinishedEvent",
    longbridge::agent::AgentToolFinishedPayload,
    [
        node_id,
        tool_use_id,
        agent_tool_name,
        status,
        started_at,
        elapsed_time,
        error,
        tool_args,
        outputs,
        tool_type,
        tips,
        #[java(objarray)]
        tip_chips,
        is_thinking
    ]
);

/// JNI-side wrapper so that
/// [`longbridge::agent::ConversationStreamEvent::HumanInteractionRequired`]
/// can go through the same `impl_java_class!` machinery as every other event
/// payload, wrapping the same [`ConversationResponse`] that
/// [`WorkflowFinishedEvent`] wraps — see the manual `IntoJValue` for
/// `ConversationStreamEvent` below, which is the only place this type is
/// used.
pub(crate) struct HumanInteractionRequiredEvent {
    pub(crate) response: ConversationResponse,
}

impl_java_class!(
    "com/longbridge/agent/HumanInteractionRequiredEvent",
    HumanInteractionRequiredEvent,
    [response]
);

impl_java_class!(
    "com/longbridge/agent/QueryMaskedEvent",
    longbridge::agent::QueryMaskedPayload,
    [raw_query, masked_query]
);

impl_java_class!(
    "com/longbridge/agent/PlanChangedEvent",
    longbridge::agent::PlanChangedPayload,
    [node_id, started_at, outputs, tool_name]
);

impl_java_class!(
    "com/longbridge/agent/ContextCompressStartedEvent",
    longbridge::agent::ContextCompressStartedPayload,
    [started_at, inputs]
);

impl_java_class!(
    "com/longbridge/agent/ContextCompressFinishedEvent",
    longbridge::agent::ContextCompressFinishedPayload,
    [created_at, inputs, outputs]
);

impl_java_class!(
    "com/longbridge/agent/ChatFinishedEvent",
    longbridge::agent::ChatFinishedPayload,
    [chat_id, chat_uid, message_id, error, error_message]
);

/// JNI-side view of [`longbridge::agent::ConversationResponse`], with
/// `references` normalized from `Option<Vec<Reference>>` down to a plain
/// `Vec` (empty when absent) so it can use the same `#[java(objarray)]`
/// convention as every other list field — mirrors how `StockPosition` above
/// collapses `Option<Decimal>`/`Option<i64>` fields with `unwrap_or_default`.
pub(crate) struct ConversationResponse {
    pub(crate) chat_uid: String,
    pub(crate) message_id: String,
    pub(crate) status: longbridge::agent::ConversationStatus,
    pub(crate) answer: String,
    pub(crate) references: Vec<longbridge::agent::Reference>,
    pub(crate) elapsed_time: f64,
    pub(crate) interrupt: Option<longbridge::agent::Interrupt>,
    pub(crate) error: Option<longbridge::agent::AgentError>,
}

impl From<longbridge::agent::ConversationResponse> for ConversationResponse {
    fn from(value: longbridge::agent::ConversationResponse) -> Self {
        Self {
            chat_uid: value.chat_uid,
            message_id: value.message_id,
            status: value.status,
            answer: value.answer,
            references: value.references.unwrap_or_default(),
            elapsed_time: value.elapsed_time,
            interrupt: value.interrupt,
            error: value.error,
        }
    }
}

impl_java_class!(
    "com/longbridge/agent/ConversationResponse",
    ConversationResponse,
    [
        chat_uid,
        message_id,
        status,
        answer,
        #[java(objarray)]
        references,
        elapsed_time,
        interrupt,
        error
    ]
);

/// JNI-side wrapper so that
/// [`longbridge::agent::ConversationStreamEvent::WorkflowFinished`] can go
/// through the same `impl_java_class!` machinery as every other event
/// payload — see the manual `IntoJValue` for `ConversationStreamEvent` below,
/// which is the only place this type is used.
pub(crate) struct WorkflowFinishedEvent {
    pub(crate) response: ConversationResponse,
}

impl_java_class!(
    "com/longbridge/agent/WorkflowFinishedEvent",
    WorkflowFinishedEvent,
    [response]
);

impl_java_class!(
    "com/longbridge/agent/ChatTitleUpdatedEvent",
    longbridge::agent::ChatTitleUpdatedPayload,
    [chat_id, chat_uid, source, title, updated_at]
);

/// JNI-side wrapper for
/// [`longbridge::agent::ConversationStreamEvent::Other`], carrying the SSE
/// envelope's `event` type name alongside the raw event JSON as a string
/// (same convention as `AlertItem.valueMap`).
pub(crate) struct OtherEvent {
    pub(crate) event: String,
    pub(crate) json: serde_json::Value,
}

impl_java_class!("com/longbridge/agent/OtherEvent", OtherEvent, [event, json]);

/// `ConversationStreamEvent` is an enum-with-payload, so unlike every type
/// above it can't go through `impl_java_class!` (which only knows how to
/// build a single Java class by reflecting named struct fields onto it).
/// Instead each variant is modeled as its own Java subclass of the
/// `ConversationStreamEvent` sealed-style hierarchy
/// (`ChatStartedEvent`/`WorkflowStartedEvent`/`MessageEvent`/`PingEvent`/
/// `ThinkingStartedEvent`/`ThinkingFinishedEvent`/`NodeToolUseStartedEvent`/
/// `NodeToolUseFinishedEvent`/`SubagentStartedEvent`/`SubagentProgressEvent`/
/// `SubagentFinishedEvent`/`AgentToolStartedEvent`/`AgentToolProgressEvent`/
/// `AgentToolFinishedEvent`/`HumanInteractionRequiredEvent`/
/// `QueryMaskedEvent`/`PlanChangedEvent`/`ContextCompressStartedEvent`/
/// `ContextCompressFinishedEvent`/`ChatFinishedEvent`/`WorkflowFinishedEvent`/
/// `ChatTitleUpdatedEvent`/`OtherEvent`), and this manual `IntoJValue` picks
/// the right one — the resulting object reference is passed to
/// `Flow.Subscriber.onNext(Object)` as-is (generics are erased on the Java
/// side, so any subclass of the sealed base is a valid argument there).
impl crate::types::IntoJValue for longbridge::agent::ConversationStreamEvent {
    fn into_jvalue<'a>(
        self,
        env: &mut jni::JNIEnv<'a>,
    ) -> jni::errors::Result<jni::objects::JValueOwned<'a>> {
        use longbridge::agent::ConversationStreamEvent::*;
        match self {
            ChatStarted(payload) => payload.into_jvalue(env),
            WorkflowStarted(payload) => payload.into_jvalue(env),
            Message(payload) => payload.into_jvalue(env),
            Ping => PingEvent {}.into_jvalue(env),
            ThinkingStarted(payload) => payload.into_jvalue(env),
            ThinkingFinished(payload) => payload.into_jvalue(env),
            NodeToolUseStarted(payload) => payload.into_jvalue(env),
            NodeToolUseFinished(payload) => payload.into_jvalue(env),
            SubagentStarted(payload) => payload.into_jvalue(env),
            SubagentProgress(payload) => payload.into_jvalue(env),
            SubagentFinished(payload) => payload.into_jvalue(env),
            AgentToolStarted(payload) => payload.into_jvalue(env),
            AgentToolProgress(payload) => payload.into_jvalue(env),
            AgentToolFinished(payload) => payload.into_jvalue(env),
            HumanInteractionRequired(resp) => HumanInteractionRequiredEvent {
                response: ConversationResponse::from(resp),
            }
            .into_jvalue(env),
            QueryMasked(payload) => payload.into_jvalue(env),
            PlanChanged(payload) => payload.into_jvalue(env),
            ContextCompressStarted(payload) => payload.into_jvalue(env),
            ContextCompressFinished(payload) => payload.into_jvalue(env),
            ChatFinished(payload) => payload.into_jvalue(env),
            WorkflowFinished(resp) => WorkflowFinishedEvent {
                response: ConversationResponse::from(resp),
            }
            .into_jvalue(env),
            ChatTitleUpdated(payload) => payload.into_jvalue(env),
            Other { event, data } => OtherEvent { event, json: data }.into_jvalue(env),
        }
    }
}
