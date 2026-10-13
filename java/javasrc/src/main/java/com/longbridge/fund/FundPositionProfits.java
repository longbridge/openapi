package com.longbridge.fund;

/**
 * FundPositionProfits
 */
public class FundPositionProfits {
    private String currency;
    private FundDatedValue[] historyValue;
    private long lastUpdateTime;
    private String sumProfit;

    /**
     * Returns currency.
     *
     * @return currency
     */
    public String getCurrency() {
        return currency;
    }

    /**
     * Returns historyValue.
     *
     * @return historyValue
     */
    public FundDatedValue[] getHistoryValue() {
        return historyValue;
    }

    /**
     * Returns lastUpdateTime.
     *
     * @return lastUpdateTime
     */
    public long getLastUpdateTime() {
        return lastUpdateTime;
    }

    /**
     * Returns sumProfit.
     *
     * @return sumProfit
     */
    public String getSumProfit() {
        return sumProfit;
    }

    @Override
    public String toString() {
        return                 "FundPositionProfits [currency=" + currency +
                ", historyValue=" + java.util.Arrays.toString(historyValue) +
                ", lastUpdateTime=" + lastUpdateTime +
                ", sumProfit=" + sumProfit + "]";
    }
}
