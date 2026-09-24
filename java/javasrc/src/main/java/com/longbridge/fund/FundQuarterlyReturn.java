package com.longbridge.fund;

/**
 * FundQuarterlyReturn
 */
public class FundQuarterlyReturn {
    private String changePercent;
    private int quarter;
    private int year;

    /**
     * Returns changePercent.
     *
     * @return changePercent
     */
    public String getChangePercent() {
        return changePercent;
    }

    /**
     * Returns quarter.
     *
     * @return quarter
     */
    public int getQuarter() {
        return quarter;
    }

    /**
     * Returns year.
     *
     * @return year
     */
    public int getYear() {
        return year;
    }

    @Override
    public String toString() {
        return                 "FundQuarterlyReturn [changePercent=" + changePercent +
                ", quarter=" + quarter +
                ", year=" + year + "]";
    }
}
