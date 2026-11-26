use std::os::raw::c_char;

use longbridge::asset::{
    AuthorizationStatus, BankCard, BankCardAuthorization, BankCardBindState, BankCardStatus,
    BankCardWitnessState, CommissionUnlimitedType, Coupon, CouponAiBenefit, CouponCash,
    CouponCoffee, CouponCoffeeStore, CouponCommission, CouponCrypto, CouponCryptoCounter,
    CouponCryptoFallback, CouponFinanceInterest, CouponFundDeduct, CouponIpo, CouponIpoCommission,
    CouponIpoInterest, CouponOrder, CouponPlatformFeeDeduct, CouponQuote, CouponStatus,
    CouponStock, CouponStockCounter, CouponStockFallback, CouponStockFragment, CouponTargetType,
    CouponsResponse, Deposit, DepositState, EddaApplication, QuoteInvestorType,
    RewardDeliverStatus, RewardRecord, RewardRecordsResponse, RewardStatus, RewardType,
    StatementItem, StockTransferIn, StockTransferInResponse, StockTransferInState,
    StockTransferOut, StockTransferOutState, TransferStock, TransferStockState, Withdrawal,
    WithdrawalState, WithdrawalsResponse,
};
use time::OffsetDateTime;

use crate::{
    asset_context::enum_types::{
        CAuthorizationStatus, CBankCardBindState, CBankCardStatus, CBankCardWitnessState,
        CCommissionUnlimitedType, CCouponStatus, CCouponTargetType, CDepositState,
        CIpoCouponTargetType, CQuoteInvestorType, CRewardDeliverStatus, CRewardStatus, CRewardType,
        CStockTransferInState, CStockTransferOutState, CTransferStockState, CWithdrawalState,
    },
    types::{CDecimal, COption, CString, CVec, ToFFI},
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
    /// The server's numeric code of `status`, also when it is a code
    /// this SDK does not know (`status` is then `Unknown`); null when the
    /// server sent none
    pub status_raw: *const i32,
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
    status_raw: Option<i32>,
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
            status_raw: v.status_raw,
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
            status_raw: self
                .status_raw
                .as_ref()
                .map(|v| v as *const i32)
                .unwrap_or(std::ptr::null()),
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
    /// The server's numeric code of `status`, also when it is a code
    /// this SDK does not know (`status` is then `Unknown`); null when the
    /// server sent none
    pub status_raw: *const i32,
    /// Witness state
    pub witness_state: CBankCardWitnessState,
    /// The server's numeric code of `witness_state`, also when it is a code
    /// this SDK does not know (`witness_state` is then `Unknown`); null when
    /// the server sent none
    pub witness_state_raw: *const i32,
    /// Whether the card supports witnessing
    pub support_witness: bool,
    /// Binding state
    pub bind_state: CBankCardBindState,
    /// The server's numeric code of `bind_state`, also when it is a code
    /// this SDK does not know (`bind_state` is then `Unknown`); null when the
    /// server sent none
    pub bind_state_raw: *const i32,
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
    status_raw: Option<i32>,
    witness_state: BankCardWitnessState,
    witness_state_raw: Option<i32>,
    support_witness: bool,
    bind_state: BankCardBindState,
    bind_state_raw: Option<i32>,
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
            status_raw: v.status_raw,
            witness_state: v.witness_state,
            witness_state_raw: v.witness_state_raw,
            support_witness: v.support_witness,
            bind_state: v.bind_state,
            bind_state_raw: v.bind_state_raw,
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
            status_raw: self
                .status_raw
                .as_ref()
                .map(|v| v as *const i32)
                .unwrap_or(std::ptr::null()),
            witness_state: self.witness_state.into(),
            witness_state_raw: self
                .witness_state_raw
                .as_ref()
                .map(|v| v as *const i32)
                .unwrap_or(std::ptr::null()),
            support_witness: self.support_witness,
            bind_state: self.bind_state.into(),
            bind_state_raw: self
                .bind_state_raw
                .as_ref()
                .map(|v| v as *const i32)
                .unwrap_or(std::ptr::null()),
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
    /// Authorization flow: `hk_edda` (authorized at the bank) or `hk_sdda`
    /// (authorized in the app)
    pub flow_id: *const c_char,
    /// Display name of the authorization
    pub name: *const c_char,
    /// Authorized currency
    pub currency: *const c_char,
    /// Application status
    pub status: CAuthorizationStatus,
    /// The server's numeric code of `status`, also when it is a code
    /// this SDK does not know (`status` is then `Unknown`); null when the
    /// server sent none
    pub status_raw: *const i32,
    /// Failure reason (empty unless the application failed)
    pub message: *const c_char,
    /// Time the application was filed as a unix timestamp (null when not
    /// set)
    pub created_at: *const i64,
    /// Time the application was last updated as a unix timestamp (null when
    /// not set)
    pub updated_at: *const i64,
}

#[derive(Debug)]
pub(crate) struct CEddaApplicationOwned {
    id: CString,
    flow_id: CString,
    name: CString,
    currency: CString,
    status: AuthorizationStatus,
    status_raw: Option<i32>,
    message: CString,
    created_at: Option<i64>,
    updated_at: Option<i64>,
}

impl From<EddaApplication> for CEddaApplicationOwned {
    fn from(v: EddaApplication) -> Self {
        Self {
            id: v.id.into(),
            flow_id: v.flow_id.into(),
            name: v.name.into(),
            currency: v.currency.into(),
            status: v.status,
            status_raw: v.status_raw,
            message: v.message.into(),
            created_at: v.created_at.map(OffsetDateTime::unix_timestamp),
            updated_at: v.updated_at.map(OffsetDateTime::unix_timestamp),
        }
    }
}

impl ToFFI for CEddaApplicationOwned {
    type FFIType = CEddaApplication;

    fn to_ffi_type(&self) -> Self::FFIType {
        CEddaApplication {
            id: self.id.to_ffi_type(),
            flow_id: self.flow_id.to_ffi_type(),
            name: self.name.to_ffi_type(),
            currency: self.currency.to_ffi_type(),
            status: self.status.into(),
            status_raw: self
                .status_raw
                .as_ref()
                .map(|v| v as *const i32)
                .unwrap_or(std::ptr::null()),
            message: self.message.to_ffi_type(),
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
    /// The server's numeric code of `state_code`, also when it is a code
    /// this SDK does not know (`state_code` is then `Unknown`); null when the
    /// server sent none
    pub state_code_raw: *const i32,
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
    state_code_raw: Option<i32>,
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
            state_code_raw: v.state_code_raw,
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
            state_code_raw: self
                .state_code_raw
                .as_ref()
                .map(|v| v as *const i32)
                .unwrap_or(std::ptr::null()),
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
    /// The server's numeric code of `state_code`, also when it is a code
    /// this SDK does not know (`state_code` is then `Unknown`); null when the
    /// server sent none
    pub state_code_raw: *const i32,
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
    state_code_raw: Option<i32>,
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
            state_code_raw: v.state_code_raw,
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
            state_code_raw: self
                .state_code_raw
                .as_ref()
                .map(|v| v as *const i32)
                .unwrap_or(std::ptr::null()),
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
    /// The server's numeric code of `state_code`, also when it is a code
    /// this SDK does not know (`state_code` is then `Unknown`); null when the
    /// server sent none
    pub state_code_raw: *const i32,
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
    state_code_raw: Option<i32>,
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
            state_code_raw: v.state_code_raw,
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
            state_code_raw: self
                .state_code_raw
                .as_ref()
                .map(|v| v as *const i32)
                .unwrap_or(std::ptr::null()),
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

/// A reward record from the reward center (prize redemption secrets
/// `coupon_secret`, cash card codes `cash_card_code` and shipping contacts are
/// not modelled)
#[repr(C)]
pub struct CRewardRecord {
    /// Reward type: named after the first detail object the server sent
    /// (`cash`, `card`, `prize`, in that order), else the server's
    /// `reward_type` code
    pub reward_type: CRewardType,
    /// The server's numeric code of `reward_type`, also when it is a code
    /// this SDK does not know (`reward_type` is then `Unknown`); null when the
    /// server sent none
    pub reward_type_raw: *const i32,
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
    /// The server's numeric code of `card_status`, also when it is a code
    /// this SDK does not know (`card_status` is then `Unknown`); null when the
    /// server sent none
    pub card_status_raw: *const i32,
    /// Prize type code (gift rewards; `0` = physical goods), null when not set
    pub prize_type: *const i32,
    /// Amount (null when not set)
    pub amount: *const CDecimal,
    /// Currency
    pub currency: *const c_char,
    /// Reward status
    pub status: CRewardStatus,
    /// The server's numeric code of `status`, also when it is a code
    /// this SDK does not know (`status` is then `Unknown`); null when the
    /// server sent none
    pub status_raw: *const i32,
    /// Delivery status (issued gift rewards only)
    pub deliver_status: CRewardDeliverStatus,
    /// The server's numeric code of `deliver_status` (e.g. 11 where
    /// `deliver_status` reads `AddressFilled`), also when it is a code this SDK
    /// does not know (`deliver_status` is then `Unknown`); null when the server
    /// sent none
    pub deliver_status_raw: *const i32,
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
    reward_type_raw: Option<i32>,
    record_id: CString,
    activity_title: CString,
    title: CString,
    description: CString,
    card_type: CString,
    card_status: CouponStatus,
    card_status_raw: Option<i32>,
    prize_type: Option<i32>,
    amount: Option<CDecimal>,
    currency: CString,
    status: RewardStatus,
    status_raw: Option<i32>,
    deliver_status: RewardDeliverStatus,
    deliver_status_raw: Option<i32>,
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
            reward_type_raw: v.reward_type_raw,
            record_id: v.record_id.into(),
            activity_title: v.activity_title.into(),
            title: v.title.into(),
            description: v.description.into(),
            card_type: v.card_type.into(),
            card_status: v.card_status,
            card_status_raw: v.card_status_raw,
            prize_type: v.prize_type,
            amount: v.amount.map(Into::into),
            currency: v.currency.into(),
            status: v.status,
            status_raw: v.status_raw,
            deliver_status: v.deliver_status,
            deliver_status_raw: v.deliver_status_raw,
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
            reward_type_raw: self
                .reward_type_raw
                .as_ref()
                .map(|v| v as *const i32)
                .unwrap_or(std::ptr::null()),
            record_id: self.record_id.to_ffi_type(),
            activity_title: self.activity_title.to_ffi_type(),
            title: self.title.to_ffi_type(),
            description: self.description.to_ffi_type(),
            card_type: self.card_type.to_ffi_type(),
            card_status: self.card_status.into(),
            card_status_raw: self
                .card_status_raw
                .as_ref()
                .map(|v| v as *const i32)
                .unwrap_or(std::ptr::null()),
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
            status_raw: self
                .status_raw
                .as_ref()
                .map(|v| v as *const i32)
                .unwrap_or(std::ptr::null()),
            deliver_status: self.deliver_status.into(),
            deliver_status_raw: self
                .deliver_status_raw
                .as_ref()
                .map(|v| v as *const i32)
                .unwrap_or(std::ptr::null()),
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

/// Commission terms of a commission coupon (`commission`)
#[repr(C)]
pub struct CCouponCommission {
    /// Markets the coupon applies to
    pub markets: *const *const c_char,
    /// Number of `markets`
    pub num_markets: usize,
    /// Rebate rate, as returned by the server
    pub rebate_rate: *const c_char,
    /// Instrument types the coupon applies to
    pub bz_types: *const CCouponTargetType,
    /// Number of `bz_types`
    pub num_bz_types: usize,
    /// The server's numeric codes of `bz_types`, in order, including codes this
    /// SDK does not know (which are `Unknown` in `bz_types`)
    pub bz_types_raw: *const i32,
    /// Number of `bz_types_raw`
    pub num_bz_types_raw: usize,
    /// Remaining commission balance (null when not set)
    pub commission_balance: *const CDecimal,
    /// Commission duration in seconds (the app reads 200 years or more as
    /// lifetime free commission)
    pub commission_duration: i64,
    /// Kind of commission coupon (the server's `unlimited` code)
    pub unlimited: CCommissionUnlimitedType,
    /// The server's numeric code of `unlimited`, also when it is a code
    /// this SDK does not know (`unlimited` is then `Unknown`); null when the
    /// server sent none
    pub unlimited_raw: *const i32,
    /// Commission expiry time as a unix timestamp (null when not set)
    pub commission_expired_time: *const i64,
    /// Number of times used
    pub usage_count: i64,
    /// Number of uses left
    pub lave_usage_count: i64,
    /// Settlement model, as returned by the server
    pub settlement_model: *const c_char,
    /// Amount cap per use of a per-use discount card (`ByRatePer`), as returned
    /// by the server
    pub amount_limit: i64,
    /// Whether the coupon can be combined with other coupons
    pub superposition: bool,
}

#[derive(Debug)]
pub(crate) struct CCouponCommissionOwned {
    markets: CVec<CString>,
    rebate_rate: CString,
    bz_types: CVec<CCouponTargetType>,
    bz_types_raw: CVec<i32>,
    commission_balance: Option<CDecimal>,
    commission_duration: i64,
    unlimited: CommissionUnlimitedType,
    unlimited_raw: Option<i32>,
    commission_expired_time: Option<i64>,
    usage_count: i64,
    lave_usage_count: i64,
    settlement_model: CString,
    amount_limit: i64,
    superposition: bool,
}

impl From<CouponCommission> for CCouponCommissionOwned {
    fn from(v: CouponCommission) -> Self {
        Self {
            markets: v.markets.into(),
            rebate_rate: v.rebate_rate.into(),
            bz_types: v.bz_types.into(),
            bz_types_raw: v.bz_types_raw.into(),
            commission_balance: v.commission_balance.map(Into::into),
            commission_duration: v.commission_duration,
            unlimited: v.unlimited,
            unlimited_raw: v.unlimited_raw,
            commission_expired_time: v
                .commission_expired_time
                .map(OffsetDateTime::unix_timestamp),
            usage_count: v.usage_count,
            lave_usage_count: v.lave_usage_count,
            settlement_model: v.settlement_model.into(),
            amount_limit: v.amount_limit,
            superposition: v.superposition,
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
            bz_types_raw: self.bz_types_raw.to_ffi_type(),
            num_bz_types_raw: self.bz_types_raw.len(),
            commission_balance: self
                .commission_balance
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            commission_duration: self.commission_duration,
            unlimited: self.unlimited.into(),
            unlimited_raw: self
                .unlimited_raw
                .as_ref()
                .map(|v| v as *const i32)
                .unwrap_or(std::ptr::null()),
            commission_expired_time: self
                .commission_expired_time
                .as_ref()
                .map(|v| v as *const i64)
                .unwrap_or(std::ptr::null()),
            usage_count: self.usage_count,
            lave_usage_count: self.lave_usage_count,
            settlement_model: self.settlement_model.to_ffi_type(),
            amount_limit: self.amount_limit,
            superposition: self.superposition,
        }
    }
}

/// Terms of an AI benefit coupon (`ai_benefit`)
#[repr(C)]
pub struct CCouponAiBenefit {
    /// Length of the benefit in days
    pub benefit_days: i32,
    /// Start of the benefit as a unix timestamp (null when not set)
    pub benefit_start: *const i64,
    /// End of the benefit as a unix timestamp (null when not set)
    pub benefit_end: *const i64,
    /// Benefit tag, as returned by the server
    pub benefit_tag: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CCouponAiBenefitOwned {
    benefit_days: i32,
    benefit_start: Option<i64>,
    benefit_end: Option<i64>,
    benefit_tag: CString,
}

impl From<CouponAiBenefit> for CCouponAiBenefitOwned {
    fn from(v: CouponAiBenefit) -> Self {
        Self {
            benefit_days: v.benefit_days,
            benefit_start: v.benefit_start.map(OffsetDateTime::unix_timestamp),
            benefit_end: v.benefit_end.map(OffsetDateTime::unix_timestamp),
            benefit_tag: v.benefit_tag.into(),
        }
    }
}

impl ToFFI for CCouponAiBenefitOwned {
    type FFIType = CCouponAiBenefit;

    fn to_ffi_type(&self) -> Self::FFIType {
        CCouponAiBenefit {
            benefit_days: self.benefit_days,
            benefit_start: self
                .benefit_start
                .as_ref()
                .map(|v| v as *const i64)
                .unwrap_or(std::ptr::null()),
            benefit_end: self
                .benefit_end
                .as_ref()
                .map(|v| v as *const i64)
                .unwrap_or(std::ptr::null()),
            benefit_tag: self.benefit_tag.to_ffi_type(),
        }
    }
}

/// Terms of a cash coupon (`cash`)
#[repr(C)]
pub struct CCouponCash {
    /// Receive duration, as returned by the server (its unit and meaning are
    /// not confirmed)
    pub receive_duration: i64,
}

#[derive(Debug)]
pub(crate) struct CCouponCashOwned {
    receive_duration: i64,
}

impl From<CouponCash> for CCouponCashOwned {
    fn from(v: CouponCash) -> Self {
        Self {
            receive_duration: v.receive_duration,
        }
    }
}

impl ToFFI for CCouponCashOwned {
    type FFIType = CCouponCash;

    fn to_ffi_type(&self) -> Self::FFIType {
        CCouponCash {
            receive_duration: self.receive_duration,
        }
    }
}

/// A store of a coffee coupon
#[repr(C)]
pub struct CCouponCoffeeStore {
    /// Store ID
    pub store_id: i64,
    /// Store name
    pub store_name: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CCouponCoffeeStoreOwned {
    store_id: i64,
    store_name: CString,
}

impl From<CouponCoffeeStore> for CCouponCoffeeStoreOwned {
    fn from(v: CouponCoffeeStore) -> Self {
        Self {
            store_id: v.store_id,
            store_name: v.store_name.into(),
        }
    }
}

impl ToFFI for CCouponCoffeeStoreOwned {
    type FFIType = CCouponCoffeeStore;

    fn to_ffi_type(&self) -> Self::FFIType {
        CCouponCoffeeStore {
            store_id: self.store_id,
            store_name: self.store_name.to_ffi_type(),
        }
    }
}

/// Terms of a coffee coupon (`coffee_coupon`)
#[repr(C)]
pub struct CCouponCoffee {
    /// Coupon channel, as returned by the server
    pub coupon_channel: *const c_char,
    /// Coupon ID, as returned by the server
    pub coupon_id: *const c_char,
    /// Voucher template code, e.g. `coffee-50` (not a personal redemption code)
    pub coupon_no: *const c_char,
    /// Whether the coupon can be used at every store
    pub is_available_all_store: bool,
    /// Period type (e.g. `day`), as returned by the server
    pub period_type: *const c_char,
    /// ID of the store the coupon was settled at (`0` when not settled)
    pub settled_store_id: i64,
    /// IDs of the stores the coupon can be used at
    pub store_id_list: *const i64,
    /// Number of `store_id_list`
    pub num_store_id_list: usize,
    /// Stores the coupon can be used at
    pub store_list: *const CCouponCoffeeStore,
    /// Number of `store_list`
    pub num_store_list: usize,
    /// Threshold, as returned by the server
    pub threshold: i64,
    /// Use limit (`total` or `period`), as returned by the server
    pub use_limit: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CCouponCoffeeOwned {
    coupon_channel: CString,
    coupon_id: CString,
    coupon_no: CString,
    is_available_all_store: bool,
    period_type: CString,
    settled_store_id: i64,
    store_id_list: CVec<i64>,
    store_list: CVec<CCouponCoffeeStoreOwned>,
    threshold: i64,
    use_limit: CString,
}

impl From<CouponCoffee> for CCouponCoffeeOwned {
    fn from(v: CouponCoffee) -> Self {
        Self {
            coupon_channel: v.coupon_channel.into(),
            coupon_id: v.coupon_id.into(),
            coupon_no: v.coupon_no.into(),
            is_available_all_store: v.is_available_all_store,
            period_type: v.period_type.into(),
            settled_store_id: v.settled_store_id,
            store_id_list: v.store_id_list.into(),
            store_list: v.store_list.into(),
            threshold: v.threshold,
            use_limit: v.use_limit.into(),
        }
    }
}

impl ToFFI for CCouponCoffeeOwned {
    type FFIType = CCouponCoffee;

    fn to_ffi_type(&self) -> Self::FFIType {
        CCouponCoffee {
            coupon_channel: self.coupon_channel.to_ffi_type(),
            coupon_id: self.coupon_id.to_ffi_type(),
            coupon_no: self.coupon_no.to_ffi_type(),
            is_available_all_store: self.is_available_all_store,
            period_type: self.period_type.to_ffi_type(),
            settled_store_id: self.settled_store_id,
            store_id_list: self.store_id_list.to_ffi_type(),
            num_store_id_list: self.store_id_list.len(),
            store_list: self.store_list.to_ffi_type(),
            num_store_list: self.store_list.len(),
            threshold: self.threshold,
            use_limit: self.use_limit.to_ffi_type(),
        }
    }
}

/// A counter (security) of a crypto coupon
#[repr(C)]
pub struct CCouponCryptoCounter {
    /// Counter ID
    pub counter_id: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CCouponCryptoCounterOwned {
    counter_id: CString,
}

impl From<CouponCryptoCounter> for CCouponCryptoCounterOwned {
    fn from(v: CouponCryptoCounter) -> Self {
        Self {
            counter_id: v.counter_id.into(),
        }
    }
}

impl ToFFI for CCouponCryptoCounterOwned {
    type FFIType = CCouponCryptoCounter;

    fn to_ffi_type(&self) -> Self::FFIType {
        CCouponCryptoCounter {
            counter_id: self.counter_id.to_ffi_type(),
        }
    }
}

/// Fallback of a crypto coupon
#[repr(C)]
pub struct CCouponCryptoFallback {
    /// Whether the fallback is enabled
    pub enabled: bool,
    /// Counter ID
    pub counter_id: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CCouponCryptoFallbackOwned {
    enabled: bool,
    counter_id: CString,
}

impl From<CouponCryptoFallback> for CCouponCryptoFallbackOwned {
    fn from(v: CouponCryptoFallback) -> Self {
        Self {
            enabled: v.enabled,
            counter_id: v.counter_id.into(),
        }
    }
}

impl ToFFI for CCouponCryptoFallbackOwned {
    type FFIType = CCouponCryptoFallback;

    fn to_ffi_type(&self) -> Self::FFIType {
        CCouponCryptoFallback {
            enabled: self.enabled,
            counter_id: self.counter_id.to_ffi_type(),
        }
    }
}

/// Terms of a crypto coupon (`crypto`)
#[repr(C)]
pub struct CCouponCrypto {
    /// Fallback
    pub fallback: CCouponCryptoFallback,
    /// Optional counters
    pub optional_counters: *const CCouponCryptoCounter,
    /// Number of `optional_counters`
    pub num_optional_counters: usize,
}

#[derive(Debug)]
pub(crate) struct CCouponCryptoOwned {
    fallback: CCouponCryptoFallbackOwned,
    optional_counters: CVec<CCouponCryptoCounterOwned>,
}

impl From<CouponCrypto> for CCouponCryptoOwned {
    fn from(v: CouponCrypto) -> Self {
        Self {
            fallback: v.fallback.into(),
            optional_counters: v.optional_counters.into(),
        }
    }
}

impl ToFFI for CCouponCryptoOwned {
    type FFIType = CCouponCrypto;

    fn to_ffi_type(&self) -> Self::FFIType {
        CCouponCrypto {
            fallback: self.fallback.to_ffi_type(),
            optional_counters: self.optional_counters.to_ffi_type(),
            num_optional_counters: self.optional_counters.len(),
        }
    }
}

/// Terms of a finance interest coupon (`finance_interest`). The meaning of
/// these fields is not confirmed; they are returned as the server sends them.
#[repr(C)]
pub struct CCouponFinanceInterest {
    /// Discount mode, as returned by the server
    pub discount_mode: i32,
    /// Discount rate (null when not set)
    pub discount_rate: *const CDecimal,
    /// Whether the coupon can be combined with other coupons
    pub superposition: bool,
}

#[derive(Debug)]
pub(crate) struct CCouponFinanceInterestOwned {
    discount_mode: i32,
    discount_rate: Option<CDecimal>,
    superposition: bool,
}

impl From<CouponFinanceInterest> for CCouponFinanceInterestOwned {
    fn from(v: CouponFinanceInterest) -> Self {
        Self {
            discount_mode: v.discount_mode,
            discount_rate: v.discount_rate.map(Into::into),
            superposition: v.superposition,
        }
    }
}

impl ToFFI for CCouponFinanceInterestOwned {
    type FFIType = CCouponFinanceInterest;

    fn to_ffi_type(&self) -> Self::FFIType {
        CCouponFinanceInterest {
            discount_mode: self.discount_mode,
            discount_rate: self
                .discount_rate
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            superposition: self.superposition,
        }
    }
}

/// Terms of a fund deduction coupon (`fund_deduct`)
#[repr(C)]
pub struct CCouponFundDeduct {
    /// Maximum risk level, as returned by the server
    pub max_risk_level: i32,
    /// Minimum order amount, as returned by the server
    pub min_order_amount: *const c_char,
    /// Order types, as returned by the server
    pub order_types: *const *const c_char,
    /// Number of `order_types`
    pub num_order_types: usize,
    /// Underlying type, as returned by the server
    pub underlying_type: *const c_char,
    /// Underlying values, as returned by the server
    pub underlying_values: *const *const c_char,
    /// Number of `underlying_values`
    pub num_underlying_values: usize,
    /// Underlying names, as returned by the server
    pub underlying_names: *const *const c_char,
    /// Number of `underlying_names`
    pub num_underlying_names: usize,
}

#[derive(Debug)]
pub(crate) struct CCouponFundDeductOwned {
    max_risk_level: i32,
    min_order_amount: CString,
    order_types: CVec<CString>,
    underlying_type: CString,
    underlying_values: CVec<CString>,
    underlying_names: CVec<CString>,
}

impl From<CouponFundDeduct> for CCouponFundDeductOwned {
    fn from(v: CouponFundDeduct) -> Self {
        Self {
            max_risk_level: v.max_risk_level,
            min_order_amount: v.min_order_amount.into(),
            order_types: v.order_types.into(),
            underlying_type: v.underlying_type.into(),
            underlying_values: v.underlying_values.into(),
            underlying_names: v.underlying_names.into(),
        }
    }
}

impl ToFFI for CCouponFundDeductOwned {
    type FFIType = CCouponFundDeduct;

    fn to_ffi_type(&self) -> Self::FFIType {
        CCouponFundDeduct {
            max_risk_level: self.max_risk_level,
            min_order_amount: self.min_order_amount.to_ffi_type(),
            order_types: self.order_types.to_ffi_type(),
            num_order_types: self.order_types.len(),
            underlying_type: self.underlying_type.to_ffi_type(),
            underlying_values: self.underlying_values.to_ffi_type(),
            num_underlying_values: self.underlying_values.len(),
            underlying_names: self.underlying_names.to_ffi_type(),
            num_underlying_names: self.underlying_names.len(),
        }
    }
}

/// Terms of a quote (market data) or fundamental-data coupon (`quote` /
/// `fundamental`)
#[repr(C)]
pub struct CCouponQuote {
    /// Current end time as a unix timestamp (null when not set)
    pub current_end_time: *const i64,
    /// Whether the coupon is inapplicable, as returned by the server
    pub inapplicable: bool,
    /// Investor type the coupon applies to
    pub investor_type: CQuoteInvestorType,
    /// The server's numeric code of `investor_type`, also when it is a code
    /// this SDK does not know (`investor_type` is then `Unknown`); null when
    /// the server sent none
    pub investor_type_raw: *const i32,
    /// Whether the coupon is activated manually
    pub manual_activate: bool,
    /// Markets
    pub markets: *const *const c_char,
    /// Number of `markets`
    pub num_markets: usize,
    /// Investor type of the member, as returned by the server
    pub member_investor_type: i32,
    /// Paired SPU number, as returned by the server
    pub pair_spu_no: *const c_char,
    /// Quote duration in seconds (the app reads 200 years or more as lifetime)
    pub quote_duration: i64,
    /// Quote market
    pub quote_market: *const c_char,
    /// Quote tags
    pub quote_tags: *const *const c_char,
    /// Number of `quote_tags`
    pub num_quote_tags: usize,
    /// Whether the coupon can be combined with other coupons
    pub superposition: bool,
}

#[derive(Debug)]
pub(crate) struct CCouponQuoteOwned {
    current_end_time: Option<i64>,
    inapplicable: bool,
    investor_type: QuoteInvestorType,
    investor_type_raw: Option<i32>,
    manual_activate: bool,
    markets: CVec<CString>,
    member_investor_type: i32,
    pair_spu_no: CString,
    quote_duration: i64,
    quote_market: CString,
    quote_tags: CVec<CString>,
    superposition: bool,
}

impl From<CouponQuote> for CCouponQuoteOwned {
    fn from(v: CouponQuote) -> Self {
        Self {
            current_end_time: v.current_end_time.map(OffsetDateTime::unix_timestamp),
            inapplicable: v.inapplicable,
            investor_type: v.investor_type,
            investor_type_raw: v.investor_type_raw,
            manual_activate: v.manual_activate,
            markets: v.markets.into(),
            member_investor_type: v.member_investor_type,
            pair_spu_no: v.pair_spu_no.into(),
            quote_duration: v.quote_duration,
            quote_market: v.quote_market.into(),
            quote_tags: v.quote_tags.into(),
            superposition: v.superposition,
        }
    }
}

impl ToFFI for CCouponQuoteOwned {
    type FFIType = CCouponQuote;

    fn to_ffi_type(&self) -> Self::FFIType {
        CCouponQuote {
            current_end_time: self
                .current_end_time
                .as_ref()
                .map(|v| v as *const i64)
                .unwrap_or(std::ptr::null()),
            inapplicable: self.inapplicable,
            investor_type: self.investor_type.into(),
            investor_type_raw: self
                .investor_type_raw
                .as_ref()
                .map(|v| v as *const i32)
                .unwrap_or(std::ptr::null()),
            manual_activate: self.manual_activate,
            markets: self.markets.to_ffi_type(),
            num_markets: self.markets.len(),
            member_investor_type: self.member_investor_type,
            pair_spu_no: self.pair_spu_no.to_ffi_type(),
            quote_duration: self.quote_duration,
            quote_market: self.quote_market.to_ffi_type(),
            quote_tags: self.quote_tags.to_ffi_type(),
            num_quote_tags: self.quote_tags.len(),
            superposition: self.superposition,
        }
    }
}

/// Terms of an IPO coupon (`ipo`)
#[repr(C)]
pub struct CCouponIpo {
    /// Subscription ID
    pub subscription_id: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CCouponIpoOwned {
    subscription_id: CString,
}

impl From<CouponIpo> for CCouponIpoOwned {
    fn from(v: CouponIpo) -> Self {
        Self {
            subscription_id: v.subscription_id.into(),
        }
    }
}

impl ToFFI for CCouponIpoOwned {
    type FFIType = CCouponIpo;

    fn to_ffi_type(&self) -> Self::FFIType {
        CCouponIpo {
            subscription_id: self.subscription_id.to_ffi_type(),
        }
    }
}

/// Terms of an IPO commission coupon (`ipo_commission`)
#[repr(C)]
pub struct CCouponIpoCommission {
    /// IPO subscription types the coupon applies to
    pub bz_types: *const CIpoCouponTargetType,
    /// Number of `bz_types`
    pub num_bz_types: usize,
    /// The server's numeric codes of `bz_types`, in order, including codes this
    /// SDK does not know (which are `Unknown` in `bz_types`)
    pub bz_types_raw: *const i32,
    /// Number of `bz_types_raw`
    pub num_bz_types_raw: usize,
    /// Counter ID
    pub counter_id: *const c_char,
    /// Counter name
    pub counter_name: *const c_char,
    /// Financing limit, as returned by the server
    pub financing_limit: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CCouponIpoCommissionOwned {
    bz_types: CVec<CIpoCouponTargetType>,
    bz_types_raw: CVec<i32>,
    counter_id: CString,
    counter_name: CString,
    financing_limit: CString,
}

impl From<CouponIpoCommission> for CCouponIpoCommissionOwned {
    fn from(v: CouponIpoCommission) -> Self {
        Self {
            bz_types: v.bz_types.into(),
            bz_types_raw: v.bz_types_raw.into(),
            counter_id: v.counter_id.into(),
            counter_name: v.counter_name.into(),
            financing_limit: v.financing_limit.into(),
        }
    }
}

impl ToFFI for CCouponIpoCommissionOwned {
    type FFIType = CCouponIpoCommission;

    fn to_ffi_type(&self) -> Self::FFIType {
        CCouponIpoCommission {
            bz_types: self.bz_types.to_ffi_type(),
            num_bz_types: self.bz_types.len(),
            bz_types_raw: self.bz_types_raw.to_ffi_type(),
            num_bz_types_raw: self.bz_types_raw.len(),
            counter_id: self.counter_id.to_ffi_type(),
            counter_name: self.counter_name.to_ffi_type(),
            financing_limit: self.financing_limit.to_ffi_type(),
        }
    }
}

/// Terms of an IPO interest coupon (`ipo_interest`)
#[repr(C)]
pub struct CCouponIpoInterest {
    /// Annual interest discount rate, as returned by the server (the app
    /// computes the daily discount as rate × financed amount / 365)
    pub discount_rate: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CCouponIpoInterestOwned {
    discount_rate: CString,
}

impl From<CouponIpoInterest> for CCouponIpoInterestOwned {
    fn from(v: CouponIpoInterest) -> Self {
        Self {
            discount_rate: v.discount_rate.into(),
        }
    }
}

impl ToFFI for CCouponIpoInterestOwned {
    type FFIType = CCouponIpoInterest;

    fn to_ffi_type(&self) -> Self::FFIType {
        CCouponIpoInterest {
            discount_rate: self.discount_rate.to_ffi_type(),
        }
    }
}

/// Terms of a platform fee deduction coupon (`platform_fee_deduct`)
#[repr(C)]
pub struct CCouponPlatformFeeDeduct {
    /// Instrument types the coupon applies to
    pub bz_types: *const CCouponTargetType,
    /// Number of `bz_types`
    pub num_bz_types: usize,
    /// The server's numeric codes of `bz_types`, in order, including codes this
    /// SDK does not know (which are `Unknown` in `bz_types`)
    pub bz_types_raw: *const i32,
    /// Number of `bz_types_raw`
    pub num_bz_types_raw: usize,
    /// Counter ID
    pub counter_id: *const c_char,
    /// Counter name
    pub counter_name: *const c_char,
    /// Markets the coupon applies to
    pub markets: *const *const c_char,
    /// Number of `markets`
    pub num_markets: usize,
    /// Number of times used
    pub usage_count: i64,
    /// Number of uses left
    pub lave_usage_count: i64,
    /// Whether the coupon can be combined with other coupons
    pub superposition: bool,
}

#[derive(Debug)]
pub(crate) struct CCouponPlatformFeeDeductOwned {
    bz_types: CVec<CCouponTargetType>,
    bz_types_raw: CVec<i32>,
    counter_id: CString,
    counter_name: CString,
    markets: CVec<CString>,
    usage_count: i64,
    lave_usage_count: i64,
    superposition: bool,
}

impl From<CouponPlatformFeeDeduct> for CCouponPlatformFeeDeductOwned {
    fn from(v: CouponPlatformFeeDeduct) -> Self {
        Self {
            bz_types: v.bz_types.into(),
            bz_types_raw: v.bz_types_raw.into(),
            counter_id: v.counter_id.into(),
            counter_name: v.counter_name.into(),
            markets: v.markets.into(),
            usage_count: v.usage_count,
            lave_usage_count: v.lave_usage_count,
            superposition: v.superposition,
        }
    }
}

impl ToFFI for CCouponPlatformFeeDeductOwned {
    type FFIType = CCouponPlatformFeeDeduct;

    fn to_ffi_type(&self) -> Self::FFIType {
        CCouponPlatformFeeDeduct {
            bz_types: self.bz_types.to_ffi_type(),
            num_bz_types: self.bz_types.len(),
            bz_types_raw: self.bz_types_raw.to_ffi_type(),
            num_bz_types_raw: self.bz_types_raw.len(),
            counter_id: self.counter_id.to_ffi_type(),
            counter_name: self.counter_name.to_ffi_type(),
            markets: self.markets.to_ffi_type(),
            num_markets: self.markets.len(),
            usage_count: self.usage_count,
            lave_usage_count: self.lave_usage_count,
            superposition: self.superposition,
        }
    }
}

/// An optional counter of a stock coupon
#[repr(C)]
pub struct CCouponStockCounter {
    /// Counter ID
    pub counter_id: *const c_char,
    /// Shares, as returned by the server
    pub shares: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CCouponStockCounterOwned {
    counter_id: CString,
    shares: CString,
}

impl From<CouponStockCounter> for CCouponStockCounterOwned {
    fn from(v: CouponStockCounter) -> Self {
        Self {
            counter_id: v.counter_id.into(),
            shares: v.shares.into(),
        }
    }
}

impl ToFFI for CCouponStockCounterOwned {
    type FFIType = CCouponStockCounter;

    fn to_ffi_type(&self) -> Self::FFIType {
        CCouponStockCounter {
            counter_id: self.counter_id.to_ffi_type(),
            shares: self.shares.to_ffi_type(),
        }
    }
}

/// Fallback of a stock coupon
#[repr(C)]
pub struct CCouponStockFallback {
    /// Whether the fallback is enabled
    pub enabled: bool,
    /// Fallback type (the server's `type`), as returned by the server
    pub fallback_type: *const c_char,
    /// Counter ID
    pub counter_id: *const c_char,
    /// Shares, as returned by the server
    pub shares: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CCouponStockFallbackOwned {
    enabled: bool,
    fallback_type: CString,
    counter_id: CString,
    shares: CString,
}

impl From<CouponStockFallback> for CCouponStockFallbackOwned {
    fn from(v: CouponStockFallback) -> Self {
        Self {
            enabled: v.enabled,
            fallback_type: v.fallback_type.into(),
            counter_id: v.counter_id.into(),
            shares: v.shares.into(),
        }
    }
}

impl ToFFI for CCouponStockFallbackOwned {
    type FFIType = CCouponStockFallback;

    fn to_ffi_type(&self) -> Self::FFIType {
        CCouponStockFallback {
            enabled: self.enabled,
            fallback_type: self.fallback_type.to_ffi_type(),
            counter_id: self.counter_id.to_ffi_type(),
            shares: self.shares.to_ffi_type(),
        }
    }
}

/// Terms of a stock coupon (`stock`)
#[repr(C)]
pub struct CCouponStock {
    /// Counter ID
    pub counter_id: *const c_char,
    /// Counter name
    pub counter_name: *const c_char,
    /// Amount (null when not set)
    pub amount: *const CDecimal,
    /// Redemption type (`by_card_value` or `shares_number`), as returned by the
    /// server
    pub redemption_type: *const c_char,
    /// Delivery way (`card_exchange` or `account_transfer`), as returned by the
    /// server
    pub send_way: *const c_char,
    /// Optional counters
    pub optional_counters: *const CCouponStockCounter,
    /// Number of `optional_counters`
    pub num_optional_counters: usize,
    /// Fallback
    pub fallback: CCouponStockFallback,
}

#[derive(Debug)]
pub(crate) struct CCouponStockOwned {
    counter_id: CString,
    counter_name: CString,
    amount: Option<CDecimal>,
    redemption_type: CString,
    send_way: CString,
    optional_counters: CVec<CCouponStockCounterOwned>,
    fallback: CCouponStockFallbackOwned,
}

impl From<CouponStock> for CCouponStockOwned {
    fn from(v: CouponStock) -> Self {
        Self {
            counter_id: v.counter_id.into(),
            counter_name: v.counter_name.into(),
            amount: v.amount.map(Into::into),
            redemption_type: v.redemption_type.into(),
            send_way: v.send_way.into(),
            optional_counters: v.optional_counters.into(),
            fallback: v.fallback.into(),
        }
    }
}

impl ToFFI for CCouponStockOwned {
    type FFIType = CCouponStock;

    fn to_ffi_type(&self) -> Self::FFIType {
        CCouponStock {
            counter_id: self.counter_id.to_ffi_type(),
            counter_name: self.counter_name.to_ffi_type(),
            amount: self
                .amount
                .as_ref()
                .map(ToFFI::to_ffi_type)
                .unwrap_or(std::ptr::null()),
            redemption_type: self.redemption_type.to_ffi_type(),
            send_way: self.send_way.to_ffi_type(),
            optional_counters: self.optional_counters.to_ffi_type(),
            num_optional_counters: self.optional_counters.len(),
            fallback: self.fallback.to_ffi_type(),
        }
    }
}

/// One `amounts` entry of [`CCouponStockFragment`]
#[repr(C)]
pub struct CCouponStockFragmentAmount {
    /// Key, as returned by the server
    pub key: *const c_char,
    /// Value, as returned by the server
    pub value: *const c_char,
}

#[derive(Debug)]
pub(crate) struct CCouponStockFragmentAmountOwned {
    key: CString,
    value: CString,
}

impl From<(String, String)> for CCouponStockFragmentAmountOwned {
    fn from((key, value): (String, String)) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
        }
    }
}

impl ToFFI for CCouponStockFragmentAmountOwned {
    type FFIType = CCouponStockFragmentAmount;

    fn to_ffi_type(&self) -> Self::FFIType {
        CCouponStockFragmentAmount {
            key: self.key.to_ffi_type(),
            value: self.value.to_ffi_type(),
        }
    }
}

/// Terms of a stock fragment coupon (`stock_fragment`)
#[repr(C)]
pub struct CCouponStockFragment {
    /// Counter ID
    pub counter_id: *const c_char,
    /// Counter name
    pub counter_name: *const c_char,
    /// Market
    pub market: *const c_char,
    /// Markets
    pub markets: *const *const c_char,
    /// Number of `markets`
    pub num_markets: usize,
    /// Instrument type the coupon applies to
    pub underlying_type: CCouponTargetType,
    /// The server's numeric code of `underlying_type`, also when it is a code
    /// this SDK does not know (`underlying_type` is then `Unknown`); null when
    /// the server sent none
    pub underlying_type_raw: *const i32,
    /// Amounts, as returned by the server, key-sorted
    pub amounts: *const CCouponStockFragmentAmount,
    /// Number of `amounts`
    pub num_amounts: usize,
}

#[derive(Debug)]
pub(crate) struct CCouponStockFragmentOwned {
    counter_id: CString,
    counter_name: CString,
    market: CString,
    markets: CVec<CString>,
    underlying_type: CouponTargetType,
    underlying_type_raw: Option<i32>,
    amounts: CVec<CCouponStockFragmentAmountOwned>,
}

impl From<CouponStockFragment> for CCouponStockFragmentOwned {
    fn from(v: CouponStockFragment) -> Self {
        Self {
            counter_id: v.counter_id.into(),
            counter_name: v.counter_name.into(),
            market: v.market.into(),
            markets: v.markets.into(),
            underlying_type: v.underlying_type,
            underlying_type_raw: v.underlying_type_raw,
            // `BTreeMap` iterates in key order, so the array is key-sorted.
            amounts: v.amounts.into_iter().collect::<Vec<_>>().into(),
        }
    }
}

impl ToFFI for CCouponStockFragmentOwned {
    type FFIType = CCouponStockFragment;

    fn to_ffi_type(&self) -> Self::FFIType {
        CCouponStockFragment {
            counter_id: self.counter_id.to_ffi_type(),
            counter_name: self.counter_name.to_ffi_type(),
            market: self.market.to_ffi_type(),
            markets: self.markets.to_ffi_type(),
            num_markets: self.markets.len(),
            underlying_type: self.underlying_type.into(),
            underlying_type_raw: self
                .underlying_type_raw
                .as_ref()
                .map(|v| v as *const i32)
                .unwrap_or(std::ptr::null()),
            amounts: self.amounts.to_ffi_type(),
            num_amounts: self.amounts.len(),
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
    /// The server's numeric code of `card_status`, also when it is a code
    /// this SDK does not know (`card_status` is then `Unknown`); null when the
    /// server sent none
    pub card_status_raw: *const i32,
    /// Start of validity as a unix timestamp (null when not set)
    pub start_time: *const i64,
    /// End of validity (none when not set or when the coupon never expires;
    /// see `no_expiry`) as a unix timestamp (null when not set)
    pub end_time: *const i64,
    /// Whether the coupon never expires (the server's end time is in or after
    /// 2100)
    pub no_expiry: bool,
    /// Activation time (none when not activated) as a unix timestamp (null when
    /// not set)
    pub activate_time: *const i64,
    /// Amount (null when not set)
    pub amount: *const CDecimal,
    /// Currency
    pub currency: *const c_char,
    /// Trading sessions the coupon applies to, as display text
    pub available_trade_session: *const c_char,
    /// Trading sessions the coupon applies to, as returned by the server
    pub trade_sessions: *const *const c_char,
    /// Number of `trade_sessions`
    pub num_trade_sessions: usize,
    /// Subscription ID, as returned by the server
    pub subscription_id: *const c_char,
    /// Terms of an AI benefit coupon (null for other coupon types)
    pub ai_benefit: *const CCouponAiBenefit,
    /// Terms of a cash coupon (null for other coupon types)
    pub cash: *const CCouponCash,
    /// Terms of a coffee coupon (null for other coupon types)
    pub coffee_coupon: *const CCouponCoffee,
    /// Terms of a commission coupon (null for other coupon types)
    pub commission: *const CCouponCommission,
    /// Terms of a crypto coupon (null for other coupon types)
    pub crypto: *const CCouponCrypto,
    /// Terms of a finance interest coupon (null for other coupon types)
    pub finance_interest: *const CCouponFinanceInterest,
    /// Terms of a fund deduction coupon (null for other coupon types)
    pub fund_deduct: *const CCouponFundDeduct,
    /// Terms of a fundamental-data coupon (null for other coupon types)
    pub fundamental: *const CCouponQuote,
    /// Terms of an IPO coupon (null for other coupon types)
    pub ipo: *const CCouponIpo,
    /// Terms of an IPO commission coupon (null for other coupon types)
    pub ipo_commission: *const CCouponIpoCommission,
    /// Terms of an IPO interest coupon (null for other coupon types)
    pub ipo_interest: *const CCouponIpoInterest,
    /// Terms of a platform fee deduction coupon (null for other coupon types)
    pub platform_fee_deduct: *const CCouponPlatformFeeDeduct,
    /// Terms of a quote (market data) coupon (null for other coupon types)
    pub quote: *const CCouponQuote,
    /// Terms of a stock coupon (null for other coupon types)
    pub stock: *const CCouponStock,
    /// Terms of a stock fragment coupon (null for other coupon types)
    pub stock_fragment: *const CCouponStockFragment,
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
    card_status_raw: Option<i32>,
    start_time: Option<i64>,
    end_time: Option<i64>,
    no_expiry: bool,
    activate_time: Option<i64>,
    amount: Option<CDecimal>,
    currency: CString,
    available_trade_session: CString,
    trade_sessions: CVec<CString>,
    subscription_id: CString,
    ai_benefit: COption<CCouponAiBenefitOwned>,
    cash: COption<CCouponCashOwned>,
    coffee_coupon: COption<CCouponCoffeeOwned>,
    commission: COption<CCouponCommissionOwned>,
    crypto: COption<CCouponCryptoOwned>,
    finance_interest: COption<CCouponFinanceInterestOwned>,
    fund_deduct: COption<CCouponFundDeductOwned>,
    fundamental: COption<CCouponQuoteOwned>,
    ipo: COption<CCouponIpoOwned>,
    ipo_commission: COption<CCouponIpoCommissionOwned>,
    ipo_interest: COption<CCouponIpoInterestOwned>,
    platform_fee_deduct: COption<CCouponPlatformFeeDeductOwned>,
    quote: COption<CCouponQuoteOwned>,
    stock: COption<CCouponStockOwned>,
    stock_fragment: COption<CCouponStockFragmentOwned>,
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
            card_status_raw: v.card_status_raw,
            start_time: v.start_time.map(OffsetDateTime::unix_timestamp),
            end_time: v.end_time.map(OffsetDateTime::unix_timestamp),
            no_expiry: v.no_expiry,
            activate_time: v.activate_time.map(OffsetDateTime::unix_timestamp),
            amount: v.amount.map(Into::into),
            currency: v.currency.into(),
            available_trade_session: v.available_trade_session.into(),
            trade_sessions: v.trade_sessions.into(),
            subscription_id: v.subscription_id.into(),
            ai_benefit: v.ai_benefit.into(),
            cash: v.cash.into(),
            coffee_coupon: v.coffee_coupon.into(),
            commission: v.commission.into(),
            crypto: v.crypto.into(),
            finance_interest: v.finance_interest.into(),
            fund_deduct: v.fund_deduct.into(),
            fundamental: v.fundamental.into(),
            ipo: v.ipo.into(),
            ipo_commission: v.ipo_commission.into(),
            ipo_interest: v.ipo_interest.into(),
            platform_fee_deduct: v.platform_fee_deduct.into(),
            quote: v.quote.into(),
            stock: v.stock.into(),
            stock_fragment: v.stock_fragment.into(),
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
            card_status_raw: self
                .card_status_raw
                .as_ref()
                .map(|v| v as *const i32)
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
            trade_sessions: self.trade_sessions.to_ffi_type(),
            num_trade_sessions: self.trade_sessions.len(),
            subscription_id: self.subscription_id.to_ffi_type(),
            ai_benefit: self.ai_benefit.to_ffi_type(),
            cash: self.cash.to_ffi_type(),
            coffee_coupon: self.coffee_coupon.to_ffi_type(),
            commission: self.commission.to_ffi_type(),
            crypto: self.crypto.to_ffi_type(),
            finance_interest: self.finance_interest.to_ffi_type(),
            fund_deduct: self.fund_deduct.to_ffi_type(),
            fundamental: self.fundamental.to_ffi_type(),
            ipo: self.ipo.to_ffi_type(),
            ipo_commission: self.ipo_commission.to_ffi_type(),
            ipo_interest: self.ipo_interest.to_ffi_type(),
            platform_fee_deduct: self.platform_fee_deduct.to_ffi_type(),
            quote: self.quote.to_ffi_type(),
            stock: self.stock.to_ffi_type(),
            stock_fragment: self.stock_fragment.to_ffi_type(),
        }
    }
}

impl ToFFI for CCouponTargetType {
    type FFIType = CCouponTargetType;

    #[inline]
    fn to_ffi_type(&self) -> Self::FFIType {
        *self
    }
}

impl ToFFI for CIpoCouponTargetType {
    type FFIType = CIpoCouponTargetType;

    #[inline]
    fn to_ffi_type(&self) -> Self::FFIType {
        *self
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
    /// The server's numeric code of `state_code`, also when it is a code
    /// this SDK does not know (`state_code` is then `Unknown`); null when the
    /// server sent none
    pub state_code_raw: *const i32,
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
    /// Number of receipts uploaded with the deposit
    pub voucher_count: i64,
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
    state_code_raw: Option<i32>,
    sub_state_code: i32,
    bank_operation_type_id: CString,
    bank_operation_type_name: CString,
    reference_no: CString,
    can_cancel: bool,
    user_bank_swift_code: CString,
    voucher_count: i64,
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
            state_code_raw: v.state_code_raw,
            sub_state_code: v.sub_state_code,
            bank_operation_type_id: v.bank_operation_type_id.into(),
            bank_operation_type_name: v.bank_operation_type_name.into(),
            reference_no: v.reference_no.into(),
            can_cancel: v.can_cancel,
            user_bank_swift_code: v.user_bank_swift_code.into(),
            voucher_count: v.voucher_count,
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
            state_code_raw: self
                .state_code_raw
                .as_ref()
                .map(|v| v as *const i32)
                .unwrap_or(std::ptr::null()),
            sub_state_code: self.sub_state_code,
            bank_operation_type_id: self.bank_operation_type_id.to_ffi_type(),
            bank_operation_type_name: self.bank_operation_type_name.to_ffi_type(),
            reference_no: self.reference_no.to_ffi_type(),
            can_cancel: self.can_cancel,
            user_bank_swift_code: self.user_bank_swift_code.to_ffi_type(),
            voucher_count: self.voucher_count,
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
    /// The server's numeric code of `state_code`, also when it is a code
    /// this SDK does not know (`state_code` is then `Unknown`); null when the
    /// server sent none
    pub state_code_raw: *const i32,
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
    state_code_raw: Option<i32>,
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
            state_code_raw: v.state_code_raw,
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
            state_code_raw: self
                .state_code_raw
                .as_ref()
                .map(|v| v as *const i32)
                .unwrap_or(std::ptr::null()),
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
    /// Page size (can be null; max 20, larger values are sent as 20)
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
