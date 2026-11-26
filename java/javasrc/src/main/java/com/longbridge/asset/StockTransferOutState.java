package com.longbridge.asset;

/**
 * State of a stock transfer-out application
 */
public enum StockTransferOutState {
    /** Unknown (an undocumented code) */
    Unknown,
    /** Submitted (the only state in which the application can still be cancelled) */
    Submitted,
    /** Processing */
    Processing,
    /** Executing */
    Executing,
    /** Completed */
    Completed,
    /** Cancelled */
    Cancelled,
    /** Partially completed */
    PartiallyCompleted,
    /** Failed */
    Failed,
    /** Rejected */
    Rejected,
}
