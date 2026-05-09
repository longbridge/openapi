package com.longbridge.dca;

/** Options for {@link DcaContext#history}. */
public class DcaHistoryOptions {
    /** Plan ID to filter history records. */
    public String planId;
    /** Page number (1-based). */
    public Integer page;
    /** Page size (number of records per page). */
    public Integer limit;
}
