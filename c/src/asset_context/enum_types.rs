use longbridge_c_macros::CEnum;

/// Verification status of a bank card
///
/// The numeric values of this enum are not the server codes; each
/// member's description names its server code.
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::asset::BankCardStatus")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CBankCardStatus {
    /// Unknown: an undocumented code (never sent to the server)
    #[c(remote = "Unknown")]
    BankCardStatusUnknown,
    /// Not verified (server code 0)
    #[c(remote = "Unverified")]
    BankCardStatusUnverified,
    /// Under review (server code 1)
    #[c(remote = "UnderReview")]
    BankCardStatusUnderReview,
    /// Verified (server code 2)
    #[c(remote = "Verified")]
    BankCardStatusVerified,
}

/// Witness (in-person / video verification) state of a bank card
///
/// The numeric values of this enum are not the server codes; each
/// member's description names its server code.
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::asset::BankCardWitnessState")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CBankCardWitnessState {
    /// Unknown: an undocumented code (never sent to the server)
    #[c(remote = "Unknown")]
    BankCardWitnessStateUnknown,
    /// Witnessing not required (server code 1)
    #[c(remote = "NotRequired")]
    BankCardWitnessStateNotRequired,
    /// Witnessing pending (server code 2)
    #[c(remote = "Pending")]
    BankCardWitnessStatePending,
    /// Witnessed (server code 3)
    #[c(remote = "Witnessed")]
    BankCardWitnessStateWitnessed,
}

/// Binding state of a bank card
///
/// The numeric values of this enum are not the server codes; each
/// member's description names its server code.
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::asset::BankCardBindState")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CBankCardBindState {
    /// Unknown: an undocumented code (never sent to the server)
    #[c(remote = "Unknown")]
    BankCardBindStateUnknown,
    /// Binding in progress (server code 0)
    #[c(remote = "Binding")]
    BankCardBindStateBinding,
    /// Bound (server code 1)
    #[c(remote = "Bound")]
    BankCardBindStateBound,
    /// Binding failed (server code 2)
    #[c(remote = "Failed")]
    BankCardBindStateFailed,
}

/// Status of a bank-card authorization or of an eDDA application
///
/// The numeric values of this enum are not the server codes; each
/// member's description names its server code.
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::asset::AuthorizationStatus")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CAuthorizationStatus {
    /// Unknown: an undocumented code (never sent to the server)
    #[c(remote = "Unknown")]
    AuthorizationStatusUnknown,
    /// Not authorized (server code 0)
    #[c(remote = "Unauthorized")]
    AuthorizationStatusUnauthorized,
    /// Authorized (server code 1)
    #[c(remote = "Authorized")]
    AuthorizationStatusAuthorized,
    /// Authorization in progress (server code 2)
    #[c(remote = "Authorizing")]
    AuthorizationStatusAuthorizing,
    /// Authorization failed (server code 3)
    #[c(remote = "Failed")]
    AuthorizationStatusFailed,
}

/// State of a stock transfer-in application
///
/// The numeric values of this enum are not the server codes; each
/// member's description names its server code.
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::asset::StockTransferInState")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CStockTransferInState {
    /// Unknown: an undocumented code (never sent to the server)
    #[c(remote = "Unknown")]
    StockTransferInStateUnknown,
    /// Submitted (server code 0)
    #[c(remote = "Submitted")]
    StockTransferInStateSubmitted,
    /// Cancelled (server code 1)
    #[c(remote = "Cancelled")]
    StockTransferInStateCancelled,
    /// Completed (server code 3)
    #[c(remote = "Completed")]
    StockTransferInStateCompleted,
    /// Processing (server code 5)
    #[c(remote = "Processing")]
    StockTransferInStateProcessing,
    /// Partially completed (server code 7)
    #[c(remote = "PartiallyCompleted")]
    StockTransferInStatePartiallyCompleted,
    /// Failed (server code 8)
    #[c(remote = "Failed")]
    StockTransferInStateFailed,
}

/// State of a stock transfer-out application
///
/// The numeric values of this enum are not the server codes; each
/// member's description names its server code.
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::asset::StockTransferOutState")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CStockTransferOutState {
    /// Unknown: an undocumented code (never sent to the server)
    #[c(remote = "Unknown")]
    StockTransferOutStateUnknown,
    /// Submitted (the only state that can still be cancelled) (server code 0)
    #[c(remote = "Submitted")]
    StockTransferOutStateSubmitted,
    /// Processing (server code 1)
    #[c(remote = "Processing")]
    StockTransferOutStateProcessing,
    /// Executing (server code 2)
    #[c(remote = "Executing")]
    StockTransferOutStateExecuting,
    /// Completed (server code 3)
    #[c(remote = "Completed")]
    StockTransferOutStateCompleted,
    /// Cancelled (server code 4)
    #[c(remote = "Cancelled")]
    StockTransferOutStateCancelled,
    /// Partially completed (server code 5)
    #[c(remote = "PartiallyCompleted")]
    StockTransferOutStatePartiallyCompleted,
    /// Failed (server code 6)
    #[c(remote = "Failed")]
    StockTransferOutStateFailed,
    /// Rejected (server code 7)
    #[c(remote = "Rejected")]
    StockTransferOutStateRejected,
}

/// State of one stock inside a stock transfer application
///
/// The numeric values of this enum are not the server codes; each
/// member's description names its server code.
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::asset::TransferStockState")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CTransferStockState {
    /// Unknown: an undocumented code (never sent to the server)
    #[c(remote = "Unknown")]
    TransferStockStateUnknown,
    /// Processing (server code 0)
    #[c(remote = "Processing")]
    TransferStockStateProcessing,
    /// Succeeded (server code 1)
    #[c(remote = "Succeeded")]
    TransferStockStateSucceeded,
    /// Failed (server code 2)
    #[c(remote = "Failed")]
    TransferStockStateFailed,
}

/// Type of a reward record
///
/// The numeric values of this enum are not the server codes; each
/// member's description names its server code.
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::asset::RewardType")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CRewardType {
    /// Unknown: an undocumented code (never sent to the server)
    #[c(remote = "Unknown")]
    RewardTypeUnknown,
    /// Cash reward (server code 0)
    #[c(remote = "Cash")]
    RewardTypeCash,
    /// Gift (prize) (server code 1)
    #[c(remote = "Gift")]
    RewardTypeGift,
    /// Coupon (server code 2)
    #[c(remote = "Coupon")]
    RewardTypeCoupon,
    /// Task reward (server code 3)
    #[c(remote = "Task")]
    RewardTypeTask,
}

/// Status of a reward record
///
/// The numeric values of this enum are not the server codes; each
/// member's description names its server code.
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::asset::RewardStatus")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CRewardStatus {
    /// Unknown: an undocumented code (never sent to the server)
    #[c(remote = "Unknown")]
    RewardStatusUnknown,
    /// Upcoming (server code 0)
    #[c(remote = "Upcoming")]
    RewardStatusUpcoming,
    /// Cancelled (server code 1)
    #[c(remote = "Cancelled")]
    RewardStatusCancelled,
    /// The user exited the activity (server code 2)
    #[c(remote = "ExitedActivity")]
    RewardStatusExitedActivity,
    /// Not claimed yet (server code 8)
    #[c(remote = "Unclaimed")]
    RewardStatusUnclaimed,
    /// Being issued (server code 10)
    #[c(remote = "Issuing")]
    RewardStatusIssuing,
    /// Issued (server code 12)
    #[c(remote = "Issued")]
    RewardStatusIssued,
    /// Issuing failed (server code 13)
    #[c(remote = "Failed")]
    RewardStatusFailed,
    /// Exchanged (server code 16)
    #[c(remote = "Exchanged")]
    RewardStatusExchanged,
    /// Expired (server code 17)
    #[c(remote = "Expired")]
    RewardStatusExpired,
    /// Refunded (server code 18)
    #[c(remote = "Refunded")]
    RewardStatusRefunded,
}

/// Delivery status of an issued prize
///
/// The numeric values of this enum are not the server codes; each
/// member's description names its server code.
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::asset::RewardDeliverStatus")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CRewardDeliverStatus {
    /// Unknown: an undocumented code, or no delivery status (never sent to the
    /// server)
    #[c(remote = "Unknown")]
    RewardDeliverStatusUnknown,
    /// Not activated (server code 0)
    #[c(remote = "NotActivated")]
    RewardDeliverStatusNotActivated,
    /// Activated, or shipping (server code 1)
    #[c(remote = "ActivatedOrShipping")]
    RewardDeliverStatusActivatedOrShipping,
    /// Consumed (server code 2)
    #[c(remote = "Consumed")]
    RewardDeliverStatusConsumed,
    /// E-voucher activating (server code 5)
    #[c(remote = "EVoucherActivating")]
    RewardDeliverStatusEVoucherActivating,
    /// E-voucher expired before activation (server code 6)
    #[c(remote = "EVoucherExpired")]
    RewardDeliverStatusEVoucherExpired,
    /// E-voucher invalid (server code 7)
    #[c(remote = "EVoucherInvalid")]
    RewardDeliverStatusEVoucherInvalid,
    /// Shipping address missing (server code 8)
    #[c(remote = "AddressMissing")]
    RewardDeliverStatusAddressMissing,
    /// Shipping address filled (server code 9)
    #[c(remote = "AddressFilled")]
    RewardDeliverStatusAddressFilled,
    /// Shipping (server code 10)
    #[c(remote = "Shipping")]
    RewardDeliverStatusShipping,
}

/// Instrument type a coupon applies to
///
/// The numeric values of this enum are not the server codes; each
/// member's description names its server code.
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::asset::CouponTargetType")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CCouponTargetType {
    /// Unknown: an undocumented code (never sent to the server)
    #[c(remote = "Unknown")]
    CouponTargetTypeUnknown,
    /// Stocks (server code 0, `StockTrade`)
    #[c(remote = "Stock")]
    CouponTargetTypeStock,
    /// US stock options (server code 1, `UsStockOptions`)
    #[c(remote = "UsOption")]
    CouponTargetTypeUsOption,
    /// HK warrants (server code 2, `HkStockWarrant`)
    #[c(remote = "HkWarrant")]
    CouponTargetTypeHkWarrant,
    /// ETF (server code 3)
    #[c(remote = "Etf")]
    CouponTargetTypeEtf,
    /// LOF (server code 4)
    #[c(remote = "Lof")]
    CouponTargetTypeLof,
}

/// IPO subscription type an IPO commission coupon applies to
///
/// The numeric values of this enum are not the server codes; each
/// member's description names its server code.
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::asset::IpoCouponTargetType")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CIpoCouponTargetType {
    /// Unknown: unset or an undocumented code (never sent to the server)
    #[c(remote = "Unknown")]
    IpoCouponTargetTypeUnknown,
    /// Financing (margin) IPO subscription (server code 1, `FinancingIPO`)
    #[c(remote = "Financing")]
    IpoCouponTargetTypeFinancing,
    /// Cash IPO subscription (server code 2, `CashIPO`)
    #[c(remote = "Cash")]
    IpoCouponTargetTypeCash,
}

/// The server's `unlimited` code of a commission coupon
///
/// The numeric values of this enum are not the server codes; each
/// member's description names its server code.
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::asset::CommissionUnlimitedType")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CCommissionUnlimitedType {
    /// Unknown: an undocumented code (never sent to the server)
    #[c(remote = "Unknown")]
    CommissionUnlimitedTypeUnknown,
    /// `Proportionate` (server code 0)
    #[c(remote = "Proportionate")]
    CommissionUnlimitedTypeProportionate,
    /// `Full` (server code 1)
    #[c(remote = "Full")]
    CommissionUnlimitedTypeFull,
    /// `Lifetime` (server code 2)
    #[c(remote = "Lifetime")]
    CommissionUnlimitedTypeLifetime,
    /// `ByAmountPer` (server code 6)
    #[c(remote = "ByAmountPer")]
    CommissionUnlimitedTypeByAmountPer,
    /// `ByRatePer` (server code 7)
    #[c(remote = "ByRatePer")]
    CommissionUnlimitedTypeByRatePer,
}

/// Investor type a quote coupon applies to
///
/// The numeric values of this enum are not the server codes; each
/// member's description names its server code.
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::asset::QuoteInvestorType")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CQuoteInvestorType {
    /// Unknown: an undocumented code (never sent to the server)
    #[c(remote = "Unknown")]
    QuoteInvestorTypeUnknown,
    /// Any investor (server code 0, `INVESTOR_TYPE_ANY`)
    #[c(remote = "Any")]
    QuoteInvestorTypeAny,
    /// Non-professional investors (server code 1,
    /// `INVESTOR_TYPE_NON_PROFESSIONAL`)
    #[c(remote = "NonProfessional")]
    QuoteInvestorTypeNonProfessional,
    /// Professional investors (server code 2, `INVESTOR_TYPE_PROFESSIONAL`)
    #[c(remote = "Professional")]
    QuoteInvestorTypeProfessional,
}

/// Status of a coupon
///
/// The numeric values of this enum are not the server codes; each
/// member's description names its server code.
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::asset::CouponStatus")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CCouponStatus {
    /// Unknown: an undocumented code (never sent to the server)
    #[c(remote = "Unknown")]
    CouponStatusUnknown,
    /// Unused (server code 0)
    #[c(remote = "Unused")]
    CouponStatusUnused,
    /// Used (server code 1)
    #[c(remote = "Used")]
    CouponStatusUsed,
    /// Expired (server code 2)
    #[c(remote = "Expired")]
    CouponStatusExpired,
    /// Being redeemed (server code 3)
    #[c(remote = "Redeeming")]
    CouponStatusRedeeming,
    /// In use (server code 4)
    #[c(remote = "InUse")]
    CouponStatusInUse,
    /// Processing (server code 5)
    #[c(remote = "Processing")]
    CouponStatusProcessing,
}

/// State of a deposit record
///
/// The numeric values of this enum are not the server codes; each
/// member's description names its server code.
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::asset::DepositState")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CDepositState {
    /// Unknown: an undocumented code (never sent to the server)
    #[c(remote = "Unknown")]
    DepositStateUnknown,
    /// Processing (server code 0)
    #[c(remote = "Processing")]
    DepositStateProcessing,
    /// Credited to the account (server code 1)
    #[c(remote = "Credited")]
    DepositStateCredited,
    /// Failed (server code 2)
    #[c(remote = "Failed")]
    DepositStateFailed,
    /// Cancelled (server code 3)
    #[c(remote = "Cancelled")]
    DepositStateCancelled,
    /// Invalidated (server code 4)
    #[c(remote = "Invalidated")]
    DepositStateInvalidated,
    /// Rejected (server code 5)
    #[c(remote = "Rejected")]
    DepositStateRejected,
}

/// State of a withdrawal record
///
/// The numeric values of this enum are not the server codes; each
/// member's description names its server code.
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::asset::WithdrawalState")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CWithdrawalState {
    /// Unknown: an undocumented code (never sent to the server)
    #[c(remote = "Unknown")]
    WithdrawalStateUnknown,
    /// Submitted (server code 0)
    #[c(remote = "Submitted")]
    WithdrawalStateSubmitted,
    /// Processing (server code 1)
    #[c(remote = "Processing")]
    WithdrawalStateProcessing,
    /// Rejected (server code 2)
    #[c(remote = "Rejected")]
    WithdrawalStateRejected,
    /// Cancelled (server code 3)
    #[c(remote = "Cancelled")]
    WithdrawalStateCancelled,
    /// Remitted (server code 4)
    #[c(remote = "Remitted")]
    WithdrawalStateRemitted,
    /// Failed (server code 5)
    #[c(remote = "Failed")]
    WithdrawalStateFailed,
    /// Refunded (server code 6)
    #[c(remote = "Refunded")]
    WithdrawalStateRefunded,
    /// Approved, awaiting payout (server code 7)
    #[c(remote = "ApprovedAwaitingPayout")]
    WithdrawalStateApprovedAwaitingPayout,
    /// Large-amount review (server code 8)
    #[c(remote = "LargeAmountReview")]
    WithdrawalStateLargeAmountReview,
    /// Change pending approval (server code 9)
    #[c(remote = "ChangePendingApproval")]
    WithdrawalStateChangePendingApproval,
    /// Deletion pending approval (server code 10)
    #[c(remote = "DeletionPendingApproval")]
    WithdrawalStateDeletionPendingApproval,
    /// Deleted (server code 11)
    #[c(remote = "Deleted")]
    WithdrawalStateDeleted,
    /// Rejection pending approval (server code 12)
    #[c(remote = "RejectionPendingApproval")]
    WithdrawalStateRejectionPendingApproval,
    /// Awaiting confirmation (server code 15)
    #[c(remote = "AwaitingConfirmation")]
    WithdrawalStateAwaitingConfirmation,
}

/// Which orders of a coupon to list
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::asset::CouponOrderType")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CCouponOrderType {
    /// Orders the coupon is locked to (in progress)
    #[c(remote = "Locked")]
    CouponOrderTypeLocked,
    /// Orders whose rebate has been settled
    #[c(remote = "Settled")]
    CouponOrderTypeSettled,
}
