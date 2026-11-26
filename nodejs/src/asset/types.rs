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
#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq, Copy, Clone)]
#[js(remote = "longbridge::asset::BankCardStatus")]
pub enum BankCardStatus {
    /// Unknown (an undocumented code)
    Unknown,
    /// Not verified
    Unverified,
    /// Under review
    UnderReview,
    /// Verified
    Verified,
}

/// Witness (in-person / video verification) state of a bank card
#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq, Copy, Clone)]
#[js(remote = "longbridge::asset::BankCardWitnessState")]
pub enum BankCardWitnessState {
    /// Unknown (an undocumented code)
    Unknown,
    /// Witnessing not required
    NotRequired,
    /// Witnessing pending
    Pending,
    /// Witnessed
    Witnessed,
}

/// Binding state of a bank card
#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq, Copy, Clone)]
#[js(remote = "longbridge::asset::BankCardBindState")]
pub enum BankCardBindState {
    /// Unknown (an undocumented code)
    Unknown,
    /// Binding in progress
    Binding,
    /// Bound
    Bound,
    /// Binding failed
    Failed,
}

/// Status of a bank-card authorization (eDDA, bank securities account, FPS) or
/// of an eDDA application
#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq, Copy, Clone)]
#[js(remote = "longbridge::asset::AuthorizationStatus")]
pub enum AuthorizationStatus {
    /// Unknown (an undocumented code)
    Unknown,
    /// Not authorized
    Unauthorized,
    /// Authorized
    Authorized,
    /// Authorization in progress
    Authorizing,
    /// Authorization failed (see the message field for the reason)
    Failed,
}

/// State of a stock transfer-in application (another broker to this account)
#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq, Copy, Clone)]
#[js(remote = "longbridge::asset::StockTransferInState")]
pub enum StockTransferInState {
    /// Unknown (an undocumented code)
    Unknown,
    /// Submitted
    Submitted,
    /// Cancelled
    Cancelled,
    /// Completed
    Completed,
    /// Processing
    Processing,
    /// Partially completed
    PartiallyCompleted,
    /// Failed
    Failed,
}

/// State of a stock transfer-out application (this account to another broker)
#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq, Copy, Clone)]
#[js(remote = "longbridge::asset::StockTransferOutState")]
pub enum StockTransferOutState {
    /// Unknown (an undocumented code)
    Unknown,
    /// Submitted (the only state in which the application can still be
    /// cancelled)
    Submitted,
    /// Processing
    Processing,
    /// Executing
    Executing,
    /// Completed
    Completed,
    /// Cancelled
    Cancelled,
    /// Partially completed
    PartiallyCompleted,
    /// Failed
    Failed,
    /// Rejected
    Rejected,
}

/// State of one stock inside a stock transfer application
#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq, Copy, Clone)]
#[js(remote = "longbridge::asset::TransferStockState")]
pub enum TransferStockState {
    /// Unknown (an undocumented code)
    Unknown,
    /// Processing
    Processing,
    /// Succeeded
    Succeeded,
    /// Failed
    Failed,
}

/// Type of a reward record
#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq, Copy, Clone)]
#[js(remote = "longbridge::asset::RewardType")]
pub enum RewardType {
    /// Unknown (an undocumented code)
    Unknown,
    /// Cash reward
    Cash,
    /// Gift (prize)
    Gift,
    /// Coupon
    Coupon,
    /// Task reward
    Task,
}

/// Status of a reward record
#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq, Copy, Clone)]
#[js(remote = "longbridge::asset::RewardStatus")]
pub enum RewardStatus {
    /// Unknown (an undocumented code)
    Unknown,
    /// Upcoming
    Upcoming,
    /// Cancelled
    Cancelled,
    /// The user exited the activity
    ExitedActivity,
    /// Not claimed yet
    Unclaimed,
    /// Being issued
    Issuing,
    /// Issued
    Issued,
    /// Issuing failed
    Failed,
    /// Exchanged
    Exchanged,
    /// Expired
    Expired,
    /// Refunded
    Refunded,
}

/// Delivery status of an issued prize (only meaningful for issued gift rewards)
#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq, Copy, Clone)]
#[js(remote = "longbridge::asset::RewardDeliverStatus")]
pub enum RewardDeliverStatus {
    /// Unknown (an undocumented code, or no delivery status)
    Unknown,
    /// Not activated
    NotActivated,
    /// Activated, or shipping
    ActivatedOrShipping,
    /// Consumed
    Consumed,
    /// E-voucher activating
    EVoucherActivating,
    /// E-voucher expired before activation
    EVoucherExpired,
    /// E-voucher invalid
    EVoucherInvalid,
    /// Shipping address missing
    AddressMissing,
    /// Shipping address filled
    AddressFilled,
    /// Shipping
    Shipping,
}

/// Status of a coupon
#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq, Copy, Clone)]
#[js(remote = "longbridge::asset::CouponStatus")]
pub enum CouponStatus {
    /// Unknown (an undocumented code)
    Unknown,
    /// Unused
    Unused,
    /// Used
    Used,
    /// Expired
    Expired,
    /// Being redeemed
    Redeeming,
    /// In use
    InUse,
    /// Processing
    Processing,
}

/// State of a deposit record
#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq, Copy, Clone)]
#[js(remote = "longbridge::asset::DepositState")]
pub enum DepositState {
    /// Unknown (an undocumented code)
    Unknown,
    /// Processing
    Processing,
    /// Credited to the account
    Credited,
    /// Failed
    Failed,
    /// Cancelled
    Cancelled,
    /// Invalidated
    Invalidated,
    /// Rejected
    Rejected,
}

/// State of a withdrawal record
#[napi_derive::napi]
#[derive(Debug, JsEnum, Hash, Eq, PartialEq, Copy, Clone)]
#[js(remote = "longbridge::asset::WithdrawalState")]
pub enum WithdrawalState {
    /// Unknown (an undocumented code)
    Unknown,
    /// Submitted
    Submitted,
    /// Processing
    Processing,
    /// Rejected
    Rejected,
    /// Cancelled
    Cancelled,
    /// Remitted
    Remitted,
    /// Failed
    Failed,
    /// Refunded
    Refunded,
    /// Approved, awaiting payout
    ApprovedAwaitingPayout,
    /// Large-amount review
    LargeAmountReview,
    /// Change pending approval
    ChangePendingApproval,
    /// Deletion pending approval
    DeletionPendingApproval,
    /// Deleted
    Deleted,
    /// Rejection pending approval
    RejectionPendingApproval,
    /// Awaiting confirmation
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
    /// Expiry date of the authorization (none when not set). A long-term
    /// authorization carries the server's 9999-12-31 sentinel here.
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
    /// Witness state
    witness_state: BankCardWitnessState,
    /// Whether the card supports witnessing
    support_witness: bool,
    /// Binding state
    bind_state: BankCardBindState,
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
    /// Application status
    status: AuthorizationStatus,
    /// Failure reason (empty unless the application failed)
    message: String,
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
    /// State of this stock in the transfer
    state: TransferStockState,
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

/// A reward record from the reward center
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::asset::RewardRecord")]
pub struct RewardRecord {
    /// Reward type
    reward_type: RewardType,
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
    /// Coupon status code (coupon rewards), as returned by the server
    card_status: i32,
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
    /// Delivery status (issued gift rewards only)
    deliver_status: RewardDeliverStatus,
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

/// Commission terms of a commission coupon
#[napi_derive::napi]
#[derive(Debug, JsObject, Clone)]
#[js(remote = "longbridge::asset::CouponCommission")]
pub struct CouponCommission {
    /// Markets the coupon applies to
    #[js(array)]
    markets: Vec<String>,
    /// Rebate rate, as returned by the server
    rebate_rate: String,
    /// Instrument types the coupon applies to: `0` stocks, `1` options, `2`
    /// warrants, `3` ETF, `4` LOF
    #[js(array)]
    bz_types: Vec<i32>,
    /// Remaining commission balance
    #[js(opt)]
    commission_balance: Option<Decimal>,
    /// Commission duration, as returned by the server
    commission_duration: String,
    /// Whether the coupon is unlimited
    unlimited: bool,
    /// Commission expiry time (none when not set)
    #[js(opt, datetime)]
    commission_expired_time: Option<DateTime<Utc>>,
    /// Number of times used
    usage_count: i64,
    /// Number of uses left
    lave_usage_count: i64,
    /// Settlement model, as returned by the server
    settlement_model: String,
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
    /// Start of validity
    #[js(opt, datetime)]
    start_time: Option<DateTime<Utc>>,
    /// End of validity (none when not set). A date after 2100 means the coupon
    /// never expires.
    #[js(opt, datetime)]
    end_time: Option<DateTime<Utc>>,
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
    /// Commission terms (commission coupons only; empty otherwise)
    commission: CouponCommission,
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
    /// State
    state: WithdrawalState,
    /// Raw state, as returned by the server
    raw_state: String,
    /// State description, as returned by the server
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
}
