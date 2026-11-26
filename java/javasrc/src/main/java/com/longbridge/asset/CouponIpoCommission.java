package com.longbridge.asset;

import java.util.Arrays;

/**
 * Terms of an IPO commission coupon ({@code ipo_commission})
 */
public class CouponIpoCommission {
    private IpoCouponTargetType[] bzTypes;
    private String counterId;
    private String counterName;
    private String financingLimit;

    /**
     * Returns the iPO subscription types the coupon applies to.
     *
     * @return the iPO subscription types the coupon applies to
     */
    public IpoCouponTargetType[] getBzTypes() {
        return bzTypes;
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
     * Returns the counter name.
     *
     * @return the counter name
     */
    public String getCounterName() {
        return counterName;
    }

    /**
     * Returns the financing limit, as returned by the server.
     *
     * @return the financing limit, as returned by the server
     */
    public String getFinancingLimit() {
        return financingLimit;
    }

    @Override
    public String toString() {
        return "CouponIpoCommission [" + "bzTypes=" + Arrays.toString(bzTypes) + ", " + "counterId=" + counterId + ", " + "counterName=" + counterName + ", " + "financingLimit=" + financingLimit + "]";
    }
}
