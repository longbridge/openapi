package com.longbridge.trade;

import java.math.BigDecimal;
import java.time.LocalDate;
import java.time.OffsetDateTime;

/**
 * Attached order detail
 */
public class AttachedOrderDetail {
    private String orderId;
    private AttachedOrderType attachedTypeDisplay;
    private BigDecimal triggerPrice;
    private BigDecimal quantity;
    private BigDecimal executedQty;
    private OrderStatus status;
    private OffsetDateTime updatedAt;
    private boolean withdrawn;
    private LocalDate gtd;
    private TimeInForceType timeInForce;
    private String counterId;
    private TriggerStatus triggerStatus;
    private BigDecimal executedAmount;
    private OrderTag tag;
    private OffsetDateTime submittedAt;
    private BigDecimal executedPrice;
    private OutsideRTH forceOnlyRth;
    private boolean reviewed;
    private OrderType activateOrderType;
    private OutsideRTH activateRth;
    private BigDecimal submitPrice;

    /**
     * Returns the order ID.
     *
     * @return order ID
     */
    public String getOrderId() {
        return orderId;
    }

    /**
     * Returns the attached type display (1=take-profit, 2=stop-loss).
     *
     * @return attached type display
     */
    public AttachedOrderType getAttachedTypeDisplay() {
        return attachedTypeDisplay;
    }

    /**
     * Returns the trigger price.
     *
     * @return trigger price
     */
    public BigDecimal getTriggerPrice() {
        return triggerPrice;
    }

    /**
     * Returns the quantity.
     *
     * @return quantity
     */
    public BigDecimal getQuantity() {
        return quantity;
    }

    /**
     * Returns the executed quantity.
     *
     * @return executed quantity
     */
    public BigDecimal getExecutedQty() {
        return executedQty;
    }

    /**
     * Returns the order status.
     *
     * @return order status
     */
    public OrderStatus getStatus() {
        return status;
    }

    /**
     * Returns the last update time.
     *
     * @return last update time
     */
    public OffsetDateTime getUpdatedAt() {
        return updatedAt;
    }

    /**
     * Returns whether the order has been withdrawn.
     *
     * @return withdrawn flag
     */
    public boolean isWithdrawn() {
        return withdrawn;
    }

    /**
     * Returns the good-till date.
     *
     * @return good-till date
     */
    public LocalDate getGtd() {
        return gtd;
    }

    /**
     * Returns the time-in-force type.
     *
     * @return time-in-force type
     */
    public TimeInForceType getTimeInForce() {
        return timeInForce;
    }

    /**
     * Returns the counter ID.
     *
     * @return counter ID
     */
    public String getCounterId() {
        return counterId;
    }

    /**
     * Returns the trigger status.
     *
     * @return trigger status
     */
    public TriggerStatus getTriggerStatus() {
        return triggerStatus;
    }

    /**
     * Returns the executed amount.
     *
     * @return executed amount
     */
    public BigDecimal getExecutedAmount() {
        return executedAmount;
    }

    /**
     * Returns the tag.
     *
     * @return tag
     */
    public OrderTag getTag() {
        return tag;
    }

    /**
     * Returns the submission time.
     *
     * @return submission time
     */
    public OffsetDateTime getSubmittedAt() {
        return submittedAt;
    }

    /**
     * Returns the executed price.
     *
     * @return executed price
     */
    public BigDecimal getExecutedPrice() {
        return executedPrice;
    }

    /**
     * Returns the force-only-RTH setting.
     *
     * @return force-only-RTH setting
     */
    public OutsideRTH getForceOnlyRth() {
        return forceOnlyRth;
    }

    /**
     * Returns whether the order has been reviewed.
     *
     * @return reviewed flag
     */
    public boolean isReviewed() {
        return reviewed;
    }

    /**
     * Returns the activate order type.
     *
     * @return activate order type
     */
    public OrderType getActivateOrderType() {
        return activateOrderType;
    }

    /**
     * Returns the activate RTH setting.
     *
     * @return activate RTH setting
     */
    public OutsideRTH getActivateRth() {
        return activateRth;
    }

    /**
     * Returns the submit price.
     *
     * @return submit price
     */
    public BigDecimal getSubmitPrice() {
        return submitPrice;
    }

    @Override
    public String toString() {
        return "AttachedOrderDetail [orderId=" + orderId + ", attachedTypeDisplay=" + attachedTypeDisplay
                + ", triggerPrice=" + triggerPrice + ", quantity=" + quantity + ", executedQty=" + executedQty
                + ", status=" + status + ", updatedAt=" + updatedAt + ", withdrawn=" + withdrawn + ", gtd=" + gtd
                + ", timeInForce=" + timeInForce + ", counterId=" + counterId + ", triggerStatus=" + triggerStatus
                + ", executedAmount=" + executedAmount + ", tag=" + tag + ", submittedAt=" + submittedAt
                + ", executedPrice=" + executedPrice + ", forceOnlyRth=" + forceOnlyRth + ", reviewed=" + reviewed
                + ", activateOrderType=" + activateOrderType + ", activateRth=" + activateRth + ", submitPrice="
                + submitPrice + "]";
    }
}
