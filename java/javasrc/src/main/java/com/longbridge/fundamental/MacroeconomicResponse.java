package com.longbridge.fundamental;

/** Response for {@link FundamentalContext#getMacroeconomic}. */
public class MacroeconomicResponse {
    public MacroeconomicIndicator info;
    public Macroeconomic[] data;
    /** Total number of historical data points. */
    public int count;
}
