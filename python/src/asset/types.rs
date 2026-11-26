use longbridge_python_macros::{PyEnum, PyObject};
use pyo3::prelude::*;

use crate::{decimal::PyDecimal, time::PyOffsetDateTimeWrapper};

/// Statement type
#[pyclass(eq, eq_int, from_py_object)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum StatementType {
    /// Daily statement
    Daily = 1,
    /// Monthly statement
    Monthly = 2,
}

impl From<StatementType> for longbridge::asset::StatementType {
    fn from(value: StatementType) -> Self {
        match value {
            StatementType::Daily => longbridge::asset::StatementType::Daily,
            StatementType::Monthly => longbridge::asset::StatementType::Monthly,
        }
    }
}

/// Statement item
#[pyclass(skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct StatementItem {
    /// Statement date (integer, e.g. 20250301)
    #[pyo3(get)]
    pub dt: i32,
    /// File key used to request the download URL
    #[pyo3(get)]
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
#[pyclass(skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct GetStatementListResponse {
    /// List of statement items
    #[pyo3(get)]
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
#[pyclass(skip_from_py_object)]
#[derive(Debug, Clone)]
pub(crate) struct GetStatementResponse {
    /// Presigned download URL
    #[pyo3(get)]
    pub url: String,
}

impl From<longbridge::asset::GetStatementResponse> for GetStatementResponse {
    fn from(resp: longbridge::asset::GetStatementResponse) -> Self {
        Self { url: resp.url }
    }
}

/// Verification status of a bank card
#[pyclass(eq, eq_int, skip_from_py_object)]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longbridge::asset::BankCardStatus")]
pub(crate) enum BankCardStatus {
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
#[pyclass(eq, eq_int, skip_from_py_object)]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longbridge::asset::BankCardWitnessState")]
pub(crate) enum BankCardWitnessState {
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
#[pyclass(eq, eq_int, skip_from_py_object)]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longbridge::asset::BankCardBindState")]
pub(crate) enum BankCardBindState {
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
#[pyclass(eq, eq_int, skip_from_py_object)]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longbridge::asset::AuthorizationStatus")]
pub(crate) enum AuthorizationStatus {
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
#[pyclass(eq, eq_int, skip_from_py_object)]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longbridge::asset::StockTransferInState")]
pub(crate) enum StockTransferInState {
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
#[pyclass(eq, eq_int, skip_from_py_object)]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longbridge::asset::StockTransferOutState")]
pub(crate) enum StockTransferOutState {
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
#[pyclass(eq, eq_int, skip_from_py_object)]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longbridge::asset::TransferStockState")]
pub(crate) enum TransferStockState {
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
#[pyclass(eq, eq_int, from_py_object)]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longbridge::asset::RewardType")]
pub(crate) enum RewardType {
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
#[pyclass(eq, eq_int, skip_from_py_object)]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longbridge::asset::RewardStatus")]
pub(crate) enum RewardStatus {
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
#[pyclass(eq, eq_int, skip_from_py_object)]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longbridge::asset::RewardDeliverStatus")]
pub(crate) enum RewardDeliverStatus {
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
#[pyclass(eq, eq_int, skip_from_py_object)]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longbridge::asset::CouponStatus")]
pub(crate) enum CouponStatus {
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
#[pyclass(eq, eq_int, from_py_object)]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longbridge::asset::DepositState")]
pub(crate) enum DepositState {
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
#[pyclass(eq, eq_int, skip_from_py_object)]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longbridge::asset::WithdrawalState")]
pub(crate) enum WithdrawalState {
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
#[pyclass(eq, eq_int, from_py_object)]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longbridge::asset::CouponOrderType")]
pub(crate) enum CouponOrderType {
    /// Orders the coupon is locked to (in progress)
    Locked,
    /// Orders whose rebate has been settled
    Settled,
}

/// An authorization attached to a bank card (eDDA, bank securities account,
/// FPS, ...)
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::asset::BankCardAuthorization")]
pub(crate) struct BankCardAuthorization {
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
    #[py(opt)]
    max_amount: Option<PyDecimal>,
    /// Whether the per-debit amount is unlimited
    max_amount_unlimited: bool,
    /// Maximum number of debits per day (none when not set)
    #[py(opt)]
    daily_max_count: Option<i64>,
    /// Whether the daily debit count is unlimited
    daily_max_count_unlimited: bool,
    /// Expiry date of the authorization (none when not set). A long-term
    /// authorization carries the server's 9999-12-31 sentinel here.
    #[py(opt)]
    expiry_date: Option<PyOffsetDateTimeWrapper>,
    /// Whether the authorization never expires
    long_term_valid: bool,
    /// Whether submitted documents are under review
    doc_reviewing: bool,
    /// Whether the "no prior deposit" check is under review
    no_prior_deposit_reviewing: bool,
}

/// A linked bank card
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::asset::BankCard")]
pub(crate) struct BankCard {
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
    #[py(opt)]
    created_at: Option<PyOffsetDateTimeWrapper>,
    /// Authorizations attached to the card
    #[py(array)]
    auth_list: Vec<BankCardAuthorization>,
}

/// An eDDA authorization application of a bank card
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::asset::EddaApplication")]
pub(crate) struct EddaApplication {
    /// Application ID
    id: String,
    /// Application status
    status: AuthorizationStatus,
    /// Failure reason (empty unless the application failed)
    message: String,
}

/// One stock in a stock transfer application
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::asset::TransferStock")]
pub(crate) struct TransferStock {
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
    #[py(opt)]
    amount: Option<PyDecimal>,
    /// Currency
    currency: String,
    /// Cost price
    #[py(opt)]
    cost: Option<PyDecimal>,
    /// State of this stock in the transfer
    state: TransferStockState,
    /// Position side
    position_side: String,
}

/// A stock transfer-in application (another broker to this account)
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::asset::StockTransferIn")]
pub(crate) struct StockTransferIn {
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
    #[py(opt)]
    created_at: Option<PyOffsetDateTimeWrapper>,
    /// Updated time
    #[py(opt)]
    updated_at: Option<PyOffsetDateTimeWrapper>,
    /// Processing start time (none when not set)
    #[py(opt)]
    process_at: Option<PyOffsetDateTimeWrapper>,
    /// Execution time (none when not executed)
    #[py(opt)]
    exec_at: Option<PyOffsetDateTimeWrapper>,
    /// Remark
    remark: String,
    /// Reason (e.g. for a failure)
    reason: String,
    /// Stocks in the application
    #[py(array)]
    stocks: Vec<TransferStock>,
}

/// Response for the stock transfer-in list
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::asset::StockTransferInResponse")]
pub(crate) struct StockTransferInResponse {
    /// Records
    #[py(array)]
    records: Vec<StockTransferIn>,
    /// Total number of records
    total: i64,
}

/// A stock transfer-out application (this account to another broker)
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::asset::StockTransferOut")]
pub(crate) struct StockTransferOut {
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
    #[py(opt)]
    created_at: Option<PyOffsetDateTimeWrapper>,
    /// Updated time
    #[py(opt)]
    updated_at: Option<PyOffsetDateTimeWrapper>,
    /// Processing start time (none when not set)
    #[py(opt)]
    process_at: Option<PyOffsetDateTimeWrapper>,
    /// Execution time (none when not executed)
    #[py(opt)]
    exec_at: Option<PyOffsetDateTimeWrapper>,
    /// Time the other broker agreed (none when not set)
    #[py(opt)]
    agree_at: Option<PyOffsetDateTimeWrapper>,
    /// Time the application was refused (none when not set)
    #[py(opt)]
    refuse_at: Option<PyOffsetDateTimeWrapper>,
    /// Time the application was cancelled (none when not set)
    #[py(opt)]
    cancel_at: Option<PyOffsetDateTimeWrapper>,
    /// Remark
    remark: String,
    /// Reason (e.g. for a rejection)
    reason: String,
    /// Stocks in the application
    #[py(array)]
    stocks: Vec<TransferStock>,
}

/// A reward record from the reward center
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::asset::RewardRecord")]
pub(crate) struct RewardRecord {
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
    #[py(opt)]
    prize_type: Option<i32>,
    /// Amount
    #[py(opt)]
    amount: Option<PyDecimal>,
    /// Currency
    currency: String,
    /// Reward status
    status: RewardStatus,
    /// Delivery status (issued gift rewards only)
    deliver_status: RewardDeliverStatus,
    /// Exit status code, as returned by the server
    exit_status: i32,
    /// Time the reward was sent
    #[py(opt)]
    send_time: Option<PyOffsetDateTimeWrapper>,
    /// Start time
    #[py(opt)]
    start_time: Option<PyOffsetDateTimeWrapper>,
    /// End time
    #[py(opt)]
    end_time: Option<PyOffsetDateTimeWrapper>,
    /// Effective time
    #[py(opt)]
    effective_time: Option<PyOffsetDateTimeWrapper>,
    /// Deadline to claim the reward
    #[py(opt)]
    receive_deadline: Option<PyOffsetDateTimeWrapper>,
}

/// Response for the reward record list
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::asset::RewardRecordsResponse")]
pub(crate) struct RewardRecordsResponse {
    /// Reward records
    #[py(array)]
    list: Vec<RewardRecord>,
    /// Total number of records
    total: i64,
}

/// Commission terms of a commission coupon
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::asset::CouponCommission")]
pub(crate) struct CouponCommission {
    /// Markets the coupon applies to
    markets: Vec<String>,
    /// Rebate rate, as returned by the server
    rebate_rate: String,
    /// Instrument types the coupon applies to: `0` stocks, `1` options, `2`
    /// warrants, `3` ETF, `4` LOF
    bz_types: Vec<i32>,
    /// Remaining commission balance
    #[py(opt)]
    commission_balance: Option<PyDecimal>,
    /// Commission duration, as returned by the server
    commission_duration: String,
    /// Whether the coupon is unlimited
    unlimited: bool,
    /// Commission expiry time (none when not set)
    #[py(opt)]
    commission_expired_time: Option<PyOffsetDateTimeWrapper>,
    /// Number of times used
    usage_count: i64,
    /// Number of uses left
    lave_usage_count: i64,
    /// Settlement model, as returned by the server
    settlement_model: String,
}

/// A coupon (reward card)
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::asset::Coupon")]
pub(crate) struct Coupon {
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
    #[py(opt)]
    start_time: Option<PyOffsetDateTimeWrapper>,
    /// End of validity (none when not set). A date after 2100 means the coupon
    /// never expires.
    #[py(opt)]
    end_time: Option<PyOffsetDateTimeWrapper>,
    /// Activation time (none when not activated)
    #[py(opt)]
    activate_time: Option<PyOffsetDateTimeWrapper>,
    /// Amount
    #[py(opt)]
    amount: Option<PyDecimal>,
    /// Currency
    currency: String,
    /// Trading sessions the coupon applies to, as display text
    available_trade_session: String,
    /// Commission terms (commission coupons only; empty otherwise)
    commission: CouponCommission,
}

/// Response for the coupon list
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::asset::CouponsResponse")]
pub(crate) struct CouponsResponse {
    /// Pinned coupons shown above the list (repeated on every page)
    #[py(array)]
    top_list: Vec<Coupon>,
    /// Coupons on this page
    #[py(array)]
    list: Vec<Coupon>,
    /// Pinned coupons shown below the list (repeated on every page)
    #[py(array)]
    bottom_list: Vec<Coupon>,
    /// Total number of coupons
    total: i64,
    /// Number of expired coupons
    expired_count: i64,
}

/// An order a coupon was applied to
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::asset::CouponOrder")]
pub(crate) struct CouponOrder {
    /// Order ID
    order_id: String,
    /// State text, as returned by the server
    state: String,
    /// State value, as returned by the server (e.g. `FilledStatus`)
    state_val: String,
    /// Executed price
    #[py(opt)]
    executed_price: Option<PyDecimal>,
    /// Executed quantity
    #[py(opt)]
    executed_quantity: Option<PyDecimal>,
    /// Currency
    currency: String,
    /// Settlement currency
    settlement_currency: String,
    /// Rebate amount
    #[py(opt)]
    rebate: Option<PyDecimal>,
    /// Rebate date, as returned by the server
    rebate_dt: String,
}

/// A deposit record
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::asset::Deposit")]
pub(crate) struct Deposit {
    /// Record ID
    id: String,
    /// Created time
    #[py(opt)]
    created_at: Option<PyOffsetDateTimeWrapper>,
    /// Updated time
    #[py(opt)]
    updated_at: Option<PyOffsetDateTimeWrapper>,
    /// Currency
    currency: String,
    /// Notified (applied) amount
    #[py(opt)]
    amount: Option<PyDecimal>,
    /// Received amount (`0` = not received yet)
    #[py(opt)]
    receivable_amount: Option<PyDecimal>,
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
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::asset::Withdrawal")]
pub(crate) struct Withdrawal {
    /// Record ID
    id: String,
    /// Order ID
    order_id: String,
    /// Created time
    #[py(opt)]
    created_at: Option<PyOffsetDateTimeWrapper>,
    /// Currency
    currency: String,
    /// Applied amount
    #[py(opt)]
    amount: Option<PyDecimal>,
    /// Fee
    #[py(opt)]
    fee: Option<PyDecimal>,
    /// Amount paid out
    #[py(opt)]
    settlement_amount: Option<PyDecimal>,
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
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::asset::WithdrawalsResponse")]
pub(crate) struct WithdrawalsResponse {
    /// Withdrawal records
    #[py(array)]
    list: Vec<Withdrawal>,
    /// Total number of records
    total: i64,
}
