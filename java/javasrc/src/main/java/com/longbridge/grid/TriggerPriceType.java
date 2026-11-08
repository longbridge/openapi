package com.longbridge.grid;

/**
 * How grid trigger thresholds are interpreted.
 */
public enum TriggerPriceType {
    /** Unknown / unset */
    Unknown,
    /** Trigger by absolute price spread */
    Spread,
    /** Trigger by percent */
    Percent,
}
