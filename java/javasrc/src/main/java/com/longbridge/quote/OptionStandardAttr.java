package com.longbridge.quote;

/**
 * Whether an option contract is a legacy contract left over from a corporate
 * action (e.g. a stock split or a merger)
 */
public enum OptionStandardAttr {
    /** Unknown */
    Unknown,
    /** A normal, active contract */
    Normal,
    /** A legacy contract produced by a corporate action */
    Old,
}
