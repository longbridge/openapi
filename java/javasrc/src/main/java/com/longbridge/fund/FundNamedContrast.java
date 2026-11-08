package com.longbridge.fund;

/**
 * FundNamedContrast
 */
public class FundNamedContrast {
    private String name;
    private String[] performances;

    /**
     * Returns name.
     *
     * @return name
     */
    public String getName() {
        return name;
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
        return                 "FundNamedContrast [name=" + name +
                ", performances=" + java.util.Arrays.toString(performances) + "]";
    }
}
