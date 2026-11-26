package com.longbridge.asset;

/**
 * State of one stock inside a stock transfer application
 */
public enum TransferStockState {
    /** Unknown (an undocumented code) */
    Unknown,
    /** Processing */
    Processing,
    /** Succeeded */
    Succeeded,
    /** Failed */
    Failed,
}
