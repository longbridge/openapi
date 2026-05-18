package com.longbridge.fundamental;

/** One historical business segments snapshot. */
public class BusinessSegmentsHistoricalItem {
    /** Report date */
    public String date;
    /** Total revenue */
    public String total;
    /** Reporting currency */
    public String currency;
    /** Business segment breakdown */
    public BusinessSegmentHistoryItem[] business;
    /** Regional breakdown */
    public BusinessSegmentHistoryItem[] regionals;
}
