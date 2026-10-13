package com.longbridge.fund;

/**
 * FundPerformancePoint
 */
public class FundPerformancePoint {
    private String date;
    private String lastDone;

    /**
     * Returns date.
     *
     * @return date
     */
    public String getDate() {
        return date;
    }

    /**
     * Returns lastDone.
     *
     * @return lastDone
     */
    public String getLastDone() {
        return lastDone;
    }

    @Override
    public String toString() {
        return                 "FundPerformancePoint [date=" + date +
                ", lastDone=" + lastDone + "]";
    }
}
