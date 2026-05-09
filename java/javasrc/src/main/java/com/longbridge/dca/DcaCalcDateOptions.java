package com.longbridge.dca;
/** Options for {@link DcaContext#calcDate}. */
public class DcaCalcDateOptions {
    /** Security symbol, e.g. {@code "700.HK"} */
    public String symbol;
    /** Investment frequency. */
    public DCAFrequency frequency;
    /** Day of week for weekly/fortnightly plans, e.g. {@code "Mon"} (optional) */
    public String dayOfWeek;
    /** Day of month for monthly plans (1–28, optional) */
    public Integer dayOfMonth;
}
