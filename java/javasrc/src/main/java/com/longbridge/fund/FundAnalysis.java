package com.longbridge.fund;

/**
 * FundAnalysis
 */
public class FundAnalysis {
    private int actualPeriod;
    private String costLevel;
    private String returnAbility;
    private String riskAbility;
    private String updatedAt;
    private String valueForMoney;
    private boolean visible;

    /**
     * Returns actualPeriod.
     *
     * @return actualPeriod
     */
    public int getActualPeriod() {
        return actualPeriod;
    }

    /**
     * Returns costLevel.
     *
     * @return costLevel
     */
    public String getCostLevel() {
        return costLevel;
    }

    /**
     * Returns returnAbility.
     *
     * @return returnAbility
     */
    public String getReturnAbility() {
        return returnAbility;
    }

    /**
     * Returns riskAbility.
     *
     * @return riskAbility
     */
    public String getRiskAbility() {
        return riskAbility;
    }

    /**
     * Returns updatedAt.
     *
     * @return updatedAt
     */
    public String getUpdatedAt() {
        return updatedAt;
    }

    /**
     * Returns valueForMoney.
     *
     * @return valueForMoney
     */
    public String getValueForMoney() {
        return valueForMoney;
    }

    /**
     * Returns visible.
     *
     * @return visible
     */
    public boolean getVisible() {
        return visible;
    }

    @Override
    public String toString() {
        return                 "FundAnalysis [actualPeriod=" + actualPeriod +
                ", costLevel=" + costLevel +
                ", returnAbility=" + returnAbility +
                ", riskAbility=" + riskAbility +
                ", updatedAt=" + updatedAt +
                ", valueForMoney=" + valueForMoney +
                ", visible=" + visible + "]";
    }
}
