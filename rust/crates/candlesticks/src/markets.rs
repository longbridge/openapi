use time::{macros::time, Duration};
use time_tz::timezones::db;

use crate::market::Market;

pub const HK: Market = Market {
    timezone: db::asia::HONG_KONG,
    trade_sessions: &[
        (time!(9:30:00), time!(12:00:00), Duration::ZERO),
        (time!(13:00:00), time!(16:00:00), Duration::minutes(10)),
    ],
    half_trade_sessions: &[
        (time!(9:30:00), time!(12:00:00), Duration::ZERO),
        (time!(13:00:00), time!(16:00:00), Duration::minutes(10)),
    ],
    lot_size: 1,
};

pub const SG: Market = Market {
    timezone: db::asia::SINGAPORE,
    trade_sessions: &[
        (time!(9:00:00), time!(12:00:00), Duration::ZERO),
        (time!(13:00:00), time!(17:15:00), Duration::minutes(5)),
    ],
    half_trade_sessions: &[(time!(9:00:00), time!(12:15:00), Duration::minutes(5))],
    lot_size: 1,
};

pub const CN: Market = Market {
    timezone: db::asia::SHANGHAI,
    trade_sessions: &[
        (time!(9:30:00), time!(11:30:00), Duration::ZERO),
        (time!(13:00:00), time!(15:00:00), Duration::minutes(10)),
    ],
    half_trade_sessions: &[],
    lot_size: 100,
};

pub const US: Market = Market {
    timezone: db::america::NEW_YORK,
    trade_sessions: &[(time!(9:30:00), time!(16:00:00), Duration::ZERO)],
    half_trade_sessions: &[(time!(9:30:00), time!(13:00:00), Duration::ZERO)],
    lot_size: 1,
};

pub const US_OPTION: Market = Market {
    timezone: db::america::NEW_YORK,
    trade_sessions: &[(time!(9:30:00), time!(16:15:00), Duration::ZERO)],
    half_trade_sessions: &[(time!(9:30:00), time!(13:15:00), Duration::ZERO)],
    lot_size: 1,
};
