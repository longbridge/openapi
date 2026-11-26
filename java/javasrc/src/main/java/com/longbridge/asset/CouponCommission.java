package com.longbridge.asset;

import java.math.BigDecimal;
import java.time.OffsetDateTime;
import java.util.Arrays;

/**
 * Commission terms of a commission coupon ({@code commission})
 */
public class CouponCommission {
    private String[] markets;
    private String rebateRate;
    private CouponTargetType[] bzTypes;
    private int[] bzTypesRaw;
    private BigDecimal commissionBalance;
    private long commissionDuration;
    private CommissionUnlimitedType unlimited;
    private Integer unlimitedRaw;
    private OffsetDateTime commissionExpiredTime;
    private long usageCount;
    private long laveUsageCount;
    private String settlementModel;
    private long amountLimit;
    private boolean superposition;

    /**
     * Returns the markets the coupon applies to.
     *
     * @return the markets the coupon applies to
     */
    public String[] getMarkets() {
        return markets;
    }

    /**
     * Returns the rebate rate, as returned by the server.
     *
     * @return the rebate rate, as returned by the server
     */
    public String getRebateRate() {
        return rebateRate;
    }

    /**
     * Returns the instrument types the coupon applies to.
     *
     * @return the instrument types the coupon applies to
     */
    public CouponTargetType[] getBzTypes() {
        return bzTypes;
    }

    /**
     * The server's numeric codes of bz_types, in order, including codes this
     * SDK does not know (which are Unknown in bz_types)
     *
     * @return the raw codes
     */
    public int[] getBzTypesRaw() {
        return bzTypesRaw;
    }

    /**
     * Returns the remaining commission balance ({@code null} when not set).
     *
     * @return the remaining commission balance
     */
    public BigDecimal getCommissionBalance() {
        return commissionBalance;
    }

    /**
     * Returns the commission duration in seconds (the app reads 200 years or more as lifetime free commission).
     *
     * @return the commission duration in seconds (the app reads 200 years or more as lifetime free commission)
     */
    public long getCommissionDuration() {
        return commissionDuration;
    }

    /**
     * Returns the kind of commission coupon (the server's {@code unlimited} code).
     *
     * @return the kind of commission coupon (the server's {@code unlimited} code)
     */
    public CommissionUnlimitedType getUnlimited() {
        return unlimited;
    }

    /**
     * The server's numeric code of unlimited, also when it is a code
     * this SDK does not know (unlimited is then Unknown); null when the
     * server sent none
     *
     * @return the raw code
     */
    public Integer getUnlimitedRaw() {
        return unlimitedRaw;
    }

    /**
     * Returns the commission expiry time ({@code null} when not set).
     *
     * @return the commission expiry time
     */
    public OffsetDateTime getCommissionExpiredTime() {
        return commissionExpiredTime;
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
     * Returns the settlement model, as returned by the server.
     *
     * @return the settlement model, as returned by the server
     */
    public String getSettlementModel() {
        return settlementModel;
    }

    /**
     * Returns the amount cap per use of a per-use discount card ({@code ByRatePer}), as returned by the server.
     *
     * @return the amount cap per use of a per-use discount card ({@code ByRatePer}), as returned by the server
     */
    public long getAmountLimit() {
        return amountLimit;
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
        return "CouponCommission [" + "markets=" + Arrays.toString(markets) + ", " + "rebateRate=" + rebateRate + ", " + "bzTypes=" + Arrays.toString(bzTypes) + ", " + "bzTypesRaw=" + Arrays.toString(bzTypesRaw) + ", " + "commissionBalance=" + commissionBalance + ", " + "commissionDuration=" + commissionDuration + ", " + "unlimited=" + unlimited + ", " + "unlimitedRaw=" + unlimitedRaw + ", " + "commissionExpiredTime=" + commissionExpiredTime + ", " + "usageCount=" + usageCount + ", " + "laveUsageCount=" + laveUsageCount + ", " + "settlementModel=" + settlementModel + ", " + "amountLimit=" + amountLimit + ", " + "superposition=" + superposition + "]";
    }
}
