package com.longbridge.asset;

import java.util.Arrays;

/**
 * Response for the withdrawal list
 */
public class WithdrawalsResponse {
    private Withdrawal[] list;
    private long total;
    private boolean hasMore;

    /**
     * Returns the withdrawal records.
     *
     * @return the withdrawal records
     */
    public Withdrawal[] getList() {
        return list;
    }

    /**
     * Returns the total number of records.
     *
     * @return the total number of records
     */
    public long getTotal() {
        return total;
    }

    /**
     * Returns whether a full page came back (as many records as the page size sent), so the next page may hold more.
     *
     * @return whether a full page came back (as many records as the page size sent), so the next page may hold more
     */
    public boolean getHasMore() {
        return hasMore;
    }

    @Override
    public String toString() {
        return "WithdrawalsResponse [" + "list=" + Arrays.toString(list) + ", " + "total=" + total + ", " + "hasMore=" + hasMore + "]";
    }
}
