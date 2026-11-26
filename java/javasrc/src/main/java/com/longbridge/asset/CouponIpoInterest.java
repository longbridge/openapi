package com.longbridge.asset;

/**
 * Terms of an IPO interest coupon ({@code ipo_interest})
 */
public class CouponIpoInterest {
    private String discountRate;

    /**
     * Returns the discount rate, as returned by the server.
     *
     * @return the discount rate, as returned by the server
     */
    public String getDiscountRate() {
        return discountRate;
    }

    @Override
    public String toString() {
        return "CouponIpoInterest [" + "discountRate=" + discountRate + "]";
    }
}
