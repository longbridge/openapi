package com.longbridge.fund;

/**
 * FundPerformanceComparison
 */
public class FundPerformanceComparison {
    private FundNamedContrast[] contrastPerformances;
    private String[] fundPerformances;

    /**
     * Returns contrastPerformances.
     *
     * @return contrastPerformances
     */
    public FundNamedContrast[] getContrastPerformances() {
        return contrastPerformances;
    }

    /**
     * Returns fundPerformances.
     *
     * @return fundPerformances
     */
    public String[] getFundPerformances() {
        return fundPerformances;
    }

    @Override
    public String toString() {
        return                 "FundPerformanceComparison [contrastPerformances=" + java.util.Arrays.toString(contrastPerformances) +
                ", fundPerformances=" + java.util.Arrays.toString(fundPerformances) + "]";
    }
}
