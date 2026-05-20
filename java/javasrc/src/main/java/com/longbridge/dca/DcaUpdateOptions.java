package com.longbridge.dca;

/** Options for {@link DcaContext#updateDca}. */
public class DcaUpdateOptions {
    /** Plan ID to update */
    public String planId;
    /** New investment amount (optional) */
    public String amount;
    /** New frequency (optional). */
    public DCAFrequency frequency;
    /** New day of week (optional) */
    public String dayOfWeek;
    /** New day of month (optional) */
    public String dayOfMonth;
    /** New margin setting (optional) */
    public boolean allowMargin;
}
