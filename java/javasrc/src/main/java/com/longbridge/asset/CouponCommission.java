package com.longbridge.asset;

import java.math.BigDecimal;
import java.time.OffsetDateTime;
import java.util.Arrays;

/**
 * Commission terms of a commission coupon
 */
public class CouponCommission {
    private String[] markets;
    private String rebateRate;
    private int[] bzTypes;
    private BigDecimal commissionBalance;
    private String commissionDuration;
    private boolean unlimited;
    private OffsetDateTime commissionExpiredTime;
    private long usageCount;
    private long laveUsageCount;
    private String settlementModel;

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
     * Returns the instrument types the coupon applies to: 0 stocks, 1 options, 2 warrants, 3 ETF, 4 LOF.
     *
     * @return the instrument types the coupon applies to: 0 stocks, 1 options, 2 warrants, 3 ETF, 4 LOF
     */
    public int[] getBzTypes() {
        return bzTypes;
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
     * Returns the commission duration, as returned by the server.
     *
     * @return the commission duration, as returned by the server
     */
    public String getCommissionDuration() {
        return commissionDuration;
    }

    /**
     * Returns whether the coupon is unlimited.
     *
     * @return whether the coupon is unlimited
     */
    public boolean isUnlimited() {
        return unlimited;
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

    @Override
    public String toString() {
        return "CouponCommission [" + "markets=" + Arrays.toString(markets) + ", " + "rebateRate=" + rebateRate + ", " + "bzTypes=" + Arrays.toString(bzTypes) + ", " + "commissionBalance=" + commissionBalance + ", " + "commissionDuration=" + commissionDuration + ", " + "unlimited=" + unlimited + ", " + "commissionExpiredTime=" + commissionExpiredTime + ", " + "usageCount=" + usageCount + ", " + "laveUsageCount=" + laveUsageCount + ", " + "settlementModel=" + settlementModel + "]";
    }
}
