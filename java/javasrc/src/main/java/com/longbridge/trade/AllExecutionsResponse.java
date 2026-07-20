package com.longbridge.trade;

import java.util.Arrays;

/**
 * Response for get all executions request
 */
public class AllExecutionsResponse {
    private boolean hasMore;
    private Execution[] trades;

    /**
     * Returns whether there are more records.
     *
     * @return true if there are more records
     */
    public boolean isHasMore() {
        return hasMore;
    }

    /**
     * Returns the execution list.
     *
     * @return execution list
     */
    public Execution[] getTrades() {
        return trades;
    }

    @Override
    public String toString() {
        return "AllExecutionsResponse [hasMore=" + hasMore + ", trades=" + Arrays.toString(trades) + "]";
    }
}
