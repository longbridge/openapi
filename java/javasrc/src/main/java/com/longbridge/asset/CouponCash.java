package com.longbridge.asset;

/**
 * Terms of a cash coupon ({@code cash})
 */
public class CouponCash {
    private long receiveDuration;

    /**
     * Returns the receive duration, as returned by the server (its unit and meaning are not confirmed).
     *
     * @return the receive duration, as returned by the server (its unit and meaning are not confirmed)
     */
    public long getReceiveDuration() {
        return receiveDuration;
    }

    @Override
    public String toString() {
        return "CouponCash [" + "receiveDuration=" + receiveDuration + "]";
    }
}
