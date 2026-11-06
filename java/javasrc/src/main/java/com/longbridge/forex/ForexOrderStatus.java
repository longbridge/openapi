package com.longbridge.forex;

/**
 * Forex order state
 */
public enum ForexOrderStatus {
    /** Processing (includes manual review); keep polling. */
    Processing,
    /** Conversion succeeded. */
    Success,
    /** Conversion failed; frozen funds have been returned. */
    Failed,
}
