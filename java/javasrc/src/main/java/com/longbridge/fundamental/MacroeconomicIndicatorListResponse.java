package com.longbridge.fundamental;

/** Response for {@link FundamentalContext#getMacroeconomicIndicators}. */
public class MacroeconomicIndicatorListResponse {
    public MacroeconomicIndicator[] data;
    /** Total number of indicators matching the query. */
    public int count;
}
