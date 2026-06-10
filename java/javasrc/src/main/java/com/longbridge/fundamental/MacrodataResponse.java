package com.longbridge.fundamental;

/** Response for {@link FundamentalContext#getEconomicIndicator}. */
public class MacrodataResponse {
    public MacrodataIndicatorInfo info;
    public MacrodataRecord[] data;
}
