package com.longbridge.grid;

import java.time.OffsetDateTime;

/**
 * A grid trigger-history entry (one triggered order)
 */
public class TriggerOrder {
    private String id;
    private String status;
    private String name;
    private String symbol;
    private String price;
    private String quantity;
    private String executedPrice;
    private String executedQty;
    private OffsetDateTime submittedAt;
    private int action;
    private String orderType;
    private String triggerPrice;
    private String msg;
    private String currency;
    private String lastDone;
    private OffsetDateTime updatedAt;
    private int timeInForce;
    private String gtd;
    private OffsetDateTime triggerAt;
    private int triggerStatus;

    /**
     * Returns id.
     *
     * @return id
     */
    public String getId() {
        return id;
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
     * Returns name.
     *
     * @return name
     */
    public String getName() {
        return name;
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
     * Returns price.
     *
     * @return price
     */
    public String getPrice() {
        return price;
    }

    /**
     * Returns quantity.
     *
     * @return quantity
     */
    public String getQuantity() {
        return quantity;
    }

    /**
     * Returns executedPrice.
     *
     * @return executedPrice
     */
    public String getExecutedPrice() {
        return executedPrice;
    }

    /**
     * Returns executedQty.
     *
     * @return executedQty
     */
    public String getExecutedQty() {
        return executedQty;
    }

    /**
     * Returns submittedAt.
     *
     * @return submittedAt
     */
    public OffsetDateTime getSubmittedAt() {
        return submittedAt;
    }

    /**
     * Returns action.
     *
     * @return action
     */
    public int getAction() {
        return action;
    }

    /**
     * Returns orderType.
     *
     * @return orderType
     */
    public String getOrderType() {
        return orderType;
    }

    /**
     * Returns triggerPrice.
     *
     * @return triggerPrice
     */
    public String getTriggerPrice() {
        return triggerPrice;
    }

    /**
     * Returns msg.
     *
     * @return msg
     */
    public String getMsg() {
        return msg;
    }

    /**
     * Returns currency.
     *
     * @return currency
     */
    public String getCurrency() {
        return currency;
    }

    /**
     * Returns lastDone.
     *
     * @return lastDone
     */
    public String getLastDone() {
        return lastDone;
    }

    /**
     * Returns updatedAt.
     *
     * @return updatedAt
     */
    public OffsetDateTime getUpdatedAt() {
        return updatedAt;
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
     * Returns gtd.
     *
     * @return gtd
     */
    public String getGtd() {
        return gtd;
    }

    /**
     * Returns triggerAt.
     *
     * @return triggerAt
     */
    public OffsetDateTime getTriggerAt() {
        return triggerAt;
    }

    /**
     * Returns triggerStatus.
     *
     * @return triggerStatus
     */
    public int getTriggerStatus() {
        return triggerStatus;
    }

    @Override
    public String toString() {
        return "TriggerOrder [id=" + id +
                ", status=" + status +
                ", name=" + name +
                ", symbol=" + symbol +
                ", price=" + price +
                ", quantity=" + quantity +
                ", executedPrice=" + executedPrice +
                ", executedQty=" + executedQty +
                ", submittedAt=" + submittedAt +
                ", action=" + action +
                ", orderType=" + orderType +
                ", triggerPrice=" + triggerPrice +
                ", msg=" + msg +
                ", currency=" + currency +
                ", lastDone=" + lastDone +
                ", updatedAt=" + updatedAt +
                ", timeInForce=" + timeInForce +
                ", gtd=" + gtd +
                ", triggerAt=" + triggerAt +
                ", triggerStatus=" + triggerStatus + "]";
    }
}
