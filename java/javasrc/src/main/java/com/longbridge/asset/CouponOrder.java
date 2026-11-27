package com.longbridge.asset;

import java.math.BigDecimal;

/**
 * An order a coupon was applied to
 */
public class CouponOrder {
    private String orderId;
    private String state;
    private String stateVal;
    private BigDecimal executedPrice;
    private BigDecimal executedQuantity;
    private String currency;
    private String settlementCurrency;
    private BigDecimal rebate;
    private String rebateDt;

    /**
     * Returns the order ID.
     *
     * @return the order ID
     */
    public String getOrderId() {
        return orderId;
    }

    /**
     * Returns the state text, as returned by the server.
     *
     * @return the state text, as returned by the server
     */
    public String getState() {
        return state;
    }

    /**
     * Returns the state value, as returned by the server (e.g. {@code FilledStatus}).
     *
     * @return the state value, as returned by the server
     */
    public String getStateVal() {
        return stateVal;
    }

    /**
     * Returns the executed price ({@code null} when not set).
     *
     * @return the executed price
     */
    public BigDecimal getExecutedPrice() {
        return executedPrice;
    }

    /**
     * Returns the executed quantity ({@code null} when not set).
     *
     * @return the executed quantity
     */
    public BigDecimal getExecutedQuantity() {
        return executedQuantity;
    }

    /**
     * Returns the currency.
     *
     * @return the currency
     */
    public String getCurrency() {
        return currency;
    }

    /**
     * Returns the settlement currency.
     *
     * @return the settlement currency
     */
    public String getSettlementCurrency() {
        return settlementCurrency;
    }

    /**
     * Returns the rebate amount ({@code null} when not set).
     *
     * @return the rebate amount
     */
    public BigDecimal getRebate() {
        return rebate;
    }

    /**
     * Returns the rebate date, as returned by the server.
     *
     * @return the rebate date, as returned by the server
     */
    public String getRebateDt() {
        return rebateDt;
    }

    @Override
    public String toString() {
        return "CouponOrder [" + "orderId=" + orderId + ", " + "state=" + state + ", " + "stateVal=" + stateVal + ", " + "executedPrice=" + executedPrice + ", " + "executedQuantity=" + executedQuantity + ", " + "currency=" + currency + ", " + "settlementCurrency=" + settlementCurrency + ", " + "rebate=" + rebate + ", " + "rebateDt=" + rebateDt + "]";
    }
}
