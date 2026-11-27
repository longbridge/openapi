package com.longbridge.asset;

/**
 * An optional counter of a stock coupon
 */
public class CouponStockCounter {
    private String counterId;
    private String shares;

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
        return "CouponStockCounter [" + "counterId=" + counterId + ", " + "shares=" + shares + "]";
    }
}
