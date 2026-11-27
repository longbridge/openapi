package com.longbridge.asset;

/**
 * Fallback of a crypto coupon
 */
public class CouponCryptoFallback {
    private boolean enabled;
    private String counterId;

    /**
     * Returns the whether the fallback is enabled.
     *
     * @return the whether the fallback is enabled
     */
    public boolean getEnabled() {
        return enabled;
    }

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
        return "CouponCryptoFallback [" + "enabled=" + enabled + ", " + "counterId=" + counterId + "]";
    }
}
