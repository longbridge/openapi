package com.longbridge.fund;

/**
 * FundOrder
 */
public class FundOrder {
    private String action;
    private String amount;
    private String counterId;
    private long createdAt;
    private String currency;
    private String fundName;
    private long id;
    private boolean isAuto;
    private String netWorth;
    private String productType;
    private String state;
    private String stateDesc;
    private String units;

    /**
     * Returns action.
     *
     * @return action
     */
    public String getAction() {
        return action;
    }

    /**
     * Returns amount.
     *
     * @return amount
     */
    public String getAmount() {
        return amount;
    }

    /**
     * Returns counterId.
     *
     * @return counterId
     */
    public String getCounterId() {
        return counterId;
    }

    /**
     * Returns createdAt.
     *
     * @return createdAt
     */
    public long getCreatedAt() {
        return createdAt;
    }

    /**
     * Returns currency.
     *
     * @return currency
     */
    public String getCurrency() {
        return currency;
    }

    /**
     * Returns fundName.
     *
     * @return fundName
     */
    public String getFundName() {
        return fundName;
    }

    /**
     * Returns id.
     *
     * @return id
     */
    public long getId() {
        return id;
    }

    /**
     * Returns isAuto.
     *
     * @return isAuto
     */
    public boolean getIsAuto() {
        return isAuto;
    }

    /**
     * Returns netWorth.
     *
     * @return netWorth
     */
    public String getNetWorth() {
        return netWorth;
    }

    /**
     * Returns productType.
     *
     * @return productType
     */
    public String getProductType() {
        return productType;
    }

    /**
     * Returns state.
     *
     * @return state
     */
    public String getState() {
        return state;
    }

    /**
     * Returns stateDesc.
     *
     * @return stateDesc
     */
    public String getStateDesc() {
        return stateDesc;
    }

    /**
     * Returns units.
     *
     * @return units
     */
    public String getUnits() {
        return units;
    }

    @Override
    public String toString() {
        return                 "FundOrder [action=" + action +
                ", amount=" + amount +
                ", counterId=" + counterId +
                ", createdAt=" + createdAt +
                ", currency=" + currency +
                ", fundName=" + fundName +
                ", id=" + id +
                ", isAuto=" + isAuto +
                ", netWorth=" + netWorth +
                ", productType=" + productType +
                ", state=" + state +
                ", stateDesc=" + stateDesc +
                ", units=" + units + "]";
    }
}
