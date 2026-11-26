package com.longbridge.asset;

import java.util.Arrays;

/**
 * Terms of an IPO commission coupon ({@code ipo_commission})
 */
public class CouponIpoCommission {
    private IpoCouponTargetType[] bzTypes;
    private int[] bzTypesRaw;
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
     * The server's numeric codes of bz_types, including codes this
     * SDK does not know (Unknown in bz_types); always the same
     * length and order as bz_types: elements that are neither a code
     * nor a known name are left out of both
     *
     * @return the raw codes
     */
    public int[] getBzTypesRaw() {
        return bzTypesRaw;
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
        return "CouponIpoCommission [" + "bzTypes=" + Arrays.toString(bzTypes) + ", " + "bzTypesRaw=" + Arrays.toString(bzTypesRaw) + ", " + "counterId=" + counterId + ", " + "counterName=" + counterName + ", " + "financingLimit=" + financingLimit + "]";
    }
}
