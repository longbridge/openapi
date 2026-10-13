package com.longbridge.fund;

/**
 * FundPositionDetailValues
 */
public class FundPositionDetailValues {
    private String amount;
    private String currency;
    private String holdingCost;
    private String holdingProfit;
    private String holdingProfitRate;
    private String holdingUnits;
    private String holdingValue;
    private String pendingBuyValue;
    private String pendingSellValue;
    private String profitAmountAccumTd;
    private String profitAmountAccumTdRate;
    private String recentProfit;
    private long recentTradingday;
    private String recentUnitValue;
    private String soldPendingConfirmUnits;

    /**
     * Returns amount.
     *
     * @return amount
     */
    public String getAmount() {
        return amount;
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
     * Returns holdingCost.
     *
     * @return holdingCost
     */
    public String getHoldingCost() {
        return holdingCost;
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
     * Returns holdingProfitRate.
     *
     * @return holdingProfitRate
     */
    public String getHoldingProfitRate() {
        return holdingProfitRate;
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
     * Returns holdingValue.
     *
     * @return holdingValue
     */
    public String getHoldingValue() {
        return holdingValue;
    }

    /**
     * Returns pendingBuyValue.
     *
     * @return pendingBuyValue
     */
    public String getPendingBuyValue() {
        return pendingBuyValue;
    }

    /**
     * Returns pendingSellValue.
     *
     * @return pendingSellValue
     */
    public String getPendingSellValue() {
        return pendingSellValue;
    }

    /**
     * Returns profitAmountAccumTd.
     *
     * @return profitAmountAccumTd
     */
    public String getProfitAmountAccumTd() {
        return profitAmountAccumTd;
    }

    /**
     * Returns profitAmountAccumTdRate.
     *
     * @return profitAmountAccumTdRate
     */
    public String getProfitAmountAccumTdRate() {
        return profitAmountAccumTdRate;
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
     * Returns recentTradingday.
     *
     * @return recentTradingday
     */
    public long getRecentTradingday() {
        return recentTradingday;
    }

    /**
     * Returns recentUnitValue.
     *
     * @return recentUnitValue
     */
    public String getRecentUnitValue() {
        return recentUnitValue;
    }

    /**
     * Returns soldPendingConfirmUnits.
     *
     * @return soldPendingConfirmUnits
     */
    public String getSoldPendingConfirmUnits() {
        return soldPendingConfirmUnits;
    }

    @Override
    public String toString() {
        return                 "FundPositionDetailValues [amount=" + amount +
                ", currency=" + currency +
                ", holdingCost=" + holdingCost +
                ", holdingProfit=" + holdingProfit +
                ", holdingProfitRate=" + holdingProfitRate +
                ", holdingUnits=" + holdingUnits +
                ", holdingValue=" + holdingValue +
                ", pendingBuyValue=" + pendingBuyValue +
                ", pendingSellValue=" + pendingSellValue +
                ", profitAmountAccumTd=" + profitAmountAccumTd +
                ", profitAmountAccumTdRate=" + profitAmountAccumTdRate +
                ", recentProfit=" + recentProfit +
                ", recentTradingday=" + recentTradingday +
                ", recentUnitValue=" + recentUnitValue +
                ", soldPendingConfirmUnits=" + soldPendingConfirmUnits + "]";
    }
}
