package com.longbridge.asset;

/**
 * Fallback of a stock coupon
 */
public class CouponStockFallback {
    private boolean enabled;
    private String fallbackType;
    private String counterId;
    private String shares;

    /**
     * Returns the whether the fallback is enabled.
     *
     * @return the whether the fallback is enabled
     */
    public boolean getEnabled() {
        return enabled;
    }

    /**
     * Returns the fallback type (the server's {@code type}), as returned by the server.
     *
     * @return the fallback type
     */
    public String getFallbackType() {
        return fallbackType;
    }

    /**
     * Returns the counter ID.
     *
     * @return the counter ID
     */
    public String getCounterId() {
        return counterId;
    }

    /**
     * Returns the shares, as returned by the server.
     *
     * @return the shares, as returned by the server
     */
    public String getShares() {
        return shares;
    }

    @Override
    public String toString() {
        return "CouponStockFallback [" + "enabled=" + enabled + ", " + "fallbackType=" + fallbackType + ", " + "counterId=" + counterId + ", " + "shares=" + shares + "]";
    }
}
