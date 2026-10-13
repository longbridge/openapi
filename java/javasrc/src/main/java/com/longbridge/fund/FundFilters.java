package com.longbridge.fund;

/**
 * FundFilters
 */
public class FundFilters {
    private String[] assetClass;
    private String[] company;
    private String[] currency;
    private String[] industryCategoryName;
    private String[] riskLevel;

    /**
     * Returns assetClass.
     *
     * @return assetClass
     */
    public String[] getAssetClass() {
        return assetClass;
    }

    /**
     * Returns company.
     *
     * @return company
     */
    public String[] getCompany() {
        return company;
    }

    /**
     * Returns currency.
     *
     * @return currency
     */
    public String[] getCurrency() {
        return currency;
    }

    /**
     * Returns industryCategoryName.
     *
     * @return industryCategoryName
     */
    public String[] getIndustryCategoryName() {
        return industryCategoryName;
    }

    /**
     * Returns riskLevel.
     *
     * @return riskLevel
     */
    public String[] getRiskLevel() {
        return riskLevel;
    }

    @Override
    public String toString() {
        return                 "FundFilters [assetClass=" + java.util.Arrays.toString(assetClass) +
                ", company=" + java.util.Arrays.toString(company) +
                ", currency=" + java.util.Arrays.toString(currency) +
                ", industryCategoryName=" + java.util.Arrays.toString(industryCategoryName) +
                ", riskLevel=" + java.util.Arrays.toString(riskLevel) + "]";
    }
}
