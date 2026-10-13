package com.longbridge.quote;

/**
 * Special expiration cycle of an option contract
 */
public enum OptionExpiryCycleType {
    /** Unknown */
    Unknown,
    /** Standard monthly option */
    Monthly,
    /** Weekly option, expires weekly */
    Weekly,
    /** Quarterly option, expires quarterly */
    Quarterly,
}
