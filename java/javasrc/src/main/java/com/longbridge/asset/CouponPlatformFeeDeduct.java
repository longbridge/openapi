package com.longbridge.asset;

import java.util.Arrays;

/**
 * Terms of a platform fee deduction coupon ({@code platform_fee_deduct})
 */
public class CouponPlatformFeeDeduct {
    private CouponTargetType[] bzTypes;
    private int[] bzTypesRaw;
    private String counterId;
    private String counterName;
    private String[] markets;
    private long usageCount;
    private long laveUsageCount;
    private boolean superposition;

    /**
     * Returns the instrument types the coupon applies to.
     *
     * @return the instrument types the coupon applies to
     */
    public CouponTargetType[] getBzTypes() {
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
     * Returns the markets the coupon applies to.
     *
     * @return the markets the coupon applies to
     */
    public String[] getMarkets() {
        return markets;
    }

    /**
     * Returns the number of times used.
     *
     * @return the number of times used
     */
    public long getUsageCount() {
        return usageCount;
    }

    /**
     * Returns the number of uses left.
     *
     * @return the number of uses left
     */
    public long getLaveUsageCount() {
        return laveUsageCount;
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
        return "CouponPlatformFeeDeduct [" + "bzTypes=" + Arrays.toString(bzTypes) + ", " + "bzTypesRaw=" + Arrays.toString(bzTypesRaw) + ", " + "counterId=" + counterId + ", " + "counterName=" + counterName + ", " + "markets=" + Arrays.toString(markets) + ", " + "usageCount=" + usageCount + ", " + "laveUsageCount=" + laveUsageCount + ", " + "superposition=" + superposition + "]";
    }
}
