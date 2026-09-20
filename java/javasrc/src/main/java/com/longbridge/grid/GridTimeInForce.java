package com.longbridge.grid;

/**
 * Time in force for a grid order.
 */
public enum GridTimeInForce {
    /** Day order */
    Day,
    /** Good-til-canceled */
    GoodTilCanceled,
    /** Good-til-date */
    GoodTilDate,
    /** Unknown value */
    Unknown,
}
