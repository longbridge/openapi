use num_enum::{FromPrimitive, IntoPrimitive};
use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error as _};
use time::OffsetDateTime;

use crate::serde_utils;

/// Response for get statement data list request
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetStatementListResponse {
    /// Available statement entries
    pub list: Vec<StatementItem>,
}

/// Statement data info
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StatementItem {
    /// Statement date as `YYYYMM` (e.g. `202401`)
    pub dt: i32,
    /// Opaque key identifying the statement file (pass to the download
    /// endpoint)
    pub file_key: String,
}

/// Response for get statement data download url request
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetStatementResponse {
    /// Temporary download URL for the statement file
    pub url: String,
}

/// Implements `Serialize` (as the integer code) and a lenient `Deserialize`
/// (a JSON integer or an integer string; anything else, including an
/// undocumented code, becomes `Unknown`) for an integer-coded account enum.
macro_rules! impl_account_code_enum {
    ($($ty:ty),* $(,)?) => {
        $(
            impl Serialize for $ty {
                fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                    let value: i32 = (*self).into();
                    value.serialize(serializer)
                }
            }

            impl<'de> Deserialize<'de> for $ty {
                fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                    let value = serde_json::Value::deserialize(deserializer)?;
                    Ok(serde_utils::lenient_int_opt(&value)
                        .and_then(|v| i32::try_from(v).ok())
                        .map(<$ty>::from)
                        .unwrap_or_default())
                }
            }
        )*
    };
}

/// Verification status of a bank card
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default, FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum BankCardStatus {
    /// Unknown (an undocumented code)
    #[default]
    Unknown = -1,
    /// Not verified
    Unverified = 0,
    /// Under review
    UnderReview = 1,
    /// Verified
    Verified = 2,
}

/// Witness (in-person / video verification) state of a bank card
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default, FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum BankCardWitnessState {
    /// Unknown (an undocumented code)
    #[default]
    Unknown = -1,
    /// Witnessing not required
    NotRequired = 1,
    /// Witnessing pending
    Pending = 2,
    /// Witnessed
    Witnessed = 3,
}

/// Binding state of a bank card
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default, FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum BankCardBindState {
    /// Unknown (an undocumented code)
    #[default]
    Unknown = -1,
    /// Binding in progress
    Binding = 0,
    /// Bound
    Bound = 1,
    /// Binding failed
    Failed = 2,
}

/// Status of a bank-card authorization (eDDA, bank securities account, FPS)
/// or of an eDDA application
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default, FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum AuthorizationStatus {
    /// Unknown (an undocumented code)
    #[default]
    Unknown = -1,
    /// Not authorized
    Unauthorized = 0,
    /// Authorized
    Authorized = 1,
    /// Authorization in progress
    Authorizing = 2,
    /// Authorization failed (see the `message` field for the reason)
    Failed = 3,
}

/// State of a stock transfer-in application (another broker → this account)
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default, FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum StockTransferInState {
    /// Unknown (an undocumented code)
    #[default]
    Unknown = -1,
    /// Submitted
    Submitted = 0,
    /// Cancelled
    Cancelled = 1,
    /// Completed
    Completed = 3,
    /// Processing
    Processing = 5,
    /// Partially completed
    PartiallyCompleted = 7,
    /// Failed
    Failed = 8,
}

/// State of a stock transfer-out application (this account → another
/// broker)
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default, FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum StockTransferOutState {
    /// Unknown (an undocumented code)
    #[default]
    Unknown = -1,
    /// Submitted (the only state in which the application can still be
    /// cancelled)
    Submitted = 0,
    /// Processing
    Processing = 1,
    /// Executing
    Executing = 2,
    /// Completed
    Completed = 3,
    /// Cancelled
    Cancelled = 4,
    /// Partially completed
    PartiallyCompleted = 5,
    /// Failed
    Failed = 6,
    /// Rejected
    Rejected = 7,
}

/// State of one stock inside a stock transfer application
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default, FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum TransferStockState {
    /// Unknown (an undocumented code)
    #[default]
    Unknown = -1,
    /// Processing
    Processing = 0,
    /// Succeeded
    Succeeded = 1,
    /// Failed
    Failed = 2,
}

/// Type of a reward record
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default, FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum RewardType {
    /// Unknown (an undocumented code)
    #[default]
    Unknown = -1,
    /// Cash reward
    Cash = 0,
    /// Gift (prize)
    Gift = 1,
    /// Coupon
    Coupon = 2,
    /// Task reward
    Task = 3,
}

/// Status of a reward record
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default, FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum RewardStatus {
    /// Unknown (an undocumented code)
    #[default]
    Unknown = -1,
    /// Upcoming
    Upcoming = 0,
    /// Cancelled
    Cancelled = 1,
    /// The user exited the activity
    ExitedActivity = 2,
    /// Not claimed yet
    Unclaimed = 8,
    /// Being issued
    Issuing = 10,
    /// Issued
    Issued = 12,
    /// Issuing failed
    Failed = 13,
    /// Exchanged
    Exchanged = 16,
    /// Expired
    Expired = 17,
    /// Refunded
    Refunded = 18,
}

/// Delivery status of an issued prize (only meaningful when the reward
/// status is [`RewardStatus::Issued`] and the reward is a gift)
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default, FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum RewardDeliverStatus {
    /// Unknown (an undocumented code, or no delivery status)
    #[default]
    Unknown = -1,
    /// Not activated
    NotActivated = 0,
    /// Activated, or shipping
    ActivatedOrShipping = 1,
    /// Consumed
    Consumed = 2,
    /// E-voucher activating
    EVoucherActivating = 5,
    /// E-voucher expired before activation
    EVoucherExpired = 6,
    /// E-voucher invalid
    EVoucherInvalid = 7,
    /// Shipping address missing
    AddressMissing = 8,
    /// Shipping address filled (the server uses both 9 and 11)
    #[num_enum(alternatives = [11])]
    AddressFilled = 9,
    /// Shipping
    Shipping = 10,
}

/// Status of a coupon
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default, FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum CouponStatus {
    /// Unknown (an undocumented code)
    #[default]
    Unknown = -1,
    /// Unused
    Unused = 0,
    /// Used
    Used = 1,
    /// Expired
    Expired = 2,
    /// Being redeemed
    Redeeming = 3,
    /// In use
    InUse = 4,
    /// Processing
    Processing = 5,
}

/// State of a deposit record
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default, FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum DepositState {
    /// Unknown (an undocumented code)
    #[default]
    Unknown = -1,
    /// Processing
    Processing = 0,
    /// Credited to the account
    Credited = 1,
    /// Failed
    Failed = 2,
    /// Cancelled
    Cancelled = 3,
    /// Invalidated
    Invalidated = 4,
    /// Rejected
    Rejected = 5,
}

/// State of a withdrawal record
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default, FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum WithdrawalState {
    /// Unknown (an undocumented code)
    #[default]
    Unknown = -1,
    /// Submitted
    Submitted = 0,
    /// Processing
    Processing = 1,
    /// Rejected
    Rejected = 2,
    /// Cancelled
    Cancelled = 3,
    /// Remitted
    Remitted = 4,
    /// Failed
    Failed = 5,
    /// Refunded
    Refunded = 6,
    /// Approved, awaiting payout
    ApprovedAwaitingPayout = 7,
    /// Large-amount review
    LargeAmountReview = 8,
    /// Change pending approval
    ChangePendingApproval = 9,
    /// Deletion pending approval
    DeletionPendingApproval = 10,
    /// Deleted
    Deleted = 11,
    /// Rejection pending approval
    RejectionPendingApproval = 12,
    /// Awaiting confirmation
    AwaitingConfirmation = 15,
}

impl_account_code_enum!(
    BankCardStatus,
    BankCardWitnessState,
    BankCardBindState,
    AuthorizationStatus,
    StockTransferInState,
    StockTransferOutState,
    TransferStockState,
    RewardType,
    RewardStatus,
    RewardDeliverStatus,
    CouponStatus,
    DepositState,
    WithdrawalState,
);

/// An authorization attached to a bank card (eDDA, bank securities account,
/// FPS, …)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct BankCardAuthorization {
    /// Authorization flow: `hk_edda` / `hk_sdda` (eDDA), `hk_bank_sec`,
    /// `hk_fps`, …
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub flow_id: String,
    /// Display name of the authorization
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub name: String,
    /// Authorized currency
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub currency: String,
    /// Authorization status
    pub status: AuthorizationStatus,
    /// Failure reason (empty unless the authorization failed)
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub message: String,
    /// Whether this authorization is supported for the card
    #[serde(deserialize_with = "serde_utils::bool_lenient")]
    pub is_support: bool,
    /// Maximum amount per debit (`None` when not set)
    #[serde(with = "serde_utils::decimal_opt_lenient_0_is_none")]
    pub max_amount: Option<Decimal>,
    /// Whether the per-debit amount is unlimited
    #[serde(deserialize_with = "serde_utils::bool_lenient")]
    pub max_amount_unlimited: bool,
    /// Maximum number of debits per day (`None` when not set)
    #[serde(with = "serde_utils::int64_opt_lenient_0_is_none")]
    pub daily_max_count: Option<i64>,
    /// Whether the daily debit count is unlimited
    #[serde(deserialize_with = "serde_utils::bool_lenient")]
    pub daily_max_count_unlimited: bool,
    /// Expiry date of the authorization (`None` when not set, and for a
    /// long-term authorization: the server's 9999-12-31 "never expires"
    /// sentinel, like any date in or after 2100, is `None`; check
    /// [`Self::long_term_valid`])
    #[serde(with = "serde_utils::account_time_opt")]
    pub expiry_date: Option<OffsetDateTime>,
    /// Whether the authorization never expires
    #[serde(deserialize_with = "serde_utils::bool_lenient")]
    pub long_term_valid: bool,
    /// Whether submitted documents are under review
    #[serde(deserialize_with = "serde_utils::bool_lenient")]
    pub doc_reviewing: bool,
    /// Whether the "no prior deposit" check is under review
    #[serde(deserialize_with = "serde_utils::bool_lenient")]
    pub no_prior_deposit_reviewing: bool,
}

/// A linked bank card
///
/// The server also returns the card holder's name and postal address and the
/// storage keys of uploaded ID documents; those are intentionally not
/// modelled.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct BankCard {
    /// Bank card ID (pass to [`crate::AssetContext::edda_applications`])
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub id: String,
    /// Bank ID
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub bank_id: String,
    /// Bank name
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub bank: String,
    /// Bank name in English
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub bank_en: String,
    /// Bank code
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub bank_code: String,
    /// Bank account number, as returned by the server (not masked)
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub account: String,
    /// Account type
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub account_type: String,
    /// Country or region of the bank (e.g. `Hong Kong`; not a currency)
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub country: String,
    /// Region code
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub region: String,
    /// Region name
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub region_name: String,
    /// SWIFT code
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub swift_code: String,
    /// Remark
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub remark: String,
    /// Verification status
    pub status: BankCardStatus,
    /// Witness state
    pub witness_state: BankCardWitnessState,
    /// Whether the card supports witnessing
    #[serde(deserialize_with = "serde_utils::bool_lenient")]
    pub support_witness: bool,
    /// Binding state
    pub bind_state: BankCardBindState,
    /// Binding hint text
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub bind_tip: String,
    /// Withdrawal fee text, as returned by the server
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub withdraw_fee: String,
    /// Time the card was added
    #[serde(with = "serde_utils::account_time_opt")]
    pub created_at: Option<OffsetDateTime>,
    /// Authorizations attached to the card
    #[serde(deserialize_with = "serde_utils::null_as_default")]
    pub auth_list: Vec<BankCardAuthorization>,
}

#[derive(Debug, Default, Deserialize)]
pub(crate) struct BankCardsResponse {
    #[serde(default, deserialize_with = "serde_utils::null_as_default")]
    pub(crate) list: Vec<BankCard>,
}

/// An eDDA authorization application of a bank card
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct EddaApplication {
    /// Application ID
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub id: String,
    /// Application status
    pub status: AuthorizationStatus,
    /// Failure reason (empty unless the application failed)
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub message: String,
}

/// The eDDA status reply: `{list: [...]}`, or a single `{id, status,
/// message}` object from older server builds.
#[derive(Debug, Default)]
pub(crate) struct EddaApplicationsResponse(pub(crate) Vec<EddaApplication>);

impl<'de> Deserialize<'de> for EddaApplicationsResponse {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = serde_json::Value::deserialize(deserializer)?;
        let items = match value {
            serde_json::Value::Object(mut obj) => match obj.remove("list") {
                Some(serde_json::Value::Array(list)) => list,
                Some(_) => Vec::new(),
                None if obj.contains_key("status") => vec![serde_json::Value::Object(obj)],
                None => Vec::new(),
            },
            _ => Vec::new(),
        };
        items
            .into_iter()
            .map(serde_json::from_value)
            .collect::<Result<Vec<_>, _>>()
            .map(Self)
            .map_err(D::Error::custom)
    }
}

/// One stock in a stock transfer application
///
/// Storage keys of uploaded proof documents are intentionally not modelled.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct TransferStock {
    /// Stock code
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub code: String,
    /// Stock name
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub name: String,
    /// Stock name in English
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub name_en: String,
    /// Market
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub market: String,
    /// Counter ID
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub counter_id: String,
    /// Quantity
    #[serde(with = "serde_utils::decimal_opt_lenient")]
    pub amount: Option<Decimal>,
    /// Currency
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub currency: String,
    /// Cost price
    #[serde(with = "serde_utils::decimal_opt_lenient")]
    pub cost: Option<Decimal>,
    /// State code of this stock in the transfer (the server's `state`)
    #[serde(rename(deserialize = "state"), alias = "state_code")]
    pub state_code: TransferStockState,
    /// Position side
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub position_side: String,
}

/// The `app_state` of a stock transfer application when the user tapped "I
/// have notified the broker" in the app (the user's own confirmation, not a
/// Longbridge action)
pub const APP_STATE_NOTICED: &str = "noticed";

/// A stock transfer-in application (another broker → this account)
///
/// The account holder's name and contact details at the other broker and the
/// storage keys of uploaded proof documents are intentionally not modelled.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct StockTransferIn {
    /// Record ID
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub id: String,
    /// Order ID
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub order_id: String,
    /// Market
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub market: String,
    /// Name of the other broker
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub broker: String,
    /// ID of the other broker
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub broker_id: String,
    /// English name of the other broker
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub broker_en: String,
    /// Broker code of the other broker
    #[serde(rename = "code", deserialize_with = "serde_utils::string_lenient")]
    pub broker_code: String,
    /// Account number at the other broker, as returned by the server (not
    /// masked)
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub account: String,
    /// State text, as returned by the server
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub state: String,
    /// State code
    pub state_code: StockTransferInState,
    /// App-side state: [`APP_STATE_NOTICED`] (`noticed`) means the user
    /// tapped "I have notified the broker" in the app (the user's own
    /// confirmation, not a Longbridge action); `default` otherwise. See
    /// [`Self::user_notified_broker`].
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub app_state: String,
    /// Whether the application can still be cancelled or changed
    #[serde(deserialize_with = "serde_utils::bool_lenient")]
    pub can_modify: bool,
    /// Created time
    #[serde(with = "serde_utils::account_time_opt")]
    pub created_at: Option<OffsetDateTime>,
    /// Updated time
    #[serde(with = "serde_utils::account_time_opt")]
    pub updated_at: Option<OffsetDateTime>,
    /// Processing start time (`None` when not set)
    #[serde(with = "serde_utils::account_time_opt")]
    pub process_at: Option<OffsetDateTime>,
    /// Execution time (`None` when not executed)
    #[serde(with = "serde_utils::account_time_opt")]
    pub exec_at: Option<OffsetDateTime>,
    /// Remark
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub remark: String,
    /// Reason (e.g. for a failure)
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub reason: String,
    /// Stocks in the application
    #[serde(deserialize_with = "serde_utils::null_as_default")]
    pub stocks: Vec<TransferStock>,
}

/// Response for the stock transfer-in list
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct StockTransferInResponse {
    /// Records
    #[serde(deserialize_with = "serde_utils::null_as_default")]
    pub records: Vec<StockTransferIn>,
    /// Total number of records
    #[serde(deserialize_with = "serde_utils::int64_lenient")]
    pub total: i64,
}

/// A stock transfer-out application (this account → another broker)
///
/// The account holder's name and contact details at the other broker and the
/// storage keys of uploaded proof documents are intentionally not modelled.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct StockTransferOut {
    /// Record ID
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub id: String,
    /// Order ID
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub order_id: String,
    /// Market
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub market: String,
    /// Name of the other broker
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub broker: String,
    /// ID of the other broker
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub broker_id: String,
    /// English name of the other broker (the server's `name_en`)
    #[serde(rename = "name_en", deserialize_with = "serde_utils::string_lenient")]
    pub broker_en: String,
    /// Broker code of the other broker
    #[serde(rename = "code", deserialize_with = "serde_utils::string_lenient")]
    pub broker_code: String,
    /// Account number at the other broker, as returned by the server (not
    /// masked)
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub account: String,
    /// State text, as returned by the server
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub state: String,
    /// State code. Only [`StockTransferOutState::Submitted`] applications can
    /// be cancelled (the server's `can_modify` is not reliable here, so it is
    /// not modelled).
    pub state_code: StockTransferOutState,
    /// App-side state: [`APP_STATE_NOTICED`] (`noticed`) means the user
    /// tapped "I have notified the broker" in the app (the user's own
    /// confirmation, not a Longbridge action); `default` otherwise. See
    /// [`Self::user_notified_broker`].
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub app_state: String,
    /// Created time
    #[serde(with = "serde_utils::account_time_opt")]
    pub created_at: Option<OffsetDateTime>,
    /// Updated time
    #[serde(with = "serde_utils::account_time_opt")]
    pub updated_at: Option<OffsetDateTime>,
    /// Processing start time (`None` when not set)
    #[serde(with = "serde_utils::account_time_opt")]
    pub process_at: Option<OffsetDateTime>,
    /// Execution time (`None` when not executed)
    #[serde(with = "serde_utils::account_time_opt")]
    pub exec_at: Option<OffsetDateTime>,
    /// Time the other broker agreed (`None` when not set)
    #[serde(with = "serde_utils::account_time_opt")]
    pub agree_at: Option<OffsetDateTime>,
    /// Time the application was refused (`None` when not set)
    #[serde(with = "serde_utils::account_time_opt")]
    pub refuse_at: Option<OffsetDateTime>,
    /// Time the application was cancelled (`None` when not set)
    #[serde(with = "serde_utils::account_time_opt")]
    pub cancel_at: Option<OffsetDateTime>,
    /// Remark
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub remark: String,
    /// Reason (e.g. for a rejection)
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub reason: String,
    /// Stocks in the application
    #[serde(deserialize_with = "serde_utils::null_as_default")]
    pub stocks: Vec<TransferStock>,
}

impl StockTransferIn {
    /// Whether the user confirmed in the app that the other broker was
    /// notified (`app_state` is [`APP_STATE_NOTICED`])
    #[inline]
    pub fn user_notified_broker(&self) -> bool {
        self.app_state == APP_STATE_NOTICED
    }
}

impl StockTransferOut {
    /// Whether the user confirmed in the app that the other broker was
    /// notified (`app_state` is [`APP_STATE_NOTICED`])
    #[inline]
    pub fn user_notified_broker(&self) -> bool {
        self.app_state == APP_STATE_NOTICED
    }

    /// Whether the application can still be cancelled (only
    /// [`StockTransferOutState::Submitted`] applications can)
    #[inline]
    pub fn cancellable(&self) -> bool {
        self.state_code == StockTransferOutState::Submitted
    }
}

#[derive(Debug, Default, Deserialize)]
pub(crate) struct StockTransferOutResponse {
    #[serde(default, deserialize_with = "serde_utils::null_as_default")]
    pub(crate) records: Vec<StockTransferOut>,
}

/// A reward record from the reward center
///
/// The server nests the details in one of `cash` / `card` / `prize`; this
/// type flattens them. Redemption secrets, voucher codes and shipping contact
/// details of prizes are intentionally not modelled.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(from = "RawRewardRecord")]
pub struct RewardRecord {
    /// Reward type
    pub reward_type: RewardType,
    /// Record ID
    pub record_id: String,
    /// Title of the activity that granted the reward
    pub activity_title: String,
    /// Reward title
    pub title: String,
    /// Reward description
    pub description: String,
    /// Coupon type (coupon rewards), e.g. `commission`
    pub card_type: String,
    /// Coupon status (coupon rewards; [`CouponStatus::Unknown`] otherwise)
    pub card_status: CouponStatus,
    /// Prize type code (gift rewards; `0` = physical goods), `None` when not
    /// set
    pub prize_type: Option<i32>,
    /// Amount
    pub amount: Option<Decimal>,
    /// Currency
    pub currency: String,
    /// Reward status
    pub status: RewardStatus,
    /// Delivery status (issued gift rewards only)
    pub deliver_status: RewardDeliverStatus,
    /// Exit status code, as returned by the server
    pub exit_status: i32,
    /// Time the reward was sent
    #[serde(with = "serde_utils::account_time_opt")]
    pub send_time: Option<OffsetDateTime>,
    /// Start time
    #[serde(with = "serde_utils::account_time_opt")]
    pub start_time: Option<OffsetDateTime>,
    /// End time
    #[serde(with = "serde_utils::account_time_opt")]
    pub end_time: Option<OffsetDateTime>,
    /// Effective time
    #[serde(with = "serde_utils::account_time_opt")]
    pub effective_time: Option<OffsetDateTime>,
    /// Deadline to claim the reward
    #[serde(with = "serde_utils::account_time_opt")]
    pub receive_deadline: Option<OffsetDateTime>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct RawRewardDetail {
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    record_id: String,
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    activity_title: String,
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    title: String,
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    description: String,
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    card_type: String,
    card_status: CouponStatus,
    prize_type: serde_json::Value,
    #[serde(with = "serde_utils::decimal_opt_lenient")]
    amount: Option<Decimal>,
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    currency: String,
    status: RewardStatus,
    deliver_status: RewardDeliverStatus,
    #[serde(deserialize_with = "serde_utils::int32_lenient")]
    exit_status: i32,
    #[serde(with = "serde_utils::account_time_opt")]
    send_time: Option<OffsetDateTime>,
    #[serde(with = "serde_utils::account_time_opt")]
    start_time: Option<OffsetDateTime>,
    #[serde(with = "serde_utils::account_time_opt")]
    end_time: Option<OffsetDateTime>,
    #[serde(with = "serde_utils::account_time_opt")]
    effective_time: Option<OffsetDateTime>,
    #[serde(with = "serde_utils::account_time_opt")]
    receive_deadline: Option<OffsetDateTime>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
struct RawRewardRecord {
    reward_type: serde_json::Value,
    cash: Option<RawRewardDetail>,
    card: Option<RawRewardDetail>,
    prize: Option<RawRewardDetail>,
}

impl From<RawRewardRecord> for RewardRecord {
    fn from(raw: RawRewardRecord) -> Self {
        let code = serde_utils::lenient_int_opt(&raw.reward_type)
            .and_then(|v| i32::try_from(v).ok())
            .map(RewardType::from);
        // Prefer the detail object the type names; otherwise take whichever
        // one is present and name the type after it.
        let (reward_type, detail) = match (code, raw.cash, raw.card, raw.prize) {
            (Some(RewardType::Cash), Some(d), _, _) => (RewardType::Cash, d),
            (Some(RewardType::Coupon), _, Some(d), _) => (RewardType::Coupon, d),
            (Some(RewardType::Gift), _, _, Some(d)) => (RewardType::Gift, d),
            (code, Some(d), _, _) => (code.unwrap_or(RewardType::Cash), d),
            (code, None, Some(d), _) => (code.unwrap_or(RewardType::Coupon), d),
            (code, None, None, Some(d)) => (code.unwrap_or(RewardType::Gift), d),
            (code, None, None, None) => (code.unwrap_or_default(), RawRewardDetail::default()),
        };
        let prize_type =
            serde_utils::lenient_int_opt(&detail.prize_type).and_then(|v| i32::try_from(v).ok());
        Self {
            reward_type,
            record_id: detail.record_id,
            activity_title: detail.activity_title,
            title: detail.title,
            description: detail.description,
            card_type: detail.card_type,
            card_status: detail.card_status,
            prize_type,
            amount: detail.amount,
            currency: detail.currency,
            status: detail.status,
            deliver_status: detail.deliver_status,
            exit_status: detail.exit_status,
            send_time: detail.send_time,
            start_time: detail.start_time,
            end_time: detail.end_time,
            effective_time: detail.effective_time,
            receive_deadline: detail.receive_deadline,
        }
    }
}

/// Response for the reward record list
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct RewardRecordsResponse {
    /// Reward records
    #[serde(deserialize_with = "serde_utils::null_as_default")]
    pub list: Vec<RewardRecord>,
    /// Total number of records
    #[serde(deserialize_with = "serde_utils::int64_lenient")]
    pub total: i64,
}

/// Commission terms of a commission coupon
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct CouponCommission {
    /// Markets the coupon applies to
    #[serde(deserialize_with = "serde_utils::string_vec_lenient")]
    pub markets: Vec<String>,
    /// Rebate rate, as returned by the server
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub rebate_rate: String,
    /// Instrument types the coupon applies to: `0` stocks, `1` options, `2`
    /// warrants, `3` ETF, `4` LOF
    #[serde(deserialize_with = "serde_utils::int32_vec_lenient")]
    pub bz_types: Vec<i32>,
    /// Remaining commission balance
    #[serde(with = "serde_utils::decimal_opt_lenient")]
    pub commission_balance: Option<Decimal>,
    /// Commission duration, as returned by the server
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub commission_duration: String,
    /// Whether the coupon is unlimited
    #[serde(deserialize_with = "serde_utils::bool_lenient")]
    pub unlimited: bool,
    /// Commission expiry time (`None` when not set)
    #[serde(with = "serde_utils::account_time_opt")]
    pub commission_expired_time: Option<OffsetDateTime>,
    /// Number of times used
    #[serde(deserialize_with = "serde_utils::int64_lenient")]
    pub usage_count: i64,
    /// Number of uses left
    #[serde(deserialize_with = "serde_utils::int64_lenient")]
    pub lave_usage_count: i64,
    /// Settlement model, as returned by the server
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub settlement_model: String,
}

/// A coupon (reward card)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Coupon {
    /// Coupon ID (32 hex characters; pass to
    /// [`crate::AssetContext::coupon_orders`])
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub card_id: String,
    /// Coupon type, e.g. `commission`, `platform_fee_deduct`, `stock`,
    /// `quote`, `cash`
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub card_type: String,
    /// Title
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub title: String,
    /// Secondary title
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub title_sec: String,
    /// Description
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub description: String,
    /// Terms and details
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub detail: String,
    /// Status
    pub card_status: CouponStatus,
    /// Start of validity
    #[serde(with = "serde_utils::account_time_opt")]
    pub start_time: Option<OffsetDateTime>,
    /// End of validity (`None` when not set or when the coupon never
    /// expires; see [`Self::no_expiry`])
    #[serde(with = "serde_utils::account_time_opt")]
    pub end_time: Option<OffsetDateTime>,
    /// Whether the coupon never expires (the server's end time is in or after
    /// 2100)
    #[serde(deserialize_with = "serde_utils::bool_lenient")]
    pub no_expiry: bool,
    /// Activation time (`None` when not activated)
    #[serde(with = "serde_utils::account_time_opt")]
    pub activate_time: Option<OffsetDateTime>,
    /// Amount
    #[serde(with = "serde_utils::decimal_opt_lenient")]
    pub amount: Option<Decimal>,
    /// Currency
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub currency: String,
    /// Trading sessions the coupon applies to, as display text
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub available_trade_session: String,
    /// Commission terms (commission coupons only; empty otherwise)
    #[serde(deserialize_with = "serde_utils::null_as_default")]
    pub commission: CouponCommission,
}

/// Response for the coupon list
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct CouponsResponse {
    /// Pinned coupons shown above the list (repeated on every page)
    #[serde(deserialize_with = "coupon_list")]
    pub top_list: Vec<Coupon>,
    /// Coupons on this page
    #[serde(deserialize_with = "coupon_list")]
    pub list: Vec<Coupon>,
    /// Pinned coupons shown below the list (repeated on every page)
    #[serde(deserialize_with = "coupon_list")]
    pub bottom_list: Vec<Coupon>,
    /// Total number of coupons
    #[serde(deserialize_with = "serde_utils::int64_lenient")]
    pub total: i64,
    /// Number of expired coupons
    #[serde(deserialize_with = "serde_utils::int64_lenient")]
    pub expired_count: i64,
}

/// Decodes a coupon list, setting [`Coupon::no_expiry`] from the server's raw
/// `end_time` (which [`Coupon::end_time`] drops when it is past 2100).
fn coupon_list<'de, D>(deserializer: D) -> Result<Vec<Coupon>, D::Error>
where
    D: Deserializer<'de>,
{
    let items = match serde_json::Value::deserialize(deserializer)? {
        serde_json::Value::Array(items) => items,
        _ => return Ok(Vec::new()),
    };
    items
        .into_iter()
        .map(|item| {
            let never_expires = item
                .get("end_time")
                .is_some_and(serde_utils::account_time_is_far_future);
            let mut coupon: Coupon = serde_json::from_value(item).map_err(D::Error::custom)?;
            coupon.no_expiry |= never_expires;
            Ok(coupon)
        })
        .collect()
}

/// An order a coupon was applied to
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct CouponOrder {
    /// Order ID
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub order_id: String,
    /// State text, as returned by the server
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub state: String,
    /// State value, as returned by the server (e.g. `FilledStatus`)
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub state_val: String,
    /// Executed price
    #[serde(with = "serde_utils::decimal_opt_lenient")]
    pub executed_price: Option<Decimal>,
    /// Executed quantity
    #[serde(with = "serde_utils::decimal_opt_lenient")]
    pub executed_quantity: Option<Decimal>,
    /// Currency
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub currency: String,
    /// Settlement currency
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub settlement_currency: String,
    /// Rebate amount
    #[serde(with = "serde_utils::decimal_opt_lenient")]
    pub rebate: Option<Decimal>,
    /// Rebate date, as returned by the server
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub rebate_dt: String,
}

#[derive(Debug, Default, Deserialize)]
pub(crate) struct CouponOrdersResponse {
    #[serde(default, deserialize_with = "serde_utils::null_as_default")]
    pub(crate) orders: Vec<CouponOrder>,
}

/// A deposit record
///
/// Receipt images uploaded with the deposit are intentionally not modelled.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Deposit {
    /// Record ID
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub id: String,
    /// Created time
    #[serde(with = "serde_utils::account_time_opt")]
    pub created_at: Option<OffsetDateTime>,
    /// Updated time
    #[serde(with = "serde_utils::account_time_opt")]
    pub updated_at: Option<OffsetDateTime>,
    /// Currency
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub currency: String,
    /// Notified (applied) amount
    #[serde(with = "serde_utils::decimal_opt_lenient")]
    pub amount: Option<Decimal>,
    /// Received amount (`0` = not received yet)
    #[serde(with = "serde_utils::decimal_opt_lenient")]
    pub receivable_amount: Option<Decimal>,
    /// State text, as returned by the server
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub state: String,
    /// State code
    pub state_code: DepositState,
    /// Sub-state code, as returned by the server
    #[serde(deserialize_with = "serde_utils::int32_lenient")]
    pub sub_state_code: i32,
    /// Deposit method ID
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub bank_operation_type_id: String,
    /// Deposit method name
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub bank_operation_type_name: String,
    /// Reference number
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub reference_no: String,
    /// Whether the deposit can still be cancelled
    #[serde(deserialize_with = "serde_utils::bool_lenient")]
    pub can_cancel: bool,
    /// SWIFT code of the paying bank
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub user_bank_swift_code: String,
}

#[derive(Debug, Default, Deserialize)]
pub(crate) struct DepositsResponse {
    #[serde(default, deserialize_with = "serde_utils::null_as_default")]
    pub(crate) items: Vec<Deposit>,
}

/// A withdrawal record
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Withdrawal {
    /// Record ID
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub id: String,
    /// Order ID
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub order_id: String,
    /// Created time
    #[serde(with = "serde_utils::account_time_opt")]
    pub created_at: Option<OffsetDateTime>,
    /// Currency
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub currency: String,
    /// Applied amount
    #[serde(with = "serde_utils::decimal_opt_lenient")]
    pub amount: Option<Decimal>,
    /// Fee
    #[serde(with = "serde_utils::decimal_opt_lenient")]
    pub fee: Option<Decimal>,
    /// Amount paid out
    #[serde(with = "serde_utils::decimal_opt_lenient")]
    pub settlement_amount: Option<Decimal>,
    /// State code (the server's `state`)
    #[serde(rename(deserialize = "state"), alias = "state_code")]
    pub state_code: WithdrawalState,
    /// Raw state, as returned by the server
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub raw_state: String,
    /// State display text, as returned by the server
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub state_description: String,
    /// Receiving bank account number, as returned by the server (not masked)
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub bank_account: String,
    /// Receiving bank account, as displayed in the app
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub display_bank_account: String,
    /// SWIFT code of the receiving bank
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub user_bank_swift_code: String,
    /// Account channel
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub account_channel: String,
}

/// Response for the withdrawal list
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct WithdrawalsResponse {
    /// Withdrawal records
    #[serde(deserialize_with = "serde_utils::null_as_default")]
    pub list: Vec<Withdrawal>,
    /// Total number of records
    #[serde(deserialize_with = "serde_utils::int64_lenient")]
    pub total: i64,
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use time::macros::datetime;

    use super::*;

    fn dec(value: &str) -> Decimal {
        Decimal::from_str(value).expect("valid decimal literal")
    }

    #[test]
    fn bank_card_decode_drops_sensitive_fields_and_placeholders() {
        let resp: BankCardsResponse = serde_json::from_str(
            r#"{"list":[{"id":12345,"bank_id":"7","bank":"HSBC","bank_en":"HSBC",
                "account":"012345678901","name":"CHAN TAI MAN","account_type":"saving",
                "country":"Hong Kong","status":1,"witness_state":"3","support_witness":true,
                "bind_state":1,"created_at":"1767196800","picture_urls":["k1"],
                "payee_street":"1 Road","auth_list":[
                    {"flow_id":"hk_edda","currency":"HKD","status":3,"message":"rejected",
                     "max_amount":"0","daily_max_count":0,"expiry_date":"0",
                     "long_term_valid":false,"is_support":1,"id":"0","created_at":"0"},
                    {"flow_id":"hk_fps","currency":"HKD","status":1,"max_amount":"50000",
                     "daily_max_count":"3","expiry_date":253402214400,"long_term_valid":true}
                ]}],"show_add_card_button":true}"#,
        )
        .expect("bank cards decode");
        let card = &resp.list[0];
        assert_eq!(card.id, "12345", "numeric id kept as text");
        assert_eq!(card.account, "012345678901", "account not masked");
        assert_eq!(card.status, BankCardStatus::UnderReview, "status code 1");
        assert_eq!(
            card.witness_state,
            BankCardWitnessState::Witnessed,
            "string code"
        );
        assert_eq!(card.bind_state, BankCardBindState::Bound, "bind code");
        assert!(card.support_witness, "bool");
        assert_eq!(
            card.created_at,
            Some(datetime!(2025-12-31 16:00 UTC)),
            "time"
        );
        let json = serde_json::to_string(card).expect("serialize");
        for key in ["CHAN TAI MAN", "picture_urls", "payee_street", "1 Road"] {
            assert!(!json.contains(key), "{key} must not be modelled");
        }
        let failed = &card.auth_list[0];
        assert_eq!(failed.status, AuthorizationStatus::Failed, "auth status");
        assert_eq!(failed.message, "rejected", "failure reason");
        assert!(failed.max_amount.is_none(), "\"0\" amount is unset");
        assert!(failed.daily_max_count.is_none(), "0 count is unset");
        assert!(failed.expiry_date.is_none(), "\"0\" expiry is unset");
        assert!(failed.is_support, "numeric bool");
        let fps = &card.auth_list[1];
        assert_eq!(fps.max_amount, Some(dec("50000")), "amount");
        assert_eq!(fps.daily_max_count, Some(3), "count");
        assert!(
            fps.expiry_date.is_none(),
            "9999-12-31 never-expires sentinel is None"
        );
        assert!(fps.long_term_valid, "long-term flag");
    }

    #[test]
    fn edda_applications_accept_list_and_single_object() {
        let list: EddaApplicationsResponse = serde_json::from_str(
            r#"{"list":[{"id":"9","status":2,"message":""},{"id":10,"status":42}]}"#,
        )
        .expect("list decodes");
        assert_eq!(list.0.len(), 2, "two applications");
        assert_eq!(list.0[0].status, AuthorizationStatus::Authorizing, "known");
        assert_eq!(
            list.0[1].status,
            AuthorizationStatus::Unknown,
            "unknown code"
        );
        assert_eq!(list.0[1].id, "10", "numeric id as text");

        let single: EddaApplicationsResponse =
            serde_json::from_str(r#"{"id":"1","status":3,"message":"bank refused"}"#)
                .expect("single object decodes");
        assert_eq!(single.0.len(), 1, "one application");
        assert_eq!(single.0[0].message, "bank refused", "message");

        let empty: EddaApplicationsResponse = serde_json::from_str("{}").expect("empty");
        assert!(empty.0.is_empty(), "no applications");
    }

    #[test]
    fn stock_transfer_in_decode_handles_placeholders() {
        let resp: StockTransferInResponse = serde_json::from_str(
            r#"{"records":[{"id":"7053","order_id":"o1","market":"US","broker":"B",
                "broker_id":88,"broker_en":"B Ltd","code":"B01","account":"11222333444555",
                "name":"CHAN TAI MAN","contact_phone":"12345678","attachments":"[\"ai/x\"]",
                "state":"已提交","state_code":0,"app_state":"noticed","can_modify":1,
                "created_at":1764064746,"updated_at":"1764064746","process_at":0,
                "exec_at":"0","cash":[],
                "stocks":[{"code":"AAPL","name":"Apple","amount":"100","state":1,
                    "usi_attachments":["k"],"street_amount":"0","cost":"150.5"}]}],
                "total":"1"}"#,
        )
        .expect("transfer-in decodes");
        assert_eq!(resp.total, 1, "string total");
        let record = &resp.records[0];
        assert_eq!(record.broker_code, "B01", "broker code from `code`");
        assert_eq!(record.broker_id, "88", "numeric broker id as text");
        assert!(record.user_notified_broker(), "noticed app state");
        assert_eq!(record.state_code, StockTransferInState::Submitted, "state");
        assert!(record.can_modify, "can_modify");
        assert!(record.created_at.is_some(), "created");
        assert!(record.process_at.is_none(), "0 placeholder");
        assert!(record.exec_at.is_none(), "\"0\" placeholder");
        assert_eq!(
            record.stocks[0].state_code,
            TransferStockState::Succeeded,
            "stock"
        );
        assert_eq!(record.stocks[0].amount, Some(dec("100")), "quantity");
        let json = serde_json::to_string(record).expect("serialize");
        for key in ["CHAN TAI MAN", "12345678", "ai/x", "usi_attachments"] {
            assert!(!json.contains(key), "{key} must not be modelled");
        }
    }

    #[test]
    fn stock_transfer_out_decode() {
        let resp: StockTransferOutResponse = serde_json::from_str(
            r#"{"records":[{"id":"1","name_en":"Futu Securities","state_code":7,
                "app_state":"default","broker_id":"12",
                "created_at":"2026-11-25T09:59:06Z","exec_at":"1970-01-01T00:00:00Z",
                "cancel_at":-2006054656,"refuse_at":1764064746,"stocks":null}]}"#,
        )
        .expect("transfer-out decodes");
        let record = &resp.records[0];
        assert_eq!(record.broker_en, "Futu Securities", "name_en is the broker");
        assert_eq!(record.state_code, StockTransferOutState::Rejected, "state");
        assert_eq!(record.broker_id, "12", "broker id");
        assert_eq!(record.app_state, "default", "app state");
        assert!(!record.user_notified_broker(), "not noticed");
        assert!(!record.cancellable(), "rejected cannot be cancelled");
        assert_eq!(
            record.created_at,
            Some(datetime!(2026-11-25 09:59:06 UTC)),
            "RFC 3339 time"
        );
        assert!(record.exec_at.is_none(), "1970 placeholder");
        assert!(record.cancel_at.is_none(), "negative placeholder");
        assert!(record.refuse_at.is_some(), "real time");
        assert!(record.stocks.is_empty(), "null stocks");
    }

    #[test]
    fn reward_record_flattens_detail_and_drops_secrets() {
        let resp: RewardRecordsResponse = serde_json::from_str(
            r#"{"list":[
                {"reward_type":1,"prize":{"record_id":"r1","activity_title":"Open account",
                    "title":"Coffee","status":12,"deliver_status":11,"prize_type":0,
                    "send_time":1790000000,"coupon_secret":{"secret":"ABC-123"},
                    "coupon_secret_id":"s1","cash_card_code":"CODE9",
                    "express_info":{"phone":"12345678"},"addresses":[{"detail":"x"}]}},
                {"reward_type":0,"cash":{"amount":"38.5","currency":"HKD","status":"12",
                    "send_time":"0"}},
                {"card":{"title":"Commission card","card_type":"commission","status":8,
                    "card_status":"2"}},
                {"reward_type":3}
            ],"total":4}"#,
        )
        .expect("rewards decode");
        assert_eq!(resp.total, 4, "total");
        let prize = &resp.list[0];
        assert_eq!(prize.reward_type, RewardType::Gift, "gift");
        assert_eq!(prize.status, RewardStatus::Issued, "issued");
        assert_eq!(
            prize.deliver_status,
            RewardDeliverStatus::AddressFilled,
            "11"
        );
        assert_eq!(prize.prize_type, Some(0), "physical prize");
        assert!(prize.send_time.is_some(), "send time");
        let json = serde_json::to_string(prize).expect("serialize");
        for key in ["ABC-123", "CODE9", "12345678", "coupon_secret", "addresses"] {
            assert!(!json.contains(key), "{key} must not be modelled");
        }
        let cash = &resp.list[1];
        assert_eq!(cash.reward_type, RewardType::Cash, "cash");
        assert_eq!(cash.amount, Some(dec("38.5")), "amount");
        assert!(cash.send_time.is_none(), "\"0\" placeholder");
        assert_eq!(cash.deliver_status, RewardDeliverStatus::Unknown, "absent");
        assert!(cash.prize_type.is_none(), "no prize type");
        let card = &resp.list[2];
        assert_eq!(
            card.reward_type,
            RewardType::Coupon,
            "typed after the detail"
        );
        assert_eq!(card.status, RewardStatus::Unclaimed, "unclaimed");
        assert_eq!(
            card.card_status,
            CouponStatus::Expired,
            "coupon status enum"
        );
        assert_eq!(cash.card_status, CouponStatus::Unknown, "not a coupon");
        assert_eq!(resp.list[3].reward_type, RewardType::Task, "task kept");
    }

    #[test]
    fn coupon_decode_handles_times() {
        let resp: CouponsResponse = serde_json::from_str(
            r#"{"top_list":null,"list":[{"card_id":"bd6b0000000000000000000000000000",
                "card_type":"commission","start_time":1750313704,"end_time":"8061747303",
                "card_status":4,"activate_time":"0","cover":"x",
                "commission":{"markets":["US"],"bz_types":[0,"2",3],"rebate_rate":"1",
                    "commission_expired_time":"0","unlimited":true,"usage_count":"2"}},
                {"card_id":"c2","card_status":2,"end_time":1798732800000}],
                "bottom_list":[],"total":"2","expired_count":5}"#,
        )
        .expect("coupons decode");
        assert!(resp.top_list.is_empty(), "null list");
        assert_eq!(resp.total, 2, "total");
        assert_eq!(resp.expired_count, 5, "expired count");
        let lifetime = &resp.list[0];
        assert_eq!(lifetime.card_status, CouponStatus::InUse, "status");
        assert!(lifetime.end_time.is_none(), "far future end time is None");
        assert!(lifetime.no_expiry, "far future means never expires");
        assert!(lifetime.activate_time.is_none(), "\"0\" placeholder");
        assert_eq!(lifetime.commission.bz_types, vec![0, 2, 3], "targets");
        assert!(lifetime.commission.unlimited, "unlimited");
        assert_eq!(lifetime.commission.usage_count, 2, "usage count");
        assert!(
            lifetime.commission.commission_expired_time.is_none(),
            "nested placeholder"
        );
        let ms = &resp.list[1];
        assert_eq!(
            ms.end_time,
            Some(datetime!(2026-12-31 16:00 UTC)),
            "milliseconds converted"
        );
        assert!(!ms.no_expiry, "a real end time expires");
        assert!(ms.commission.markets.is_empty(), "no commission terms");
        let json = serde_json::to_value(lifetime).expect("serialize");
        assert_eq!(json["no_expiry"], true, "no_expiry serialized");
        let back: Coupon = serde_json::from_value(json).expect("round trip");
        assert!(back.no_expiry, "no_expiry survives a round trip");
    }

    #[test]
    fn deposit_and_withdrawal_decode() {
        let deposits: DepositsResponse = serde_json::from_str(
            r#"{"items":[{"id":1,"created_at":1764064746,"currency":"HKD","amount":"1,000.00",
                "receivable_amount":"0","state":"处理中","state_code":0,"can_cancel":true,
                "vouchers":[{"key":"k","img_url":"https://x/receipt.png"}]}],"total":20}"#,
        )
        .expect("deposits decode");
        let deposit = &deposits.items[0];
        assert_eq!(deposit.amount, Some(dec("1000.00")), "thousands separator");
        assert_eq!(
            deposit.receivable_amount,
            Some(dec("0")),
            "0 means not received"
        );
        assert_eq!(deposit.state_code, DepositState::Processing, "state");
        let json = serde_json::to_string(deposit).expect("serialize");
        assert!(
            !json.contains("receipt.png"),
            "vouchers must not be modelled"
        );

        let withdrawals: WithdrawalsResponse = serde_json::from_str(
            r#"{"list":[{"id":"w1","state":15,"state_description":"待确认","amount":"100","fee":"0",
                "bank_account":"012345678901","display_bank_account":"尾号 8901"}],"total":1}"#,
        )
        .expect("withdrawals decode");
        let withdrawal = &withdrawals.list[0];
        assert_eq!(
            withdrawal.state_code,
            WithdrawalState::AwaitingConfirmation,
            "state 15"
        );
        assert_eq!(withdrawal.state_description, "待确认", "display text");
        let json = serde_json::to_value(withdrawal).expect("serialize");
        assert_eq!(json["state_code"], 15, "serialized as state_code");
        let back: Withdrawal = serde_json::from_value(json).expect("round trip");
        assert_eq!(
            back.state_code,
            WithdrawalState::AwaitingConfirmation,
            "state_code survives a round trip"
        );
        assert_eq!(withdrawal.bank_account, "012345678901", "not masked");
        assert_eq!(withdrawal.fee, Some(dec("0")), "zero fee kept");
    }

    #[test]
    fn enums_round_trip_codes() {
        assert_eq!(i32::from(DepositState::Rejected), 5, "code");
        assert_eq!(DepositState::from(99), DepositState::Unknown, "fallback");
        assert_eq!(
            serde_json::to_string(&WithdrawalState::Remitted).expect("serialize"),
            "4",
            "serialized as the code"
        );
        let state: CouponStatus = serde_json::from_str("\"x\"").expect("lenient");
        assert_eq!(state, CouponStatus::Unknown, "non-numeric is unknown");
    }
}
