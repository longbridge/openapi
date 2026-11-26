package com.longbridge.asset;

import java.util.Arrays;

/**
 * Terms of a platform fee deduction coupon ({@code platform_fee_deduct})
 */
public class CouponPlatformFeeDeduct {
    private CouponTargetType[] bzTypes;
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
        return "CouponPlatformFeeDeduct [" + "bzTypes=" + Arrays.toString(bzTypes) + ", " + "counterId=" + counterId + ", " + "counterName=" + counterName + ", " + "markets=" + Arrays.toString(markets) + ", " + "usageCount=" + usageCount + ", " + "laveUsageCount=" + laveUsageCount + ", " + "superposition=" + superposition + "]";
    }
}
