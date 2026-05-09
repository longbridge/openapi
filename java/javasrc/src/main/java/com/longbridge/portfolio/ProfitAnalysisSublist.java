package com.longbridge.portfolio;

/** Per-security P&amp;L breakdown. */
public class ProfitAnalysisSublist {
    /** Start time (unix timestamp string). */
    public String start;
    /** End time (unix timestamp string). */
    public String end;
    /** Start date string. */
    public String startDate;
    /** End date string. */
    public String endDate;
    /** Last updated time (unix timestamp string). */
    public String updatedAt;
    /** Last updated date string. */
    public String updatedDate;
    /** Per-security P&amp;L items. */
    public ProfitAnalysisItem[] items;
}
