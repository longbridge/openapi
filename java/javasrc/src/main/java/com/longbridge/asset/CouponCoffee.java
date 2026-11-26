package com.longbridge.asset;

import java.util.Arrays;

/**
 * Terms of a coffee coupon ({@code coffee_coupon})
 */
public class CouponCoffee {
    private String couponChannel;
    private String couponId;
    private String couponNo;
    private boolean isAvailableAllStore;
    private String periodType;
    private long settledStoreId;
    private long[] storeIdList;
    private CouponCoffeeStore[] storeList;
    private long threshold;
    private String useLimit;

    /**
     * Returns the coupon channel, as returned by the server.
     *
     * @return the coupon channel, as returned by the server
     */
    public String getCouponChannel() {
        return couponChannel;
    }

    /**
     * Returns the coupon ID, as returned by the server.
     *
     * @return the coupon ID, as returned by the server
     */
    public String getCouponId() {
        return couponId;
    }

    /**
     * Returns the coupon number, as returned by the server.
     *
     * @return the coupon number, as returned by the server
     */
    public String getCouponNo() {
        return couponNo;
    }

    /**
     * Returns the whether the coupon can be used at every store.
     *
     * @return the whether the coupon can be used at every store
     */
    public boolean getIsAvailableAllStore() {
        return isAvailableAllStore;
    }

    /**
     * Returns the period type (e.g. {@code day}), as returned by the server.
     *
     * @return the period type
     */
    public String getPeriodType() {
        return periodType;
    }

    /**
     * Returns the iD of the store the coupon was settled at ({@code 0} when not settled).
     *
     * @return the iD of the store the coupon was settled at
     */
    public long getSettledStoreId() {
        return settledStoreId;
    }

    /**
     * Returns the iDs of the stores the coupon can be used at.
     *
     * @return the iDs of the stores the coupon can be used at
     */
    public long[] getStoreIdList() {
        return storeIdList;
    }

    /**
     * Returns the stores the coupon can be used at.
     *
     * @return the stores the coupon can be used at
     */
    public CouponCoffeeStore[] getStoreList() {
        return storeList;
    }

    /**
     * Returns the threshold, as returned by the server.
     *
     * @return the threshold, as returned by the server
     */
    public long getThreshold() {
        return threshold;
    }

    /**
     * Returns the use limit ({@code total} or {@code period}), as returned by the server.
     *
     * @return the use limit
     */
    public String getUseLimit() {
        return useLimit;
    }

    @Override
    public String toString() {
        return "CouponCoffee [" + "couponChannel=" + couponChannel + ", " + "couponId=" + couponId + ", " + "couponNo=" + couponNo + ", " + "isAvailableAllStore=" + isAvailableAllStore + ", " + "periodType=" + periodType + ", " + "settledStoreId=" + settledStoreId + ", " + "storeIdList=" + Arrays.toString(storeIdList) + ", " + "storeList=" + Arrays.toString(storeList) + ", " + "threshold=" + threshold + ", " + "useLimit=" + useLimit + "]";
    }
}
