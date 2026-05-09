package com.longbridge.calendar;

import java.math.BigDecimal;

/** One key-value data pair in a calendar event. */
public class CalendarDataKv {
    /** Key (may be empty). */
    public String key;
    /** Formatted display value. */
    public String value;
    /** Value type code, e.g. {@code "estimate_eps"}. */
    public String valueType;
    /** Raw numeric value. */
    public BigDecimal valueRaw;
}
