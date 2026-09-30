package com.longbridge.fund;

/**
 * FundPosition
 */
public class FundPosition {
    private String amount;
    private String counterId;
    private String currency;
    private String freezeUnits;
    private String holdingProfit;
    private String holdingUnits;
    private String name;
    private String recentProfit;
    private long recentTradingDay;
    private String sumRecentProfit;

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
     * Returns currency.
     *
     * @return currency
     */
    public String getCurrency() {
        return currency;
    }

    /**
     * Returns freezeUnits.
     *
     * @return freezeUnits
     */
    public String getFreezeUnits() {
        return freezeUnits;
    }

    /**
     * Returns holdingProfit.
     *
     * @return holdingProfit
     */
    public String getHoldingProfit() {
        return holdingProfit;
    }

    /**
     * Returns holdingUnits.
     *
     * @return holdingUnits
     */
    public String getHoldingUnits() {
        return holdingUnits;
    }

    /**
     * Returns name.
     *
     * @return name
     */
    public String getName() {
        return name;
    }

    /**
     * Returns recentProfit.
     *
     * @return recentProfit
     */
    public String getRecentProfit() {
        return recentProfit;
    }

    /**
     * Returns recentTradingDay.
     *
     * @return recentTradingDay
     */
    public long getRecentTradingDay() {
        return recentTradingDay;
    }

    /**
     * Returns sumRecentProfit.
     *
     * @return sumRecentProfit
     */
    public String getSumRecentProfit() {
        return sumRecentProfit;
    }

    @Override
    public String toString() {
        return                 "FundPosition [amount=" + amount +
                ", counterId=" + counterId +
                ", currency=" + currency +
                ", freezeUnits=" + freezeUnits +
                ", holdingProfit=" + holdingProfit +
                ", holdingUnits=" + holdingUnits +
                ", name=" + name +
                ", recentProfit=" + recentProfit +
                ", recentTradingDay=" + recentTradingDay +
                ", sumRecentProfit=" + sumRecentProfit + "]";
    }
}
