package com.longbridge.asset;

/**
 * State of a stock transfer-in application
 */
public enum StockTransferInState {
    /** Unknown (an undocumented code) */
    Unknown,
    /** Submitted */
    Submitted,
    /** Cancelled */
    Cancelled,
    /** Completed */
    Completed,
    /** Processing */
    Processing,
    /** Partially completed */
    PartiallyCompleted,
    /** Failed */
    Failed,
}
