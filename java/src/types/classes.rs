use std::borrow::Borrow;

use longport::Decimal;
use longport_java_macros::impl_java_class;
use time::Date;

impl_java_class!(
    "com/longport/quote/Trade",
    longport::quote::Trade,
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
    "com/longport/quote/Brokers",
    longport::quote::Brokers,
    [
        position,
        #[java(priarray)]
        broker_ids
    ]
);

impl_java_class!(
    "com/longport/quote/Depth",
    longport::quote::Depth,
    [position, price, volume, order_num]
);

impl_java_class!(
    "com/longport/quote/Subscription",
    longport::quote::Subscription,
    [
        symbol,
        sub_types,
        #[java(objarray)]
        candlesticks
    ]
);

impl_java_class!(
    "com/longport/quote/PushQuote",
    longport::quote::PushQuote,
    [
        last_done,
        open,
        high,
        low,
        timestamp,
        volume,
        turnover,
        trade_status,
        trade_session
    ]
);

impl_java_class!(
    "com/longport/quote/PushDepth",
    longport::quote::PushDepth,
    [
        #[java(objarray)]
        asks,
        #[java(objarray)]
        bids
    ]
);

impl_java_class!(
    "com/longport/quote/PushBrokers",
    longport::quote::PushBrokers,
    [
        #[java(objarray)]
        ask_brokers,
        #[java(objarray)]
        bid_brokers
    ]
);

impl_java_class!(
    "com/longport/quote/PushTrades",
    longport::quote::PushTrades,
    [
        #[java(objarray)]
        trades,
    ]
);

impl_java_class!(
    "com/longport/quote/PushCandlestick",
    longport::quote::PushCandlestick,
    [period, candlestick]
);

impl_java_class!(
    "com/longport/quote/SecurityStaticInfo",
    longport::quote::SecurityStaticInfo,
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
        #[java(derivative_types)]
        stock_derivatives,
        board,
    ]
);

impl_java_class!(
    "com/longport/quote/PrePostQuote",
    longport::quote::PrePostQuote,
    [last_done, timestamp, volume, turnover, high, low, prev_close]
);

impl_java_class!(
    "com/longport/quote/SecurityQuote",
    longport::quote::SecurityQuote,
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
        post_market_quote
    ]
);

impl_java_class!(
    "com/longport/quote/OptionQuote",
    longport::quote::OptionQuote,
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
    "com/longport/quote/WarrantQuote",
    longport::quote::WarrantQuote,
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
    "com/longport/quote/SecurityDepth",
    longport::quote::SecurityDepth,
    [
        #[java(objarray)]
        asks,
        #[java(objarray)]
        bids
    ]
);

impl_java_class!(
    "com/longport/quote/SecurityBrokers",
    longport::quote::SecurityBrokers,
    [
        #[java(objarray)]
        ask_brokers,
        #[java(objarray)]
        bid_brokers
    ]
);

impl_java_class!(
    "com/longport/quote/ParticipantInfo",
    longport::quote::ParticipantInfo,
    [
        #[java(priarray)]
        broker_ids,
        name_cn,
        name_en,
        name_hk
    ]
);

impl_java_class!(
    "com/longport/quote/IntradayLine",
    longport::quote::IntradayLine,
    [price, timestamp, volume, turnover, avg_price]
);

impl_java_class!(
    "com/longport/quote/Candlestick",
    longport::quote::Candlestick,
    [close, open, low, high, volume, turnover, timestamp]
);

impl_java_class!(
    "com/longport/quote/StrikePriceInfo",
    longport::quote::StrikePriceInfo,
    [price, call_symbol, put_symbol, standard]
);

impl_java_class!(
    "com/longport/quote/IssuerInfo",
    longport::quote::IssuerInfo,
    [issuer_id, name_cn, name_en, name_hk]
);

impl_java_class!(
    "com/longport/quote/MarketTradingSession",
    longport::quote::MarketTradingSession,
    [
        market,
        #[java(objarray)]
        trade_sessions
    ]
);

impl_java_class!(
    "com/longport/quote/TradingSessionInfo",
    longport::quote::TradingSessionInfo,
    [begin_time, end_time, trade_session]
);

impl_java_class!(
    "com/longport/quote/MarketTradingDays",
    longport::quote::MarketTradingDays,
    [
        #[java(objarray)]
        trading_days,
        #[java(objarray)]
        half_trading_days
    ]
);

impl_java_class!(
    "com/longport/quote/CapitalFlowLine",
    longport::quote::CapitalFlowLine,
    [inflow, timestamp]
);

impl_java_class!(
    "com/longport/quote/CapitalDistribution",
    longport::quote::CapitalDistribution,
    [large, medium, small]
);

pub(crate) struct SecurityCalcIndex {
    pub(crate) symbol: String,
    pub(crate) last_done: Option<Decimal>,
    pub(crate) change_value: Option<Decimal>,
    pub(crate) change_rate: f64,
    pub(crate) volume: i64,
    pub(crate) turnover: Option<Decimal>,
    pub(crate) ytd_change_rate: f64,
    pub(crate) turnover_rate: f64,
    pub(crate) total_market_value: Option<Decimal>,
    pub(crate) capital_flow: Option<Decimal>,
    pub(crate) amplitude: f64,
    pub(crate) volume_ratio: f64,
    pub(crate) pe_ttm_ratio: f64,
    pub(crate) pb_ratio: f64,
    pub(crate) dividend_ratio_ttm: f64,
    pub(crate) five_day_change_rate: f64,
    pub(crate) ten_day_change_rate: f64,
    pub(crate) half_year_change_rate: f64,
    pub(crate) five_minutes_change_rate: f64,
    pub(crate) expiry_date: Option<Date>,
    pub(crate) strike_price: Option<Decimal>,
    pub(crate) upper_strike_price: Option<Decimal>,
    pub(crate) lower_strike_price: Option<Decimal>,
    pub(crate) outstanding_qty: i64,
    pub(crate) outstanding_ratio: f64,
    pub(crate) premium: f64,
    pub(crate) itm_otm: f64,
    pub(crate) implied_volatility: f64,
    pub(crate) warrant_delta: f64,
    pub(crate) call_price: Option<Decimal>,
    pub(crate) to_call_price: Option<Decimal>,
    pub(crate) effective_leverage: f64,
    pub(crate) leverage_ratio: f64,
    pub(crate) conversion_ratio: f64,
    pub(crate) balance_point: f64,
    pub(crate) open_interest: i64,
    pub(crate) delta: f64,
    pub(crate) gamma: f64,
    pub(crate) theta: f64,
    pub(crate) vega: f64,
    pub(crate) rho: f64,
}

impl From<longport::quote::SecurityCalcIndex> for SecurityCalcIndex {
    fn from(
        longport::quote::SecurityCalcIndex {
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
        }: longport::quote::SecurityCalcIndex,
    ) -> Self {
        Self {
            symbol,
            last_done,
            change_value,
            change_rate: change_rate.unwrap_or_default(),
            volume: volume.unwrap_or_default(),
            turnover,
            ytd_change_rate: ytd_change_rate.unwrap_or_default(),
            turnover_rate: turnover_rate.unwrap_or_default(),
            total_market_value,
            capital_flow,
            amplitude: amplitude.unwrap_or_default(),
            volume_ratio: volume_ratio.unwrap_or_default(),
            pe_ttm_ratio: pe_ttm_ratio.unwrap_or_default(),
            pb_ratio: pb_ratio.unwrap_or_default(),
            dividend_ratio_ttm: dividend_ratio_ttm.unwrap_or_default(),
            five_day_change_rate: five_day_change_rate.unwrap_or_default(),
            ten_day_change_rate: ten_day_change_rate.unwrap_or_default(),
            half_year_change_rate: half_year_change_rate.unwrap_or_default(),
            five_minutes_change_rate: five_minutes_change_rate.unwrap_or_default(),
            expiry_date,
            strike_price,
            upper_strike_price,
            lower_strike_price,
            outstanding_qty: outstanding_qty.unwrap_or_default(),
            outstanding_ratio: outstanding_ratio.unwrap_or_default(),
            premium: premium.unwrap_or_default(),
            itm_otm: itm_otm.unwrap_or_default(),
            implied_volatility: implied_volatility.unwrap_or_default(),
            warrant_delta: warrant_delta.unwrap_or_default(),
            call_price,
            to_call_price,
            effective_leverage: effective_leverage.unwrap_or_default(),
            leverage_ratio: leverage_ratio.unwrap_or_default(),
            conversion_ratio: conversion_ratio.unwrap_or_default(),
            balance_point: balance_point.unwrap_or_default(),
            open_interest: open_interest.unwrap_or_default(),
            delta: delta.unwrap_or_default(),
            gamma: gamma.unwrap_or_default(),
            theta: theta.unwrap_or_default(),
            vega: vega.unwrap_or_default(),
            rho: rho.unwrap_or_default(),
        }
    }
}

impl_java_class!(
    "com/longport/quote/SecurityCalcIndex",
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
    "com/longport/quote/WatchlistGroup",
    longport::quote::WatchlistGroup,
    [
        id,
        name,
        #[java(objarray)]
        securities
    ]
);

impl_java_class!(
    "com/longport/quote/WatchlistSecurity",
    longport::quote::WatchlistSecurity,
    [symbol, market, name, watched_price, watched_at]
);

pub(crate) struct CreateWatchlistGroupResponse {
    pub(crate) id: i64,
}

impl_java_class!(
    "com/longport/quote/CreateWatchlistGroupResponse",
    CreateWatchlistGroupResponse,
    [id]
);

impl_java_class!(
    "com/longport/quote/CapitalDistributionResponse",
    longport::quote::CapitalDistributionResponse,
    [timestamp, capital_in, capital_out]
);

impl_java_class!(
    "com/longport/quote/RealtimeQuote",
    longport::quote::RealtimeQuote,
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
    "com/longport/trade/PushOrderChanged",
    longport::trade::PushOrderChanged,
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
        last_price
    ]
);

impl_java_class!(
    "com/longport/trade/Execution",
    longport::trade::Execution,
    [order_id, trade_id, symbol, trade_done_at, quantity, price]
);

impl_java_class!(
    "com/longport/trade/Order",
    longport::trade::Order,
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
        remark
    ]
);

impl_java_class!(
    "com/longport/trade/SubmitOrderResponse",
    longport::trade::SubmitOrderResponse,
    [order_id]
);

impl_java_class!(
    "com/longport/trade/CashInfo",
    longport::trade::CashInfo,
    [
        withdraw_cash,
        available_cash,
        frozen_cash,
        settling_cash,
        currency
    ]
);

impl_java_class!(
    "com/longport/trade/AccountBalance",
    longport::trade::AccountBalance,
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
        maintenance_margin
    ]
);

impl_java_class!(
    "com/longport/trade/CashFlow",
    longport::trade::CashFlow,
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
    "com/longport/trade/FundPositionsResponse",
    longport::trade::FundPositionsResponse,
    [
        #[java(objarray)]
        channels
    ]
);

impl_java_class!(
    "com/longport/trade/FundPositionChannel",
    longport::trade::FundPositionChannel,
    [
        account_channel,
        #[java(objarray)]
        positions
    ]
);

impl_java_class!(
    "com/longport/trade/FundPosition",
    longport::trade::FundPosition,
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

impl_java_class!(
    "com/longport/trade/StockPositionsResponse",
    longport::trade::StockPositionsResponse,
    [
        #[java(objarray)]
        channels
    ]
);

impl_java_class!(
    "com/longport/trade/StockPositionChannel",
    longport::trade::StockPositionChannel,
    [
        account_channel,
        #[java(objarray)]
        positions
    ]
);

impl_java_class!(
    "com/longport/trade/StockPosition",
    longport::trade::StockPosition,
    [
        symbol,
        symbol_name,
        quantity,
        available_quantity,
        currency,
        cost_price,
        market
    ]
);

impl_java_class!(
    "com/longport/trade/MarginRatio",
    longport::trade::MarginRatio,
    [im_factor, mm_factor, fm_factor]
);

impl_java_class!(
    "com/longport/trade/OrderHistoryDetail",
    longport::trade::OrderHistoryDetail,
    [price, quantity, status, msg, time]
);

impl_java_class!(
    "com/longport/trade/OrderChargeFee",
    longport::trade::OrderChargeFee,
    [code, name, amount, currency]
);

impl_java_class!(
    "com/longport/trade/OrderChargeItem",
    longport::trade::OrderChargeItem,
    [
        code,
        name,
        #[java(objarray)]
        fees
    ]
);

impl_java_class!(
    "com/longport/trade/OrderChargeDetail",
    longport::trade::OrderChargeDetail,
    [
        total_amount,
        currency,
        #[java(objarray)]
        items
    ]
);

impl_java_class!(
    "com/longport/trade/OrderDetail",
    longport::trade::OrderDetail,
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
    "com/longport/trade/EstimateMaxPurchaseQuantityResponse",
    longport::trade::EstimateMaxPurchaseQuantityResponse,
    [cash_max_qty, margin_max_qty]
);
