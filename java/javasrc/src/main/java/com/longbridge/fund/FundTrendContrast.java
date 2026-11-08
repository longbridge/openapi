package com.longbridge.fund;

/**
 * FundTrendContrast
 */
public class FundTrendContrast {
    private String benchmarkName;
    private String[] performances;

    /**
     * Returns benchmarkName.
     *
     * @return benchmarkName
     */
    public String getBenchmarkName() {
        return benchmarkName;
    }

    /**
     * Returns performances.
     *
     * @return performances
     */
    public String[] getPerformances() {
        return performances;
    }

    @Override
    public String toString() {
        return                 "FundTrendContrast [benchmarkName=" + benchmarkName +
                ", performances=" + java.util.Arrays.toString(performances) + "]";
    }
}
