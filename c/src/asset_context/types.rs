use std::os::raw::c_char;

use longbridge::asset::{
    AuthorizationStatus, BankCard, BankCardAuthorization, BankCardBindState, BankCardStatus,
    BankCardWitnessState, Coupon, CouponCommission, CouponOrder, CouponStatus, CouponsResponse,
    Deposit, DepositState, EddaApplication, RewardDeliverStatus, RewardRecord,
    RewardRecordsResponse, RewardStatus, RewardType, StatementItem, StockTransferIn,
    StockTransferInResponse, StockTransferInState, StockTransferOut, StockTransferOutState,
    TransferStock, TransferStockState, Withdrawal, WithdrawalState, WithdrawalsResponse,
};
use time::OffsetDateTime;

use crate::{
    asset_context::enum_types::{
        CAuthorizationStatus, CBankCardBindState, CBankCardStatus, CBankCardWitnessState,
        CCouponStatus, CDepositState, CRewardDeliverStatus, CRewardStatus, CRewardType,
        CStockTransferInState, CStockTransferOutState, CTransferStockState, CWithdrawalState,
    },
    types::{CDecimal, CString, CVec, ToFFI},
};

/// Statement item
#[repr(C)]
pub struct CStatementItem {
    /// Statement date (integer, e.g. 20250301)
    pub dt: i32,
    /// File key
    pub file_key: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CStatementItemOwned {
    dt: i32,
    file_key: CString,
}

impl From<StatementItem> for CStatementItemOwned {
    fn from(item: StatementItem) -> Self {
        Self {
            dt: item.dt,
            file_key: item.file_key.into(),
        }
    }
}

impl ToFFI for CStatementItemOwned {
    type FFIType = CStatementItem;
    fn to_ffi_type(&self) -> CStatementItem {
        CStatementItem {
            dt: self.dt,
            file_key: self.file_key.to_ffi_type(),
        }
    }
}

/// An authorization attached to a bank card (eDDA, bank securities account,
/// FPS, ...)
#[repr(C)]
pub struct CBankCardAuthorization {
    /// Authorization flow: `hk_edda` / `hk_sdda` (eDDA), `hk_bank_sec`,
    /// `hk_fps`, ...
    pub flow_id: *const c_char,
    /// Display name of the authorization
    pub name: *const c_char,
    /// Authorized currency
    pub currency: *const c_char,
    /// Authorization status
    pub status: CAuthorizationStatus,
    /// Failure reason (empty unless the authorization failed)
    pub message: *const c_char,
    /// Whether this authorization is supported for the card
    pub is_support: bool,
    /// Maximum amount per debit (null when not set)
    pub max_amount: *const CDecimal,
    /// Whether the per-debit amount is unlimited
    pub max_amount_unlimited: bool,
    /// Maximum number of debits per day (null when not set)
    pub daily_max_count: *const i64,
    /// Whether the daily debit count is unlimited
    pub daily_max_count_unlimited: bool,
    /// Expiry date of the authorization as a unix timestamp (null when not set
    /// and for a long-term authorization: the server's 9999-12-31 "never
    /// expires" sentinel is null; see `long_term_valid`)
    pub expiry_date: *const i64,
    /// Whether the authorization never expires
    pub long_term_valid: bool,
    /// Whether submitted documents are under review
    pub doc_reviewing: bool,
    /// Whether the "no prior deposit" check is under review
    pub no_prior_deposit_reviewing: bool,
}

#[derive(Debug)]
pub(crate) struct CBankCardAuthorizationOwned {
    flow_id: CString,
    name: CString,
    currency: CString,
    status: AuthorizationStatus,
    message: CString,
    is_support: bool,
    max_amount: Option<CDecimal>,
    max_amount_unlimited: bool,
    daily_max_count: Option<i64>,
    daily_max_count_unlimited: bool,
    expiry_date: Option<i64>,
    long_term_valid: bool,
    doc_reviewing: bool,
    no_prior_deposit_reviewing: bool,
}

impl From<BankCardAuthorization> for CBankCardAuthorizationOwned {
    fn from(v: BankCardAuthorization) -> Self {
        Self {
            flow_id: v.flow_id.into(),
            name: v.name.into(),
            currency: v.currency.into(),
            status: v.status,
            message: v.message.into(),
            is_support: v.is_support,
            max_amount: v.max_amount.map(Into::into),
            max_amount_unlimited: v.max_amount_unlimited,
            daily_max_count: v.daily_max_count,
            daily_max_count_unlimited: v.daily_max_count_unlimited,
            expiry_date: v.expiry_date.map(OffsetDateTime::unix_timestamp),
            long_term_valid: v.long_term_valid,
            doc_reviewing: v.doc_reviewing,
            no_prior_deposit_reviewing: v.no_prior_deposit_reviewing,
        }
    }
}

impl ToFFI for CBankCardAuthorizationOwned {
    type FFIType = CBankCardAuthorization;

    fn to_ffi_type(&self) -> Self::FFIType {
        CBankCardAuthorization {
            flow_id: self.flow_id.to_ffi_type(),
            name: self.name.to_ffi_type(),
            currency: self.currency.to_ffi_type(),
            status: self.status.into(),
            message: self.message.to_ffi_type(),
            is_support: self.is_support,
            max_amount: self
                .max_amount
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            max_amount_unlimited: self.max_amount_unlimited,
            daily_max_count: self
                .daily_max_count
                .as_ref()
                .map(|v| v as *const i64)
                .unwrap_or(std::ptr::null()),
            daily_max_count_unlimited: self.daily_max_count_unlimited,
            expiry_date: self
                .expiry_date
                .as_ref()
                .map(|v| v as *const i64)
                .unwrap_or(std::ptr::null()),
            long_term_valid: self.long_term_valid,
            doc_reviewing: self.doc_reviewing,
            no_prior_deposit_reviewing: self.no_prior_deposit_reviewing,
        }
    }
}

/// A linked bank card (holder name, address and ID documents are not modelled)
#[repr(C)]
pub struct CBankCard {
    /// Bank card ID (pass to edda_applications)
    pub id: *const c_char,
    /// Bank ID
    pub bank_id: *const c_char,
    /// Bank name
    pub bank: *const c_char,
    /// Bank name in English
    pub bank_en: *const c_char,
    /// Bank code
    pub bank_code: *const c_char,
    /// Bank account number, as returned by the server (not masked)
    pub account: *const c_char,
    /// Account type
    pub account_type: *const c_char,
    /// Country or region of the bank (e.g. `Hong Kong`; not a currency)
    pub country: *const c_char,
    /// Region code
    pub region: *const c_char,
    /// Region name
    pub region_name: *const c_char,
    /// SWIFT code
    pub swift_code: *const c_char,
    /// Remark
    pub remark: *const c_char,
    /// Verification status
    pub status: CBankCardStatus,
    /// Witness state
    pub witness_state: CBankCardWitnessState,
    /// Whether the card supports witnessing
    pub support_witness: bool,
    /// Binding state
    pub bind_state: CBankCardBindState,
    /// Binding hint text
    pub bind_tip: *const c_char,
    /// Withdrawal fee text, as returned by the server
    pub withdraw_fee: *const c_char,
    /// Time the card was added as a unix timestamp (null when not set)
    pub created_at: *const i64,
    /// Authorizations attached to the card
    pub auth_list: *const CBankCardAuthorization,
    /// Number of `auth_list`
    pub num_auth_list: usize,
}

#[derive(Debug)]
pub(crate) struct CBankCardOwned {
    id: CString,
    bank_id: CString,
    bank: CString,
    bank_en: CString,
    bank_code: CString,
    account: CString,
    account_type: CString,
    country: CString,
    region: CString,
    region_name: CString,
    swift_code: CString,
    remark: CString,
    status: BankCardStatus,
    witness_state: BankCardWitnessState,
    support_witness: bool,
    bind_state: BankCardBindState,
    bind_tip: CString,
    withdraw_fee: CString,
    created_at: Option<i64>,
    auth_list: CVec<CBankCardAuthorizationOwned>,
}

impl From<BankCard> for CBankCardOwned {
    fn from(v: BankCard) -> Self {
        Self {
            id: v.id.into(),
            bank_id: v.bank_id.into(),
            bank: v.bank.into(),
            bank_en: v.bank_en.into(),
            bank_code: v.bank_code.into(),
            account: v.account.into(),
            account_type: v.account_type.into(),
            country: v.country.into(),
            region: v.region.into(),
            region_name: v.region_name.into(),
            swift_code: v.swift_code.into(),
            remark: v.remark.into(),
            status: v.status,
            witness_state: v.witness_state,
            support_witness: v.support_witness,
            bind_state: v.bind_state,
            bind_tip: v.bind_tip.into(),
            withdraw_fee: v.withdraw_fee.into(),
            created_at: v.created_at.map(OffsetDateTime::unix_timestamp),
            auth_list: v.auth_list.into(),
        }
    }
}

impl ToFFI for CBankCardOwned {
    type FFIType = CBankCard;

    fn to_ffi_type(&self) -> Self::FFIType {
        CBankCard {
            id: self.id.to_ffi_type(),
            bank_id: self.bank_id.to_ffi_type(),
            bank: self.bank.to_ffi_type(),
            bank_en: self.bank_en.to_ffi_type(),
            bank_code: self.bank_code.to_ffi_type(),
            account: self.account.to_ffi_type(),
            account_type: self.account_type.to_ffi_type(),
            country: self.country.to_ffi_type(),
            region: self.region.to_ffi_type(),
            region_name: self.region_name.to_ffi_type(),
            swift_code: self.swift_code.to_ffi_type(),
            remark: self.remark.to_ffi_type(),
            status: self.status.into(),
            witness_state: self.witness_state.into(),
            support_witness: self.support_witness,
            bind_state: self.bind_state.into(),
            bind_tip: self.bind_tip.to_ffi_type(),
            withdraw_fee: self.withdraw_fee.to_ffi_type(),
            created_at: self
                .created_at
                .as_ref()
                .map(|v| v as *const i64)
                .unwrap_or(std::ptr::null()),
            auth_list: self.auth_list.to_ffi_type(),
            num_auth_list: self.auth_list.len(),
        }
    }
}

/// An eDDA authorization application of a bank card
#[repr(C)]
pub struct CEddaApplication {
    /// Application ID
    pub id: *const c_char,
    /// Application status
    pub status: CAuthorizationStatus,
    /// Failure reason (empty unless the application failed)
    pub message: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CEddaApplicationOwned {
    id: CString,
    status: AuthorizationStatus,
    message: CString,
}

impl From<EddaApplication> for CEddaApplicationOwned {
    fn from(v: EddaApplication) -> Self {
        Self {
            id: v.id.into(),
            status: v.status,
            message: v.message.into(),
        }
    }
}

impl ToFFI for CEddaApplicationOwned {
    type FFIType = CEddaApplication;

    fn to_ffi_type(&self) -> Self::FFIType {
        CEddaApplication {
            id: self.id.to_ffi_type(),
            status: self.status.into(),
            message: self.message.to_ffi_type(),
        }
    }
}

/// One stock in a stock transfer application
#[repr(C)]
pub struct CTransferStock {
    /// Stock code
    pub code: *const c_char,
    /// Stock name
    pub name: *const c_char,
    /// Stock name in English
    pub name_en: *const c_char,
    /// Market
    pub market: *const c_char,
    /// Counter ID
    pub counter_id: *const c_char,
    /// Quantity (null when not set)
    pub amount: *const CDecimal,
    /// Currency
    pub currency: *const c_char,
    /// Cost price (null when not set)
    pub cost: *const CDecimal,
    /// State code of this stock in the transfer
    pub state_code: CTransferStockState,
    /// Position side
    pub position_side: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CTransferStockOwned {
    code: CString,
    name: CString,
    name_en: CString,
    market: CString,
    counter_id: CString,
    amount: Option<CDecimal>,
    currency: CString,
    cost: Option<CDecimal>,
    state_code: TransferStockState,
    position_side: CString,
}

impl From<TransferStock> for CTransferStockOwned {
    fn from(v: TransferStock) -> Self {
        Self {
            code: v.code.into(),
            name: v.name.into(),
            name_en: v.name_en.into(),
            market: v.market.into(),
            counter_id: v.counter_id.into(),
            amount: v.amount.map(Into::into),
            currency: v.currency.into(),
            cost: v.cost.map(Into::into),
            state_code: v.state_code,
            position_side: v.position_side.into(),
        }
    }
}

impl ToFFI for CTransferStockOwned {
    type FFIType = CTransferStock;

    fn to_ffi_type(&self) -> Self::FFIType {
        CTransferStock {
            code: self.code.to_ffi_type(),
            name: self.name.to_ffi_type(),
            name_en: self.name_en.to_ffi_type(),
            market: self.market.to_ffi_type(),
            counter_id: self.counter_id.to_ffi_type(),
            amount: self
                .amount
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            currency: self.currency.to_ffi_type(),
            cost: self
                .cost
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            state_code: self.state_code.into(),
            position_side: self.position_side.to_ffi_type(),
        }
    }
}

/// A stock transfer-in application (another broker -> this account)
#[repr(C)]
pub struct CStockTransferIn {
    /// Record ID
    pub id: *const c_char,
    /// Order ID
    pub order_id: *const c_char,
    /// Market
    pub market: *const c_char,
    /// Name of the other broker
    pub broker: *const c_char,
    /// ID of the other broker
    pub broker_id: *const c_char,
    /// English name of the other broker
    pub broker_en: *const c_char,
    /// Broker code of the other broker
    pub broker_code: *const c_char,
    /// Account number at the other broker, as returned by the server (not
    /// masked)
    pub account: *const c_char,
    /// State text, as returned by the server
    pub state: *const c_char,
    /// State code
    pub state_code: CStockTransferInState,
    /// App-side state: `noticed` means the user tapped "I have notified the
    /// broker" in the app; `default` otherwise
    pub app_state: *const c_char,
    /// Whether the application can still be cancelled or changed
    pub can_modify: bool,
    /// Created time as a unix timestamp (null when not set)
    pub created_at: *const i64,
    /// Updated time as a unix timestamp (null when not set)
    pub updated_at: *const i64,
    /// Processing start time as a unix timestamp (null when not set)
    pub process_at: *const i64,
    /// Execution time as a unix timestamp (null when not executed)
    pub exec_at: *const i64,
    /// Remark
    pub remark: *const c_char,
    /// Reason (e.g. for a failure)
    pub reason: *const c_char,
    /// Stocks in the application
    pub stocks: *const CTransferStock,
    /// Number of `stocks`
    pub num_stocks: usize,
}

#[derive(Debug)]
pub(crate) struct CStockTransferInOwned {
    id: CString,
    order_id: CString,
    market: CString,
    broker: CString,
    broker_id: CString,
    broker_en: CString,
    broker_code: CString,
    account: CString,
    state: CString,
    state_code: StockTransferInState,
    app_state: CString,
    can_modify: bool,
    created_at: Option<i64>,
    updated_at: Option<i64>,
    process_at: Option<i64>,
    exec_at: Option<i64>,
    remark: CString,
    reason: CString,
    stocks: CVec<CTransferStockOwned>,
}

impl From<StockTransferIn> for CStockTransferInOwned {
    fn from(v: StockTransferIn) -> Self {
        Self {
            id: v.id.into(),
            order_id: v.order_id.into(),
            market: v.market.into(),
            broker: v.broker.into(),
            broker_id: v.broker_id.into(),
            broker_en: v.broker_en.into(),
            broker_code: v.broker_code.into(),
            account: v.account.into(),
            state: v.state.into(),
            state_code: v.state_code,
            app_state: v.app_state.into(),
            can_modify: v.can_modify,
            created_at: v.created_at.map(OffsetDateTime::unix_timestamp),
            updated_at: v.updated_at.map(OffsetDateTime::unix_timestamp),
            process_at: v.process_at.map(OffsetDateTime::unix_timestamp),
            exec_at: v.exec_at.map(OffsetDateTime::unix_timestamp),
            remark: v.remark.into(),
            reason: v.reason.into(),
            stocks: v.stocks.into(),
        }
    }
}

impl ToFFI for CStockTransferInOwned {
    type FFIType = CStockTransferIn;

    fn to_ffi_type(&self) -> Self::FFIType {
        CStockTransferIn {
            id: self.id.to_ffi_type(),
            order_id: self.order_id.to_ffi_type(),
            market: self.market.to_ffi_type(),
            broker: self.broker.to_ffi_type(),
            broker_id: self.broker_id.to_ffi_type(),
            broker_en: self.broker_en.to_ffi_type(),
            broker_code: self.broker_code.to_ffi_type(),
            account: self.account.to_ffi_type(),
            state: self.state.to_ffi_type(),
            state_code: self.state_code.into(),
            app_state: self.app_state.to_ffi_type(),
            can_modify: self.can_modify,
            created_at: self
                .created_at
                .as_ref()
                .map(|v| v as *const i64)
                .unwrap_or(std::ptr::null()),
            updated_at: self
                .updated_at
                .as_ref()
                .map(|v| v as *const i64)
                .unwrap_or(std::ptr::null()),
            process_at: self
                .process_at
                .as_ref()
                .map(|v| v as *const i64)
                .unwrap_or(std::ptr::null()),
            exec_at: self
                .exec_at
                .as_ref()
                .map(|v| v as *const i64)
                .unwrap_or(std::ptr::null()),
            remark: self.remark.to_ffi_type(),
            reason: self.reason.to_ffi_type(),
            stocks: self.stocks.to_ffi_type(),
            num_stocks: self.stocks.len(),
        }
    }
}

/// Response for the stock transfer-in list
#[repr(C)]
pub struct CStockTransferInResponse {
    /// Records
    pub records: *const CStockTransferIn,
    /// Number of `records`
    pub num_records: usize,
    /// Total number of records
    pub total: i64,
}

#[derive(Debug)]
pub(crate) struct CStockTransferInResponseOwned {
    records: CVec<CStockTransferInOwned>,
    total: i64,
}

impl From<StockTransferInResponse> for CStockTransferInResponseOwned {
    fn from(v: StockTransferInResponse) -> Self {
        Self {
            records: v.records.into(),
            total: v.total,
        }
    }
}

impl ToFFI for CStockTransferInResponseOwned {
    type FFIType = CStockTransferInResponse;

    fn to_ffi_type(&self) -> Self::FFIType {
        CStockTransferInResponse {
            records: self.records.to_ffi_type(),
            num_records: self.records.len(),
            total: self.total,
        }
    }
}

/// A stock transfer-out application (this account -> another broker)
#[repr(C)]
pub struct CStockTransferOut {
    /// Record ID
    pub id: *const c_char,
    /// Order ID
    pub order_id: *const c_char,
    /// Market
    pub market: *const c_char,
    /// Name of the other broker
    pub broker: *const c_char,
    /// ID of the other broker
    pub broker_id: *const c_char,
    /// English name of the other broker
    pub broker_en: *const c_char,
    /// Broker code of the other broker
    pub broker_code: *const c_char,
    /// Account number at the other broker, as returned by the server (not
    /// masked)
    pub account: *const c_char,
    /// State text, as returned by the server
    pub state: *const c_char,
    /// State code (only `Submitted` applications can be cancelled)
    pub state_code: CStockTransferOutState,
    /// App-side state: `noticed` means the user tapped "I have notified the
    /// broker" in the app (the user's own confirmation, not a Longbridge
    /// action); `default` otherwise
    pub app_state: *const c_char,
    /// Created time as a unix timestamp (null when not set)
    pub created_at: *const i64,
    /// Updated time as a unix timestamp (null when not set)
    pub updated_at: *const i64,
    /// Processing start time as a unix timestamp (null when not set)
    pub process_at: *const i64,
    /// Execution time as a unix timestamp (null when not executed)
    pub exec_at: *const i64,
    /// Time the other broker agreed as a unix timestamp (null when not set)
    pub agree_at: *const i64,
    /// Time the application was refused as a unix timestamp (null when not set)
    pub refuse_at: *const i64,
    /// Time the application was cancelled as a unix timestamp (null when not
    /// set)
    pub cancel_at: *const i64,
    /// Remark
    pub remark: *const c_char,
    /// Reason (e.g. for a rejection)
    pub reason: *const c_char,
    /// Stocks in the application
    pub stocks: *const CTransferStock,
    /// Number of `stocks`
    pub num_stocks: usize,
}

#[derive(Debug)]
pub(crate) struct CStockTransferOutOwned {
    id: CString,
    order_id: CString,
    market: CString,
    broker: CString,
    broker_id: CString,
    broker_en: CString,
    broker_code: CString,
    account: CString,
    state: CString,
    state_code: StockTransferOutState,
    app_state: CString,
    created_at: Option<i64>,
    updated_at: Option<i64>,
    process_at: Option<i64>,
    exec_at: Option<i64>,
    agree_at: Option<i64>,
    refuse_at: Option<i64>,
    cancel_at: Option<i64>,
    remark: CString,
    reason: CString,
    stocks: CVec<CTransferStockOwned>,
}

impl From<StockTransferOut> for CStockTransferOutOwned {
    fn from(v: StockTransferOut) -> Self {
        Self {
            id: v.id.into(),
            order_id: v.order_id.into(),
            market: v.market.into(),
            broker: v.broker.into(),
            broker_id: v.broker_id.into(),
            broker_en: v.broker_en.into(),
            broker_code: v.broker_code.into(),
            account: v.account.into(),
            state: v.state.into(),
            state_code: v.state_code,
            app_state: v.app_state.into(),
            created_at: v.created_at.map(OffsetDateTime::unix_timestamp),
            updated_at: v.updated_at.map(OffsetDateTime::unix_timestamp),
            process_at: v.process_at.map(OffsetDateTime::unix_timestamp),
            exec_at: v.exec_at.map(OffsetDateTime::unix_timestamp),
            agree_at: v.agree_at.map(OffsetDateTime::unix_timestamp),
            refuse_at: v.refuse_at.map(OffsetDateTime::unix_timestamp),
            cancel_at: v.cancel_at.map(OffsetDateTime::unix_timestamp),
            remark: v.remark.into(),
            reason: v.reason.into(),
            stocks: v.stocks.into(),
        }
    }
}

impl ToFFI for CStockTransferOutOwned {
    type FFIType = CStockTransferOut;

    fn to_ffi_type(&self) -> Self::FFIType {
        CStockTransferOut {
            id: self.id.to_ffi_type(),
            order_id: self.order_id.to_ffi_type(),
            market: self.market.to_ffi_type(),
            broker: self.broker.to_ffi_type(),
            broker_id: self.broker_id.to_ffi_type(),
            broker_en: self.broker_en.to_ffi_type(),
            broker_code: self.broker_code.to_ffi_type(),
            account: self.account.to_ffi_type(),
            state: self.state.to_ffi_type(),
            state_code: self.state_code.into(),
            app_state: self.app_state.to_ffi_type(),
            created_at: self
                .created_at
                .as_ref()
                .map(|v| v as *const i64)
                .unwrap_or(std::ptr::null()),
            updated_at: self
                .updated_at
                .as_ref()
                .map(|v| v as *const i64)
                .unwrap_or(std::ptr::null()),
            process_at: self
                .process_at
                .as_ref()
                .map(|v| v as *const i64)
                .unwrap_or(std::ptr::null()),
            exec_at: self
                .exec_at
                .as_ref()
                .map(|v| v as *const i64)
                .unwrap_or(std::ptr::null()),
            agree_at: self
                .agree_at
                .as_ref()
                .map(|v| v as *const i64)
                .unwrap_or(std::ptr::null()),
            refuse_at: self
                .refuse_at
                .as_ref()
                .map(|v| v as *const i64)
                .unwrap_or(std::ptr::null()),
            cancel_at: self
                .cancel_at
                .as_ref()
                .map(|v| v as *const i64)
                .unwrap_or(std::ptr::null()),
            remark: self.remark.to_ffi_type(),
            reason: self.reason.to_ffi_type(),
            stocks: self.stocks.to_ffi_type(),
            num_stocks: self.stocks.len(),
        }
    }
}

/// A reward record from the reward center (prize secrets and shipping contacts
/// are not modelled)
#[repr(C)]
pub struct CRewardRecord {
    /// Reward type
    pub reward_type: CRewardType,
    /// Record ID
    pub record_id: *const c_char,
    /// Title of the activity that granted the reward
    pub activity_title: *const c_char,
    /// Reward title
    pub title: *const c_char,
    /// Reward description
    pub description: *const c_char,
    /// Coupon type (coupon rewards), e.g. `commission`
    pub card_type: *const c_char,
    /// Coupon status code (coupon rewards), as returned by the server
    pub card_status: CCouponStatus,
    /// Prize type code (gift rewards; `0` = physical goods), null when not set
    pub prize_type: *const i32,
    /// Amount (null when not set)
    pub amount: *const CDecimal,
    /// Currency
    pub currency: *const c_char,
    /// Reward status
    pub status: CRewardStatus,
    /// Delivery status (issued gift rewards only)
    pub deliver_status: CRewardDeliverStatus,
    /// Exit status code, as returned by the server
    pub exit_status: i32,
    /// Time the reward was sent as a unix timestamp (null when not set)
    pub send_time: *const i64,
    /// Start time as a unix timestamp (null when not set)
    pub start_time: *const i64,
    /// End time as a unix timestamp (null when not set)
    pub end_time: *const i64,
    /// Effective time as a unix timestamp (null when not set)
    pub effective_time: *const i64,
    /// Deadline to claim the reward as a unix timestamp (null when not set)
    pub receive_deadline: *const i64,
}

#[derive(Debug)]
pub(crate) struct CRewardRecordOwned {
    reward_type: RewardType,
    record_id: CString,
    activity_title: CString,
    title: CString,
    description: CString,
    card_type: CString,
    card_status: CouponStatus,
    prize_type: Option<i32>,
    amount: Option<CDecimal>,
    currency: CString,
    status: RewardStatus,
    deliver_status: RewardDeliverStatus,
    exit_status: i32,
    send_time: Option<i64>,
    start_time: Option<i64>,
    end_time: Option<i64>,
    effective_time: Option<i64>,
    receive_deadline: Option<i64>,
}

impl From<RewardRecord> for CRewardRecordOwned {
    fn from(v: RewardRecord) -> Self {
        Self {
            reward_type: v.reward_type,
            record_id: v.record_id.into(),
            activity_title: v.activity_title.into(),
            title: v.title.into(),
            description: v.description.into(),
            card_type: v.card_type.into(),
            card_status: v.card_status,
            prize_type: v.prize_type,
            amount: v.amount.map(Into::into),
            currency: v.currency.into(),
            status: v.status,
            deliver_status: v.deliver_status,
            exit_status: v.exit_status,
            send_time: v.send_time.map(OffsetDateTime::unix_timestamp),
            start_time: v.start_time.map(OffsetDateTime::unix_timestamp),
            end_time: v.end_time.map(OffsetDateTime::unix_timestamp),
            effective_time: v.effective_time.map(OffsetDateTime::unix_timestamp),
            receive_deadline: v.receive_deadline.map(OffsetDateTime::unix_timestamp),
        }
    }
}

impl ToFFI for CRewardRecordOwned {
    type FFIType = CRewardRecord;

    fn to_ffi_type(&self) -> Self::FFIType {
        CRewardRecord {
            reward_type: self.reward_type.into(),
            record_id: self.record_id.to_ffi_type(),
            activity_title: self.activity_title.to_ffi_type(),
            title: self.title.to_ffi_type(),
            description: self.description.to_ffi_type(),
            card_type: self.card_type.to_ffi_type(),
            card_status: self.card_status.into(),
            prize_type: self
                .prize_type
                .as_ref()
                .map(|v| v as *const i32)
                .unwrap_or(std::ptr::null()),
            amount: self
                .amount
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            currency: self.currency.to_ffi_type(),
            status: self.status.into(),
            deliver_status: self.deliver_status.into(),
            exit_status: self.exit_status,
            send_time: self
                .send_time
                .as_ref()
                .map(|v| v as *const i64)
                .unwrap_or(std::ptr::null()),
            start_time: self
                .start_time
                .as_ref()
                .map(|v| v as *const i64)
                .unwrap_or(std::ptr::null()),
            end_time: self
                .end_time
                .as_ref()
                .map(|v| v as *const i64)
                .unwrap_or(std::ptr::null()),
            effective_time: self
                .effective_time
                .as_ref()
                .map(|v| v as *const i64)
                .unwrap_or(std::ptr::null()),
            receive_deadline: self
                .receive_deadline
                .as_ref()
                .map(|v| v as *const i64)
                .unwrap_or(std::ptr::null()),
        }
    }
}

/// Response for the reward record list
#[repr(C)]
pub struct CRewardRecordsResponse {
    /// Reward records
    pub list: *const CRewardRecord,
    /// Number of `list`
    pub num_list: usize,
    /// Total number of records
    pub total: i64,
}

#[derive(Debug)]
pub(crate) struct CRewardRecordsResponseOwned {
    list: CVec<CRewardRecordOwned>,
    total: i64,
}

impl From<RewardRecordsResponse> for CRewardRecordsResponseOwned {
    fn from(v: RewardRecordsResponse) -> Self {
        Self {
            list: v.list.into(),
            total: v.total,
        }
    }
}

impl ToFFI for CRewardRecordsResponseOwned {
    type FFIType = CRewardRecordsResponse;

    fn to_ffi_type(&self) -> Self::FFIType {
        CRewardRecordsResponse {
            list: self.list.to_ffi_type(),
            num_list: self.list.len(),
            total: self.total,
        }
    }
}

/// Commission terms of a commission coupon
#[repr(C)]
pub struct CCouponCommission {
    /// Markets the coupon applies to
    pub markets: *const *const c_char,
    /// Number of `markets`
    pub num_markets: usize,
    /// Rebate rate, as returned by the server
    pub rebate_rate: *const c_char,
    /// Instrument types the coupon applies to: `0` stocks, `1` options, `2`
    /// warrants, `3` ETF, `4` LOF
    pub bz_types: *const i32,
    /// Number of `bz_types`
    pub num_bz_types: usize,
    /// Remaining commission balance (null when not set)
    pub commission_balance: *const CDecimal,
    /// Commission duration, as returned by the server
    pub commission_duration: *const c_char,
    /// Whether the coupon is unlimited
    pub unlimited: bool,
    /// Commission expiry time as a unix timestamp (null when not set)
    pub commission_expired_time: *const i64,
    /// Number of times used
    pub usage_count: i64,
    /// Number of uses left
    pub lave_usage_count: i64,
    /// Settlement model, as returned by the server
    pub settlement_model: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CCouponCommissionOwned {
    markets: CVec<CString>,
    rebate_rate: CString,
    bz_types: CVec<i32>,
    commission_balance: Option<CDecimal>,
    commission_duration: CString,
    unlimited: bool,
    commission_expired_time: Option<i64>,
    usage_count: i64,
    lave_usage_count: i64,
    settlement_model: CString,
}

impl From<CouponCommission> for CCouponCommissionOwned {
    fn from(v: CouponCommission) -> Self {
        Self {
            markets: v.markets.into(),
            rebate_rate: v.rebate_rate.into(),
            bz_types: v.bz_types.into(),
            commission_balance: v.commission_balance.map(Into::into),
            commission_duration: v.commission_duration.into(),
            unlimited: v.unlimited,
            commission_expired_time: v
                .commission_expired_time
                .map(OffsetDateTime::unix_timestamp),
            usage_count: v.usage_count,
            lave_usage_count: v.lave_usage_count,
            settlement_model: v.settlement_model.into(),
        }
    }
}

impl ToFFI for CCouponCommissionOwned {
    type FFIType = CCouponCommission;

    fn to_ffi_type(&self) -> Self::FFIType {
        CCouponCommission {
            markets: self.markets.to_ffi_type(),
            num_markets: self.markets.len(),
            rebate_rate: self.rebate_rate.to_ffi_type(),
            bz_types: self.bz_types.to_ffi_type(),
            num_bz_types: self.bz_types.len(),
            commission_balance: self
                .commission_balance
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            commission_duration: self.commission_duration.to_ffi_type(),
            unlimited: self.unlimited,
            commission_expired_time: self
                .commission_expired_time
                .as_ref()
                .map(|v| v as *const i64)
                .unwrap_or(std::ptr::null()),
            usage_count: self.usage_count,
            lave_usage_count: self.lave_usage_count,
            settlement_model: self.settlement_model.to_ffi_type(),
        }
    }
}

/// A coupon (reward card)
#[repr(C)]
pub struct CCoupon {
    /// Coupon ID (32 hex characters; pass to coupon_orders)
    pub card_id: *const c_char,
    /// Coupon type, e.g. `commission`, `platform_fee_deduct`, `stock`, `quote`,
    /// `cash`
    pub card_type: *const c_char,
    /// Title
    pub title: *const c_char,
    /// Secondary title
    pub title_sec: *const c_char,
    /// Description
    pub description: *const c_char,
    /// Terms and details
    pub detail: *const c_char,
    /// Status
    pub card_status: CCouponStatus,
    /// Start of validity as a unix timestamp (null when not set)
    pub start_time: *const i64,
    /// End of validity as a unix timestamp (null when not set or when the
    /// coupon never expires; see `no_expiry`)
    pub end_time: *const i64,
    /// Whether the coupon never expires (the server's end time is in or after
    /// 2100)
    pub no_expiry: bool,
    /// Activation time as a unix timestamp (null when not activated)
    pub activate_time: *const i64,
    /// Amount (null when not set)
    pub amount: *const CDecimal,
    /// Currency
    pub currency: *const c_char,
    /// Trading sessions the coupon applies to, as display text
    pub available_trade_session: *const c_char,
    /// Commission terms (commission coupons only; empty otherwise)
    pub commission: CCouponCommission,
}

#[derive(Debug)]
pub(crate) struct CCouponOwned {
    card_id: CString,
    card_type: CString,
    title: CString,
    title_sec: CString,
    description: CString,
    detail: CString,
    card_status: CouponStatus,
    start_time: Option<i64>,
    end_time: Option<i64>,
    no_expiry: bool,
    activate_time: Option<i64>,
    amount: Option<CDecimal>,
    currency: CString,
    available_trade_session: CString,
    commission: CCouponCommissionOwned,
}

impl From<Coupon> for CCouponOwned {
    fn from(v: Coupon) -> Self {
        Self {
            card_id: v.card_id.into(),
            card_type: v.card_type.into(),
            title: v.title.into(),
            title_sec: v.title_sec.into(),
            description: v.description.into(),
            detail: v.detail.into(),
            card_status: v.card_status,
            start_time: v.start_time.map(OffsetDateTime::unix_timestamp),
            end_time: v.end_time.map(OffsetDateTime::unix_timestamp),
            no_expiry: v.no_expiry,
            activate_time: v.activate_time.map(OffsetDateTime::unix_timestamp),
            amount: v.amount.map(Into::into),
            currency: v.currency.into(),
            available_trade_session: v.available_trade_session.into(),
            commission: v.commission.into(),
        }
    }
}

impl ToFFI for CCouponOwned {
    type FFIType = CCoupon;

    fn to_ffi_type(&self) -> Self::FFIType {
        CCoupon {
            card_id: self.card_id.to_ffi_type(),
            card_type: self.card_type.to_ffi_type(),
            title: self.title.to_ffi_type(),
            title_sec: self.title_sec.to_ffi_type(),
            description: self.description.to_ffi_type(),
            detail: self.detail.to_ffi_type(),
            card_status: self.card_status.into(),
            start_time: self
                .start_time
                .as_ref()
                .map(|v| v as *const i64)
                .unwrap_or(std::ptr::null()),
            end_time: self
                .end_time
                .as_ref()
                .map(|v| v as *const i64)
                .unwrap_or(std::ptr::null()),
            no_expiry: self.no_expiry,
            activate_time: self
                .activate_time
                .as_ref()
                .map(|v| v as *const i64)
                .unwrap_or(std::ptr::null()),
            amount: self
                .amount
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            currency: self.currency.to_ffi_type(),
            available_trade_session: self.available_trade_session.to_ffi_type(),
            commission: self.commission.to_ffi_type(),
        }
    }
}

/// Response for the coupon list
#[repr(C)]
pub struct CCouponsResponse {
    /// Pinned coupons shown above the list (repeated on every page)
    pub top_list: *const CCoupon,
    /// Number of `top_list`
    pub num_top_list: usize,
    /// Coupons on this page
    pub list: *const CCoupon,
    /// Number of `list`
    pub num_list: usize,
    /// Pinned coupons shown below the list (repeated on every page)
    pub bottom_list: *const CCoupon,
    /// Number of `bottom_list`
    pub num_bottom_list: usize,
    /// Total number of coupons
    pub total: i64,
    /// Number of expired coupons
    pub expired_count: i64,
}

#[derive(Debug)]
pub(crate) struct CCouponsResponseOwned {
    top_list: CVec<CCouponOwned>,
    list: CVec<CCouponOwned>,
    bottom_list: CVec<CCouponOwned>,
    total: i64,
    expired_count: i64,
}

impl From<CouponsResponse> for CCouponsResponseOwned {
    fn from(v: CouponsResponse) -> Self {
        Self {
            top_list: v.top_list.into(),
            list: v.list.into(),
            bottom_list: v.bottom_list.into(),
            total: v.total,
            expired_count: v.expired_count,
        }
    }
}

impl ToFFI for CCouponsResponseOwned {
    type FFIType = CCouponsResponse;

    fn to_ffi_type(&self) -> Self::FFIType {
        CCouponsResponse {
            top_list: self.top_list.to_ffi_type(),
            num_top_list: self.top_list.len(),
            list: self.list.to_ffi_type(),
            num_list: self.list.len(),
            bottom_list: self.bottom_list.to_ffi_type(),
            num_bottom_list: self.bottom_list.len(),
            total: self.total,
            expired_count: self.expired_count,
        }
    }
}

/// An order a coupon was applied to
#[repr(C)]
pub struct CCouponOrder {
    /// Order ID
    pub order_id: *const c_char,
    /// State text, as returned by the server
    pub state: *const c_char,
    /// State value, as returned by the server (e.g. `FilledStatus`)
    pub state_val: *const c_char,
    /// Executed price (null when not set)
    pub executed_price: *const CDecimal,
    /// Executed quantity (null when not set)
    pub executed_quantity: *const CDecimal,
    /// Currency
    pub currency: *const c_char,
    /// Settlement currency
    pub settlement_currency: *const c_char,
    /// Rebate amount (null when not set)
    pub rebate: *const CDecimal,
    /// Rebate date, as returned by the server
    pub rebate_dt: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CCouponOrderOwned {
    order_id: CString,
    state: CString,
    state_val: CString,
    executed_price: Option<CDecimal>,
    executed_quantity: Option<CDecimal>,
    currency: CString,
    settlement_currency: CString,
    rebate: Option<CDecimal>,
    rebate_dt: CString,
}

impl From<CouponOrder> for CCouponOrderOwned {
    fn from(v: CouponOrder) -> Self {
        Self {
            order_id: v.order_id.into(),
            state: v.state.into(),
            state_val: v.state_val.into(),
            executed_price: v.executed_price.map(Into::into),
            executed_quantity: v.executed_quantity.map(Into::into),
            currency: v.currency.into(),
            settlement_currency: v.settlement_currency.into(),
            rebate: v.rebate.map(Into::into),
            rebate_dt: v.rebate_dt.into(),
        }
    }
}

impl ToFFI for CCouponOrderOwned {
    type FFIType = CCouponOrder;

    fn to_ffi_type(&self) -> Self::FFIType {
        CCouponOrder {
            order_id: self.order_id.to_ffi_type(),
            state: self.state.to_ffi_type(),
            state_val: self.state_val.to_ffi_type(),
            executed_price: self
                .executed_price
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            executed_quantity: self
                .executed_quantity
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            currency: self.currency.to_ffi_type(),
            settlement_currency: self.settlement_currency.to_ffi_type(),
            rebate: self
                .rebate
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            rebate_dt: self.rebate_dt.to_ffi_type(),
        }
    }
}

/// A deposit record (receipt images are not modelled)
#[repr(C)]
pub struct CDeposit {
    /// Record ID
    pub id: *const c_char,
    /// Created time as a unix timestamp (null when not set)
    pub created_at: *const i64,
    /// Updated time as a unix timestamp (null when not set)
    pub updated_at: *const i64,
    /// Currency
    pub currency: *const c_char,
    /// Notified (applied) amount (null when not set)
    pub amount: *const CDecimal,
    /// Received amount (`0` = not received yet; null when not set)
    pub receivable_amount: *const CDecimal,
    /// State text, as returned by the server
    pub state: *const c_char,
    /// State code
    pub state_code: CDepositState,
    /// Sub-state code, as returned by the server
    pub sub_state_code: i32,
    /// Deposit method ID
    pub bank_operation_type_id: *const c_char,
    /// Deposit method name
    pub bank_operation_type_name: *const c_char,
    /// Reference number
    pub reference_no: *const c_char,
    /// Whether the deposit can still be cancelled
    pub can_cancel: bool,
    /// SWIFT code of the paying bank
    pub user_bank_swift_code: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CDepositOwned {
    id: CString,
    created_at: Option<i64>,
    updated_at: Option<i64>,
    currency: CString,
    amount: Option<CDecimal>,
    receivable_amount: Option<CDecimal>,
    state: CString,
    state_code: DepositState,
    sub_state_code: i32,
    bank_operation_type_id: CString,
    bank_operation_type_name: CString,
    reference_no: CString,
    can_cancel: bool,
    user_bank_swift_code: CString,
}

impl From<Deposit> for CDepositOwned {
    fn from(v: Deposit) -> Self {
        Self {
            id: v.id.into(),
            created_at: v.created_at.map(OffsetDateTime::unix_timestamp),
            updated_at: v.updated_at.map(OffsetDateTime::unix_timestamp),
            currency: v.currency.into(),
            amount: v.amount.map(Into::into),
            receivable_amount: v.receivable_amount.map(Into::into),
            state: v.state.into(),
            state_code: v.state_code,
            sub_state_code: v.sub_state_code,
            bank_operation_type_id: v.bank_operation_type_id.into(),
            bank_operation_type_name: v.bank_operation_type_name.into(),
            reference_no: v.reference_no.into(),
            can_cancel: v.can_cancel,
            user_bank_swift_code: v.user_bank_swift_code.into(),
        }
    }
}

impl ToFFI for CDepositOwned {
    type FFIType = CDeposit;

    fn to_ffi_type(&self) -> Self::FFIType {
        CDeposit {
            id: self.id.to_ffi_type(),
            created_at: self
                .created_at
                .as_ref()
                .map(|v| v as *const i64)
                .unwrap_or(std::ptr::null()),
            updated_at: self
                .updated_at
                .as_ref()
                .map(|v| v as *const i64)
                .unwrap_or(std::ptr::null()),
            currency: self.currency.to_ffi_type(),
            amount: self
                .amount
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            receivable_amount: self
                .receivable_amount
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            state: self.state.to_ffi_type(),
            state_code: self.state_code.into(),
            sub_state_code: self.sub_state_code,
            bank_operation_type_id: self.bank_operation_type_id.to_ffi_type(),
            bank_operation_type_name: self.bank_operation_type_name.to_ffi_type(),
            reference_no: self.reference_no.to_ffi_type(),
            can_cancel: self.can_cancel,
            user_bank_swift_code: self.user_bank_swift_code.to_ffi_type(),
        }
    }
}

/// A withdrawal record
#[repr(C)]
pub struct CWithdrawal {
    /// Record ID
    pub id: *const c_char,
    /// Order ID
    pub order_id: *const c_char,
    /// Created time as a unix timestamp (null when not set)
    pub created_at: *const i64,
    /// Currency
    pub currency: *const c_char,
    /// Applied amount (null when not set)
    pub amount: *const CDecimal,
    /// Fee (null when not set)
    pub fee: *const CDecimal,
    /// Amount paid out (null when not set)
    pub settlement_amount: *const CDecimal,
    /// State code
    pub state_code: CWithdrawalState,
    /// Raw state, as returned by the server
    pub raw_state: *const c_char,
    /// State display text, as returned by the server
    pub state_description: *const c_char,
    /// Receiving bank account number, as returned by the server (not masked)
    pub bank_account: *const c_char,
    /// Receiving bank account, as displayed in the app
    pub display_bank_account: *const c_char,
    /// SWIFT code of the receiving bank
    pub user_bank_swift_code: *const c_char,
    /// Account channel
    pub account_channel: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CWithdrawalOwned {
    id: CString,
    order_id: CString,
    created_at: Option<i64>,
    currency: CString,
    amount: Option<CDecimal>,
    fee: Option<CDecimal>,
    settlement_amount: Option<CDecimal>,
    state_code: WithdrawalState,
    raw_state: CString,
    state_description: CString,
    bank_account: CString,
    display_bank_account: CString,
    user_bank_swift_code: CString,
    account_channel: CString,
}

impl From<Withdrawal> for CWithdrawalOwned {
    fn from(v: Withdrawal) -> Self {
        Self {
            id: v.id.into(),
            order_id: v.order_id.into(),
            created_at: v.created_at.map(OffsetDateTime::unix_timestamp),
            currency: v.currency.into(),
            amount: v.amount.map(Into::into),
            fee: v.fee.map(Into::into),
            settlement_amount: v.settlement_amount.map(Into::into),
            state_code: v.state_code,
            raw_state: v.raw_state.into(),
            state_description: v.state_description.into(),
            bank_account: v.bank_account.into(),
            display_bank_account: v.display_bank_account.into(),
            user_bank_swift_code: v.user_bank_swift_code.into(),
            account_channel: v.account_channel.into(),
        }
    }
}

impl ToFFI for CWithdrawalOwned {
    type FFIType = CWithdrawal;

    fn to_ffi_type(&self) -> Self::FFIType {
        CWithdrawal {
            id: self.id.to_ffi_type(),
            order_id: self.order_id.to_ffi_type(),
            created_at: self
                .created_at
                .as_ref()
                .map(|v| v as *const i64)
                .unwrap_or(std::ptr::null()),
            currency: self.currency.to_ffi_type(),
            amount: self
                .amount
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            fee: self
                .fee
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            settlement_amount: self
                .settlement_amount
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            state_code: self.state_code.into(),
            raw_state: self.raw_state.to_ffi_type(),
            state_description: self.state_description.to_ffi_type(),
            bank_account: self.bank_account.to_ffi_type(),
            display_bank_account: self.display_bank_account.to_ffi_type(),
            user_bank_swift_code: self.user_bank_swift_code.to_ffi_type(),
            account_channel: self.account_channel.to_ffi_type(),
        }
    }
}

/// Response for the withdrawal list
#[repr(C)]
pub struct CWithdrawalsResponse {
    /// Withdrawal records
    pub list: *const CWithdrawal,
    /// Number of `list`
    pub num_list: usize,
    /// Total number of records
    pub total: i64,
}

#[derive(Debug)]
pub(crate) struct CWithdrawalsResponseOwned {
    list: CVec<CWithdrawalOwned>,
    total: i64,
}

impl From<WithdrawalsResponse> for CWithdrawalsResponseOwned {
    fn from(v: WithdrawalsResponse) -> Self {
        Self {
            list: v.list.into(),
            total: v.total,
        }
    }
}

impl ToFFI for CWithdrawalsResponseOwned {
    type FFIType = CWithdrawalsResponse;

    fn to_ffi_type(&self) -> Self::FFIType {
        CWithdrawalsResponse {
            list: self.list.to_ffi_type(),
            num_list: self.list.len(),
            total: self.total,
        }
    }
}

/// Options for the stock transfer-in list
#[repr(C)]
pub struct CGetStockTransfersInOptions {
    /// Market filter: `HK`, `US`, `CN` or `SG` (can be null)
    pub market: *const c_char,
    /// State filter (can be null for all states; `StockTransferInStateUnknown`
    /// is ignored). Sent to the server as the state's server code, not as the C
    /// enum value.
    pub state: *const CStockTransferInState,
    /// Only applications created at or after this unix timestamp (can be
    /// null)
    pub start: *const i64,
    /// Only applications created at or before this unix timestamp (can be
    /// null)
    pub end: *const i64,
    /// Page number (can be null, default 1)
    pub page: *const i32,
    /// Page size (can be null, default 20, max 50)
    pub size: *const i32,
}

/// Options for the stock transfer-out list
#[repr(C)]
pub struct CGetStockTransfersOutOptions {
    /// Market filter: `HK`, `US`, `CN` or `SG` (can be null)
    pub market: *const c_char,
    /// State filter (can be null for all states; `StockTransferOutStateUnknown`
    /// is ignored). Sent to the server as the state's server code, not as the C
    /// enum value.
    pub state: *const CStockTransferOutState,
    /// Only applications created at or after this unix timestamp (can be
    /// null)
    pub start: *const i64,
    /// Only applications created at or before this unix timestamp (can be
    /// null)
    pub end: *const i64,
    /// Page number (can be null, default 1)
    pub page: *const i32,
    /// Page size (can be null, default 20, max 50)
    pub size: *const i32,
}

/// Options for the reward record list
#[repr(C)]
pub struct CGetRewardRecordsOptions {
    /// Page number (can be null, default 1)
    pub page: *const i32,
    /// Page size (can be null, max 50)
    pub page_size: *const i32,
    /// Reward type filter (can be null for all types; `RewardTypeUnknown` is
    /// ignored)
    pub reward_type: *const CRewardType,
}

/// Options for the coupon list
#[repr(C)]
pub struct CGetCouponsOptions {
    /// Page number (can be null, default 1)
    pub page: *const i32,
    /// Page size (can be null, max 50)
    pub page_size: *const i32,
    /// List expired coupons instead of the ones that have not expired
    pub expired: bool,
    /// Coupon type filter, e.g. `commission` (can be null for all types)
    pub card_type: *const c_char,
}

/// Options for the deposit list
#[repr(C)]
pub struct CGetDepositsOptions {
    /// Page number (can be null, default 1)
    pub page: *const i32,
    /// Page size (can be null)
    pub size: *const i32,
    /// State filter (can be null)
    pub states: *const CDepositState,
    /// Number of states
    pub num_states: usize,
    /// Currency filter, e.g. `HKD` (can be null)
    pub currencies: *const *const c_char,
    /// Number of currencies
    pub num_currencies: usize,
    /// Only deposits applied at or after this unix timestamp (can be null)
    pub start: *const i64,
    /// Only deposits applied at or before this unix timestamp (can be null)
    pub end: *const i64,
}

/// Options for the withdrawal list
#[repr(C)]
pub struct CGetWithdrawalsOptions {
    /// Page number (can be null, default 1)
    pub page: *const i32,
    /// Page size (can be null)
    pub size: *const i32,
    /// Only withdrawals applied at or after this unix timestamp (can be null)
    pub start: *const i64,
    /// Only withdrawals applied at or before this unix timestamp (can be null)
    pub end: *const i64,
}
