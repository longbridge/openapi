use longbridge_c_macros::CEnum;

/// Verification status of a bank card
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::asset::BankCardStatus")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CBankCardStatus {
    /// Unknown (an undocumented code)
    #[c(remote = "Unknown")]
    BankCardStatusUnknown,
    /// Not verified
    #[c(remote = "Unverified")]
    BankCardStatusUnverified,
    /// Under review
    #[c(remote = "UnderReview")]
    BankCardStatusUnderReview,
    /// Verified
    #[c(remote = "Verified")]
    BankCardStatusVerified,
}

/// Witness (in-person / video verification) state of a bank card
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::asset::BankCardWitnessState")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CBankCardWitnessState {
    /// Unknown (an undocumented code)
    #[c(remote = "Unknown")]
    BankCardWitnessStateUnknown,
    /// Witnessing not required
    #[c(remote = "NotRequired")]
    BankCardWitnessStateNotRequired,
    /// Witnessing pending
    #[c(remote = "Pending")]
    BankCardWitnessStatePending,
    /// Witnessed
    #[c(remote = "Witnessed")]
    BankCardWitnessStateWitnessed,
}

/// Binding state of a bank card
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::asset::BankCardBindState")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CBankCardBindState {
    /// Unknown (an undocumented code)
    #[c(remote = "Unknown")]
    BankCardBindStateUnknown,
    /// Binding in progress
    #[c(remote = "Binding")]
    BankCardBindStateBinding,
    /// Bound
    #[c(remote = "Bound")]
    BankCardBindStateBound,
    /// Binding failed
    #[c(remote = "Failed")]
    BankCardBindStateFailed,
}

/// Status of a bank-card authorization or of an eDDA application
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::asset::AuthorizationStatus")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CAuthorizationStatus {
    /// Unknown (an undocumented code)
    #[c(remote = "Unknown")]
    AuthorizationStatusUnknown,
    /// Not authorized
    #[c(remote = "Unauthorized")]
    AuthorizationStatusUnauthorized,
    /// Authorized
    #[c(remote = "Authorized")]
    AuthorizationStatusAuthorized,
    /// Authorization in progress
    #[c(remote = "Authorizing")]
    AuthorizationStatusAuthorizing,
    /// Authorization failed
    #[c(remote = "Failed")]
    AuthorizationStatusFailed,
}

/// State of a stock transfer-in application
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::asset::StockTransferInState")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CStockTransferInState {
    /// Unknown (an undocumented code)
    #[c(remote = "Unknown")]
    StockTransferInStateUnknown,
    /// Submitted
    #[c(remote = "Submitted")]
    StockTransferInStateSubmitted,
    /// Cancelled
    #[c(remote = "Cancelled")]
    StockTransferInStateCancelled,
    /// Completed
    #[c(remote = "Completed")]
    StockTransferInStateCompleted,
    /// Processing
    #[c(remote = "Processing")]
    StockTransferInStateProcessing,
    /// Partially completed
    #[c(remote = "PartiallyCompleted")]
    StockTransferInStatePartiallyCompleted,
    /// Failed
    #[c(remote = "Failed")]
    StockTransferInStateFailed,
}

/// State of a stock transfer-out application
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::asset::StockTransferOutState")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CStockTransferOutState {
    /// Unknown (an undocumented code)
    #[c(remote = "Unknown")]
    StockTransferOutStateUnknown,
    /// Submitted (the only state that can still be cancelled)
    #[c(remote = "Submitted")]
    StockTransferOutStateSubmitted,
    /// Processing
    #[c(remote = "Processing")]
    StockTransferOutStateProcessing,
    /// Executing
    #[c(remote = "Executing")]
    StockTransferOutStateExecuting,
    /// Completed
    #[c(remote = "Completed")]
    StockTransferOutStateCompleted,
    /// Cancelled
    #[c(remote = "Cancelled")]
    StockTransferOutStateCancelled,
    /// Partially completed
    #[c(remote = "PartiallyCompleted")]
    StockTransferOutStatePartiallyCompleted,
    /// Failed
    #[c(remote = "Failed")]
    StockTransferOutStateFailed,
    /// Rejected
    #[c(remote = "Rejected")]
    StockTransferOutStateRejected,
}

/// State of one stock inside a stock transfer application
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::asset::TransferStockState")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CTransferStockState {
    /// Unknown (an undocumented code)
    #[c(remote = "Unknown")]
    TransferStockStateUnknown,
    /// Processing
    #[c(remote = "Processing")]
    TransferStockStateProcessing,
    /// Succeeded
    #[c(remote = "Succeeded")]
    TransferStockStateSucceeded,
    /// Failed
    #[c(remote = "Failed")]
    TransferStockStateFailed,
}

/// Type of a reward record
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::asset::RewardType")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CRewardType {
    /// Unknown (an undocumented code)
    #[c(remote = "Unknown")]
    RewardTypeUnknown,
    /// Cash reward
    #[c(remote = "Cash")]
    RewardTypeCash,
    /// Gift (prize)
    #[c(remote = "Gift")]
    RewardTypeGift,
    /// Coupon
    #[c(remote = "Coupon")]
    RewardTypeCoupon,
    /// Task reward
    #[c(remote = "Task")]
    RewardTypeTask,
}

/// Status of a reward record
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::asset::RewardStatus")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CRewardStatus {
    /// Unknown (an undocumented code)
    #[c(remote = "Unknown")]
    RewardStatusUnknown,
    /// Upcoming
    #[c(remote = "Upcoming")]
    RewardStatusUpcoming,
    /// Cancelled
    #[c(remote = "Cancelled")]
    RewardStatusCancelled,
    /// The user exited the activity
    #[c(remote = "ExitedActivity")]
    RewardStatusExitedActivity,
    /// Not claimed yet
    #[c(remote = "Unclaimed")]
    RewardStatusUnclaimed,
    /// Being issued
    #[c(remote = "Issuing")]
    RewardStatusIssuing,
    /// Issued
    #[c(remote = "Issued")]
    RewardStatusIssued,
    /// Issuing failed
    #[c(remote = "Failed")]
    RewardStatusFailed,
    /// Exchanged
    #[c(remote = "Exchanged")]
    RewardStatusExchanged,
    /// Expired
    #[c(remote = "Expired")]
    RewardStatusExpired,
    /// Refunded
    #[c(remote = "Refunded")]
    RewardStatusRefunded,
}

/// Delivery status of an issued prize
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::asset::RewardDeliverStatus")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CRewardDeliverStatus {
    /// Unknown (an undocumented code, or no delivery status)
    #[c(remote = "Unknown")]
    RewardDeliverStatusUnknown,
    /// Not activated
    #[c(remote = "NotActivated")]
    RewardDeliverStatusNotActivated,
    /// Activated, or shipping
    #[c(remote = "ActivatedOrShipping")]
    RewardDeliverStatusActivatedOrShipping,
    /// Consumed
    #[c(remote = "Consumed")]
    RewardDeliverStatusConsumed,
    /// E-voucher activating
    #[c(remote = "EVoucherActivating")]
    RewardDeliverStatusEVoucherActivating,
    /// E-voucher expired before activation
    #[c(remote = "EVoucherExpired")]
    RewardDeliverStatusEVoucherExpired,
    /// E-voucher invalid
    #[c(remote = "EVoucherInvalid")]
    RewardDeliverStatusEVoucherInvalid,
    /// Shipping address missing
    #[c(remote = "AddressMissing")]
    RewardDeliverStatusAddressMissing,
    /// Shipping address filled
    #[c(remote = "AddressFilled")]
    RewardDeliverStatusAddressFilled,
    /// Shipping
    #[c(remote = "Shipping")]
    RewardDeliverStatusShipping,
}

/// Status of a coupon
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::asset::CouponStatus")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CCouponStatus {
    /// Unknown (an undocumented code)
    #[c(remote = "Unknown")]
    CouponStatusUnknown,
    /// Unused
    #[c(remote = "Unused")]
    CouponStatusUnused,
    /// Used
    #[c(remote = "Used")]
    CouponStatusUsed,
    /// Expired
    #[c(remote = "Expired")]
    CouponStatusExpired,
    /// Being redeemed
    #[c(remote = "Redeeming")]
    CouponStatusRedeeming,
    /// In use
    #[c(remote = "InUse")]
    CouponStatusInUse,
    /// Processing
    #[c(remote = "Processing")]
    CouponStatusProcessing,
}

/// State of a deposit record
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::asset::DepositState")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CDepositState {
    /// Unknown (an undocumented code)
    #[c(remote = "Unknown")]
    DepositStateUnknown,
    /// Processing
    #[c(remote = "Processing")]
    DepositStateProcessing,
    /// Credited to the account
    #[c(remote = "Credited")]
    DepositStateCredited,
    /// Failed
    #[c(remote = "Failed")]
    DepositStateFailed,
    /// Cancelled
    #[c(remote = "Cancelled")]
    DepositStateCancelled,
    /// Invalidated
    #[c(remote = "Invalidated")]
    DepositStateInvalidated,
    /// Rejected
    #[c(remote = "Rejected")]
    DepositStateRejected,
}

/// State of a withdrawal record
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::asset::WithdrawalState")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CWithdrawalState {
    /// Unknown (an undocumented code)
    #[c(remote = "Unknown")]
    WithdrawalStateUnknown,
    /// Submitted
    #[c(remote = "Submitted")]
    WithdrawalStateSubmitted,
    /// Processing
    #[c(remote = "Processing")]
    WithdrawalStateProcessing,
    /// Rejected
    #[c(remote = "Rejected")]
    WithdrawalStateRejected,
    /// Cancelled
    #[c(remote = "Cancelled")]
    WithdrawalStateCancelled,
    /// Remitted
    #[c(remote = "Remitted")]
    WithdrawalStateRemitted,
    /// Failed
    #[c(remote = "Failed")]
    WithdrawalStateFailed,
    /// Refunded
    #[c(remote = "Refunded")]
    WithdrawalStateRefunded,
    /// Approved, awaiting payout
    #[c(remote = "ApprovedAwaitingPayout")]
    WithdrawalStateApprovedAwaitingPayout,
    /// Large-amount review
    #[c(remote = "LargeAmountReview")]
    WithdrawalStateLargeAmountReview,
    /// Change pending approval
    #[c(remote = "ChangePendingApproval")]
    WithdrawalStateChangePendingApproval,
    /// Deletion pending approval
    #[c(remote = "DeletionPendingApproval")]
    WithdrawalStateDeletionPendingApproval,
    /// Deleted
    #[c(remote = "Deleted")]
    WithdrawalStateDeleted,
    /// Rejection pending approval
    #[c(remote = "RejectionPendingApproval")]
    WithdrawalStateRejectionPendingApproval,
    /// Awaiting confirmation
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
