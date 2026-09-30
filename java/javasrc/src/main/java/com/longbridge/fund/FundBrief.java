package com.longbridge.fund;

/**
 * FundBrief
 */
public class FundBrief {
    private int assetClass;
    private String assetClassName;
    private String code;
    private String counterId;
    private String currency;
    private String description;
    private String earningRate;
    private boolean holding;
    private String isin;
    private String name;
    private String product;
    private String purchaseAmount;
    private String recommendationText;
    private int riskLevel;
    private String riskLevelName;
    private String timeInterval;
    private String unitValue;

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
     * Returns code.
     *
     * @return code
     */
    public String getCode() {
        return code;
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
     * Returns description.
     *
     * @return description
     */
    public String getDescription() {
        return description;
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
     * Returns holding.
     *
     * @return holding
     */
    public boolean getHolding() {
        return holding;
    }

    /**
     * Returns isin.
     *
     * @return isin
     */
    public String getIsin() {
        return isin;
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
     * Returns product.
     *
     * @return product
     */
    public String getProduct() {
        return product;
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
        return                 "FundBrief [assetClass=" + assetClass +
                ", assetClassName=" + assetClassName +
                ", code=" + code +
                ", counterId=" + counterId +
                ", currency=" + currency +
                ", description=" + description +
                ", earningRate=" + earningRate +
                ", holding=" + holding +
                ", isin=" + isin +
                ", name=" + name +
                ", product=" + product +
                ", purchaseAmount=" + purchaseAmount +
                ", recommendationText=" + recommendationText +
                ", riskLevel=" + riskLevel +
                ", riskLevelName=" + riskLevelName +
                ", timeInterval=" + timeInterval +
                ", unitValue=" + unitValue + "]";
    }
}
