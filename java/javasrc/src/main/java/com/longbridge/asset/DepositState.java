package com.longbridge.asset;

/**
 * State of a deposit record
 */
public enum DepositState {
    /** Unknown (an undocumented code) */
    Unknown,
    /** Processing */
    Processing,
    /** Credited to the account */
    Credited,
    /** Failed */
    Failed,
    /** Cancelled */
    Cancelled,
    /** Invalidated */
    Invalidated,
    /** Rejected */
    Rejected,
}
