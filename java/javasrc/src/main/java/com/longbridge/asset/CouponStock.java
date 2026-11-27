package com.longbridge.asset;

import java.math.BigDecimal;
import java.util.Arrays;

/**
 * Terms of a stock coupon ({@code stock})
 */
public class CouponStock {
    private String counterId;
    private String counterName;
    private BigDecimal amount;
    private String redemptionType;
    private String sendWay;
    private CouponStockCounter[] optionalCounters;
    private CouponStockFallback fallback;

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
     * Returns the amount ({@code null} when not set).
     *
     * @return the amount
     */
    public BigDecimal getAmount() {
        return amount;
    }

    /**
     * Returns the redemption type ({@code by_card_value} or {@code shares_number}), as returned by the server.
     *
     * @return the redemption type
     */
    public String getRedemptionType() {
        return redemptionType;
    }

    /**
     * Returns the delivery way ({@code card_exchange} or {@code account_transfer}), as returned by the server.
     *
     * @return the delivery way
     */
    public String getSendWay() {
        return sendWay;
    }

    /**
     * Returns the optional counters.
     *
     * @return the optional counters
     */
    public CouponStockCounter[] getOptionalCounters() {
        return optionalCounters;
    }

    /**
     * Returns the fallback.
     *
     * @return the fallback
     */
    public CouponStockFallback getFallback() {
        return fallback;
    }

    @Override
    public String toString() {
        return "CouponStock [" + "counterId=" + counterId + ", " + "counterName=" + counterName + ", " + "amount=" + amount + ", " + "redemptionType=" + redemptionType + ", " + "sendWay=" + sendWay + ", " + "optionalCounters=" + Arrays.toString(optionalCounters) + ", " + "fallback=" + fallback + "]";
    }
}
