package com.longbridge.asset;

import java.util.Arrays;

/**
 * Response for the withdrawal list
 */
public class WithdrawalsResponse {
    private Withdrawal[] list;
    private long total;

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

    @Override
    public String toString() {
        return "WithdrawalsResponse [" + "list=" + Arrays.toString(list) + ", " + "total=" + total + "]";
    }
}
