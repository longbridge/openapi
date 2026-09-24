package com.longbridge.fund;

/**
 * FundHolding
 */
public class FundHolding {
    private String bondType;
    private String bondTypeName;
    private String countryName;
    private String holdingType;
    private String industryName;
    private String marketValue;
    private String maturityDate;
    private String name;
    private String shareChange;
    private String shareChangePercent;
    private String shares;
    private String weighting;

    /**
     * Returns bondType.
     *
     * @return bondType
     */
    public String getBondType() {
        return bondType;
    }

    /**
     * Returns bondTypeName.
     *
     * @return bondTypeName
     */
    public String getBondTypeName() {
        return bondTypeName;
    }

    /**
     * Returns countryName.
     *
     * @return countryName
     */
    public String getCountryName() {
        return countryName;
    }

    /**
     * Returns holdingType.
     *
     * @return holdingType
     */
    public String getHoldingType() {
        return holdingType;
    }

    /**
     * Returns industryName.
     *
     * @return industryName
     */
    public String getIndustryName() {
        return industryName;
    }

    /**
     * Returns marketValue.
     *
     * @return marketValue
     */
    public String getMarketValue() {
        return marketValue;
    }

    /**
     * Returns maturityDate.
     *
     * @return maturityDate
     */
    public String getMaturityDate() {
        return maturityDate;
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
     * Returns shareChange.
     *
     * @return shareChange
     */
    public String getShareChange() {
        return shareChange;
    }

    /**
     * Returns shareChangePercent.
     *
     * @return shareChangePercent
     */
    public String getShareChangePercent() {
        return shareChangePercent;
    }

    /**
     * Returns shares.
     *
     * @return shares
     */
    public String getShares() {
        return shares;
    }

    /**
     * Returns weighting.
     *
     * @return weighting
     */
    public String getWeighting() {
        return weighting;
    }

    @Override
    public String toString() {
        return                 "FundHolding [bondType=" + bondType +
                ", bondTypeName=" + bondTypeName +
                ", countryName=" + countryName +
                ", holdingType=" + holdingType +
                ", industryName=" + industryName +
                ", marketValue=" + marketValue +
                ", maturityDate=" + maturityDate +
                ", name=" + name +
                ", shareChange=" + shareChange +
                ", shareChangePercent=" + shareChangePercent +
                ", shares=" + shares +
                ", weighting=" + weighting + "]";
    }
}
