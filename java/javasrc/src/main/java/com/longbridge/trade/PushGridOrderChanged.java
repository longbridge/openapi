package com.longbridge.trade;

/**
 * Real-time grid trading master-order change push event
 */
public class PushGridOrderChanged {
    private String orderId;
    private String status;
    private String symbol;
    private String suspendReason;
    private String submittedBasePrice;
    private String currentBasePrice;
    private String upperLimitPrice;
    private String lowerLimitPrice;
    private int triggerPriceType;
    private String triggerQuantity;
    private String settlementCurrency;
    private int timeInForce;
    private int rth;
    private String gridOrderTypeUp;
    private String gridOrderTypeDown;

    /**
     * Returns orderId.
     *
     * @return orderId
     */
    public String getOrderId() {
        return orderId;
    }

    /**
     * Returns status.
     *
     * @return status
     */
    public String getStatus() {
        return status;
    }

    /**
     * Returns symbol.
     *
     * @return symbol
     */
    public String getSymbol() {
        return symbol;
    }

    /**
     * Returns suspendReason.
     *
     * @return suspendReason
     */
    public String getSuspendReason() {
        return suspendReason;
    }

    /**
     * Returns submittedBasePrice.
     *
     * @return submittedBasePrice
     */
    public String getSubmittedBasePrice() {
        return submittedBasePrice;
    }

    /**
     * Returns currentBasePrice.
     *
     * @return currentBasePrice
     */
    public String getCurrentBasePrice() {
        return currentBasePrice;
    }

    /**
     * Returns upperLimitPrice.
     *
     * @return upperLimitPrice
     */
    public String getUpperLimitPrice() {
        return upperLimitPrice;
    }

    /**
     * Returns lowerLimitPrice.
     *
     * @return lowerLimitPrice
     */
    public String getLowerLimitPrice() {
        return lowerLimitPrice;
    }

    /**
     * Returns triggerPriceType.
     *
     * @return triggerPriceType
     */
    public int getTriggerPriceType() {
        return triggerPriceType;
    }

    /**
     * Returns triggerQuantity.
     *
     * @return triggerQuantity
     */
    public String getTriggerQuantity() {
        return triggerQuantity;
    }

    /**
     * Returns settlementCurrency.
     *
     * @return settlementCurrency
     */
    public String getSettlementCurrency() {
        return settlementCurrency;
    }

    /**
     * Returns timeInForce.
     *
     * @return timeInForce
     */
    public int getTimeInForce() {
        return timeInForce;
    }

    /**
     * Returns rth.
     *
     * @return rth
     */
    public int getRth() {
        return rth;
    }

    /**
     * Returns gridOrderTypeUp.
     *
     * @return gridOrderTypeUp
     */
    public String getGridOrderTypeUp() {
        return gridOrderTypeUp;
    }

    /**
     * Returns gridOrderTypeDown.
     *
     * @return gridOrderTypeDown
     */
    public String getGridOrderTypeDown() {
        return gridOrderTypeDown;
    }

    @Override
    public String toString() {
        return "PushGridOrderChanged [orderId=" + orderId +
                ", status=" + status +
                ", symbol=" + symbol +
                ", suspendReason=" + suspendReason +
                ", submittedBasePrice=" + submittedBasePrice +
                ", currentBasePrice=" + currentBasePrice +
                ", upperLimitPrice=" + upperLimitPrice +
                ", lowerLimitPrice=" + lowerLimitPrice +
                ", triggerPriceType=" + triggerPriceType +
                ", triggerQuantity=" + triggerQuantity +
                ", settlementCurrency=" + settlementCurrency +
                ", timeInForce=" + timeInForce +
                ", rth=" + rth +
                ", gridOrderTypeUp=" + gridOrderTypeUp +
                ", gridOrderTypeDown=" + gridOrderTypeDown + "]";
    }
}
