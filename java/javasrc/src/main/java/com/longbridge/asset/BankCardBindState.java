package com.longbridge.asset;

/**
 * Binding state of a bank card
 */
public enum BankCardBindState {
    /** Unknown (an undocumented code) */
    Unknown,
    /** Binding in progress */
    Binding,
    /** Bound */
    Bound,
    /** Binding failed */
    Failed,
}
