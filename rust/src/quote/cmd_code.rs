/// Query User Quote Profile
pub(crate) const QUERY_USER_QUOTE_PROFILE: u8 = 4;

/// Subscribe Quote
pub(crate) const SUBSCRIBE: u8 = 6;

/// Unsubscribe Quote
pub(crate) const UNSUBSCRIBE: u8 = 7;

/// Get Trading Session Of The Day
pub(crate) const GET_TRADING_SESSION: u8 = 8;

/// Get Market Trading Days
pub(crate) const GET_TRADING_DAYS: u8 = 9;

/// Get Basic Information Of Securities
pub(crate) const GET_BASIC_INFO: u8 = 10;

/// Get Real-time Quotes Of Securities
pub(crate) const GET_REALTIME_QUOTE: u8 = 11;

/// Get Real-time Quotes Of Option Securities
pub(crate) const GET_REALTIME_OPTION_QUOTE: u8 = 12;

/// Get Real-time Quotes Of Warrant Securities
pub(crate) const GET_REALTIME_WARRANT_QUOTE: u8 = 13;

/// Get Security Depth
pub(crate) const GET_SECURITY_DEPTH: u8 = 14;

/// Get Security Brokers
pub(crate) const GET_SECURITY_BROKERS: u8 = 15;

/// Get Broker IDs
pub(crate) const GET_BROKER_IDS: u8 = 16;

/// Get Security Trades
pub(crate) const GET_SECURITY_TRADES: u8 = 17;

/// Get Security Intraday
pub(crate) const GET_SECURITY_INTRADAY: u8 = 18;

/// Get Security Candlesticks
pub(crate) const GET_SECURITY_CANDLESTICKS: u8 = 19;

/// Get Option Chain Expiry Date List
pub(crate) const GET_OPTION_CHAIN_EXPIRY_DATE_LIST: u8 = 20;

/// Get Warrant Issuer IDs
pub(crate) const GET_WARRANT_ISSUER_IDS: u8 = 22;

/// Get Filtered Warrant
pub(crate) const GET_FILTERED_WARRANT: u8 = 23;

/// Get Security Capital Flow Intraday
pub(crate) const GET_CAPITAL_FLOW_INTRADAY: u8 = 24;

/// Get Security Capital Distribution
pub(crate) const GET_SECURITY_CAPITAL_DISTRIBUTION: u8 = 25;

/// Get Calc indexes
pub(crate) const GET_CALC_INDEXES: u8 = 26;

/// Get History candlesticks
pub(crate) const GET_SECURITY_HISTORY_CANDLESTICKS: u8 = 27;

/// Push Real-time Quote
pub(crate) const PUSH_REALTIME_QUOTE: u8 = 101;

/// Push Real-time Depth
pub(crate) const PUSH_REALTIME_DEPTH: u8 = 102;

/// Push Real-time Brokers
pub(crate) const PUSH_REALTIME_BROKERS: u8 = 103;

/// Push Real-time Trades
pub(crate) const PUSH_REALTIME_TRADES: u8 = 104;

/// Returns the REST path (`POST`) that serves the same request/response
/// messages as the WebSocket command `command_code`, if there is one.
pub(crate) fn http_path(command_code: u8) -> Option<&'static str> {
    Some(match command_code {
        GET_TRADING_SESSION => "/quote/markets/trading-sessions",
        GET_TRADING_DAYS => "/quote/markets/trading-days",
        GET_BASIC_INFO => "/quote/static-info",
        GET_REALTIME_QUOTE => "/quote/quotes",
        GET_REALTIME_OPTION_QUOTE => "/quote/options/quotes",
        GET_REALTIME_WARRANT_QUOTE => "/quote/warrants/quotes",
        GET_SECURITY_DEPTH => "/quote/depth",
        GET_SECURITY_BROKERS => "/quote/brokers",
        GET_BROKER_IDS => "/quote/participants",
        GET_SECURITY_TRADES => "/quote/trades",
        GET_SECURITY_INTRADAY => "/quote/intraday",
        GET_SECURITY_CANDLESTICKS => "/quote/candlesticks",
        GET_OPTION_CHAIN_EXPIRY_DATE_LIST => "/quote/options/expiry-dates",
        GET_WARRANT_ISSUER_IDS => "/quote/warrants/issuers",
        GET_FILTERED_WARRANT => "/quote/warrants",
        GET_CAPITAL_FLOW_INTRADAY => "/quote/capital-flow",
        GET_SECURITY_CAPITAL_DISTRIBUTION => "/quote/capital-distribution",
        GET_CALC_INDEXES => "/quote/calc-indexes",
        GET_SECURITY_HISTORY_CANDLESTICKS => "/quote/history-candlesticks",
        _ => return None,
    })
}
