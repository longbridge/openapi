package com.longbridge.asset;

import java.util.Arrays;

/**
 * Terms of a fund deduction coupon ({@code fund_deduct})
 */
public class CouponFundDeduct {
    private int maxRiskLevel;
    private String minOrderAmount;
    private String[] orderTypes;
    private String underlyingType;
    private String[] underlyingValues;
    private String[] underlyingNames;

    /**
     * Returns the maximum risk level, as returned by the server.
     *
     * @return the maximum risk level, as returned by the server
     */
    public int getMaxRiskLevel() {
        return maxRiskLevel;
    }

    /**
     * Returns the minimum order amount, as returned by the server.
     *
     * @return the minimum order amount, as returned by the server
     */
    public String getMinOrderAmount() {
        return minOrderAmount;
    }

    /**
     * Returns the order types, as returned by the server.
     *
     * @return the order types, as returned by the server
     */
    public String[] getOrderTypes() {
        return orderTypes;
    }

    /**
     * Returns the underlying type, as returned by the server.
     *
     * @return the underlying type, as returned by the server
     */
    public String getUnderlyingType() {
        return underlyingType;
    }

    /**
     * Returns the underlying values, as returned by the server.
     *
     * @return the underlying values, as returned by the server
     */
    public String[] getUnderlyingValues() {
        return underlyingValues;
    }

    /**
     * Returns the underlying names, as returned by the server.
     *
     * @return the underlying names, as returned by the server
     */
    public String[] getUnderlyingNames() {
        return underlyingNames;
    }

    @Override
    public String toString() {
        return "CouponFundDeduct [" + "maxRiskLevel=" + maxRiskLevel + ", " + "minOrderAmount=" + minOrderAmount + ", " + "orderTypes=" + Arrays.toString(orderTypes) + ", " + "underlyingType=" + underlyingType + ", " + "underlyingValues=" + Arrays.toString(underlyingValues) + ", " + "underlyingNames=" + Arrays.toString(underlyingNames) + "]";
    }
}
