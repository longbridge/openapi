package com.longbridge.fund;

/**
 * FundUnitValue
 */
public class FundUnitValue {
    private long date;
    private String dayIncreaseRate;
    private String totalValue;
    private String unitValue;

    /**
     * Returns date.
     *
     * @return date
     */
    public long getDate() {
        return date;
    }

    /**
     * Returns dayIncreaseRate.
     *
     * @return dayIncreaseRate
     */
    public String getDayIncreaseRate() {
        return dayIncreaseRate;
    }

    /**
     * Returns totalValue.
     *
     * @return totalValue
     */
    public String getTotalValue() {
        return totalValue;
    }

    /**
     * Returns unitValue.
     *
     * @return unitValue
     */
    public String getUnitValue() {
        return unitValue;
    }

    @Override
    public String toString() {
        return                 "FundUnitValue [date=" + date +
                ", dayIncreaseRate=" + dayIncreaseRate +
                ", totalValue=" + totalValue +
                ", unitValue=" + unitValue + "]";
    }
}
