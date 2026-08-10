package com.longbridge.trade;

import java.time.OffsetDateTime;

/**
 * A triggered sub-order carried in the grid order detail
 */
public class GridOrderSubOrder {
    private String id;
    private String price;
    private String orderType;
    private String quantity;
    private String executedQty;
    private int action;
    private String status;
    private OffsetDateTime submittedAt;
    private int rth;

    /**
     * Returns id.
     *
     * @return id
     */
    public String getId() {
        return id;
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
     * Returns orderType.
     *
     * @return orderType
     */
    public String getOrderType() {
        return orderType;
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
     * Returns executedQty.
     *
     * @return executedQty
     */
    public String getExecutedQty() {
        return executedQty;
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
     * Returns status.
     *
     * @return status
     */
    public String getStatus() {
        return status;
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
     * Returns rth.
     *
     * @return rth
     */
    public int getRth() {
        return rth;
    }

    @Override
    public String toString() {
        return "GridOrderSubOrder [id=" + id +
                ", price=" + price +
                ", orderType=" + orderType +
                ", quantity=" + quantity +
                ", executedQty=" + executedQty +
                ", action=" + action +
                ", status=" + status +
                ", submittedAt=" + submittedAt +
                ", rth=" + rth + "]";
    }
}
