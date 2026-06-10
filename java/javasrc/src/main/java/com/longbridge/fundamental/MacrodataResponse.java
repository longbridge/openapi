package com.longbridge.fundamental;

/** Response for {@link FundamentalContext#getMacrodata}. */
public class MacrodataResponse {
    public MacrodataIndicator info;
    public Macrodata[] data;
    /** Total number of historical data points. */
    public int count;
}
