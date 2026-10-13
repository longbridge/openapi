package com.longbridge.fund;

/**
 * HotFund
 */
public class HotFund {
    private int assetClass;
    private String assetClassName;
    private String counterId;
    private String currency;
    private String earningRate;
    private FundPerformancePoint[] fundPerformances;
    private String name;
    private String purchaseAmount;
    private String recommendationText;
    private int riskLevel;
    private String riskLevelName;
    private String timeInterval;

    /**
     * Returns assetClass.
     *
     * @return assetClass
     */
    public int getAssetClass() {
        return assetClass;
    }

    /**
     * Returns assetClassName.
     *
     * @return assetClassName
     */
    public String getAssetClassName() {
        return assetClassName;
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
     * Returns earningRate.
     *
     * @return earningRate
     */
    public String getEarningRate() {
        return earningRate;
    }

    /**
     * Returns fundPerformances.
     *
     * @return fundPerformances
     */
    public FundPerformancePoint[] getFundPerformances() {
        return fundPerformances;
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
     * Returns purchaseAmount.
     *
     * @return purchaseAmount
     */
    public String getPurchaseAmount() {
        return purchaseAmount;
    }

    /**
     * Returns recommendationText.
     *
     * @return recommendationText
     */
    public String getRecommendationText() {
        return recommendationText;
    }

    /**
     * Returns riskLevel.
     *
     * @return riskLevel
     */
    public int getRiskLevel() {
        return riskLevel;
    }

    /**
     * Returns riskLevelName.
     *
     * @return riskLevelName
     */
    public String getRiskLevelName() {
        return riskLevelName;
    }

    /**
     * Returns timeInterval.
     *
     * @return timeInterval
     */
    public String getTimeInterval() {
        return timeInterval;
    }

    @Override
    public String toString() {
        return                 "HotFund [assetClass=" + assetClass +
                ", assetClassName=" + assetClassName +
                ", counterId=" + counterId +
                ", currency=" + currency +
                ", earningRate=" + earningRate +
                ", fundPerformances=" + java.util.Arrays.toString(fundPerformances) +
                ", name=" + name +
                ", purchaseAmount=" + purchaseAmount +
                ", recommendationText=" + recommendationText +
                ", riskLevel=" + riskLevel +
                ", riskLevelName=" + riskLevelName +
                ", timeInterval=" + timeInterval + "]";
    }
}
