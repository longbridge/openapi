package com.longport.trade;

public enum OrderStatus {
    Unknown,
    NotReported,
    ReplacedNotReported,
    ProtectedNotReported,
    VarietiesNotReported,
    Filled,
    WaitToNew,
    New,
    WaitToReplace,
    PendingReplace,
    Replaced,
    PartialFilled,
    WaitToCancel,
    PendingCancel,
    Rejected,
    Canceled,
    Expired,
    PartialWithdrawal,
}
