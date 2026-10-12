package com.longbridge.forex;

import java.math.BigDecimal;

/**
 * A forex order's state.
 */
public class ForexOrderDetail {
    private ForexOrderStatus state;
    private BigDecimal rate;
    private BigDecimal fromAmount;
    private BigDecimal toAmount;
    private String failReason;

    /**
     * Returns the order state.
     *
     * @return state
     */
    public ForexOrderStatus getState() {
        return state;
    }

    /**
     * Returns the execution rate (standard currency-pair terms); null until filled.
     *
     * @return rate
     */
    public BigDecimal getRate() {
        return rate;
    }

    /**
     * Returns the converted-from amount (actual value once filled); null until filled.
     *
     * @return fromAmount
     */
    public BigDecimal getFromAmount() {
        return fromAmount;
    }

    /**
     * Returns the converted-to amount (actual value once filled); null until filled.
     *
     * @return toAmount
     */
    public BigDecimal getToAmount() {
        return toAmount;
    }

    /**
     * Returns the failure reason; empty when not failed.
     *
     * @return failReason
     */
    public String getFailReason() {
        return failReason;
    }

    @Override
    public String toString() {
        return "ForexOrderDetail [state=" + state +
                ", rate=" + rate +
                ", fromAmount=" + fromAmount +
                ", toAmount=" + toAmount +
                ", failReason=" + failReason + "]";
    }
}
