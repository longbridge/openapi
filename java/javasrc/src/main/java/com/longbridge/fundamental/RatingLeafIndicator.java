package com.longbridge.fundamental;

/** A leaf rating indicator with a raw value for {@link RatingSubIndicatorGroup}. */
public class RatingLeafIndicator {
    /** Indicator display name */
    public String name;
    /** Formatted value string */
    public String value;
    /** Value type hint, e.g. "percent" */
    public String valueType;
    /** Score; may be null */
    public Double score;
    /** Letter grade */
    public String letter;
}
