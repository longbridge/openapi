package com.longbridge.dca;

/** Options for {@link DcaContext#createDca}. */
public class DcaCreateOptions {
    /** Security symbol, e.g. {@code "700.HK"} */
    public String symbol;
    /** Investment amount per period */
    public String amount;
    /** Frequency: 0=Daily, 1=Weekly, 2=Fortnightly, 3=Monthly */
    public Integer frequency;
    /** Day of week for weekly plans, e.g. {@code "Mon"} (optional) */
    public String dayOfWeek;
    /** Day of month for monthly plans, e.g. {@code "15"} (optional) */
    public String dayOfMonth;
    /** Whether to allow margin financing */
    public boolean allowMargin;
}
