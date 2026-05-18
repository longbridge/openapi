package com.longbridge.fundamental;

/** Response for {@link FundamentalContext#getBusinessSegments}. */
public class BusinessSegments {
    /** Report date */
    public String date;
    /** Total revenue */
    public String total;
    /** Reporting currency */
    public String currency;
    /** Business segment breakdown */
    public BusinessSegmentItem[] business;
}
