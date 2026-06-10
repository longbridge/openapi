package com.longbridge.fundamental;

/** Response for {@link FundamentalContext#getMacrodataIndicators}. */
public class MacrodataIndicatorListResponse {
    public MacrodataIndicator[] data;
    /** Total number of indicators matching the query. */
    public int count;
}
