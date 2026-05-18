package com.longbridge.fundamental;

/** Options for {@link FundamentalContext#getBusinessSegmentsHistory}. */
public class BusinessSegmentsHistoryOptions {
    /** Security symbol */
    public String symbol;
    /** Report type: "qf", "saf", "af", or null */
    public String report;
    /** Category filter, or null */
    public String cate;
}
