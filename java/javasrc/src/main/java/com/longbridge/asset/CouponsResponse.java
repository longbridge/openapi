package com.longbridge.asset;

import java.util.Arrays;

/**
 * Response for the coupon list
 */
public class CouponsResponse {
    private Coupon[] topList;
    private Coupon[] list;
    private Coupon[] bottomList;
    private long total;
    private long expiredCount;

    /**
     * Returns the pinned coupons shown above the list (repeated on every page).
     *
     * @return the pinned coupons shown above the list
     */
    public Coupon[] getTopList() {
        return topList;
    }

    /**
     * Returns the coupons on this page.
     *
     * @return the coupons on this page
     */
    public Coupon[] getList() {
        return list;
    }

    /**
     * Returns the pinned coupons shown below the list (repeated on every page).
     *
     * @return the pinned coupons shown below the list
     */
    public Coupon[] getBottomList() {
        return bottomList;
    }

    /**
     * Returns the total number of coupons.
     *
     * @return the total number of coupons
     */
    public long getTotal() {
        return total;
    }

    /**
     * Returns the number of expired coupons.
     *
     * @return the number of expired coupons
     */
    public long getExpiredCount() {
        return expiredCount;
    }

    @Override
    public String toString() {
        return "CouponsResponse [" + "topList=" + Arrays.toString(topList) + ", " + "list=" + Arrays.toString(list) + ", " + "bottomList=" + Arrays.toString(bottomList) + ", " + "total=" + total + ", " + "expiredCount=" + expiredCount + "]";
    }
}
