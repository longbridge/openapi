use time::{Duration, macros::time};
use time_tz::timezones::db;

use crate::{TradeSession, market::Market};

pub const HK: Market = Market {
    timezone: db::asia::HONG_KONG,
    trade_sessions: &[
        // Normal
        &[
            TradeSession::new(time!(9:30:00), time!(12:00:00)),
            TradeSession::new(time!(13:00:00), time!(16:00:00)).with_timeout(Duration::minutes(10)),
        ],
    ],
    half_trade_sessions: &[
        // Normal
        &[TradeSession::new(time!(9:30:00), time!(12:00:00)).with_timeout(Duration::minutes(10))],
    ],
    lot_size: 1,
};

pub const SG: Market = Market {
    timezone: db::asia::SINGAPORE,
    trade_sessions: &[
        // Normal
        &[
            TradeSession::new(time!(9:00:00), time!(12:00:00)),
            TradeSession::new(time!(13:00:00), time!(17:15:00)).with_timeout(Duration::minutes(5)),
        ],
    ],
    half_trade_sessions: &[
        // Normal
        &[TradeSession::new(time!(9:00:00), time!(12:15:00)).with_timeout(Duration::minutes(5))],
    ],
    lot_size: 1,
};

pub const CN: Market = Market {
    timezone: db::asia::SHANGHAI,
    trade_sessions: &[
        // Normal
        &[
            TradeSession::new(time!(9:30:00), time!(11:30:00)),
            TradeSession::new(time!(13:00:00), time!(15:00:00)).with_timeout(Duration::minutes(10)),
        ],
    ],
    half_trade_sessions: &[],
    lot_size: 100,
};

pub const US: Market = Market {
    timezone: db::america::NEW_YORK,
    trade_sessions: &[
        // Normal
        &[TradeSession::new(time!(9:30:00), time!(16:00:00))],
        // Pre
        &[TradeSession::new(time!(4:00:00), time!(9:30:00))],
        // Post
        &[TradeSession::new(time!(16:00:00), time!(20:00:00))],
        // Overnight
        &[
            TradeSession::new(time!(0:00:00), time!(4:00:00)),
            TradeSession::new(time!(20:00:00), time!(23:59:59)).with_inclusive(),
        ],
    ],
    half_trade_sessions: &[
        // Normal
        &[TradeSession::new(time!(9:30:00), time!(13:00:00))],
        // Pre
        &[TradeSession::new(time!(4:00:00), time!(9:30:00))],
        // Post
        &[TradeSession::new(time!(13:00:00), time!(17:00:00))],
        // Overnight
        &[
            TradeSession::new(time!(0:00:00), time!(4:00:00)),
            TradeSession::new(time!(20:00:00), time!(23:59:59)).with_inclusive(),
        ],
    ],
    lot_size: 1,
};

pub const US_OPTION: Market = Market {
    timezone: db::america::NEW_YORK,
    trade_sessions: &[
        // Normal
        &[TradeSession::new(time!(9:30:00), time!(16:15:00))],
    ],
    half_trade_sessions: &[
        // Normal
        &[TradeSession::new(time!(9:30:00), time!(13:15:00))],
    ],
    lot_size: 1,
};
