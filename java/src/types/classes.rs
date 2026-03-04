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
    [symbol, market, name, watched_price, watched_at]
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
