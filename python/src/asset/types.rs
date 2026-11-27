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
#[pyclass(eq, eq_int, skip_from_py_object)]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longbridge::asset::BankCardWitnessState")]
pub(crate) enum BankCardWitnessState {
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
#[pyclass(eq, eq_int, skip_from_py_object)]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longbridge::asset::BankCardBindState")]
pub(crate) enum BankCardBindState {
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
#[pyclass(eq, eq_int, skip_from_py_object)]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longbridge::asset::AuthorizationStatus")]
pub(crate) enum AuthorizationStatus {
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
#[pyclass(eq, eq_int, from_py_object)]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longbridge::asset::StockTransferInState")]
pub(crate) enum StockTransferInState {
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
#[pyclass(eq, eq_int, from_py_object)]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longbridge::asset::StockTransferOutState")]
pub(crate) enum StockTransferOutState {
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
#[pyclass(eq, eq_int, skip_from_py_object)]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longbridge::asset::TransferStockState")]
pub(crate) enum TransferStockState {
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
#[pyclass(eq, eq_int, from_py_object)]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longbridge::asset::RewardType")]
pub(crate) enum RewardType {
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
#[pyclass(eq, eq_int, skip_from_py_object)]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longbridge::asset::RewardDetailKind")]
pub(crate) enum RewardDetailKind {
    /// No known detail object (e.g. a task reward); only the reward type is
    /// set
    #[py(remote = "None")]
    None_,
    /// The `cash` object
    Cash,
    /// The `card` (coupon) object
    Card,
    /// The `prize` (gift) object
    Prize,
}

/// Status of a reward record
#[pyclass(eq, eq_int, skip_from_py_object)]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longbridge::asset::RewardStatus")]
pub(crate) enum RewardStatus {
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
#[pyclass(eq, eq_int, skip_from_py_object)]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longbridge::asset::RewardDeliverStatus")]
pub(crate) enum RewardDeliverStatus {
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
#[pyclass(eq, eq_int, skip_from_py_object)]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longbridge::asset::CouponTargetType")]
pub(crate) enum CouponTargetType {
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
#[pyclass(eq, eq_int, skip_from_py_object)]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longbridge::asset::IpoCouponTargetType")]
pub(crate) enum IpoCouponTargetType {
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
#[pyclass(eq, eq_int, skip_from_py_object)]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longbridge::asset::CommissionUnlimitedType")]
pub(crate) enum CommissionUnlimitedType {
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
#[pyclass(eq, eq_int, skip_from_py_object)]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longbridge::asset::QuoteInvestorType")]
pub(crate) enum QuoteInvestorType {
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
#[pyclass(eq, eq_int, skip_from_py_object)]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longbridge::asset::CouponStatus")]
pub(crate) enum CouponStatus {
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
#[pyclass(eq, eq_int, from_py_object)]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longbridge::asset::DepositState")]
pub(crate) enum DepositState {
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
#[pyclass(eq, eq_int, skip_from_py_object)]
#[derive(Debug, PyEnum, Copy, Clone, Hash, Eq, PartialEq)]
#[py(remote = "longbridge::asset::WithdrawalState")]
pub(crate) enum WithdrawalState {
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
    /// The server's numeric code of `status`, also when it is a code
    /// this SDK does not know (`status` is then `Unknown`); none when the
    /// value is missing or is not a code
    #[py(opt)]
    status_raw: Option<i32>,
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
    /// Expiry date of the authorization (none when not set and for a
    /// long-term authorization: the server's 9999-12-31 "never expires"
    /// sentinel is none; see `long_term_valid`)
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
    /// this SDK does not know (`status` is then `Unknown`); none when the
    /// value is missing or is not a code
    #[py(opt)]
    status_raw: Option<i32>,
    /// Witness state
    witness_state: BankCardWitnessState,
    /// The server's numeric code of `witness_state`, also when it is a code
    /// this SDK does not know (`witness_state` is then `Unknown`); none when
    /// the value is missing or is not a code
    #[py(opt)]
    witness_state_raw: Option<i32>,
    /// Whether the card supports witnessing
    support_witness: bool,
    /// Binding state
    bind_state: BankCardBindState,
    /// The server's numeric code of `bind_state`, also when it is a code
    /// this SDK does not know (`bind_state` is then `Unknown`); none when the
    /// value is missing or is not a code
    #[py(opt)]
    bind_state_raw: Option<i32>,
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
    /// this SDK does not know (`status` is then `Unknown`); none when the
    /// value is missing or is not a code
    #[py(opt)]
    status_raw: Option<i32>,
    /// Failure reason (empty unless the application failed)
    message: String,
    /// Time the application was filed
    #[py(opt)]
    created_at: Option<PyOffsetDateTimeWrapper>,
    /// Time the application was last updated
    #[py(opt)]
    updated_at: Option<PyOffsetDateTimeWrapper>,
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
    /// State code of this stock in the transfer
    state_code: TransferStockState,
    /// The server's numeric code of `state_code`, also when it is a code
    /// this SDK does not know (`state_code` is then `Unknown`); none when the
    /// value is missing or is not a code
    #[py(opt)]
    state_code_raw: Option<i32>,
    /// Quantity held in street name, as returned by the server
    #[py(opt)]
    street_amount: Option<PyDecimal>,
    /// Quantity held by a nominee, as returned by the server
    #[py(opt)]
    nominee_amount: Option<PyDecimal>,
    /// Quantity held in the holder's own name, as returned by the server
    #[py(opt)]
    own_amount: Option<PyDecimal>,
    /// Number of share certificates, as returned by the server
    certificate_count: i64,
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
    /// this SDK does not know (`state_code` is then `Unknown`); none when the
    /// value is missing or is not a code
    #[py(opt)]
    state_code_raw: Option<i32>,
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
    /// this SDK does not know (`state_code` is then `Unknown`); none when the
    /// value is missing or is not a code
    #[py(opt)]
    state_code_raw: Option<i32>,
    /// App-side state: `noticed` means the user tapped "I have notified the
    /// broker" in the app (the user's own confirmation, not a Longbridge
    /// action); `default` otherwise
    app_state: String,
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

/// A reward record from the reward center (prize redemption secrets
/// `coupon_secret`, cash card codes `cash_card_code` and shipping contacts are
/// not modelled)
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::asset::RewardRecord")]
pub(crate) struct RewardRecord {
    /// Reward type: named after the first detail object the server sent
    /// (`cash`, `card`, `prize`, in that order), else the server's
    /// `reward_type` code
    reward_type: RewardType,
    /// The server's numeric code of `reward_type`, also when it is a code
    /// this SDK does not know (`reward_type` is then `Unknown`); none when the
    /// value is missing or is not a code
    #[py(opt)]
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
    /// this SDK does not know (`card_status` is then `Unknown`); none when the
    /// value is missing or is not a code
    #[py(opt)]
    card_status_raw: Option<i32>,
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
    /// The server's numeric code of `status`, also when it is a code
    /// this SDK does not know (`status` is then `Unknown`); none when the
    /// value is missing or is not a code
    #[py(opt)]
    status_raw: Option<i32>,
    /// Delivery status (issued gift rewards only)
    deliver_status: RewardDeliverStatus,
    /// The server's numeric code of `deliver_status` (e.g. 11 where
    /// `deliver_status` reads `AddressFilled`), also when it is a code this SDK
    /// does not know (`deliver_status` is then `Unknown`); none when the value
    /// is missing or is not a code
    #[py(opt)]
    deliver_status_raw: Option<i32>,
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

/// Commission terms of a commission coupon (`commission`)
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::asset::CouponCommission")]
pub(crate) struct CouponCommission {
    /// Markets the coupon applies to
    markets: Vec<String>,
    /// Rebate rate, as returned by the server
    rebate_rate: String,
    /// Instrument types the coupon applies to
    #[py(array)]
    bz_types: Vec<CouponTargetType>,
    /// The server's numeric codes of `bz_types`, including codes this
    /// SDK does not know (`Unknown` in `bz_types`); always the same
    /// length and order as `bz_types`: elements that are neither a code
    /// nor a known name are left out of both
    bz_types_raw: Vec<i32>,
    /// Remaining commission balance
    #[py(opt)]
    commission_balance: Option<PyDecimal>,
    /// Commission duration in seconds (the app reads 200 years or more as
    /// lifetime free commission)
    commission_duration: i64,
    /// Kind of commission coupon (the server's `unlimited` code)
    unlimited: CommissionUnlimitedType,
    /// The server's numeric code of `unlimited`, also when it is a code
    /// this SDK does not know (`unlimited` is then `Unknown`); none when the
    /// value is missing or is not a code
    #[py(opt)]
    unlimited_raw: Option<i32>,
    /// Commission expiry time (none when not set)
    #[py(opt)]
    commission_expired_time: Option<PyOffsetDateTimeWrapper>,
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
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::asset::CouponAiBenefit")]
pub(crate) struct CouponAiBenefit {
    /// Length of the benefit in days
    benefit_days: i32,
    /// Start of the benefit (none when not set)
    #[py(opt)]
    benefit_start: Option<PyOffsetDateTimeWrapper>,
    /// End of the benefit (none when not set)
    #[py(opt)]
    benefit_end: Option<PyOffsetDateTimeWrapper>,
    /// Benefit tag, as returned by the server
    benefit_tag: String,
}

/// Terms of a cash coupon (`cash`)
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::asset::CouponCash")]
pub(crate) struct CouponCash {
    /// Receive duration, as returned by the server (its unit and meaning are
    /// not confirmed)
    receive_duration: i64,
}

/// A store of a coffee coupon
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::asset::CouponCoffeeStore")]
pub(crate) struct CouponCoffeeStore {
    /// Store ID
    store_id: i64,
    /// Store name
    store_name: String,
}

/// Terms of a coffee coupon (`coffee_coupon`)
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::asset::CouponCoffee")]
pub(crate) struct CouponCoffee {
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
    store_id_list: Vec<i64>,
    /// Stores the coupon can be used at
    #[py(array)]
    store_list: Vec<CouponCoffeeStore>,
    /// Threshold, as returned by the server
    threshold: i64,
    /// Use limit (`total` or `period`), as returned by the server
    use_limit: String,
}

/// A counter (security) of a crypto coupon
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::asset::CouponCryptoCounter")]
pub(crate) struct CouponCryptoCounter {
    /// Counter ID
    counter_id: String,
}

/// Fallback of a crypto coupon
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::asset::CouponCryptoFallback")]
pub(crate) struct CouponCryptoFallback {
    /// Whether the fallback is enabled
    enabled: bool,
    /// Counter ID
    counter_id: String,
}

/// Terms of a crypto coupon (`crypto`)
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::asset::CouponCrypto")]
pub(crate) struct CouponCrypto {
    /// Fallback
    fallback: CouponCryptoFallback,
    /// Optional counters
    #[py(array)]
    optional_counters: Vec<CouponCryptoCounter>,
}

/// Terms of a finance interest coupon (`finance_interest`). The meaning of
/// these fields is not confirmed; they are returned as the server sends them.
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::asset::CouponFinanceInterest")]
pub(crate) struct CouponFinanceInterest {
    /// Discount mode, as returned by the server
    discount_mode: i32,
    /// Discount rate
    #[py(opt)]
    discount_rate: Option<PyDecimal>,
    /// Whether the coupon can be combined with other coupons
    superposition: bool,
}

/// Terms of a fund deduction coupon (`fund_deduct`)
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::asset::CouponFundDeduct")]
pub(crate) struct CouponFundDeduct {
    /// Maximum risk level, as returned by the server
    max_risk_level: i32,
    /// Minimum order amount, as returned by the server
    min_order_amount: String,
    /// Order types, as returned by the server
    order_types: Vec<String>,
    /// Underlying type, as returned by the server
    underlying_type: String,
    /// Underlying values, as returned by the server
    underlying_values: Vec<String>,
    /// Underlying names, as returned by the server
    underlying_names: Vec<String>,
}

/// Terms of a quote (market data) or fundamental-data coupon (`quote` /
/// `fundamental`)
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::asset::CouponQuote")]
pub(crate) struct CouponQuote {
    /// Current end time (none when not set)
    #[py(opt)]
    current_end_time: Option<PyOffsetDateTimeWrapper>,
    /// Whether the coupon is inapplicable, as returned by the server
    inapplicable: bool,
    /// Investor type the coupon applies to
    investor_type: QuoteInvestorType,
    /// The server's numeric code of `investor_type`, also when it is a code
    /// this SDK does not know (`investor_type` is then `Unknown`); none when
    /// the value is missing or is not a code
    #[py(opt)]
    investor_type_raw: Option<i32>,
    /// Whether the coupon is activated manually
    manual_activate: bool,
    /// Markets
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
    quote_tags: Vec<String>,
    /// Whether the coupon can be combined with other coupons
    superposition: bool,
}

/// Terms of an IPO coupon (`ipo`)
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::asset::CouponIpo")]
pub(crate) struct CouponIpo {
    /// Subscription ID
    subscription_id: String,
}

/// Terms of an IPO commission coupon (`ipo_commission`)
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::asset::CouponIpoCommission")]
pub(crate) struct CouponIpoCommission {
    /// IPO subscription types the coupon applies to
    #[py(array)]
    bz_types: Vec<IpoCouponTargetType>,
    /// The server's numeric codes of `bz_types`, including codes this
    /// SDK does not know (`Unknown` in `bz_types`); always the same
    /// length and order as `bz_types`: elements that are neither a code
    /// nor a known name are left out of both
    bz_types_raw: Vec<i32>,
    /// Counter ID
    counter_id: String,
    /// Counter name
    counter_name: String,
    /// Financing limit, as returned by the server
    financing_limit: String,
}

/// Terms of an IPO interest coupon (`ipo_interest`)
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::asset::CouponIpoInterest")]
pub(crate) struct CouponIpoInterest {
    /// Annual interest discount rate, as returned by the server (the app
    /// computes the daily discount as rate × financed amount / 365)
    discount_rate: String,
}

/// Terms of a platform fee deduction coupon (`platform_fee_deduct`)
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::asset::CouponPlatformFeeDeduct")]
pub(crate) struct CouponPlatformFeeDeduct {
    /// Instrument types the coupon applies to
    #[py(array)]
    bz_types: Vec<CouponTargetType>,
    /// The server's numeric codes of `bz_types`, including codes this
    /// SDK does not know (`Unknown` in `bz_types`); always the same
    /// length and order as `bz_types`: elements that are neither a code
    /// nor a known name are left out of both
    bz_types_raw: Vec<i32>,
    /// Counter ID
    counter_id: String,
    /// Counter name
    counter_name: String,
    /// Markets the coupon applies to
    markets: Vec<String>,
    /// Number of times used
    usage_count: i64,
    /// Number of uses left
    lave_usage_count: i64,
    /// Whether the coupon can be combined with other coupons
    superposition: bool,
}

/// An optional counter of a stock coupon
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::asset::CouponStockCounter")]
pub(crate) struct CouponStockCounter {
    /// Counter ID
    counter_id: String,
    /// Shares, as returned by the server
    shares: String,
}

/// Fallback of a stock coupon
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::asset::CouponStockFallback")]
pub(crate) struct CouponStockFallback {
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
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::asset::CouponStock")]
pub(crate) struct CouponStock {
    /// Counter ID
    counter_id: String,
    /// Counter name
    counter_name: String,
    /// Amount
    #[py(opt)]
    amount: Option<PyDecimal>,
    /// Redemption type (`by_card_value` or `shares_number`), as returned by the
    /// server
    redemption_type: String,
    /// Delivery way (`card_exchange` or `account_transfer`), as returned by the
    /// server
    send_way: String,
    /// Optional counters
    #[py(array)]
    optional_counters: Vec<CouponStockCounter>,
    /// Fallback
    fallback: CouponStockFallback,
}

/// Terms of a stock fragment coupon (`stock_fragment`)
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::asset::CouponStockFragment")]
pub(crate) struct CouponStockFragment {
    /// Counter ID
    counter_id: String,
    /// Counter name
    counter_name: String,
    /// Market
    market: String,
    /// Markets
    markets: Vec<String>,
    /// Instrument type the coupon applies to
    underlying_type: CouponTargetType,
    /// The server's numeric code of `underlying_type`, also when it is a code
    /// this SDK does not know (`underlying_type` is then `Unknown`); none when
    /// the value is missing or is not a code
    #[py(opt)]
    underlying_type_raw: Option<i32>,
    /// Amounts, as returned by the server (a `dict` of strings)
    amounts: std::collections::BTreeMap<String, String>,
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
    /// The server's numeric code of `card_status`, also when it is a code
    /// this SDK does not know (`card_status` is then `Unknown`); none when the
    /// value is missing or is not a code
    #[py(opt)]
    card_status_raw: Option<i32>,
    /// Start of validity
    #[py(opt)]
    start_time: Option<PyOffsetDateTimeWrapper>,
    /// End of validity (none when not set or when the coupon never expires;
    /// see `no_expiry`)
    #[py(opt)]
    end_time: Option<PyOffsetDateTimeWrapper>,
    /// Whether the coupon never expires (the server's end time is in or after
    /// 2100)
    no_expiry: bool,
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
    /// Trading sessions the coupon applies to, as returned by the server
    trade_sessions: Vec<String>,
    /// Subscription ID, as returned by the server
    subscription_id: String,
    /// Terms of an AI benefit coupon (none for other coupon types)
    #[py(opt)]
    ai_benefit: Option<CouponAiBenefit>,
    /// Terms of a cash coupon (none for other coupon types)
    #[py(opt)]
    cash: Option<CouponCash>,
    /// Terms of a coffee coupon (none for other coupon types)
    #[py(opt)]
    coffee_coupon: Option<CouponCoffee>,
    /// Terms of a commission coupon (none for other coupon types)
    #[py(opt)]
    commission: Option<CouponCommission>,
    /// Terms of a crypto coupon (none for other coupon types)
    #[py(opt)]
    crypto: Option<CouponCrypto>,
    /// Terms of a finance interest coupon (none for other coupon types)
    #[py(opt)]
    finance_interest: Option<CouponFinanceInterest>,
    /// Terms of a fund deduction coupon (none for other coupon types)
    #[py(opt)]
    fund_deduct: Option<CouponFundDeduct>,
    /// Terms of a fundamental-data coupon (none for other coupon types)
    #[py(opt)]
    fundamental: Option<CouponQuote>,
    /// Terms of an IPO coupon (none for other coupon types)
    #[py(opt)]
    ipo: Option<CouponIpo>,
    /// Terms of an IPO commission coupon (none for other coupon types)
    #[py(opt)]
    ipo_commission: Option<CouponIpoCommission>,
    /// Terms of an IPO interest coupon (none for other coupon types)
    #[py(opt)]
    ipo_interest: Option<CouponIpoInterest>,
    /// Terms of a platform fee deduction coupon (none for other coupon types)
    #[py(opt)]
    platform_fee_deduct: Option<CouponPlatformFeeDeduct>,
    /// Terms of a quote (market data) coupon (none for other coupon types)
    #[py(opt)]
    quote: Option<CouponQuote>,
    /// Terms of a stock coupon (none for other coupon types)
    #[py(opt)]
    stock: Option<CouponStock>,
    /// Terms of a stock fragment coupon (none for other coupon types)
    #[py(opt)]
    stock_fragment: Option<CouponStockFragment>,
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
    /// The server's numeric code of `state_code`, also when it is a code
    /// this SDK does not know (`state_code` is then `Unknown`); none when the
    /// value is missing or is not a code
    #[py(opt)]
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
    /// State code
    state_code: WithdrawalState,
    /// The server's numeric code of `state_code`, also when it is a code
    /// this SDK does not know (`state_code` is then `Unknown`); none when the
    /// value is missing or is not a code
    #[py(opt)]
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
/// than the record count, so it is not modelled; see `has_more`.
#[pyclass(skip_from_py_object)]
#[derive(Debug, PyObject, Clone)]
#[py(remote = "longbridge::asset::DepositsResponse")]
pub(crate) struct DepositsResponse {
    /// Deposit records, newest first
    #[py(array)]
    items: Vec<Deposit>,
    /// A full page came back (as many records as the page size sent, at
    /// most 20), so the next page may hold more
    has_more: bool,
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
    /// A full page came back (as many records as the page size sent), so the
    /// next page may hold more
    has_more: bool,
}

#[pymethods]
impl BankCardStatus {
    /// The server code of this value (`-1` for `Unknown`, which is not a
    /// server code); the Python enum value itself is not the server code
    #[getter]
    fn code(&self) -> i32 {
        longbridge::asset::BankCardStatus::from(*self).into()
    }
}

#[pymethods]
impl BankCardWitnessState {
    /// The server code of this value (`-1` for `Unknown`, which is not a
    /// server code); the Python enum value itself is not the server code
    #[getter]
    fn code(&self) -> i32 {
        longbridge::asset::BankCardWitnessState::from(*self).into()
    }
}

#[pymethods]
impl BankCardBindState {
    /// The server code of this value (`-1` for `Unknown`, which is not a
    /// server code); the Python enum value itself is not the server code
    #[getter]
    fn code(&self) -> i32 {
        longbridge::asset::BankCardBindState::from(*self).into()
    }
}

#[pymethods]
impl AuthorizationStatus {
    /// The server code of this value (`-1` for `Unknown`, which is not a
    /// server code); the Python enum value itself is not the server code
    #[getter]
    fn code(&self) -> i32 {
        longbridge::asset::AuthorizationStatus::from(*self).into()
    }
}

#[pymethods]
impl StockTransferInState {
    /// The server code of this value (`-1` for `Unknown`, which is not a
    /// server code); the Python enum value itself is not the server code
    #[getter]
    fn code(&self) -> i32 {
        longbridge::asset::StockTransferInState::from(*self).into()
    }
}

#[pymethods]
impl StockTransferOutState {
    /// The server code of this value (`-1` for `Unknown`, which is not a
    /// server code); the Python enum value itself is not the server code
    #[getter]
    fn code(&self) -> i32 {
        longbridge::asset::StockTransferOutState::from(*self).into()
    }
}

#[pymethods]
impl TransferStockState {
    /// The server code of this value (`-1` for `Unknown`, which is not a
    /// server code); the Python enum value itself is not the server code
    #[getter]
    fn code(&self) -> i32 {
        longbridge::asset::TransferStockState::from(*self).into()
    }
}

#[pymethods]
impl RewardType {
    /// The server code of this value (`-1` for `Unknown`, which is not a
    /// server code); the Python enum value itself is not the server code
    #[getter]
    fn code(&self) -> i32 {
        longbridge::asset::RewardType::from(*self).into()
    }
}

#[pymethods]
impl RewardStatus {
    /// The server code of this value (`-1` for `Unknown`, which is not a
    /// server code); the Python enum value itself is not the server code
    #[getter]
    fn code(&self) -> i32 {
        longbridge::asset::RewardStatus::from(*self).into()
    }
}

#[pymethods]
impl RewardDeliverStatus {
    /// The server code of this value (`-1` for `Unknown`, which is not a
    /// server code); the Python enum value itself is not the server code
    #[getter]
    fn code(&self) -> i32 {
        longbridge::asset::RewardDeliverStatus::from(*self).into()
    }
}

#[pymethods]
impl CouponStatus {
    /// The server code of this value (`-1` for `Unknown`, which is not a
    /// server code); the Python enum value itself is not the server code
    #[getter]
    fn code(&self) -> i32 {
        longbridge::asset::CouponStatus::from(*self).into()
    }
}

#[pymethods]
impl CouponTargetType {
    /// The server code of this value (`-1` for `Unknown`, which is not a
    /// server code); the Python enum value itself is not the server code
    #[getter]
    fn code(&self) -> i32 {
        longbridge::asset::CouponTargetType::from(*self).into()
    }
}

#[pymethods]
impl IpoCouponTargetType {
    /// The server code of this value (`-1` for `Unknown`, which is not a
    /// server code); the Python enum value itself is not the server code
    #[getter]
    fn code(&self) -> i32 {
        longbridge::asset::IpoCouponTargetType::from(*self).into()
    }
}

#[pymethods]
impl CommissionUnlimitedType {
    /// The server code of this value (`-1` for `Unknown`, which is not a
    /// server code); the Python enum value itself is not the server code
    #[getter]
    fn code(&self) -> i32 {
        longbridge::asset::CommissionUnlimitedType::from(*self).into()
    }
}

#[pymethods]
impl QuoteInvestorType {
    /// The server code of this value (`-1` for `Unknown`, which is not a
    /// server code); the Python enum value itself is not the server code
    #[getter]
    fn code(&self) -> i32 {
        longbridge::asset::QuoteInvestorType::from(*self).into()
    }
}

#[pymethods]
impl DepositState {
    /// The server code of this value (`-1` for `Unknown`, which is not a
    /// server code); the Python enum value itself is not the server code
    #[getter]
    fn code(&self) -> i32 {
        longbridge::asset::DepositState::from(*self).into()
    }
}

#[pymethods]
impl WithdrawalState {
    /// The server code of this value (`-1` for `Unknown`, which is not a
    /// server code); the Python enum value itself is not the server code
    #[getter]
    fn code(&self) -> i32 {
        longbridge::asset::WithdrawalState::from(*self).into()
    }
}
