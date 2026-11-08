package com.longbridge.fund;

/**
 * FundPositionDetail
 */
public class FundPositionDetail {
    private FundPositionDetailValues detailValues;
    private FundDatedValue[] sumProfit;
    private FundUnitValue[] utValue;

    /**
     * Returns detailValues.
     *
     * @return detailValues
     */
    public FundPositionDetailValues getDetailValues() {
        return detailValues;
    }

    /**
     * Returns sumProfit.
     *
     * @return sumProfit
     */
    public FundDatedValue[] getSumProfit() {
        return sumProfit;
    }

    /**
     * Returns utValue.
     *
     * @return utValue
     */
    public FundUnitValue[] getUtValue() {
        return utValue;
    }

    @Override
    public String toString() {
        return                 "FundPositionDetail [detailValues=" + detailValues +
                ", sumProfit=" + java.util.Arrays.toString(sumProfit) +
                ", utValue=" + java.util.Arrays.toString(utValue) + "]";
    }
}
