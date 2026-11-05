package com.longbridge.trade;

/**
 * Order status
 */
public enum OrderStatus {
    /** Unknown */
    Unknown,
    /** Not reported */
    NotReported,
    /** Replaced but not reported */
    ReplacedNotReported,
    /** Protected but not reported */
    ProtectedNotReported,
    /** Varieties not reported */
    VarietiesNotReported,
    /** Filled */
    Filled,
    /** Wait to new */
    WaitToNew,
    /** New */
    New,
    /** Wait to replace */
    WaitToReplace,
    /** Pending replace */
    PendingReplace,
    /** Replaced */
    Replaced,
    /** Partial filled */
    PartialFilled,
    /** Wait to cancel */
    WaitToCancel,
    /** Pending cancel */
    PendingCancel,
    /** Rejected */
    Rejected,
    /** Canceled */
    Canceled,
    /** Expired */
    Expired,
    /** Partial withdrawal */
    PartialWithdrawal,
    /** Monitoring (time-conditional order) */
    DelayedNotReported,
}
