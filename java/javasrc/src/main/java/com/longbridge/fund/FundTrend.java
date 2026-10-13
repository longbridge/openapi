package com.longbridge.fund;

/**
 * FundTrend
 */
public class FundTrend {
    private int actualPeriod;
    private int[] availablePeriods;
    private String[] categoryAveragePerformances;
    private FundTrendContrast contrastPerformances;
    private String[] fundPerformances;

    /**
     * Returns actualPeriod.
     *
     * @return actualPeriod
     */
    public int getActualPeriod() {
        return actualPeriod;
    }

    /**
     * Returns availablePeriods.
     *
     * @return availablePeriods
     */
    public int[] getAvailablePeriods() {
        return availablePeriods;
    }

    /**
     * Returns categoryAveragePerformances.
     *
     * @return categoryAveragePerformances
     */
    public String[] getCategoryAveragePerformances() {
        return categoryAveragePerformances;
    }

    /**
     * Returns contrastPerformances.
     *
     * @return contrastPerformances
     */
    public FundTrendContrast getContrastPerformances() {
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
        return                 "FundTrend [actualPeriod=" + actualPeriod +
                ", availablePeriods=" + java.util.Arrays.toString(availablePeriods) +
                ", categoryAveragePerformances=" + java.util.Arrays.toString(categoryAveragePerformances) +
                ", contrastPerformances=" + contrastPerformances +
                ", fundPerformances=" + java.util.Arrays.toString(fundPerformances) + "]";
    }
}
