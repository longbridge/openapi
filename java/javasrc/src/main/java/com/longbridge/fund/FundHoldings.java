package com.longbridge.fund;

/**
 * FundHoldings
 */
public class FundHoldings {
    private FundHolding[] holdings;
    private String reportDate;
    private String weighting;

    /**
     * Returns holdings.
     *
     * @return holdings
     */
    public FundHolding[] getHoldings() {
        return holdings;
    }

    /**
     * Returns reportDate.
     *
     * @return reportDate
     */
    public String getReportDate() {
        return reportDate;
    }

    /**
     * Returns weighting.
     *
     * @return weighting
     */
    public String getWeighting() {
        return weighting;
    }

    @Override
    public String toString() {
        return                 "FundHoldings [holdings=" + java.util.Arrays.toString(holdings) +
                ", reportDate=" + reportDate +
                ", weighting=" + weighting + "]";
    }
}
