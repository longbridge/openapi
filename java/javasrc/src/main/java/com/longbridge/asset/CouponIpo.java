package com.longbridge.asset;

/**
 * Terms of an IPO coupon ({@code ipo})
 */
public class CouponIpo {
    private String subscriptionId;

    /**
     * Returns the subscription ID.
     *
     * @return the subscription ID
     */
    public String getSubscriptionId() {
        return subscriptionId;
    }

    @Override
    public String toString() {
        return "CouponIpo [" + "subscriptionId=" + subscriptionId + "]";
    }
}
