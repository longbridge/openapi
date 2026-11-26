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
#[serde(default, remote = "Self")]
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
    /// The server's numeric code of `status`, also when it is a code this SDK
    /// does not know (`status` is then `Unknown`); `None` when the server sent
    /// none
    pub status_raw: Option<i32>,
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
#[serde(default, remote = "Self")]
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
    /// The server's numeric code of `status`, also when it is a code this SDK
    /// does not know (`status` is then `Unknown`); `None` when the server sent
    /// none
    pub status_raw: Option<i32>,
    /// Witness state
    pub witness_state: BankCardWitnessState,
    /// The server's numeric code of `witness_state`, also when it is a code
    /// this SDK does not know (`witness_state` is then `Unknown`); `None` when
    /// the server sent none
    pub witness_state_raw: Option<i32>,
    /// Whether the card supports witnessing
    #[serde(deserialize_with = "serde_utils::bool_lenient")]
    pub support_witness: bool,
    /// Binding state
    pub bind_state: BankCardBindState,
    /// The server's numeric code of `bind_state`, also when it is a code this
    /// SDK does not know (`bind_state` is then `Unknown`); `None` when the
    /// server sent none
    pub bind_state_raw: Option<i32>,
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
#[serde(default, remote = "Self")]
pub struct EddaApplication {
    /// Application ID
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub id: String,
    /// Authorization flow: `hk_edda` (authorized at the bank) or `hk_sdda`
    /// (authorized in the app)
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub flow_id: String,
    /// Display name of the authorization
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub name: String,
    /// Authorized currency
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub currency: String,
    /// Application status
    pub status: AuthorizationStatus,
    /// The server's numeric code of `status`, also when it is a code this SDK
    /// does not know (`status` is then `Unknown`); `None` when the server sent
    /// none
    pub status_raw: Option<i32>,
    /// Failure reason (empty unless the application failed)
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub message: String,
    /// Time the application was filed (`None` when not set)
    #[serde(with = "serde_utils::account_time_opt")]
    pub created_at: Option<OffsetDateTime>,
    /// Time the application was last updated (`None` when not set)
    #[serde(with = "serde_utils::account_time_opt")]
    pub updated_at: Option<OffsetDateTime>,
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
#[serde(default, remote = "Self")]
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
    /// The server's numeric code of `state_code`, also when it is a code this
    /// SDK does not know (`state_code` is then `Unknown`); `None` when the
    /// server sent none
    pub state_code_raw: Option<i32>,
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
#[serde(default, remote = "Self")]
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
    /// The server's numeric code of `state_code`, also when it is a code this
    /// SDK does not know (`state_code` is then `Unknown`); `None` when the
    /// server sent none
    pub state_code_raw: Option<i32>,
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
#[serde(default, remote = "Self")]
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
    /// The server's numeric code of `state_code`, also when it is a code this
    /// SDK does not know (`state_code` is then `Unknown`); `None` when the
    /// server sent none
    pub state_code_raw: Option<i32>,
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
/// type flattens them. Prize redemption secrets (`coupon_secret`), cash card
/// codes (`cash_card_code`) and shipping contact details of prizes are
/// intentionally not modelled.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(from = "RawRewardRecord")]
pub struct RewardRecord {
    /// Reward type: named after the detail object the server sent (`cash`,
    /// `card` or `prize`, checked in that order), or the server's
    /// `reward_type` code when it sent no detail object
    pub reward_type: RewardType,
    /// The server's numeric `reward_type` code, also when it is a code this
    /// SDK does not know; `None` when the server sent none
    pub reward_type_raw: Option<i32>,
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
    /// The server's numeric `card_status` code, also when it is a code this
    /// SDK does not know; `None` when the server sent none
    pub card_status_raw: Option<i32>,
    /// Prize type code (gift rewards; `0` = physical goods), `None` when not
    /// set
    pub prize_type: Option<i32>,
    /// Amount
    pub amount: Option<Decimal>,
    /// Currency
    pub currency: String,
    /// Reward status
    pub status: RewardStatus,
    /// The server's numeric `status` code, also when it is a code this SDK
    /// does not know; `None` when the server sent none
    pub status_raw: Option<i32>,
    /// Delivery status (issued gift rewards only). The server uses both 9
    /// and 11 for [`RewardDeliverStatus::AddressFilled`];
    /// [`Self::deliver_status_raw`] tells them apart.
    pub deliver_status: RewardDeliverStatus,
    /// The server's numeric `deliver_status` code (e.g. 11 where
    /// `deliver_status` reads `AddressFilled`), also when it is a code this
    /// SDK does not know; `None` when the server sent none
    pub deliver_status_raw: Option<i32>,
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
    card_status: serde_json::Value,
    prize_type: serde_json::Value,
    #[serde(with = "serde_utils::decimal_opt_lenient")]
    amount: Option<Decimal>,
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    currency: String,
    status: serde_json::Value,
    deliver_status: serde_json::Value,
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
    #[serde(deserialize_with = "object_opt")]
    cash: Option<RawRewardDetail>,
    #[serde(deserialize_with = "object_opt")]
    card: Option<RawRewardDetail>,
    #[serde(deserialize_with = "object_opt")]
    prize: Option<RawRewardDetail>,
}

/// A typed code and the server's numeric code, decoded from a wire value.
fn typed_and_raw<T>(wire: &serde_json::Value) -> (T, Option<i32>)
where
    T: serde::de::DeserializeOwned + Into<i32> + Copy + Default + PartialEq,
{
    let typed: T = serde_json::from_value(wire.clone()).unwrap_or_default();
    let present = !wire.is_null();
    (
        typed,
        present.then(|| raw_code(typed, Some(wire))).flatten(),
    )
}

impl From<RawRewardRecord> for RewardRecord {
    fn from(raw: RawRewardRecord) -> Self {
        let (code, reward_type_raw) = typed_and_raw::<RewardType>(&raw.reward_type);
        // Like the app, read the first detail object present and name the
        // type after it; without one, fall back to the server's code.
        let (reward_type, detail) = match (raw.cash, raw.card, raw.prize) {
            (Some(d), _, _) => (RewardType::Cash, d),
            (None, Some(d), _) => (RewardType::Coupon, d),
            (None, None, Some(d)) => (RewardType::Gift, d),
            (None, None, None) => (code, RawRewardDetail::default()),
        };
        let prize_type =
            serde_utils::lenient_int_opt(&detail.prize_type).and_then(|v| i32::try_from(v).ok());
        let (card_status, card_status_raw) = typed_and_raw(&detail.card_status);
        let (status, status_raw) = typed_and_raw(&detail.status);
        let (deliver_status, deliver_status_raw) = typed_and_raw(&detail.deliver_status);
        Self {
            reward_type,
            reward_type_raw,
            record_id: detail.record_id,
            activity_title: detail.activity_title,
            title: detail.title,
            description: detail.description,
            card_type: detail.card_type,
            card_status,
            card_status_raw,
            prize_type,
            amount: detail.amount,
            currency: detail.currency,
            status,
            status_raw,
            deliver_status,
            deliver_status_raw,
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

/// Implements `Serialize` (as the integer code) and a lenient `Deserialize`
/// for a proto enum that the server sends either as its name or as its number
/// (a JSON integer or an integer string). Anything else, including an
/// undocumented name or code, becomes `Unknown`.
macro_rules! impl_named_code_enum {
    ($($ty:ty { $($name:literal => $variant:ident),* $(,)? })*) => {
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
                    if let serde_json::Value::String(name) = &value {
                        match name.trim() {
                            $($name => return Ok(<$ty>::$variant),)*
                            _ => {}
                        }
                    }
                    Ok(serde_utils::lenient_int_opt(&value)
                        .and_then(|v| i32::try_from(v).ok())
                        .map(<$ty>::from)
                        .unwrap_or_default())
                }
            }
        )*
    };
}

/// Instrument type a coupon applies to (the server's `bz_types` /
/// `underlying_type`)
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default, FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum CouponTargetType {
    /// Unknown (an undocumented code)
    #[default]
    Unknown = -1,
    /// Stocks (`StockTrade`)
    Stock = 0,
    /// US stock options (`UsStockOptions`)
    UsOption = 1,
    /// HK warrants (`HkStockWarrant`)
    HkWarrant = 2,
    /// ETF
    Etf = 3,
    /// LOF
    Lof = 4,
}

/// IPO subscription type an IPO commission coupon applies to
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default, FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum IpoCouponTargetType {
    /// Unknown (unset, or an undocumented code)
    #[default]
    Unknown = -1,
    /// Financing (margin) IPO subscription (`FinancingIPO`)
    Financing = 1,
    /// Cash IPO subscription (`CashIPO`)
    Cash = 2,
}

/// Kind of commission coupon (the server's `unlimited` code)
///
/// The meanings are the ones the Longbridge app gives the codes; the proto
/// names, which read differently, are given in brackets.
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default, FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum CommissionUnlimitedType {
    /// Unknown (an undocumented code)
    #[default]
    Unknown = -1,
    /// Time-limited free commission (`Proportionate`, code 0)
    Proportionate = 0,
    /// Commission rebate (`Full`, code 1)
    Full = 1,
    /// Commission deduction (`Lifetime`, code 2)
    Lifetime = 2,
    /// Per-use card with a fixed amount per use: `amount / usage_count`
    /// (`ByAmountPer`, code 6)
    ByAmountPer = 6,
    /// Per-use discount card, capped at `amount_limit` per use (`ByRatePer`,
    /// code 7)
    ByRatePer = 7,
}

/// Investor type a quote coupon applies to
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Default, FromPrimitive, IntoPrimitive)]
#[repr(i32)]
pub enum QuoteInvestorType {
    /// Unknown (an undocumented code)
    #[default]
    Unknown = -1,
    /// Any investor (`INVESTOR_TYPE_ANY`)
    Any = 0,
    /// Non-professional investors (`INVESTOR_TYPE_NON_PROFESSIONAL`)
    NonProfessional = 1,
    /// Professional investors (`INVESTOR_TYPE_PROFESSIONAL`)
    Professional = 2,
}

impl_named_code_enum!(
    CouponTargetType {
        "StockTrade" => Stock,
        "UsStockOptions" => UsOption,
        "HkStockWarrant" => HkWarrant,
        "ETF" => Etf,
        "LOF" => Lof,
    }
    IpoCouponTargetType {
        "FinancingIPO" => Financing,
        "CashIPO" => Cash,
    }
    CommissionUnlimitedType {
        "Proportionate" => Proportionate,
        "Full" => Full,
        "Lifetime" => Lifetime,
        "ByAmountPer" => ByAmountPer,
        "ByRatePer" => ByRatePer,
    }
    QuoteInvestorType {
        "INVESTOR_TYPE_ANY" => Any,
        "INVESTOR_TYPE_NON_PROFESSIONAL" => NonProfessional,
        "INVESTOR_TYPE_PROFESSIONAL" => Professional,
    }
);

/// Decodes a list of enum values (each a name or a code); anything other
/// than a JSON array becomes empty.
fn enum_vec<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: serde::de::DeserializeOwned,
{
    match serde_json::Value::deserialize(deserializer)? {
        serde_json::Value::Array(items) => items
            .into_iter()
            .map(serde_json::from_value)
            .collect::<Result<Vec<_>, _>>()
            .map_err(D::Error::custom),
        _ => Ok(Vec::new()),
    }
}

/// Decodes an optional nested object: `null` or anything other than a JSON
/// object (the server sends `null` for the detail objects of other coupon
/// types) becomes `None`.
fn object_opt<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: serde::de::DeserializeOwned,
{
    match serde_json::Value::deserialize(deserializer)? {
        value @ serde_json::Value::Object(_) => serde_json::from_value(value)
            .map(Some)
            .map_err(D::Error::custom),
        _ => Ok(None),
    }
}

/// Decodes a list of nested objects: anything other than a JSON array becomes
/// empty, and elements that are not JSON objects are skipped.
fn object_vec<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: serde::de::DeserializeOwned,
{
    match serde_json::Value::deserialize(deserializer)? {
        serde_json::Value::Array(items) => items
            .into_iter()
            .filter(serde_json::Value::is_object)
            .map(serde_json::from_value)
            .collect::<Result<Vec<_>, _>>()
            .map_err(D::Error::custom),
        _ => Ok(Vec::new()),
    }
}

/// Decodes a nested object, or its default when the value is `null` or not a
/// JSON object.
fn object_or_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: serde::de::DeserializeOwned + Default,
{
    Ok(object_opt(deserializer)?.unwrap_or_default())
}

/// Decodes a list of 64-bit integers leniently (numbers or integer strings);
/// anything other than a JSON array becomes empty.
fn int64_vec<'de, D>(deserializer: D) -> Result<Vec<i64>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(match serde_json::Value::deserialize(deserializer)? {
        serde_json::Value::Array(items) => items
            .iter()
            .map(|v| serde_utils::lenient_int_opt(v).unwrap_or_default())
            .collect(),
        _ => Vec::new(),
    })
}

/// Commission terms of a commission coupon (`commission`)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default, remote = "Self")]
pub struct CouponCommission {
    /// Markets the coupon applies to
    #[serde(deserialize_with = "serde_utils::string_vec_lenient")]
    pub markets: Vec<String>,
    /// Rebate rate, as returned by the server
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub rebate_rate: String,
    /// Instrument types the coupon applies to
    #[serde(deserialize_with = "enum_vec")]
    pub bz_types: Vec<CouponTargetType>,
    /// The server's numeric codes of `bz_types`, in order, including codes this
    /// SDK does not know (which are `Unknown` in `bz_types`)
    pub bz_types_raw: Vec<i32>,
    /// Remaining commission balance
    #[serde(with = "serde_utils::decimal_opt_lenient")]
    pub commission_balance: Option<Decimal>,
    /// Commission duration in seconds (the app reads 200 years or more as
    /// lifetime free commission)
    #[serde(deserialize_with = "serde_utils::int64_lenient")]
    pub commission_duration: i64,
    /// Kind of commission coupon (the server's `unlimited` code)
    pub unlimited: CommissionUnlimitedType,
    /// The server's numeric code of `unlimited`, also when it is a code this
    /// SDK does not know (`unlimited` is then `Unknown`); `None` when the
    /// server sent none
    pub unlimited_raw: Option<i32>,
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
    /// Amount cap per use of a per-use discount card
    /// ([`CommissionUnlimitedType::ByRatePer`]), as returned by the server
    #[serde(deserialize_with = "serde_utils::int64_lenient")]
    pub amount_limit: i64,
    /// Whether the coupon can be combined with other coupons
    #[serde(deserialize_with = "serde_utils::bool_lenient")]
    pub superposition: bool,
}

/// Terms of an AI benefit coupon (`ai_benefit`)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct CouponAiBenefit {
    /// Length of the benefit in days
    #[serde(deserialize_with = "serde_utils::int32_lenient")]
    pub benefit_days: i32,
    /// Start of the benefit (`None` when not set)
    #[serde(with = "serde_utils::account_time_opt")]
    pub benefit_start: Option<OffsetDateTime>,
    /// End of the benefit (`None` when not set)
    #[serde(with = "serde_utils::account_time_opt")]
    pub benefit_end: Option<OffsetDateTime>,
    /// Benefit tag, as returned by the server
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub benefit_tag: String,
}

/// Terms of a cash coupon (`cash`)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct CouponCash {
    /// Receive duration, as returned by the server (its unit and meaning are
    /// not confirmed)
    #[serde(deserialize_with = "serde_utils::int64_lenient")]
    pub receive_duration: i64,
}

/// A store of a coffee coupon
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct CouponCoffeeStore {
    /// Store ID
    #[serde(deserialize_with = "serde_utils::int64_lenient")]
    pub store_id: i64,
    /// Store name
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub store_name: String,
}

/// Terms of a coffee coupon (`coffee_coupon`)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct CouponCoffee {
    /// Coupon channel, as returned by the server
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub coupon_channel: String,
    /// Coupon ID, as returned by the server
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub coupon_id: String,
    /// Voucher template code, e.g. `coffee-50` (not a personal redemption
    /// code)
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub coupon_no: String,
    /// Whether the coupon can be used at every store
    #[serde(deserialize_with = "serde_utils::bool_lenient")]
    pub is_available_all_store: bool,
    /// Period type (e.g. `day`), as returned by the server
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub period_type: String,
    /// ID of the store the coupon was settled at (`0` when not settled)
    #[serde(deserialize_with = "serde_utils::int64_lenient")]
    pub settled_store_id: i64,
    /// IDs of the stores the coupon can be used at
    #[serde(deserialize_with = "int64_vec")]
    pub store_id_list: Vec<i64>,
    /// Stores the coupon can be used at
    #[serde(deserialize_with = "object_vec")]
    pub store_list: Vec<CouponCoffeeStore>,
    /// Threshold, as returned by the server
    #[serde(deserialize_with = "serde_utils::int64_lenient")]
    pub threshold: i64,
    /// Use limit (`total` or `period`), as returned by the server
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub use_limit: String,
}

/// A counter (security) of a crypto coupon
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct CouponCryptoCounter {
    /// Counter ID
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub counter_id: String,
}

/// Fallback of a crypto coupon
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct CouponCryptoFallback {
    /// Whether the fallback is enabled
    #[serde(deserialize_with = "serde_utils::bool_lenient")]
    pub enabled: bool,
    /// Counter ID
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub counter_id: String,
}

/// Terms of a crypto coupon (`crypto`)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct CouponCrypto {
    /// Fallback
    #[serde(deserialize_with = "object_or_default")]
    pub fallback: CouponCryptoFallback,
    /// Optional counters
    #[serde(deserialize_with = "object_vec")]
    pub optional_counters: Vec<CouponCryptoCounter>,
}

/// Terms of a finance interest coupon (`finance_interest`)
///
/// The meaning of these fields is not confirmed; they are returned as the
/// server sends them.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct CouponFinanceInterest {
    /// Discount mode, as returned by the server
    #[serde(deserialize_with = "serde_utils::int32_lenient")]
    pub discount_mode: i32,
    /// Discount rate
    #[serde(with = "serde_utils::decimal_opt_lenient")]
    pub discount_rate: Option<Decimal>,
    /// Whether the coupon can be combined with other coupons
    #[serde(deserialize_with = "serde_utils::bool_lenient")]
    pub superposition: bool,
}

/// Terms of a fund deduction coupon (`fund_deduct`)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct CouponFundDeduct {
    /// Maximum risk level, as returned by the server
    #[serde(deserialize_with = "serde_utils::int32_lenient")]
    pub max_risk_level: i32,
    /// Minimum order amount, as returned by the server
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub min_order_amount: String,
    /// Order types, as returned by the server
    #[serde(deserialize_with = "serde_utils::string_vec_lenient")]
    pub order_types: Vec<String>,
    /// Underlying type, as returned by the server
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub underlying_type: String,
    /// Underlying values, as returned by the server
    #[serde(deserialize_with = "serde_utils::string_vec_lenient")]
    pub underlying_values: Vec<String>,
    /// Underlying names, as returned by the server
    #[serde(deserialize_with = "serde_utils::string_vec_lenient")]
    pub underlying_names: Vec<String>,
}

/// Terms of a quote (market data) or fundamental-data coupon (`quote` /
/// `fundamental`)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default, remote = "Self")]
pub struct CouponQuote {
    /// Current end time (`None` when not set)
    #[serde(with = "serde_utils::account_time_opt")]
    pub current_end_time: Option<OffsetDateTime>,
    /// Whether the coupon is inapplicable, as returned by the server
    #[serde(deserialize_with = "serde_utils::bool_lenient")]
    pub inapplicable: bool,
    /// Investor type the coupon applies to
    pub investor_type: QuoteInvestorType,
    /// The server's numeric code of `investor_type`, also when it is a code
    /// this SDK does not know (`investor_type` is then `Unknown`); `None` when
    /// the server sent none
    pub investor_type_raw: Option<i32>,
    /// Whether the coupon is activated manually
    #[serde(deserialize_with = "serde_utils::bool_lenient")]
    pub manual_activate: bool,
    /// Markets
    #[serde(deserialize_with = "serde_utils::string_vec_lenient")]
    pub markets: Vec<String>,
    /// Investor type of the member, as returned by the server
    #[serde(deserialize_with = "serde_utils::int32_lenient")]
    pub member_investor_type: i32,
    /// Paired SPU number, as returned by the server
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub pair_spu_no: String,
    /// Quote duration in seconds (the app reads 200 years or more as
    /// lifetime)
    #[serde(deserialize_with = "serde_utils::int64_lenient")]
    pub quote_duration: i64,
    /// Quote market
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub quote_market: String,
    /// Quote tags
    #[serde(deserialize_with = "serde_utils::string_vec_lenient")]
    pub quote_tags: Vec<String>,
    /// Whether the coupon can be combined with other coupons
    #[serde(deserialize_with = "serde_utils::bool_lenient")]
    pub superposition: bool,
}

/// Terms of an IPO coupon (`ipo`)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct CouponIpo {
    /// Subscription ID
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub subscription_id: String,
}

/// Terms of an IPO commission coupon (`ipo_commission`)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default, remote = "Self")]
pub struct CouponIpoCommission {
    /// IPO subscription types the coupon applies to
    #[serde(deserialize_with = "enum_vec")]
    pub bz_types: Vec<IpoCouponTargetType>,
    /// The server's numeric codes of `bz_types`, in order, including codes this
    /// SDK does not know (which are `Unknown` in `bz_types`)
    pub bz_types_raw: Vec<i32>,
    /// Counter ID
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub counter_id: String,
    /// Counter name
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub counter_name: String,
    /// Financing limit, as returned by the server
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub financing_limit: String,
}

/// Terms of an IPO interest coupon (`ipo_interest`)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct CouponIpoInterest {
    /// Annual interest discount rate, as returned by the server (the app
    /// computes the daily discount as rate × financed amount / 365)
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub discount_rate: String,
}

/// Terms of a platform fee deduction coupon (`platform_fee_deduct`)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default, remote = "Self")]
pub struct CouponPlatformFeeDeduct {
    /// Instrument types the coupon applies to
    #[serde(deserialize_with = "enum_vec")]
    pub bz_types: Vec<CouponTargetType>,
    /// The server's numeric codes of `bz_types`, in order, including codes this
    /// SDK does not know (which are `Unknown` in `bz_types`)
    pub bz_types_raw: Vec<i32>,
    /// Counter ID
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub counter_id: String,
    /// Counter name
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub counter_name: String,
    /// Markets the coupon applies to
    #[serde(deserialize_with = "serde_utils::string_vec_lenient")]
    pub markets: Vec<String>,
    /// Number of times used
    #[serde(deserialize_with = "serde_utils::int64_lenient")]
    pub usage_count: i64,
    /// Number of uses left
    #[serde(deserialize_with = "serde_utils::int64_lenient")]
    pub lave_usage_count: i64,
    /// Whether the coupon can be combined with other coupons
    #[serde(deserialize_with = "serde_utils::bool_lenient")]
    pub superposition: bool,
}

/// An optional counter of a stock coupon
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct CouponStockCounter {
    /// Counter ID
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub counter_id: String,
    /// Shares, as returned by the server
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub shares: String,
}

/// Fallback of a stock coupon
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct CouponStockFallback {
    /// Whether the fallback is enabled
    #[serde(deserialize_with = "serde_utils::bool_lenient")]
    pub enabled: bool,
    /// Fallback type (the server's `type`), as returned by the server
    #[serde(
        rename(deserialize = "type"),
        alias = "fallback_type",
        deserialize_with = "serde_utils::string_lenient"
    )]
    pub fallback_type: String,
    /// Counter ID
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub counter_id: String,
    /// Shares, as returned by the server
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub shares: String,
}

/// Terms of a stock coupon (`stock`)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct CouponStock {
    /// Counter ID
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub counter_id: String,
    /// Counter name
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub counter_name: String,
    /// Amount
    #[serde(with = "serde_utils::decimal_opt_lenient")]
    pub amount: Option<Decimal>,
    /// Redemption type (`by_card_value` or `shares_number`), as returned by
    /// the server
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub redemption_type: String,
    /// Delivery way (`card_exchange` or `account_transfer`), as returned by
    /// the server
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub send_way: String,
    /// Optional counters
    #[serde(deserialize_with = "object_vec")]
    pub optional_counters: Vec<CouponStockCounter>,
    /// Fallback
    #[serde(deserialize_with = "object_or_default")]
    pub fallback: CouponStockFallback,
}

/// Terms of a stock fragment coupon (`stock_fragment`)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default, remote = "Self")]
pub struct CouponStockFragment {
    /// Counter ID
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub counter_id: String,
    /// Counter name
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub counter_name: String,
    /// Market
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub market: String,
    /// Markets
    #[serde(deserialize_with = "serde_utils::string_vec_lenient")]
    pub markets: Vec<String>,
    /// Instrument type the coupon applies to
    pub underlying_type: CouponTargetType,
    /// The server's numeric code of `underlying_type`, also when it is a code
    /// this SDK does not know (`underlying_type` is then `Unknown`); `None`
    /// when the server sent none
    pub underlying_type_raw: Option<i32>,
    /// Amounts, as returned by the server (key-ordered)
    #[serde(deserialize_with = "serde_utils::string_map_lenient")]
    pub amounts: std::collections::BTreeMap<String, String>,
}

/// A coupon (reward card)
///
/// The type-specific terms are in the field named after the coupon type
/// (`commission`, `quote`, `stock`, …); the fields of other types are
/// `None`. App styling (cover, background, …) and the server's deprecated
/// top-level duplicates of the commission terms are not modelled, nor is the
/// `fund_interest` object, which the server does not describe.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default, remote = "Self")]
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
    /// The server's numeric code of `card_status`, also when it is a code this
    /// SDK does not know (`card_status` is then `Unknown`); `None` when the
    /// server sent none
    pub card_status_raw: Option<i32>,
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
    /// Trading sessions the coupon applies to, as returned by the server
    #[serde(deserialize_with = "serde_utils::string_vec_lenient")]
    pub trade_sessions: Vec<String>,
    /// Subscription ID, as returned by the server
    #[serde(deserialize_with = "serde_utils::string_lenient")]
    pub subscription_id: String,
    /// Terms of an AI benefit coupon
    #[serde(deserialize_with = "object_opt")]
    pub ai_benefit: Option<CouponAiBenefit>,
    /// Terms of a cash coupon
    #[serde(deserialize_with = "object_opt")]
    pub cash: Option<CouponCash>,
    /// Terms of a coffee coupon
    #[serde(deserialize_with = "object_opt")]
    pub coffee_coupon: Option<CouponCoffee>,
    /// Terms of a commission coupon
    #[serde(deserialize_with = "object_opt")]
    pub commission: Option<CouponCommission>,
    /// Terms of a crypto coupon
    #[serde(deserialize_with = "object_opt")]
    pub crypto: Option<CouponCrypto>,
    /// Terms of a finance interest coupon
    #[serde(deserialize_with = "object_opt")]
    pub finance_interest: Option<CouponFinanceInterest>,
    /// Terms of a fund deduction coupon
    #[serde(deserialize_with = "object_opt")]
    pub fund_deduct: Option<CouponFundDeduct>,
    /// Terms of a fundamental-data coupon
    #[serde(deserialize_with = "object_opt")]
    pub fundamental: Option<CouponQuote>,
    /// Terms of an IPO coupon
    #[serde(deserialize_with = "object_opt")]
    pub ipo: Option<CouponIpo>,
    /// Terms of an IPO commission coupon
    #[serde(deserialize_with = "object_opt")]
    pub ipo_commission: Option<CouponIpoCommission>,
    /// Terms of an IPO interest coupon
    #[serde(deserialize_with = "object_opt")]
    pub ipo_interest: Option<CouponIpoInterest>,
    /// Terms of a platform fee deduction coupon
    #[serde(deserialize_with = "object_opt")]
    pub platform_fee_deduct: Option<CouponPlatformFeeDeduct>,
    /// Terms of a quote (market data) coupon
    #[serde(deserialize_with = "object_opt")]
    pub quote: Option<CouponQuote>,
    /// Terms of a stock coupon
    #[serde(deserialize_with = "object_opt")]
    pub stock: Option<CouponStock>,
    /// Terms of a stock fragment coupon
    #[serde(deserialize_with = "object_opt")]
    pub stock_fragment: Option<CouponStockFragment>,
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
/// Receipt images uploaded with the deposit are intentionally not modelled;
/// only their number is ([`Self::voucher_count`]).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default, remote = "Self")]
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
    /// The server's numeric code of `state_code`, also when it is a code this
    /// SDK does not know (`state_code` is then `Unknown`); `None` when the
    /// server sent none
    pub state_code_raw: Option<i32>,
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
    /// Number of receipts uploaded with the deposit (the length of the
    /// server's `vouchers`)
    #[serde(
        rename(deserialize = "vouchers"),
        alias = "voucher_count",
        deserialize_with = "item_count"
    )]
    pub voucher_count: i64,
}

/// Decodes the number of items of a JSON array (the server's `vouchers`,
/// whose items are not modelled); a number (`voucher_count`, on a round trip)
/// is read as the count itself.
fn item_count<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(match serde_json::Value::deserialize(deserializer)? {
        serde_json::Value::Array(items) => i64::try_from(items.len()).unwrap_or(i64::MAX),
        other => serde_utils::lenient_int_opt(&other).unwrap_or_default(),
    })
}

#[derive(Debug, Default, Deserialize)]
pub(crate) struct DepositsResponse {
    #[serde(default, deserialize_with = "serde_utils::null_as_default")]
    pub(crate) items: Vec<Deposit>,
}

/// A withdrawal record
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default, remote = "Self")]
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
    /// The server's numeric code of `state_code`, also when it is a code this
    /// SDK does not know (`state_code` is then `Unknown`); `None` when the
    /// server sent none
    pub state_code_raw: Option<i32>,
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

/// The server's numeric code behind a typed value: the wire value when it is
/// a number or an integer string, else the typed value's code when it is a
/// known one (the server sent its name), else `None`.
fn raw_code<T>(typed: T, wire: Option<&serde_json::Value>) -> Option<i32>
where
    T: Into<i32> + Copy + Default + PartialEq,
{
    wire.and_then(serde_utils::lenient_int_opt)
        .and_then(|v| i32::try_from(v).ok())
        .or_else(|| (typed != T::default()).then(|| typed.into()))
}

/// [`raw_code`] for each element of a list; elements with no numeric code
/// are left out.
fn raw_codes<T>(typed: &[T], wire: Option<&serde_json::Value>) -> Vec<i32>
where
    T: Into<i32> + Copy + Default + PartialEq,
{
    match wire {
        Some(serde_json::Value::Array(items)) => items
            .iter()
            .zip(typed.iter().copied().chain(std::iter::repeat(T::default())))
            .filter_map(|(item, typed)| raw_code(typed, Some(item)))
            .collect(),
        _ => Vec::new(),
    }
}

/// Implements `Serialize` / `Deserialize` for a record decoded with
/// `#[serde(remote = "Self")]`, filling each `*_raw` companion field with the
/// server's numeric code read from the record's own keys (the first key
/// present wins). A record that already carries the `*_raw` key (one this SDK
/// serialized) keeps that value, so a round trip loses nothing.
macro_rules! impl_raw_codes {
    (@one $out:ident $value:ident $field:ident $raw:ident [$($key:literal),+] scalar) => {
        if $value.get(stringify!($raw)).is_none() {
            $out.$raw = raw_code($out.$field, [$($key),+].iter().find_map(|k| $value.get(*k)));
        }
    };
    (@one $out:ident $value:ident $field:ident $raw:ident [$($key:literal),+] vec) => {
        if $value.get(stringify!($raw)).is_none() {
            $out.$raw = raw_codes(&$out.$field, [$($key),+].iter().find_map(|k| $value.get(*k)));
        }
    };
    (@kind $out:ident $value:ident [$field:ident] => $raw:ident [$($key:literal),+]) => {
        impl_raw_codes!(@one $out $value $field $raw [$($key),+] vec);
    };
    (@kind $out:ident $value:ident $field:ident => $raw:ident [$($key:literal),+]) => {
        impl_raw_codes!(@one $out $value $field $raw [$($key),+] scalar);
    };
    ($($ty:ident { $($spec:tt)* })*) => {
        $(
            impl Serialize for $ty {
                fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                    $ty::serialize(self, serializer)
                }
            }

            impl<'de> Deserialize<'de> for $ty {
                fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                    let value = serde_json::Value::deserialize(deserializer)?;
                    #[allow(unused_mut)]
                    let mut out = $ty::deserialize(&value).map_err(D::Error::custom)?;
                    impl_raw_codes!(@fields out value $($spec)*);
                    Ok(out)
                }
            }
        )*
    };
    (@fields $out:ident $value:ident) => {};
    (@fields $out:ident $value:ident [$field:ident] => $raw:ident [$($key:literal),+], $($rest:tt)*) => {
        impl_raw_codes!(@kind $out $value [$field] => $raw [$($key),+]);
        impl_raw_codes!(@fields $out $value $($rest)*);
    };
    (@fields $out:ident $value:ident $field:ident => $raw:ident [$($key:literal),+], $($rest:tt)*) => {
        impl_raw_codes!(@kind $out $value $field => $raw [$($key),+]);
        impl_raw_codes!(@fields $out $value $($rest)*);
    };
}

impl_raw_codes!(
    BankCardAuthorization {
        status => status_raw ["status"],
    }
    BankCard {
        status => status_raw ["status"],
        witness_state => witness_state_raw ["witness_state"],
        bind_state => bind_state_raw ["bind_state"],
    }
    EddaApplication {
        status => status_raw ["status"],
    }
    TransferStock {
        state_code => state_code_raw ["state", "state_code"],
    }
    StockTransferIn {
        state_code => state_code_raw ["state_code"],
    }
    StockTransferOut {
        state_code => state_code_raw ["state_code"],
    }
    CouponCommission {
        [bz_types] => bz_types_raw ["bz_types"],
        unlimited => unlimited_raw ["unlimited"],
    }
    CouponQuote {
        investor_type => investor_type_raw ["investor_type"],
    }
    CouponIpoCommission {
        [bz_types] => bz_types_raw ["bz_types"],
    }
    CouponPlatformFeeDeduct {
        [bz_types] => bz_types_raw ["bz_types"],
    }
    CouponStockFragment {
        underlying_type => underlying_type_raw ["underlying_type"],
    }
    Coupon {
        card_status => card_status_raw ["card_status"],
    }
    Deposit {
        state_code => state_code_raw ["state_code"],
    }
    Withdrawal {
        state_code => state_code_raw ["state", "state_code"],
    }
);

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
            r#"{"list":[{"id":"9","flow_id":"hk_sdda","name":"eDDA","currency":"HKD",
                "status":2,"message":"","created_at":1764064746,"updated_at":"0"},
                {"id":10,"status":42}]}"#,
        )
        .expect("list decodes");
        assert_eq!(list.0.len(), 2, "two applications");
        assert_eq!(list.0[0].status, AuthorizationStatus::Authorizing, "known");
        assert_eq!(list.0[0].flow_id, "hk_sdda", "flow");
        assert_eq!(list.0[0].currency, "HKD", "currency");
        assert!(list.0[0].created_at.is_some(), "filed time");
        assert!(list.0[0].updated_at.is_none(), "\"0\" placeholder");
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
                "card_status":4,"activate_time":"0","cover":"x","trade_sessions":["intraday","overnight"],
                "commission":{"markets":["US"],"bz_types":[0,"HkStockWarrant",3],"rebate_rate":"1",
                    "commission_expired_time":"0","unlimited":2,"usage_count":"2",
                    "amount_limit":"500","superposition":true},
                "cash":null,"quote":null,"stock":[]},
                {"card_id":"c2","card_status":2,"end_time":1798732800000,"commission":null}],
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
        let commission = lifetime.commission.as_ref().expect("commission terms");
        assert_eq!(
            commission.bz_types,
            vec![
                CouponTargetType::Stock,
                CouponTargetType::HkWarrant,
                CouponTargetType::Etf
            ],
            "targets by code or name"
        );
        assert!(lifetime.cash.is_none(), "null detail is None");
        assert!(lifetime.stock.is_none(), "non-object detail is None");
        assert_eq!(
            lifetime.trade_sessions,
            vec!["intraday", "overnight"],
            "trade sessions"
        );
        assert_eq!(
            commission.unlimited,
            CommissionUnlimitedType::Lifetime,
            "unlimited code"
        );
        assert_eq!(commission.usage_count, 2, "usage count");
        assert_eq!(commission.amount_limit, 500, "amount limit");
        assert!(commission.superposition, "superposition");
        assert!(
            commission.commission_expired_time.is_none(),
            "nested placeholder"
        );
        let ms = &resp.list[1];
        assert_eq!(
            ms.end_time,
            Some(datetime!(2026-12-31 16:00 UTC)),
            "milliseconds converted"
        );
        assert!(!ms.no_expiry, "a real end time expires");
        assert!(ms.commission.is_none(), "no commission terms");
        let json = serde_json::to_value(lifetime).expect("serialize");
        assert_eq!(json["no_expiry"], true, "no_expiry serialized");
        let back: Coupon = serde_json::from_value(json).expect("round trip");
        assert!(back.no_expiry, "no_expiry survives a round trip");
        assert_eq!(
            back.commission.map(|c| c.unlimited),
            Some(CommissionUnlimitedType::Lifetime),
            "commission survives a round trip"
        );
    }

    #[test]
    fn coupon_type_terms_decode() {
        // Synthetic values in the server's wire format: int64 as strings,
        // enums as numbers or names, "0" for unset times.
        let coupon: Coupon = serde_json::from_str(
            r#"{"card_id":"c","card_type":"x","subscription_id":"",
                "ai_benefit":{"benefit_days":30,"benefit_start":"1767196800",
                    "benefit_end":"0","benefit_tag":"tag"},
                "cash":{"receive_duration":"7"},
                "coffee_coupon":{"coupon_channel":"chan","coupon_id":"","coupon_no":"N1",
                    "is_available_all_store":true,"period_type":"day","settled_store_id":"0",
                    "store_id_list":["11",12],"store_list":[{"store_id":"11","store_name":"S"}],
                    "threshold":"2","use_limit":"total"},
                "crypto":{"fallback":{"enabled":true,"counter_id":"C1"},
                    "optional_counters":[{"counter_id":"C2"}]},
                "finance_interest":{"discount_mode":1,"discount_rate":"0.5","superposition":false},
                "fund_deduct":{"max_risk_level":"3","min_order_amount":"100",
                    "order_types":["buy"],"underlying_type":"fund","underlying_values":["F1"],
                    "underlying_names":["Fund"]},
                "fund_interest":{},
                "fundamental":{"investor_type":"INVESTOR_TYPE_PROFESSIONAL","markets":["US"]},
                "ipo":{"subscription_id":"S1"},
                "ipo_commission":{"bz_types":[1,"CashIPO",0],"counter_id":"C3",
                    "counter_name":"IPO","financing_limit":"10"},
                "ipo_interest":{"discount_rate":"0.8"},
                "platform_fee_deduct":{"bz_types":["UsStockOptions"],"markets":["US"],
                    "usage_count":"1","lave_usage_count":"4","superposition":true},
                "quote":{"current_end_time":"1767196800","investor_type":1,
                    "member_investor_type":"2","quote_duration":"30","quote_tags":["L2"],
                    "manual_activate":true},
                "stock":{"amount":12.5,"counter_id":"C4","redemption_type":"shares_number",
                    "send_way":"account_transfer","optional_counters":[{"counter_id":"C5",
                    "shares":"3"}],"fallback":{"enabled":true,"type":"cash","shares":"1"}},
                "stock_fragment":{"amounts":{"a":"1"},"market":"US","markets":["US"],
                    "underlying_type":"ETF"}}"#,
        )
        .expect("coupon decodes");
        let ai = coupon.ai_benefit.as_ref().expect("ai benefit");
        assert_eq!(ai.benefit_days, 30, "days");
        assert!(
            ai.benefit_start.is_some() && ai.benefit_end.is_none(),
            "times"
        );
        assert_eq!(coupon.cash.as_ref().map(|c| c.receive_duration), Some(7));
        let coffee = coupon.coffee_coupon.as_ref().expect("coffee");
        assert_eq!(coffee.store_id_list, vec![11, 12], "store ids");
        assert_eq!(coffee.store_list[0].store_id, 11, "store");
        assert_eq!(coffee.settled_store_id, 0, "unset store");
        let crypto = coupon.crypto.as_ref().expect("crypto");
        assert!(crypto.fallback.enabled, "fallback");
        assert_eq!(crypto.optional_counters[0].counter_id, "C2", "counters");
        let finance = coupon.finance_interest.as_ref().expect("finance");
        assert_eq!(finance.discount_rate, Some(dec("0.5")), "rate");
        assert_eq!(
            coupon.fund_deduct.as_ref().map(|f| f.max_risk_level),
            Some(3),
            "risk level"
        );
        assert_eq!(
            coupon.fundamental.as_ref().map(|q| q.investor_type),
            Some(QuoteInvestorType::Professional),
            "investor type by name"
        );
        assert_eq!(
            coupon.ipo_commission.as_ref().map(|i| i.bz_types.clone()),
            Some(vec![
                IpoCouponTargetType::Financing,
                IpoCouponTargetType::Cash,
                IpoCouponTargetType::Unknown
            ]),
            "IPO types"
        );
        assert_eq!(
            coupon
                .platform_fee_deduct
                .as_ref()
                .map(|p| p.bz_types.clone()),
            Some(vec![CouponTargetType::UsOption]),
            "platform fee types"
        );
        let quote = coupon.quote.as_ref().expect("quote");
        assert!(quote.current_end_time.is_some(), "time");
        assert_eq!(quote.investor_type, QuoteInvestorType::NonProfessional);
        assert_eq!(quote.member_investor_type, 2, "member type");
        let stock = coupon.stock.as_ref().expect("stock");
        assert_eq!(stock.amount, Some(dec("12.5")), "amount");
        assert_eq!(stock.fallback.fallback_type, "cash", "fallback type");
        assert_eq!(stock.optional_counters[0].shares, "3", "shares");
        let fragment = coupon.stock_fragment.as_ref().expect("fragment");
        assert_eq!(fragment.underlying_type, CouponTargetType::Etf, "by name");
        assert_eq!(fragment.amounts.get("a").map(String::as_str), Some("1"));
        let back: Coupon =
            serde_json::from_value(serde_json::to_value(&coupon).expect("serialize"))
                .expect("round trip");
        assert_eq!(
            back.stock.map(|s| s.fallback.fallback_type),
            Some("cash".to_string()),
            "renamed field survives a round trip"
        );
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
        assert_eq!(deposit.voucher_count, 1, "receipts counted");
        let back: Deposit = serde_json::from_str(&json).expect("round trip");
        assert_eq!(back.voucher_count, 1, "voucher_count survives a round trip");

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
    fn raw_codes_keep_undocumented_values() {
        let withdrawals: WithdrawalsResponse = serde_json::from_str(
            r#"{"list":[{"id":"w1","state":"42"},{"id":"w2","state":4},{"id":"w3"}]}"#,
        )
        .expect("withdrawals decode");
        assert_eq!(
            withdrawals.list[0].state_code,
            WithdrawalState::Unknown,
            "undocumented"
        );
        assert_eq!(withdrawals.list[0].state_code_raw, Some(42), "raw kept");
        assert_eq!(withdrawals.list[1].state_code_raw, Some(4), "known code");
        assert_eq!(withdrawals.list[2].state_code_raw, None, "absent");
        let json = serde_json::to_value(&withdrawals.list[0]).expect("serialize");
        let back: Withdrawal = serde_json::from_value(json).expect("round trip");
        assert_eq!(back.state_code_raw, Some(42), "raw survives a round trip");

        let cards: BankCardsResponse = serde_json::from_str(
            r#"{"list":[{"id":1,"status":9,"witness_state":"2","bind_state":null,
                "auth_list":[{"status":7}]}]}"#,
        )
        .expect("cards decode");
        let card = &cards.list[0];
        assert_eq!(card.status, BankCardStatus::Unknown, "undocumented");
        assert_eq!(card.status_raw, Some(9), "card status raw");
        assert_eq!(card.witness_state_raw, Some(2), "string code");
        assert_eq!(card.bind_state_raw, None, "null is absent");
        assert_eq!(card.auth_list[0].status_raw, Some(7), "auth raw");

        let coupon: Coupon = serde_json::from_str(
            r#"{"card_id":"c","card_status":9,
                "commission":{"bz_types":[0,"LOF",9,"Other"],"unlimited":"ByRatePer"},
                "stock_fragment":{"underlying_type":"12"},
                "quote":{"investor_type":5}}"#,
        )
        .expect("coupon decodes");
        assert_eq!(coupon.card_status_raw, Some(9), "coupon status raw");
        let commission = coupon.commission.expect("commission");
        assert_eq!(
            commission.bz_types_raw,
            vec![0, 4, 9],
            "names mapped, unknown name left out"
        );
        assert_eq!(commission.unlimited_raw, Some(7), "name mapped to its code");
        assert_eq!(
            coupon.stock_fragment.and_then(|f| f.underlying_type_raw),
            Some(12),
            "raw underlying type"
        );
        assert_eq!(
            coupon.quote.and_then(|q| q.investor_type_raw),
            Some(5),
            "raw investor type"
        );

        let transfers: StockTransferInResponse =
            serde_json::from_str(r#"{"records":[{"state_code":11,"stocks":[{"state":"6"}]}]}"#)
                .expect("transfers decode");
        assert_eq!(transfers.records[0].state_code_raw, Some(11), "record");
        assert_eq!(
            transfers.records[0].stocks[0].state_code_raw,
            Some(6),
            "stock"
        );
        let deposit: Deposit = serde_json::from_str(r#"{"state_code":8}"#).expect("deposit");
        assert_eq!(deposit.state_code_raw, Some(8), "deposit");
        let app: EddaApplication = serde_json::from_str(r#"{"status":5}"#).expect("edda");
        assert_eq!(app.status_raw, Some(5), "edda");
    }

    #[test]
    fn reward_raw_codes_and_detail_kind() {
        let resp: RewardRecordsResponse = serde_json::from_str(
            r#"{"list":[
                {"reward_type":1,"prize":{"status":12,"deliver_status":11,"card_status":"9"}},
                {"reward_type":7,"cash":{"status":99}},
                {"card":{"status":12}},
                {"reward_type":1,"cash":{"status":12},"prize":{"status":12}},
                {"reward_type":3,"cash":[],"card":"x"}
            ]}"#,
        )
        .expect("rewards decode");
        let prize = &resp.list[0];
        assert_eq!(
            prize.deliver_status,
            RewardDeliverStatus::AddressFilled,
            "11 reads as address filled"
        );
        assert_eq!(prize.deliver_status_raw, Some(11), "and keeps its own code");
        assert_eq!(prize.card_status_raw, Some(9), "card status raw");
        let unknown = &resp.list[1];
        assert_eq!(
            unknown.reward_type,
            RewardType::Cash,
            "named after the detail"
        );
        assert_eq!(unknown.reward_type_raw, Some(7), "raw type kept");
        assert_eq!(unknown.status, RewardStatus::Unknown, "undocumented status");
        assert_eq!(unknown.status_raw, Some(99), "raw status kept");
        assert_eq!(resp.list[2].reward_type_raw, None, "no type sent");
        assert_eq!(resp.list[2].deliver_status_raw, None, "no delivery sent");
        assert_eq!(
            resp.list[3].reward_type,
            RewardType::Cash,
            "first detail object wins, as in the app"
        );
        assert_eq!(
            resp.list[4].reward_type,
            RewardType::Task,
            "non-object details skipped"
        );
    }

    #[test]
    fn coupon_lists_skip_malformed_elements() {
        let coupon: Coupon = serde_json::from_str(
            r#"{"coffee_coupon":{"store_list":[{"store_id":"1"},"x",null,2]},
                "crypto":{"fallback":"x","optional_counters":[{"counter_id":"C"},[]]},
                "stock":{"fallback":[1],"optional_counters":{"a":1}}}"#,
        )
        .expect("malformed nested elements do not fail the coupon");
        assert_eq!(
            coupon.coffee_coupon.map(|c| c.store_list.len()),
            Some(1),
            "non-object stores skipped"
        );
        let crypto = coupon.crypto.expect("crypto");
        assert!(
            !crypto.fallback.enabled,
            "malformed fallback is the default"
        );
        assert_eq!(
            crypto.optional_counters.len(),
            1,
            "non-object counter skipped"
        );
        let stock = coupon.stock.expect("stock");
        assert!(
            stock.optional_counters.is_empty(),
            "non-array counters are empty"
        );
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
