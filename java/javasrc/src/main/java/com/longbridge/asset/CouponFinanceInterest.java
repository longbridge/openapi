package com.longbridge.asset;

import java.math.BigDecimal;

/**
 * Terms of a finance interest coupon ({@code finance_interest}). The meaning
 * of these fields is not confirmed; they are returned as the server sends them.
 */
public class CouponFinanceInterest {
    private int discountMode;
    private BigDecimal discountRate;
    private boolean superposition;

    /**
     * Returns the discount mode, as returned by the server.
     *
     * @return the discount mode, as returned by the server
     */
    public int getDiscountMode() {
        return discountMode;
    }

    /**
     * Returns the discount rate ({@code null} when not set).
     *
     * @return the discount rate
     */
    public BigDecimal getDiscountRate() {
        return discountRate;
    }

    /**
     * Returns the whether the coupon can be combined with other coupons.
     *
     * @return the whether the coupon can be combined with other coupons
     */
    public boolean getSuperposition() {
        return superposition;
    }

    @Override
    public String toString() {
        return "CouponFinanceInterest [" + "discountMode=" + discountMode + ", " + "discountRate=" + discountRate + ", " + "superposition=" + superposition + "]";
    }
}
