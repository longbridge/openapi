package com.longbridge.fundamental;

/** One business or regional segment item (latest snapshot). */
public class BusinessSegmentItem {
    /** Segment name */
    public String name;
    /** Percentage of total revenue */
    public String percent;
    /**
     * Absolute value, in the response's currency. Empty when the server does
     * not supply one.
     */
    public String value;
    /**
     * Stable identifier for the segment, usable to align the same segment
     * across periods. Empty when the server does not supply one.
     */
    public String id;
    /**
     * Year-on-year change for this segment. Empty when the server does not
     * supply one.
     */
    public String yoy;
}
