package com.longbridge.fund;

/**
 * FundAnnualReturn
 */
public class FundAnnualReturn {
    private String changePercent;
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
     * Returns year.
     *
     * @return year
     */
    public int getYear() {
        return year;
    }

    @Override
    public String toString() {
        return                 "FundAnnualReturn [changePercent=" + changePercent +
                ", year=" + year + "]";
    }
}
