package com.longbridge.dca;

/** Response for {@link DcaContext#history}. */
public class DcaHistoryResponse {
    /** Execution history records. */
    public DcaHistoryRecord[] records;
    /** Whether more records exist. */
    public boolean hasMore;
}
