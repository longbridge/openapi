use serde::{Serialize, Serializer};
use time::OffsetDateTime;

use crate::asset::{DepositState, RewardType, StockTransferInState, StockTransferOutState};

fn unix_seconds_opt<S>(value: &Option<OffsetDateTime>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        Some(value) => serializer.serialize_i64(value.unix_timestamp()),
        None => serializer.serialize_none(),
    }
}

/// Options for the stock transfer-in and transfer-out lists
///
/// `S` is the state enum of the list: use [`GetStockTransfersInOptions`] for
/// [`crate::AssetContext::stock_transfers_in`] and
/// [`GetStockTransfersOutOptions`] for
/// [`crate::AssetContext::stock_transfers_out`]. The two lists use different
/// state codes, so a state of one list cannot be passed to the other.
#[derive(Debug, Default, Serialize, Clone)]
pub struct GetStockTransfersOptions<S>
where
    S: Serialize,
{
    #[serde(skip_serializing_if = "Option::is_none")]
    market: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    state: Option<S>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "unix_seconds_opt"
    )]
    start: Option<OffsetDateTime>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "unix_seconds_opt"
    )]
    end: Option<OffsetDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<i32>,
}

mod sealed {
    pub trait Sealed {}

    impl Sealed for crate::asset::StockTransferInState {}
    impl Sealed for crate::asset::StockTransferOutState {}
}

/// The state enum of a stock transfer list: [`StockTransferInState`] or
/// [`StockTransferOutState`] (sealed)
pub trait StockTransferStateFilter:
    Serialize + Default + PartialEq + Copy + sealed::Sealed
{
}

impl StockTransferStateFilter for StockTransferInState {}
impl StockTransferStateFilter for StockTransferOutState {}

/// Options for [`crate::AssetContext::stock_transfers_in`]
pub type GetStockTransfersInOptions = GetStockTransfersOptions<StockTransferInState>;

/// Options for [`crate::AssetContext::stock_transfers_out`]
pub type GetStockTransfersOutOptions = GetStockTransfersOptions<StockTransferOutState>;

impl<S> GetStockTransfersOptions<S>
where
    S: StockTransferStateFilter,
{
    /// Create new options (no filters)
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by market (`HK`, `US`, `CN`, `SG`)
    #[inline]
    #[must_use]
    pub fn market(self, market: impl Into<String>) -> Self {
        Self {
            market: Some(market.into().trim().to_uppercase()),
            ..self
        }
    }

    /// Filter by state; it is sent as the state's server code. The `Unknown`
    /// state is not a server code, so it clears the filter instead.
    #[inline]
    #[must_use]
    pub fn state(self, state: S) -> Self {
        Self {
            state: (state != S::default()).then_some(state),
            ..self
        }
    }

    /// Only applications created at or after this time
    #[inline]
    #[must_use]
    pub fn start(self, start: OffsetDateTime) -> Self {
        Self {
            start: Some(start),
            ..self
        }
    }

    /// Only applications created at or before this time
    #[inline]
    #[must_use]
    pub fn end(self, end: OffsetDateTime) -> Self {
        Self {
            end: Some(end),
            ..self
        }
    }

    /// Set the page number (default 1)
    #[inline]
    #[must_use]
    pub fn page(self, page: i32) -> Self {
        Self {
            page: Some(page),
            ..self
        }
    }

    /// Set the page size (default 20, max 50)
    #[inline]
    #[must_use]
    pub fn size(self, size: i32) -> Self {
        Self {
            size: Some(size),
            ..self
        }
    }
}

/// Options for the reward record list
#[derive(Debug, Default, Serialize, Clone)]
pub struct GetRewardRecordsOptions {
    #[serde(rename = "current", skip_serializing_if = "Option::is_none")]
    page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reward_type: Option<RewardType>,
}

impl GetRewardRecordsOptions {
    /// Create a new `GetRewardRecordsOptions`
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the page number (default 1)
    #[inline]
    #[must_use]
    pub fn page(self, page: i32) -> Self {
        Self {
            page: Some(page),
            ..self
        }
    }

    /// Set the page size (max 50)
    #[inline]
    #[must_use]
    pub fn page_size(self, page_size: i32) -> Self {
        Self {
            page_size: Some(page_size),
            ..self
        }
    }

    /// Filter by reward type (default: all types). [`RewardType::Unknown`] is
    /// not a server code, so it clears the filter instead.
    #[inline]
    #[must_use]
    pub fn reward_type(self, reward_type: RewardType) -> Self {
        Self {
            reward_type: (reward_type != RewardType::Unknown).then_some(reward_type),
            ..self
        }
    }
}

/// Options for the coupon list
#[derive(Debug, Default, Serialize, Clone)]
pub struct GetCouponsOptions {
    #[serde(rename = "current", skip_serializing_if = "Option::is_none")]
    page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page_size: Option<i32>,
    #[serde(serialize_with = "serialize_card_status")]
    card_status: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    card_type: Option<String>,
}

/// `card_status` filter: `-1` lists coupons that have not expired, `2` the
/// expired ones.
fn serialize_card_status<S>(expired: &bool, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_i32(if *expired { 2 } else { -1 })
}

impl GetCouponsOptions {
    /// Create a new `GetCouponsOptions` (lists coupons that have not
    /// expired)
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the page number (default 1)
    #[inline]
    #[must_use]
    pub fn page(self, page: i32) -> Self {
        Self {
            page: Some(page),
            ..self
        }
    }

    /// Set the page size (max 50)
    #[inline]
    #[must_use]
    pub fn page_size(self, page_size: i32) -> Self {
        Self {
            page_size: Some(page_size),
            ..self
        }
    }

    /// List expired coupons instead of the ones that have not expired
    #[inline]
    #[must_use]
    pub fn expired(self, expired: bool) -> Self {
        Self {
            card_status: expired,
            ..self
        }
    }

    /// Filter by coupon type (e.g. `commission`, `platform_fee_deduct`,
    /// `stock`, `quote`, `cash`; default: all types)
    #[inline]
    #[must_use]
    pub fn card_type(self, card_type: impl Into<String>) -> Self {
        Self {
            card_type: Some(card_type.into()),
            ..self
        }
    }
}

/// Which orders of a coupon to list
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum CouponOrderType {
    /// Orders the coupon is locked to (in progress)
    Locked,
    /// Orders whose rebate has been settled
    Settled,
}

impl CouponOrderType {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            CouponOrderType::Locked => "locked",
            CouponOrderType::Settled => "settled",
        }
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct GetCouponOrdersQuery {
    pub(crate) card_id: String,
    pub(crate) order_type: &'static str,
}

#[derive(Debug, Serialize)]
pub(crate) struct GetEddaApplicationsQuery {
    pub(crate) card_id: String,
    pub(crate) currency: String,
}

/// Options for the deposit list
#[derive(Debug, Default, Serialize, Clone)]
pub struct GetDepositsOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    states: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    currencies: Option<String>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "unix_seconds_opt"
    )]
    from_date: Option<OffsetDateTime>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "unix_seconds_opt"
    )]
    to_date: Option<OffsetDateTime>,
}

impl GetDepositsOptions {
    /// Create a new `GetDepositsOptions`
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the page number (default 1)
    #[inline]
    #[must_use]
    pub fn page(self, page: i32) -> Self {
        Self {
            page: Some(page),
            ..self
        }
    }

    /// Set the page size
    #[inline]
    #[must_use]
    pub fn size(self, size: i32) -> Self {
        Self {
            size: Some(size),
            ..self
        }
    }

    /// Filter by states (sent as their numeric codes;
    /// [`DepositState::Unknown`] is ignored)
    #[must_use]
    pub fn states(self, states: impl IntoIterator<Item = DepositState>) -> Self {
        let codes = states
            .into_iter()
            .filter(|s| *s != DepositState::Unknown)
            .map(|s| i32::from(s).to_string())
            .collect::<Vec<_>>();
        Self {
            states: (!codes.is_empty()).then(|| codes.join(",")),
            ..self
        }
    }

    /// Filter by currencies (e.g. `HKD`, `USD`)
    #[must_use]
    pub fn currencies<I, T>(self, currencies: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<String>,
    {
        let currencies = currencies
            .into_iter()
            .map(|c| c.into().trim().to_uppercase())
            .filter(|c| !c.is_empty())
            .collect::<Vec<_>>();
        Self {
            currencies: (!currencies.is_empty()).then(|| currencies.join(",")),
            ..self
        }
    }

    /// Only deposits applied at or after this time
    #[inline]
    #[must_use]
    pub fn start(self, start: OffsetDateTime) -> Self {
        Self {
            from_date: Some(start),
            ..self
        }
    }

    /// Only deposits applied at or before this time
    #[inline]
    #[must_use]
    pub fn end(self, end: OffsetDateTime) -> Self {
        Self {
            to_date: Some(end),
            ..self
        }
    }
}

/// Options for the withdrawal list
#[derive(Debug, Default, Serialize, Clone)]
pub struct GetWithdrawalsOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<i32>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "unix_seconds_opt"
    )]
    begin: Option<OffsetDateTime>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "unix_seconds_opt"
    )]
    end: Option<OffsetDateTime>,
}

impl GetWithdrawalsOptions {
    /// Create a new `GetWithdrawalsOptions`
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the page number (default 1)
    #[inline]
    #[must_use]
    pub fn page(self, page: i32) -> Self {
        Self {
            page: Some(page),
            ..self
        }
    }

    /// Set the page size
    #[inline]
    #[must_use]
    pub fn size(self, size: i32) -> Self {
        Self {
            size: Some(size),
            ..self
        }
    }

    /// Only withdrawals applied at or after this time
    #[inline]
    #[must_use]
    pub fn start(self, start: OffsetDateTime) -> Self {
        Self {
            begin: Some(start),
            ..self
        }
    }

    /// Only withdrawals applied at or before this time
    #[inline]
    #[must_use]
    pub fn end(self, end: OffsetDateTime) -> Self {
        Self {
            end: Some(end),
            ..self
        }
    }
}
