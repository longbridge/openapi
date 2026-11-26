package com.longbridge.asset;

/**
 * Witness state of a bank card
 */
public enum BankCardWitnessState {
    /** Unknown (an undocumented code) */
    Unknown,
    /** Witnessing not required */
    NotRequired,
    /** Witnessing pending */
    Pending,
    /** Witnessed */
    Witnessed,
}
