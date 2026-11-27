package com.longbridge.asset;

/**
 * Terms of an IPO interest coupon ({@code ipo_interest})
 */
public class CouponIpoInterest {
    private String discountRate;

    /**
     * Returns the annual interest discount rate, as returned by the server (the app computes the daily discount as rate × financed amount / 365).
     *
     * @return the annual interest discount rate, as returned by the server (the app computes the daily discount as rate × financed amount / 365)
     */
    public String getDiscountRate() {
        return discountRate;
    }

    @Override
    public String toString() {
        return "CouponIpoInterest [" + "discountRate=" + discountRate + "]";
    }
}
