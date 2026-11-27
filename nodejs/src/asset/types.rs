use chrono::{DateTime, Utc};
use longbridge_nodejs_macros::{JsEnum, JsObject};

use crate::decimal::Decimal;

/// Statement item
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct StatementItem {
    /// Statement date (integer, e.g. 20250301)
    pub dt: i32,
    /// File key used to request the download URL
    pub file_key: String,
}

impl From<longbridge::asset::StatementItem> for StatementItem {
    fn from(item: longbridge::asset::StatementItem) -> Self {
        Self {
            dt: item.dt,
            file_key: item.file_key,
        }
    }
}

/// Response for get statement list
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct GetStatementListResponse {
    /// List of statement items
    pub list: Vec<StatementItem>,
}

impl From<longbridge::asset::GetStatementListResponse> for GetStatementListResponse {
    fn from(resp: longbridge::asset::GetStatementListResponse) -> Self {
        Self {
            list: resp.list.into_iter().map(Into::into).collect(),
        }
    }
}

/// Response for get statement download URL
#[napi_derive::napi(object)]
#[derive(Debug, Clone)]
pub struct GetStatementDownloadUrlResponse {
    /// Presigned download URL
    pub url: String,
}

impl From<longbridge::asset::GetStatementResponse> for GetStatementDownloadUrlResponse {
    fn from(resp: longbridge::asset::GetStatementResponse) -> Self {
        Self { url: resp.url }
    }
}

/// Verification status of a bank card
///
/// The numeric values of this enum are not the server codes; each
/// member's description names its server code.
#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq, Copy, Clone)]
#[js(remote = "longbridge::asset::BankCardStatus")]
pub enum BankCardStatus {
    /// Unknown: an undocumented code (never sent to the server)
    Unknown,
    /// Not verified (server code 0)
    Unverified,
    /// Under review (server code 1)
    UnderReview,
    /// Verified (server code 2)
    Verified,
}

/// Witness (in-person / video verification) state of a bank card
///
/// The numeric values of this enum are not the server codes; each
/// member's description names its server code.
#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq, Copy, Clone)]
#[js(remote = "longbridge::asset::BankCardWitnessState")]
pub enum BankCardWitnessState {
    /// Unknown: an undocumented code (never sent to the server)
    Unknown,
    /// Witnessing not required (server code 1)
    NotRequired,
    /// Witnessing pending (server code 2)
    Pending,
    /// Witnessed (server code 3)
    Witnessed,
}

/// Binding state of a bank card
///
/// The numeric values of this enum are not the server codes; each
/// member's description names its server code.
#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq, Copy, Clone)]
#[js(remote = "longbridge::asset::BankCardBindState")]
pub enum BankCardBindState {
    /// Unknown: an undocumented code (never sent to the server)
    Unknown,
    /// Binding in progress (server code 0)
    Binding,
    /// Bound (server code 1)
    Bound,
    /// Binding failed (server code 2)
    Failed,
}

/// Status of a bank-card authorization (eDDA, bank securities account, FPS) or
/// of an eDDA application
///
/// The numeric values of this enum are not the server codes; each
/// member's description names its server code.
#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq, Copy, Clone)]
#[js(remote = "longbridge::asset::AuthorizationStatus")]
pub enum AuthorizationStatus {
    /// Unknown: an undocumented code (never sent to the server)
    Unknown,
    /// Not authorized (server code 0)
    Unauthorized,
    /// Authorized (server code 1)
    Authorized,
    /// Authorization in progress (server code 2)
    Authorizing,
    /// Authorization failed (see the message field for the reason) (server code
    /// 3)
    Failed,
}

/// State of a stock transfer-in application (another broker to this account)
///
/// The numeric values of this enum are not the server codes; each
/// member's description names its server code.
#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq, Copy, Clone)]
#[js(remote = "longbridge::asset::StockTransferInState")]
pub enum StockTransferInState {
    /// Unknown: an undocumented code (never sent to the server)
    Unknown,
    /// Submitted (server code 0)
    Submitted,
    /// Cancelled (server code 1)
    Cancelled,
    /// Completed (server code 3)
    Completed,
    /// Processing (server code 5)
    Processing,
    /// Partially completed (server code 7)
    PartiallyCompleted,
    /// Failed (server code 8)
    Failed,
}

/// State of a stock transfer-out application (this account to another broker)
///
/// The numeric values of this enum are not the server codes; each
/// member's description names its server code.
#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq, Copy, Clone)]
#[js(remote = "longbridge::asset::StockTransferOutState")]
pub enum StockTransferOutState {
    /// Unknown: an undocumented code (never sent to the server)
    Unknown,
    /// Submitted (the only state in which the application can still be
    /// cancelled) (server code 0)
    Submitted,
    /// Processing (server code 1)
    Processing,
    /// Executing (server code 2)
    Executing,
    /// Completed (server code 3)
    Completed,
    /// Cancelled (server code 4)
    Cancelled,
    /// Partially completed (server code 5)
    PartiallyCompleted,
    /// Failed (server code 6)
    Failed,
    /// Rejected (server code 7)
    Rejected,
}

/// State of one stock inside a stock transfer application
///
/// The numeric values of this enum are not the server codes; each
/// member's description names its server code.
#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq, Copy, Clone)]
#[js(remote = "longbridge::asset::TransferStockState")]
pub enum TransferStockState {
    /// Unknown: an undocumented code (never sent to the server)
    Unknown,
    /// Processing (server code 0)
    Processing,
    /// Succeeded (server code 1)
    Succeeded,
    /// Failed (server code 2)
    Failed,
}

/// Type of a reward record
///
/// The numeric values of this enum are not the server codes; each
/// member's description names its server code.
#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq, Copy, Clone)]
#[js(remote = "longbridge::asset::RewardType")]
pub enum RewardType {
    /// Unknown: an undocumented code (never sent to the server)
    Unknown,
    /// Cash reward (server code 0)
    Cash,
    /// Gift (prize) (server code 1)
    Gift,
    /// Coupon (server code 2)
    Coupon,
    /// Task reward (server code 3)
    Task,
}

/// Which upstream detail object a reward record came from
#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq, Copy, Clone)]
#[js(remote = "longbridge::asset::RewardDetailKind")]
pub enum RewardDetailKind {
    /// No known detail object (e.g. a task reward); only the reward type is
    /// set
    None,
    /// The `cash` object
    Cash,
    /// The `card` (coupon) object
    Card,
    /// The `prize` (gift) object
    Prize,
}

/// Status of a reward record
///
/// The numeric values of this enum are not the server codes; each
/// member's description names its server code.
#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq, Copy, Clone)]
#[js(remote = "longbridge::asset::RewardStatus")]
pub enum RewardStatus {
    /// Unknown: an undocumented code (never sent to the server)
    Unknown,
    /// Upcoming (server code 0)
    Upcoming,
    /// Cancelled (server code 1)
    Cancelled,
    /// The user exited the activity (server code 2)
    ExitedActivity,
    /// Not claimed yet (server code 8)
    Unclaimed,
    /// Being issued (server code 10)
    Issuing,
    /// Issued (server code 12)
    Issued,
    /// Issuing failed (server code 13)
    Failed,
    /// Exchanged (server code 16)
    Exchanged,
    /// Expired (server code 17)
    Expired,
    /// Refunded (server code 18)
    Refunded,
}

/// Delivery status of an issued prize (only meaningful for issued gift rewards)
///
/// The numeric values of this enum are not the server codes; each
/// member's description names its server code.
#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq, Copy, Clone)]
#[js(remote = "longbridge::asset::RewardDeliverStatus")]
pub enum RewardDeliverStatus {
    /// Unknown: an undocumented code, or no delivery status (never sent to the
    /// server)
    Unknown,
    /// Not activated (server code 0)
    NotActivated,
    /// Activated, or shipping (server code 1)
    ActivatedOrShipping,
    /// Consumed (server code 2)
    Consumed,
    /// E-voucher activating (server code 5)
    EVoucherActivating,
    /// E-voucher expired before activation (server code 6)
    EVoucherExpired,
    /// E-voucher invalid (server code 7)
    EVoucherInvalid,
    /// Shipping address missing (server code 8)
    AddressMissing,
    /// Shipping address filled (server code 9)
    AddressFilled,
    /// Shipping (server code 10)
    Shipping,
}

/// Instrument type a coupon applies to
///
/// The numeric values of this enum are not the server codes; each
/// member's description names its server code.
#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq, Copy, Clone)]
#[js(remote = "longbridge::asset::CouponTargetType")]
pub enum CouponTargetType {
    /// Unknown: an undocumented code (never sent to the server)
    Unknown,
    /// Stocks (server code 0, `StockTrade`)
    Stock,
    /// US stock options (server code 1, `UsStockOptions`)
    UsOption,
    /// HK warrants (server code 2, `HkStockWarrant`)
    HkWarrant,
    /// ETF (server code 3)
    Etf,
    /// LOF (server code 4)
    Lof,
}

/// IPO subscription type an IPO commission coupon applies to
///
/// The numeric values of this enum are not the server codes; each
/// member's description names its server code.
#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq, Copy, Clone)]
#[js(remote = "longbridge::asset::IpoCouponTargetType")]
pub enum IpoCouponTargetType {
    /// Unknown: unset or an undocumented code (never sent to the server)
    Unknown,
    /// Financing (margin) IPO subscription (server code 1, `FinancingIPO`)
    Financing,
    /// Cash IPO subscription (server code 2, `CashIPO`)
    Cash,
}

/// Kind of commission coupon (the server's `unlimited` code). The meanings are
/// the ones the Longbridge app gives the codes; the proto names are given in
/// brackets.
///
/// The numeric values of this enum are not the server codes; each
/// member's description names its server code.
#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq, Copy, Clone)]
#[js(remote = "longbridge::asset::CommissionUnlimitedType")]
pub enum CommissionUnlimitedType {
    /// Unknown: an undocumented code (never sent to the server)
    Unknown,
    /// Time-limited free commission (`Proportionate`, server code 0)
    Proportionate,
    /// Commission rebate (`Full`, server code 1)
    Full,
    /// Commission deduction (`Lifetime`, server code 2)
    Lifetime,
    /// Per-use card with a fixed amount per use: `amount / usage_count`
    /// (`ByAmountPer`, server code 6)
    ByAmountPer,
    /// Per-use discount card, capped at `amount_limit` per use (`ByRatePer`,
    /// server code 7)
    ByRatePer,
}

/// Investor type a quote coupon applies to
///
/// The numeric values of this enum are not the server codes; each
/// member's description names its server code.
#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq, Copy, Clone)]
#[js(remote = "longbridge::asset::QuoteInvestorType")]
pub enum QuoteInvestorType {
    /// Unknown: an undocumented code (never sent to the server)
    Unknown,
    /// Any investor (server code 0, `INVESTOR_TYPE_ANY`)
    Any,
    /// Non-professional investors (server code 1,
    /// `INVESTOR_TYPE_NON_PROFESSIONAL`)
    NonProfessional,
    /// Professional investors (server code 2, `INVESTOR_TYPE_PROFESSIONAL`)
    Professional,
}

/// Status of a coupon
///
/// The numeric values of this enum are not the server codes; each
/// member's description names its server code.
#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq, Copy, Clone)]
#[js(remote = "longbridge::asset::CouponStatus")]
pub enum CouponStatus {
    /// Unknown: an undocumented code (never sent to the server)
    Unknown,
    /// Unused (server code 0)
    Unused,
    /// Used (server code 1)
    Used,
    /// Expired (server code 2)
    Expired,
    /// Being redeemed (server code 3)
    Redeeming,
    /// In use (server code 4)
    InUse,
    /// Processing (server code 5)
    Processing,
}

/// State of a deposit record
///
/// The numeric values of this enum are not the server codes; each
/// member's description names its server code.
#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq, Copy, Clone)]
#[js(remote = "longbridge::asset::DepositState")]
pub enum DepositState {
    /// Unknown: an undocumented code (never sent to the server)
    Unknown,
    /// Processing (server code 0)
    Processing,
    /// Credited to the account (server code 1)
    Credited,
    /// Failed (server code 2)
    Failed,
    /// Cancelled (server code 3)
    Cancelled,
    /// Invalidated (server code 4)
    Invalidated,
    /// Rejected (server code 5)
    Rejected,
}

/// State of a withdrawal record
///
/// The numeric values of this enum are not the server codes; each
/// member's description names its server code.
#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq, Copy, Clone)]
#[js(remote = "longbridge::asset::WithdrawalState")]
pub enum WithdrawalState {
    /// Unknown: an undocumented code (never sent to the server)
    Unknown,
    /// Submitted (server code 0)
    Submitted,
    /// Processing (server code 1)
    Processing,
    /// Rejected (server code 2)
    Rejected,
    /// Cancelled (server code 3)
    Cancelled,
    /// Remitted (server code 4)
    Remitted,
    /// Failed (server code 5)
    Failed,
    /// Refunded (server code 6)
    Refunded,
    /// Approved, awaiting payout (server code 7)
    ApprovedAwaitingPayout,
    /// Large-amount review (server code 8)
    LargeAmountReview,
    /// Change pending approval (server code 9)
    ChangePendingApproval,
    /// Deletion pending approval (server code 10)
    DeletionPendingApproval,
    /// Deleted (server code 11)
    Deleted,
    /// Rejection pending approval (server code 12)
    RejectionPendingApproval,
    /// Awaiting confirmation (server code 15)
    AwaitingConfirmation,
}

/// Which orders of a coupon to list
#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq, Copy, Clone)]
#[js(remote = "longbridge::asset::CouponOrderType")]
pub enum CouponOrderType {
    /// Orders the coupon is locked to (in progress)
    Locked,
    /// Orders whose rebate has been settled
    Settled,
}

/// An authorization attached to a bank card (eDDA, bank securities account,
/// FPS, ...)
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::asset::BankCardAuthorization")]
pub struct BankCardAuthorization {
    /// Authorization flow: `hk_edda` / `hk_sdda` (eDDA), `hk_bank_sec`,
    /// `hk_fps`, ...
    flow_id: String,
    /// Display name of the authorization
    name: String,
    /// Authorized currency
    currency: String,
    /// Authorization status
    status: AuthorizationStatus,
    /// The server's numeric code of `status`, also when it is a code
    /// this SDK does not know (`status` is then `Unknown`); null when the
    /// value is missing or is not a code
    #[js(opt)]
    status_raw: Option<i32>,
    /// Failure reason (empty unless the authorization failed)
    message: String,
    /// Whether this authorization is supported for the card
    is_support: bool,
    /// Maximum amount per debit (none when not set)
    #[js(opt)]
    max_amount: Option<Decimal>,
    /// Whether the per-debit amount is unlimited
    max_amount_unlimited: bool,
    /// Maximum number of debits per day (none when not set)
    #[js(opt)]
    daily_max_count: Option<i64>,
    /// Whether the daily debit count is unlimited
    daily_max_count_unlimited: bool,
    /// Expiry date of the authorization (none when not set and for a
    /// long-term authorization: the server's 9999-12-31 "never expires"
    /// sentinel is none; see `long_term_valid`)
    #[js(opt, datetime)]
    expiry_date: Option<DateTime<Utc>>,
    /// Whether the authorization never expires
    long_term_valid: bool,
    /// Whether submitted documents are under review
    doc_reviewing: bool,
    /// Whether the "no prior deposit" check is under review
    no_prior_deposit_reviewing: bool,
}

/// A linked bank card
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::asset::BankCard")]
pub struct BankCard {
    /// Bank card ID (pass to edda_applications)
    id: String,
    /// Bank ID
    bank_id: String,
    /// Bank name
    bank: String,
    /// Bank name in English
    bank_en: String,
    /// Bank code
    bank_code: String,
    /// Bank account number, as returned by the server (not masked)
    account: String,
    /// Account type
    account_type: String,
    /// Country or region of the bank (e.g. `Hong Kong`; not a currency)
    country: String,
    /// Country or region as displayed in the app, as returned by the server
    country_display: String,
    /// Region code
    region: String,
    /// Region name
    region_name: String,
    /// SWIFT code
    swift_code: String,
    /// Remark
    remark: String,
    /// Verification status
    status: BankCardStatus,
    /// The server's numeric code of `status`, also when it is a code
    /// this SDK does not know (`status` is then `Unknown`); null when the
    /// value is missing or is not a code
    #[js(opt)]
    status_raw: Option<i32>,
    /// Witness state
    witness_state: BankCardWitnessState,
    /// The server's numeric code of `witness_state`, also when it is a code
    /// this SDK does not know (`witness_state` is then `Unknown`); null when
    /// the value is missing or is not a code
    #[js(opt)]
    witness_state_raw: Option<i32>,
    /// Whether the card supports witnessing
    support_witness: bool,
    /// Binding state
    bind_state: BankCardBindState,
    /// The server's numeric code of `bind_state`, also when it is a code
    /// this SDK does not know (`bind_state` is then `Unknown`); null when the
    /// value is missing or is not a code
    #[js(opt)]
    bind_state_raw: Option<i32>,
    /// Binding hint text
    bind_tip: String,
    /// Withdrawal fee text, as returned by the server
    withdraw_fee: String,
    /// Time the card was added
    #[js(opt, datetime)]
    created_at: Option<DateTime<Utc>>,
    /// Authorizations attached to the card
    #[js(array)]
    auth_list: Vec<BankCardAuthorization>,
}

/// An eDDA authorization application of a bank card
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::asset::EddaApplication")]
pub struct EddaApplication {
    /// Application ID
    id: String,
    /// Authorization flow: `hk_edda` (authorized at the bank) or `hk_sdda`
    /// (authorized in the app)
    flow_id: String,
    /// Display name of the authorization
    name: String,
    /// Authorized currency
    currency: String,
    /// Application status
    status: AuthorizationStatus,
    /// The server's numeric code of `status`, also when it is a code
    /// this SDK does not know (`status` is then `Unknown`); null when the
    /// value is missing or is not a code
    #[js(opt)]
    status_raw: Option<i32>,
    /// Failure reason (empty unless the application failed)
    message: String,
    /// Time the application was filed
    #[js(opt, datetime)]
    created_at: Option<DateTime<Utc>>,
    /// Time the application was last updated
    #[js(opt, datetime)]
    updated_at: Option<DateTime<Utc>>,
}

/// One stock in a stock transfer application
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::asset::TransferStock")]
pub struct TransferStock {
    /// Stock code
    code: String,
    /// Stock name
    name: String,
    /// Stock name in English
    name_en: String,
    /// Market
    market: String,
    /// Counter ID
    counter_id: String,
    /// Quantity
    #[js(opt)]
    amount: Option<Decimal>,
    /// Currency
    currency: String,
    /// Cost price
    #[js(opt)]
    cost: Option<Decimal>,
    /// State code of this stock in the transfer
    state_code: TransferStockState,
    /// The server's numeric code of `state_code`, also when it is a code
    /// this SDK does not know (`state_code` is then `Unknown`); null when the
    /// value is missing or is not a code
    #[js(opt)]
    state_code_raw: Option<i32>,
    /// Quantity held in street name, as returned by the server
    #[js(opt)]
    street_amount: Option<Decimal>,
    /// Quantity held by a nominee, as returned by the server
    #[js(opt)]
    nominee_amount: Option<Decimal>,
    /// Quantity held in the holder's own name, as returned by the server
    #[js(opt)]
    own_amount: Option<Decimal>,
    /// Number of share certificates, as returned by the server
    certificate_count: i64,
    /// Position side
    position_side: String,
}

/// A stock transfer-in application (another broker to this account)
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::asset::StockTransferIn")]
pub struct StockTransferIn {
    /// Record ID
    id: String,
    /// Order ID
    order_id: String,
    /// Market
    market: String,
    /// Name of the other broker
    broker: String,
    /// ID of the other broker
    broker_id: String,
    /// English name of the other broker
    broker_en: String,
    /// Broker code of the other broker
    broker_code: String,
    /// Account number at the other broker, as returned by the server (not
    /// masked)
    account: String,
    /// State text, as returned by the server
    state: String,
    /// State code
    state_code: StockTransferInState,
    /// The server's numeric code of `state_code`, also when it is a code
    /// this SDK does not know (`state_code` is then `Unknown`); null when the
    /// value is missing or is not a code
    #[js(opt)]
    state_code_raw: Option<i32>,
    /// App-side state: `noticed` means the user tapped "I have notified the
    /// broker" in the app (the user's own confirmation, not a Longbridge
    /// action); `default` otherwise
    app_state: String,
    /// Whether the application can still be cancelled or changed
    can_modify: bool,
    /// Created time
    #[js(opt, datetime)]
    created_at: Option<DateTime<Utc>>,
    /// Updated time
    #[js(opt, datetime)]
    updated_at: Option<DateTime<Utc>>,
    /// Processing start time (none when not set)
    #[js(opt, datetime)]
    process_at: Option<DateTime<Utc>>,
    /// Execution time (none when not executed)
    #[js(opt, datetime)]
    exec_at: Option<DateTime<Utc>>,
    /// Remark
    remark: String,
    /// Reason (e.g. for a failure)
    reason: String,
    /// Stocks in the application
    #[js(array)]
    stocks: Vec<TransferStock>,
}

/// Response for the stock transfer-in list
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::asset::StockTransferInResponse")]
pub struct StockTransferInResponse {
    /// Records
    #[js(array)]
    records: Vec<StockTransferIn>,
    /// Total number of records
    total: i64,
}

/// A stock transfer-out application (this account to another broker)
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::asset::StockTransferOut")]
pub struct StockTransferOut {
    /// Record ID
    id: String,
    /// Order ID
    order_id: String,
    /// Market
    market: String,
    /// Name of the other broker
    broker: String,
    /// ID of the other broker
    broker_id: String,
    /// English name of the other broker
    broker_en: String,
    /// Broker code of the other broker
    broker_code: String,
    /// Account number at the other broker, as returned by the server (not
    /// masked)
    account: String,
    /// State text, as returned by the server
    state: String,
    /// State code. Only `Submitted` applications can be cancelled
    state_code: StockTransferOutState,
    /// The server's numeric code of `state_code`, also when it is a code
    /// this SDK does not know (`state_code` is then `Unknown`); null when the
    /// value is missing or is not a code
    #[js(opt)]
    state_code_raw: Option<i32>,
    /// App-side state: `noticed` means the user tapped "I have notified the
    /// broker" in the app (the user's own confirmation, not a Longbridge
    /// action); `default` otherwise
    app_state: String,
    /// Created time
    #[js(opt, datetime)]
    created_at: Option<DateTime<Utc>>,
    /// Updated time
    #[js(opt, datetime)]
    updated_at: Option<DateTime<Utc>>,
    /// Processing start time (none when not set)
    #[js(opt, datetime)]
    process_at: Option<DateTime<Utc>>,
    /// Execution time (none when not executed)
    #[js(opt, datetime)]
    exec_at: Option<DateTime<Utc>>,
    /// Time the other broker agreed (none when not set)
    #[js(opt, datetime)]
    agree_at: Option<DateTime<Utc>>,
    /// Time the application was refused (none when not set)
    #[js(opt, datetime)]
    refuse_at: Option<DateTime<Utc>>,
    /// Time the application was cancelled (none when not set)
    #[js(opt, datetime)]
    cancel_at: Option<DateTime<Utc>>,
    /// Remark
    remark: String,
    /// Reason (e.g. for a rejection)
    reason: String,
    /// Stocks in the application
    #[js(array)]
    stocks: Vec<TransferStock>,
}

/// A reward record from the reward center (prize redemption secrets
/// `coupon_secret`, cash card codes `cash_card_code` and shipping contacts are
/// not modelled)
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::asset::RewardRecord")]
pub struct RewardRecord {
    /// Reward type: named after the first detail object the server sent
    /// (`cash`, `card`, `prize`, in that order), else the server's
    /// `reward_type` code
    reward_type: RewardType,
    /// The server's numeric code of `reward_type`, also when it is a code
    /// this SDK does not know (`reward_type` is then `Unknown`); null when the
    /// value is missing or is not a code
    #[js(opt)]
    reward_type_raw: Option<i32>,
    /// The detail object the fields below came from
    detail_kind: RewardDetailKind,
    /// Record ID
    record_id: String,
    /// Title of the activity that granted the reward
    activity_title: String,
    /// Reward title
    title: String,
    /// Reward description
    description: String,
    /// Coupon type (coupon rewards), e.g. `commission`
    card_type: String,
    /// Coupon status (coupon rewards; `CouponStatus.Unknown` otherwise)
    card_status: CouponStatus,
    /// The server's numeric code of `card_status`, also when it is a code
    /// this SDK does not know (`card_status` is then `Unknown`); null when the
    /// value is missing or is not a code
    #[js(opt)]
    card_status_raw: Option<i32>,
    /// Prize type code (gift rewards; `0` = physical goods), none when not set
    #[js(opt)]
    prize_type: Option<i32>,
    /// Amount
    #[js(opt)]
    amount: Option<Decimal>,
    /// Currency
    currency: String,
    /// Reward status
    status: RewardStatus,
    /// The server's numeric code of `status`, also when it is a code
    /// this SDK does not know (`status` is then `Unknown`); null when the
    /// value is missing or is not a code
    #[js(opt)]
    status_raw: Option<i32>,
    /// Delivery status (issued gift rewards only)
    deliver_status: RewardDeliverStatus,
    /// The server's numeric code of `deliver_status` (e.g. 11 where
    /// `deliver_status` reads `AddressFilled`), also when it is a code this SDK
    /// does not know (`deliver_status` is then `Unknown`); null when the value
    /// is missing or is not a code
    #[js(opt)]
    deliver_status_raw: Option<i32>,
    /// Exit status code, as returned by the server
    exit_status: i32,
    /// Time the reward was sent
    #[js(opt, datetime)]
    send_time: Option<DateTime<Utc>>,
    /// Start time
    #[js(opt, datetime)]
    start_time: Option<DateTime<Utc>>,
    /// End time
    #[js(opt, datetime)]
    end_time: Option<DateTime<Utc>>,
    /// Effective time
    #[js(opt, datetime)]
    effective_time: Option<DateTime<Utc>>,
    /// Deadline to claim the reward
    #[js(opt, datetime)]
    receive_deadline: Option<DateTime<Utc>>,
}

/// Response for the reward record list
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::asset::RewardRecordsResponse")]
pub struct RewardRecordsResponse {
    /// Reward records
    #[js(array)]
    list: Vec<RewardRecord>,
    /// Total number of records
    total: i64,
}

/// Commission terms of a commission coupon (`commission`)
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::asset::CouponCommission")]
pub struct CouponCommission {
    /// Markets the coupon applies to
    #[js(array)]
    markets: Vec<String>,
    /// Rebate rate, as returned by the server
    rebate_rate: String,
    /// Instrument types the coupon applies to
    #[js(array)]
    bz_types: Vec<CouponTargetType>,
    /// The server's numeric codes of `bz_types`, including codes this
    /// SDK does not know (`Unknown` in `bz_types`); always the same
    /// length and order as `bz_types`: elements that are neither a code
    /// nor a known name are left out of both
    #[js(array)]
    bz_types_raw: Vec<i32>,
    /// Remaining commission balance
    #[js(opt)]
    commission_balance: Option<Decimal>,
    /// Commission duration in seconds (the app reads 200 years or more as
    /// lifetime free commission)
    commission_duration: i64,
    /// Kind of commission coupon (the server's `unlimited` code)
    unlimited: CommissionUnlimitedType,
    /// The server's numeric code of `unlimited`, also when it is a code
    /// this SDK does not know (`unlimited` is then `Unknown`); null when the
    /// value is missing or is not a code
    #[js(opt)]
    unlimited_raw: Option<i32>,
    /// Commission expiry time (none when not set)
    #[js(opt, datetime)]
    commission_expired_time: Option<DateTime<Utc>>,
    /// Number of times used
    usage_count: i64,
    /// Number of uses left
    lave_usage_count: i64,
    /// Settlement model, as returned by the server
    settlement_model: String,
    /// Amount cap per use of a per-use discount card (`ByRatePer`), as returned
    /// by the server
    amount_limit: i64,
    /// Whether the coupon can be combined with other coupons
    superposition: bool,
}

/// Terms of an AI benefit coupon (`ai_benefit`)
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::asset::CouponAiBenefit")]
pub struct CouponAiBenefit {
    /// Length of the benefit in days
    benefit_days: i32,
    /// Start of the benefit (none when not set)
    #[js(opt, datetime)]
    benefit_start: Option<DateTime<Utc>>,
    /// End of the benefit (none when not set)
    #[js(opt, datetime)]
    benefit_end: Option<DateTime<Utc>>,
    /// Benefit tag, as returned by the server
    benefit_tag: String,
}

/// Terms of a cash coupon (`cash`)
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::asset::CouponCash")]
pub struct CouponCash {
    /// Receive duration, as returned by the server (its unit and meaning are
    /// not confirmed)
    receive_duration: i64,
}

/// A store of a coffee coupon
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::asset::CouponCoffeeStore")]
pub struct CouponCoffeeStore {
    /// Store ID
    store_id: i64,
    /// Store name
    store_name: String,
}

/// Terms of a coffee coupon (`coffee_coupon`)
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::asset::CouponCoffee")]
pub struct CouponCoffee {
    /// Coupon channel, as returned by the server
    coupon_channel: String,
    /// Coupon ID, as returned by the server
    coupon_id: String,
    /// Voucher template code, e.g. `coffee-50` (not a personal redemption code)
    coupon_no: String,
    /// Whether the coupon can be used at every store
    is_available_all_store: bool,
    /// Period type (e.g. `day`), as returned by the server
    period_type: String,
    /// ID of the store the coupon was settled at (`0` when not settled)
    settled_store_id: i64,
    /// IDs of the stores the coupon can be used at
    #[js(array)]
    store_id_list: Vec<i64>,
    /// Stores the coupon can be used at
    #[js(array)]
    store_list: Vec<CouponCoffeeStore>,
    /// Threshold, as returned by the server
    threshold: i64,
    /// Use limit (`total` or `period`), as returned by the server
    use_limit: String,
}

/// A counter (security) of a crypto coupon
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::asset::CouponCryptoCounter")]
pub struct CouponCryptoCounter {
    /// Counter ID
    counter_id: String,
}

/// Fallback of a crypto coupon
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::asset::CouponCryptoFallback")]
pub struct CouponCryptoFallback {
    /// Whether the fallback is enabled
    enabled: bool,
    /// Counter ID
    counter_id: String,
}

/// Terms of a crypto coupon (`crypto`)
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::asset::CouponCrypto")]
pub struct CouponCrypto {
    /// Fallback
    fallback: CouponCryptoFallback,
    /// Optional counters
    #[js(array)]
    optional_counters: Vec<CouponCryptoCounter>,
}

/// Terms of a finance interest coupon (`finance_interest`). The meaning of
/// these fields is not confirmed; they are returned as the server sends them.
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::asset::CouponFinanceInterest")]
pub struct CouponFinanceInterest {
    /// Discount mode, as returned by the server
    discount_mode: i32,
    /// Discount rate
    #[js(opt)]
    discount_rate: Option<Decimal>,
    /// Whether the coupon can be combined with other coupons
    superposition: bool,
}

/// Terms of a fund deduction coupon (`fund_deduct`)
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::asset::CouponFundDeduct")]
pub struct CouponFundDeduct {
    /// Maximum risk level, as returned by the server
    max_risk_level: i32,
    /// Minimum order amount, as returned by the server
    min_order_amount: String,
    /// Order types, as returned by the server
    #[js(array)]
    order_types: Vec<String>,
    /// Underlying type, as returned by the server
    underlying_type: String,
    /// Underlying values, as returned by the server
    #[js(array)]
    underlying_values: Vec<String>,
    /// Underlying names, as returned by the server
    #[js(array)]
    underlying_names: Vec<String>,
}

/// Terms of a quote (market data) or fundamental-data coupon (`quote` /
/// `fundamental`)
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::asset::CouponQuote")]
pub struct CouponQuote {
    /// Current end time (none when not set)
    #[js(opt, datetime)]
    current_end_time: Option<DateTime<Utc>>,
    /// Whether the coupon is inapplicable, as returned by the server
    inapplicable: bool,
    /// Investor type the coupon applies to
    investor_type: QuoteInvestorType,
    /// The server's numeric code of `investor_type`, also when it is a code
    /// this SDK does not know (`investor_type` is then `Unknown`); null when
    /// the value is missing or is not a code
    #[js(opt)]
    investor_type_raw: Option<i32>,
    /// Whether the coupon is activated manually
    manual_activate: bool,
    /// Markets
    #[js(array)]
    markets: Vec<String>,
    /// Investor type of the member, as returned by the server
    member_investor_type: i32,
    /// Paired SPU number, as returned by the server
    pair_spu_no: String,
    /// Quote duration in seconds (the app reads 200 years or more as lifetime)
    quote_duration: i64,
    /// Quote market
    quote_market: String,
    /// Quote tags
    #[js(array)]
    quote_tags: Vec<String>,
    /// Whether the coupon can be combined with other coupons
    superposition: bool,
}

/// Terms of an IPO coupon (`ipo`)
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::asset::CouponIpo")]
pub struct CouponIpo {
    /// Subscription ID
    subscription_id: String,
}

/// Terms of an IPO commission coupon (`ipo_commission`)
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::asset::CouponIpoCommission")]
pub struct CouponIpoCommission {
    /// IPO subscription types the coupon applies to
    #[js(array)]
    bz_types: Vec<IpoCouponTargetType>,
    /// The server's numeric codes of `bz_types`, including codes this
    /// SDK does not know (`Unknown` in `bz_types`); always the same
    /// length and order as `bz_types`: elements that are neither a code
    /// nor a known name are left out of both
    #[js(array)]
    bz_types_raw: Vec<i32>,
    /// Counter ID
    counter_id: String,
    /// Counter name
    counter_name: String,
    /// Financing limit, as returned by the server
    financing_limit: String,
}

/// Terms of an IPO interest coupon (`ipo_interest`)
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::asset::CouponIpoInterest")]
pub struct CouponIpoInterest {
    /// Annual interest discount rate, as returned by the server (the app
    /// computes the daily discount as rate × financed amount / 365)
    discount_rate: String,
}

/// Terms of a platform fee deduction coupon (`platform_fee_deduct`)
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::asset::CouponPlatformFeeDeduct")]
pub struct CouponPlatformFeeDeduct {
    /// Instrument types the coupon applies to
    #[js(array)]
    bz_types: Vec<CouponTargetType>,
    /// The server's numeric codes of `bz_types`, including codes this
    /// SDK does not know (`Unknown` in `bz_types`); always the same
    /// length and order as `bz_types`: elements that are neither a code
    /// nor a known name are left out of both
    #[js(array)]
    bz_types_raw: Vec<i32>,
    /// Counter ID
    counter_id: String,
    /// Counter name
    counter_name: String,
    /// Markets the coupon applies to
    #[js(array)]
    markets: Vec<String>,
    /// Number of times used
    usage_count: i64,
    /// Number of uses left
    lave_usage_count: i64,
    /// Whether the coupon can be combined with other coupons
    superposition: bool,
}

/// An optional counter of a stock coupon
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::asset::CouponStockCounter")]
pub struct CouponStockCounter {
    /// Counter ID
    counter_id: String,
    /// Shares, as returned by the server
    shares: String,
}

/// Fallback of a stock coupon
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::asset::CouponStockFallback")]
pub struct CouponStockFallback {
    /// Whether the fallback is enabled
    enabled: bool,
    /// Fallback type (the server's `type`), as returned by the server
    fallback_type: String,
    /// Counter ID
    counter_id: String,
    /// Shares, as returned by the server
    shares: String,
}

/// Terms of a stock coupon (`stock`)
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::asset::CouponStock")]
pub struct CouponStock {
    /// Counter ID
    counter_id: String,
    /// Counter name
    counter_name: String,
    /// Amount
    #[js(opt)]
    amount: Option<Decimal>,
    /// Redemption type (`by_card_value` or `shares_number`), as returned by the
    /// server
    redemption_type: String,
    /// Delivery way (`card_exchange` or `account_transfer`), as returned by the
    /// server
    send_way: String,
    /// Optional counters
    #[js(array)]
    optional_counters: Vec<CouponStockCounter>,
    /// Fallback
    fallback: CouponStockFallback,
}

/// Terms of a stock fragment coupon (`stock_fragment`)
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::asset::CouponStockFragment")]
pub struct CouponStockFragment {
    /// Counter ID
    counter_id: String,
    /// Counter name
    counter_name: String,
    /// Market
    market: String,
    /// Markets
    #[js(array)]
    markets: Vec<String>,
    /// Instrument type the coupon applies to
    underlying_type: CouponTargetType,
    /// The server's numeric code of `underlying_type`, also when it is a code
    /// this SDK does not know (`underlying_type` is then `Unknown`); null when
    /// the value is missing or is not a code
    #[js(opt)]
    underlying_type_raw: Option<i32>,
    /// Amounts, as returned by the server (an object of strings)
    amounts: std::collections::BTreeMap<String, String>,
}

/// A coupon (reward card)
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::asset::Coupon")]
pub struct Coupon {
    /// Coupon ID (32 hex characters; pass to coupon_orders)
    card_id: String,
    /// Coupon type, e.g. `commission`, `platform_fee_deduct`, `stock`, `quote`,
    /// `cash`
    card_type: String,
    /// Title
    title: String,
    /// Secondary title
    title_sec: String,
    /// Description
    description: String,
    /// Terms and details
    detail: String,
    /// Status
    card_status: CouponStatus,
    /// The server's numeric code of `card_status`, also when it is a code
    /// this SDK does not know (`card_status` is then `Unknown`); null when the
    /// value is missing or is not a code
    #[js(opt)]
    card_status_raw: Option<i32>,
    /// Start of validity
    #[js(opt, datetime)]
    start_time: Option<DateTime<Utc>>,
    /// End of validity (none when not set or when the coupon never expires;
    /// see `noExpiry`)
    #[js(opt, datetime)]
    end_time: Option<DateTime<Utc>>,
    /// Whether the coupon never expires (the server's end time is in or after
    /// 2100)
    no_expiry: bool,
    /// Activation time (none when not activated)
    #[js(opt, datetime)]
    activate_time: Option<DateTime<Utc>>,
    /// Amount
    #[js(opt)]
    amount: Option<Decimal>,
    /// Currency
    currency: String,
    /// Trading sessions the coupon applies to, as display text
    available_trade_session: String,
    /// Trading sessions the coupon applies to, as returned by the server
    #[js(array)]
    trade_sessions: Vec<String>,
    /// Subscription ID, as returned by the server
    subscription_id: String,
    /// Terms of an AI benefit coupon (none for other coupon types)
    #[js(opt)]
    ai_benefit: Option<CouponAiBenefit>,
    /// Terms of a cash coupon (none for other coupon types)
    #[js(opt)]
    cash: Option<CouponCash>,
    /// Terms of a coffee coupon (none for other coupon types)
    #[js(opt)]
    coffee_coupon: Option<CouponCoffee>,
    /// Terms of a commission coupon (none for other coupon types)
    #[js(opt)]
    commission: Option<CouponCommission>,
    /// Terms of a crypto coupon (none for other coupon types)
    #[js(opt)]
    crypto: Option<CouponCrypto>,
    /// Terms of a finance interest coupon (none for other coupon types)
    #[js(opt)]
    finance_interest: Option<CouponFinanceInterest>,
    /// Terms of a fund deduction coupon (none for other coupon types)
    #[js(opt)]
    fund_deduct: Option<CouponFundDeduct>,
    /// Terms of a fundamental-data coupon (none for other coupon types)
    #[js(opt)]
    fundamental: Option<CouponQuote>,
    /// Terms of an IPO coupon (none for other coupon types)
    #[js(opt)]
    ipo: Option<CouponIpo>,
    /// Terms of an IPO commission coupon (none for other coupon types)
    #[js(opt)]
    ipo_commission: Option<CouponIpoCommission>,
    /// Terms of an IPO interest coupon (none for other coupon types)
    #[js(opt)]
    ipo_interest: Option<CouponIpoInterest>,
    /// Terms of a platform fee deduction coupon (none for other coupon types)
    #[js(opt)]
    platform_fee_deduct: Option<CouponPlatformFeeDeduct>,
    /// Terms of a quote (market data) coupon (none for other coupon types)
    #[js(opt)]
    quote: Option<CouponQuote>,
    /// Terms of a stock coupon (none for other coupon types)
    #[js(opt)]
    stock: Option<CouponStock>,
    /// Terms of a stock fragment coupon (none for other coupon types)
    #[js(opt)]
    stock_fragment: Option<CouponStockFragment>,
}

/// Response for the coupon list
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::asset::CouponsResponse")]
pub struct CouponsResponse {
    /// Pinned coupons shown above the list (repeated on every page)
    #[js(array)]
    top_list: Vec<Coupon>,
    /// Coupons on this page
    #[js(array)]
    list: Vec<Coupon>,
    /// Pinned coupons shown below the list (repeated on every page)
    #[js(array)]
    bottom_list: Vec<Coupon>,
    /// Total number of coupons
    total: i64,
    /// Number of expired coupons
    expired_count: i64,
}

/// An order a coupon was applied to
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::asset::CouponOrder")]
pub struct CouponOrder {
    /// Order ID
    order_id: String,
    /// State text, as returned by the server
    state: String,
    /// State value, as returned by the server (e.g. `FilledStatus`)
    state_val: String,
    /// Executed price
    #[js(opt)]
    executed_price: Option<Decimal>,
    /// Executed quantity
    #[js(opt)]
    executed_quantity: Option<Decimal>,
    /// Currency
    currency: String,
    /// Settlement currency
    settlement_currency: String,
    /// Rebate amount
    #[js(opt)]
    rebate: Option<Decimal>,
    /// Rebate date, as returned by the server
    rebate_dt: String,
}

/// A deposit record
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::asset::Deposit")]
pub struct Deposit {
    /// Record ID
    id: String,
    /// Created time
    #[js(opt, datetime)]
    created_at: Option<DateTime<Utc>>,
    /// Updated time
    #[js(opt, datetime)]
    updated_at: Option<DateTime<Utc>>,
    /// Currency
    currency: String,
    /// Notified (applied) amount
    #[js(opt)]
    amount: Option<Decimal>,
    /// Received amount (`0` = not received yet)
    #[js(opt)]
    receivable_amount: Option<Decimal>,
    /// State text, as returned by the server
    state: String,
    /// State code
    state_code: DepositState,
    /// The server's numeric code of `state_code`, also when it is a code
    /// this SDK does not know (`state_code` is then `Unknown`); null when the
    /// value is missing or is not a code
    #[js(opt)]
    state_code_raw: Option<i32>,
    /// Sub-state code, as returned by the server
    sub_state_code: i32,
    /// Deposit method ID
    bank_operation_type_id: String,
    /// Deposit method name
    bank_operation_type_name: String,
    /// Reference number
    reference_no: String,
    /// Whether the deposit can still be cancelled
    can_cancel: bool,
    /// SWIFT code of the paying bank
    user_bank_swift_code: String,
    /// Number of receipts uploaded with the deposit
    voucher_count: i64,
}

/// A withdrawal record
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::asset::Withdrawal")]
pub struct Withdrawal {
    /// Record ID
    id: String,
    /// Order ID
    order_id: String,
    /// Created time
    #[js(opt, datetime)]
    created_at: Option<DateTime<Utc>>,
    /// Currency
    currency: String,
    /// Applied amount
    #[js(opt)]
    amount: Option<Decimal>,
    /// Fee
    #[js(opt)]
    fee: Option<Decimal>,
    /// Amount paid out
    #[js(opt)]
    settlement_amount: Option<Decimal>,
    /// State code
    state_code: WithdrawalState,
    /// The server's numeric code of `state_code`, also when it is a code
    /// this SDK does not know (`state_code` is then `Unknown`); null when the
    /// value is missing or is not a code
    #[js(opt)]
    state_code_raw: Option<i32>,
    /// Raw state, as returned by the server
    raw_state: String,
    /// State display text, as returned by the server
    state_description: String,
    /// Receiving bank account number, as returned by the server (not masked)
    bank_account: String,
    /// Receiving bank account, as displayed in the app
    display_bank_account: String,
    /// SWIFT code of the receiving bank
    user_bank_swift_code: String,
    /// Account channel
    account_channel: String,
}

/// One page of deposit records
///
/// The server also returns a `total`, but it follows the page size rather
/// than the record count, so it is not modelled; see `hasMore`.
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::asset::DepositsResponse")]
pub struct DepositsResponse {
    /// Deposit records, newest first
    #[js(array)]
    items: Vec<Deposit>,
    /// A full page came back (as many records as the page size sent, at
    /// most 20), so the next page may hold more
    has_more: bool,
}

/// Response for the withdrawal list
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::asset::WithdrawalsResponse")]
pub struct WithdrawalsResponse {
    /// Withdrawal records
    #[js(array)]
    list: Vec<Withdrawal>,
    /// Total number of records
    total: i64,
    /// A full page came back (as many records as the page size sent), so the
    /// next page may hold more
    has_more: bool,
}
