package com.longbridge.asset;

/**
 * A counter (security) of a crypto coupon
 */
public class CouponCryptoCounter {
    private String counterId;

    /**
     * Returns the counter ID.
     *
     * @return the counter ID
     */
    public String getCounterId() {
        return counterId;
    }

    @Override
    public String toString() {
        return "CouponCryptoCounter [" + "counterId=" + counterId + "]";
    }
}
