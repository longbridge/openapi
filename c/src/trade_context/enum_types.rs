use longbridge_c_macros::CEnum;

/// Topic type
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::trade::TopicType")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CTopicType {
    /// Trading
    #[c(remote = "Private")]
    TopicPrivate,
}

/// Order side
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::trade::OrderSide")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum COrderSide {
    /// Unknown
    #[c(remote = "Unknown")]
    OrderSideUnknown,
    /// Buy
    #[c(remote = "Buy")]
    OrderSideBuy,
    /// Sell
    #[c(remote = "Sell")]
    OrderSideSell,
}

/// Order type
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::trade::OrderType")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum COrderType {
    /// Unknown
    #[c(remote = "Unknown")]
    OrderTypeUnknown,
    /// Limit Order
    #[c(remote = "LO")]
    OrderTypeLO,
    /// Enhanced Limit Order
    #[c(remote = "ELO")]
    OrderTypeELO,
    /// Market Order
    #[c(remote = "MO")]
    OrderTypeMO,
    /// At-auction Order
    #[c(remote = "AO")]
    OrderTypeAO,
    /// At-auction Limit Order
    #[c(remote = "ALO")]
    OrderTypeALO,
    /// Odd Lots
    #[c(remote = "ODD")]
    OrderTypeODD,
    /// Limit If Touched
    #[c(remote = "LIT")]
    OrderTypeLIT,
    /// Market If Touched
    #[c(remote = "MIT")]
    OrderTypeMIT,
    /// Trailing Limit If Touched (Trailing Amount)
    #[c(remote = "TSLPAMT")]
    OrderTypeTSLPAMT,
    /// Trailing Limit If Touched (Trailing Percent)
    #[c(remote = "TSLPPCT")]
    OrderTypeTSLPPCT,
    /// Trailing Market If Touched (Trailing Amount)
    #[c(remote = "TSMAMT")]
    OrderTypeTSMAMT,
    /// Trailing Market If Touched (Trailing Percent)
    #[c(remote = "TSMPCT")]
    OrderTypeTSMPCT,
    /// Special Limit Order
    #[c(remote = "SLO")]
    OrderTypeSLO,
}

/// Order status
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::trade::OrderStatus")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum COrderStatus {
    /// Unknown
    #[c(remote = "Unknown")]
    OrderStatusUnknown,
    /// Not reported
    #[c(remote = "NotReported")]
    OrderStatusNotReported,
    /// Not reported (Replaced Order)
    #[c(remote = "ReplacedNotReported")]
    OrderStatusReplacedNotReported,
    /// Not reported (Protected Order)
    #[c(remote = "ProtectedNotReported")]
    OrderStatusProtectedNotReported,
    /// Not reported (Conditional Order)
    #[c(remote = "VarietiesNotReported")]
    OrderStatusVarietiesNotReported,
    /// Filled
    #[c(remote = "Filled")]
    OrderStatusFilled,
    /// Wait To New
    #[c(remote = "WaitToNew")]
    OrderStatusWaitToNew,
    /// New
    #[c(remote = "New")]
    OrderStatusNew,
    /// Wait To Replace
    #[c(remote = "WaitToReplace")]
    OrderStatusWaitToReplace,
    /// Pending Replace
    #[c(remote = "PendingReplace")]
    OrderStatusPendingReplace,
    /// Replaced
    #[c(remote = "Replaced")]
    OrderStatusReplaced,
    /// Partial Filled
    #[c(remote = "PartialFilled")]
    OrderStatusPartialFilled,
    /// Wait To Cancel
    #[c(remote = "WaitToCancel")]
    OrderStatusWaitToCancel,
    /// Pending Cancel
    #[c(remote = "PendingCancel")]
    OrderStatusPendingCancel,
    /// Rejected
    #[c(remote = "Rejected")]
    OrderStatusRejected,
    /// Canceled
    #[c(remote = "Canceled")]
    OrderStatusCanceled,
    /// Expired
    #[c(remote = "Expired")]
    OrderStatusExpired,
    /// Partial Withdrawal
    #[c(remote = "PartialWithdrawal")]
    OrderStatusPartialWithdrawal,
}

/// Order tag
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::trade::OrderTag")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum COrderTag {
    /// Unknown
    #[c(remote = "Unknown")]
    OrderTagUnknown,
    /// Normal Order
    #[c(remote = "Normal")]
    OrderTagNormal,
    /// Long term Order
    #[c(remote = "LongTerm")]
    OrderTagLongTerm,
    /// Grey Order
    #[c(remote = "Grey")]
    OrderTagGrey,
    /// Force Selling
    #[c(remote = "MarginCall")]
    OrderTagMarginCall,
    /// OTC
    #[c(remote = "Offline")]
    OrderTagOffline,
    /// Option Exercise Long
    #[c(remote = "Creditor")]
    OrderTagCreditor,
    /// Option Exercise Short
    #[c(remote = "Debtor")]
    OrderTagDebtor,
    /// Wavier Of Option Exercise
    #[c(remote = "NonExercise")]
    OrderTagNonExercise,
    /// Trade Allocation
    #[c(remote = "AllocatedSub")]
    OrderTagAllocatedSub,
}

/// Order tag
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::trade::TriggerStatus")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CTriggerStatus {
    /// Unknown
    #[c(remote = "Unknown")]
    TriggerStatusUnknown,
    /// Deactive
    #[c(remote = "Deactive")]
    TriggerStatusDeactive,
    /// Active
    #[c(remote = "Active")]
    TriggerStatusActive,
    /// Released
    #[c(remote = "Released")]
    TriggerStatusReleased,
}

/// Enable or disable outside regular trading hours
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::trade::OutsideRTH")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum COutsideRTH {
    /// Unknown
    #[c(remote = "Unknown")]
    OutsideRTHUnknown,
    /// Regular trading hour only
    #[c(remote = "RTHOnly")]
    OutsideRTHOnly,
    /// Any time
    #[c(remote = "AnyTime")]
    OutsideRTHAnyTime,
}

/// Time in force Type
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::trade::TimeInForceType")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CTimeInForceType {
    /// Unknown
    #[c(remote = "Unknown")]
    TimeInForceUnknown,
    /// Day Order
    #[c(remote = "Day")]
    TimeInForceDay,
    /// Good Til Canceled Order
    #[c(remote = "GoodTilCanceled")]
    TimeInForceGoodTilCanceled,
    /// Good Til Date Order
    #[c(remote = "GoodTilDate")]
    TimeInForceGoodTilDate,
}

/// Cash flow direction
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::trade::CashFlowDirection ")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CCashFlowDirection {
    /// Unknown
    #[c(remote = "Unknown")]
    CashFlowDirectionUnknown,
    /// Out
    #[c(remote = "Out")]
    CashFlowDirectionOut,
    /// In
    #[c(remote = "In")]
    CashFlowDirectionIn,
}

/// Balance type
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::trade::BalanceType")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CBalanceType {
    /// Unknown
    #[c(remote = "Unknown")]
    BalanceTypeUnknown,
    /// Cash
    #[c(remote = "Cash")]
    BalanceTypeCash,
    /// Stock
    #[c(remote = "Stock")]
    BalanceTypeStock,
    /// Fund
    #[c(remote = "Fund")]
    BalanceTypeFund,
}

/// Commission-free Status
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::trade::CommissionFreeStatus")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CCommissionFreeStatus {
    /// Unknown
    #[c(remote = "Unknown")]
    CommissionFreeStatusUnknown,
    /// None
    #[c(remote = "None")]
    CommissionFreeStatusNone,
    /// Commission-free amount to be calculated
    #[c(remote = "Calculated")]
    CommissionFreeStatusCalculated,
    /// Pending commission-free
    #[c(remote = "Pending")]
    CommissionFreeStatusPending,
    /// Commission-free applied
    #[c(remote = "Ready")]
    CommissionFreeStatusReady,
}

/// Deduction status
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::trade::DeductionStatus")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CDeductionStatus {
    /// Unknown
    #[c(remote = "Unknown")]
    DeductionStatusUnknown,
    /// Pending Settlement
    #[c(remote = "None")]
    DeductionStatusNone,
    /// Commission-free amount to be calculated
    #[c(remote = "NoData")]
    DeductionStatusNoData,
    /// Pending commission-free
    #[c(remote = "Pending")]
    DeductionStatusPending,
    /// Commission-free applied
    #[c(remote = "Done")]
    DeductionStatusDone,
}

/// Charge category code
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::trade::ChargeCategoryCode")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CChargeCategoryCode {
    /// Unknown
    #[c(remote = "Unknown")]
    ChargeCategoryCodeUnknown,
    /// Broker
    #[c(remote = "Broker")]
    ChargeCategoryCodeBroker,
    /// Third
    #[c(remote = "Third")]
    ChargeCategoryCodeThird,
}
