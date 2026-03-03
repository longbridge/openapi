use std::borrow::Borrow;

use longport::{Decimal, Market};
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
        trade_session,
        current_volume,
        current_turnover
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
    [period, candlestick, is_confirmed]
);

impl_java_class!(
    "com/longport/quote/Security",
    longport::quote::Security,
    [symbol, name_cn, name_en, name_hk,]
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
        #[java(set_as = crate::types::enum_types::DerivativeTypes)]
        stock_derivatives,
        board,
    ]
);

impl_java_class!(
    "com/longport/quote/PrePostQuote",
    longport::quote::PrePostQuote,
    [
        last_done, timestamp, volume, turnover, high, low, prev_close
    ]
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
        post_market_quote,
        overnight_quote
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
    "com/longport/quote/WarrantInfo",
    longport::quote::WarrantInfo,
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
        last_price,
        remark
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
        #[java(set_as_opt = crate::types::JavaInteger)]
        limit_depth_level,
        #[java(set_as_opt = crate::types::JavaInteger)]
        trigger_count,
        monitor_price,
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
    "com/longport/trade/FrozenTransactionFee",
    longport::trade::FrozenTransactionFee,
    [currency, frozen_transaction_fee]
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
        maintenance_margin,
        buy_power,
        #[java(objarray)]
        frozen_transaction_fees
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

pub(crate) struct StockPositionsResponse {
    channels: Vec<StockPositionChannel>,
}

impl From<longport::trade::StockPositionsResponse> for StockPositionsResponse {
    fn from(value: longport::trade::StockPositionsResponse) -> Self {
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
    "com/longport/trade/StockPositionsResponse",
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

impl From<longport::trade::StockPositionChannel> for StockPositionChannel {
    fn from(value: longport::trade::StockPositionChannel) -> Self {
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
    "com/longport/trade/StockPositionChannel",
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

impl From<longport::trade::StockPosition> for StockPosition {
    fn from(value: longport::trade::StockPosition) -> Self {
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
    "com/longport/trade/StockPosition",
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
    "com/longport/trade/EstimateMaxPurchaseQuantityResponse",
    longport::trade::EstimateMaxPurchaseQuantityResponse,
    [cash_max_qty, margin_max_qty]
);

impl_java_class!(
    "com/longport/quote/QuotePackageDetail",
    longport::quote::QuotePackageDetail,
    [key, name, description, start_at, end_at]
);

impl_java_class!(
    "com/longport/quote/MarketTemperature",
    longport::quote::MarketTemperature,
    [temperature, description, valuation, sentiment, timestamp]
);

impl_java_class!(
    "com/longport/quote/HistoryMarketTemperatureResponse",
    longport::quote::HistoryMarketTemperatureResponse,
    [
        granularity,
        #[java(objarray)]
        records
    ]
);
