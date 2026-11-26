package com.longbridge.asset;

import java.util.Arrays;

/**
 * Response for the reward record list
 */
public class RewardRecordsResponse {
    private RewardRecord[] list;
    private long total;

    /**
     * Returns the reward records.
     *
     * @return the reward records
     */
    public RewardRecord[] getList() {
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
        return "RewardRecordsResponse [" + "list=" + Arrays.toString(list) + ", " + "total=" + total + "]";
    }
}
