package com.longbridge.trade;

import java.math.BigDecimal;
import java.time.LocalDate;
import java.time.OffsetDateTime;
import java.util.Arrays;

/**
 * Detailed order information
 */
public class OrderDetail {
    private String orderId;
    private OrderStatus status;
    private String stockName;
    private BigDecimal quantity;
    private BigDecimal executedQuantity;
    private BigDecimal price;
    private BigDecimal executedPrice;
    private OffsetDateTime submittedAt;
    private OrderSide side;
    private String symbol;
    private OrderType orderType;
    private BigDecimal lastDone;
    private BigDecimal triggerPrice;
    private String msg;
    private OrderTag tag;
    private TimeInForceType timeInForce;
    private LocalDate expireDate;
    private OffsetDateTime updatedAt;
    private OffsetDateTime triggerAt;
    private BigDecimal trailingAmount;
    private BigDecimal trailingPercent;
    private BigDecimal limitOffset;
    private TriggerStatus triggerStatus;
    private String currency;
    private OutsideRTH outsideRth;
    private Integer limitDepthLevel;
    private Integer triggerCount;
    private BigDecimal monitorPrice;
    private String remark;
    private CommissionFreeStatus freeStatus;
    private BigDecimal freeAmount;
    private String freeCurrency;
    private DeductionStatus deductionsStatus;
    private BigDecimal deductionsAmount;
    private String deductionsCurrency;
    private DeductionStatus platformDeductedStatus;
    private BigDecimal platformDeductedAmount;
    private String platformDeductedCurrency;
    private OrderHistoryDetail[] history;
    private OrderChargeDetail chargeDetail;
    private AttachedOrderDetail[] attachedOrders = new AttachedOrderDetail[0];

    /**
     * Returns the order ID.
     *
     * @return order ID
     */
    public String getOrderId() {
        return orderId;
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
     * Returns the security name.
     *
     * @return security name
     */
    public String getStockName() {
        return stockName;
    }

    /**
     * Returns the order quantity.
     *
     * @return order quantity
     */
    public BigDecimal getQuantity() {
        return quantity;
    }

    /**
     * Returns the executed quantity.
     *
     * @return executed quantity
     */
    public BigDecimal getExecutedQuantity() {
        return executedQuantity;
    }

    /**
     * Returns the order price.
     *
     * @return order price
     */
    public BigDecimal getPrice() {
        return price;
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
     * Returns the submission time.
     *
     * @return submission time
     */
    public OffsetDateTime getSubmittedAt() {
        return submittedAt;
    }

    /**
     * Returns the order side.
     *
     * @return order side
     */
    public OrderSide getSide() {
        return side;
    }

    /**
     * Returns the security code.
     *
     * @return security code
     */
    public String getSymbol() {
        return symbol;
    }

    /**
     * Returns the order type.
     *
     * @return order type
     */
    public OrderType getOrderType() {
        return orderType;
    }

    /**
     * Returns the last trade price.
     *
     * @return last trade price
     */
    public BigDecimal getLastDone() {
        return lastDone;
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
     * Returns the rejection or system remark message.
     *
     * @return rejection or system remark message
     */
    public String getMsg() {
        return msg;
    }

    /**
     * Returns the order tag.
     *
     * @return order tag
     */
    public OrderTag getTag() {
        return tag;
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
     * Returns the expiry date (for GoodTilDate orders).
     *
     * @return expiry date
     */
    public LocalDate getExpireDate() {
        return expireDate;
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
     * Returns the trigger time.
     *
     * @return trigger time
     */
    public OffsetDateTime getTriggerAt() {
        return triggerAt;
    }

    /**
     * Returns the trailing amount.
     *
     * @return trailing amount
     */
    public BigDecimal getTrailingAmount() {
        return trailingAmount;
    }

    /**
     * Returns the trailing percentage.
     *
     * @return trailing percentage
     */
    public BigDecimal getTrailingPercent() {
        return trailingPercent;
    }

    /**
     * Returns the limit offset.
     *
     * @return limit offset
     */
    public BigDecimal getLimitOffset() {
        return limitOffset;
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
     * Returns the currency.
     *
     * @return currency
     */
    public String getCurrency() {
        return currency;
    }

    /**
     * Returns the outside-RTH setting.
     *
     * @return outside-RTH setting
     */
    public OutsideRTH getOutsideRth() {
        return outsideRth;
    }

    /**
     * Returns the limit depth level.
     *
     * @return limit depth level
     */
    public Integer getLimitDepthLevel() {
        return limitDepthLevel;
    }

    /**
     * Returns the trigger count.
     *
     * @return trigger count
     */
    public Integer getTriggerCount() {
        return triggerCount;
    }

    /**
     * Returns the monitor price.
     *
     * @return monitor price
     */
    public BigDecimal getMonitorPrice() {
        return monitorPrice;
    }

    /**
     * Returns the remark.
     *
     * @return remark
     */
    public String getRemark() {
        return remark;
    }

    /**
     * Returns the commission-free status.
     *
     * @return commission-free status
     */
    public CommissionFreeStatus getFreeStatus() {
        return freeStatus;
    }

    /**
     * Returns the commission-free amount.
     *
     * @return commission-free amount
     */
    public BigDecimal getFreeAmount() {
        return freeAmount;
    }

    /**
     * Returns the commission-free currency.
     *
     * @return commission-free currency
     */
    public String getFreeCurrency() {
        return freeCurrency;
    }

    /**
     * Returns the deductions status.
     *
     * @return deductions status
     */
    public DeductionStatus getDeductionsStatus() {
        return deductionsStatus;
    }

    /**
     * Returns the deductions amount.
     *
     * @return deductions amount
     */
    public BigDecimal getDeductionsAmount() {
        return deductionsAmount;
    }

    /**
     * Returns the deductions currency.
     *
     * @return deductions currency
     */
    public String getDeductionsCurrency() {
        return deductionsCurrency;
    }

    /**
     * Returns the platform deducted status.
     *
     * @return platform deducted status
     */
    public DeductionStatus getPlatformDeductedStatus() {
        return platformDeductedStatus;
    }

    /**
     * Returns the platform deducted amount.
     *
     * @return platform deducted amount
     */
    public BigDecimal getPlatformDeductedAmount() {
        return platformDeductedAmount;
    }

    /**
     * Returns the platform deducted currency.
     *
     * @return platform deducted currency
     */
    public String getPlatformDeductedCurrency() {
        return platformDeductedCurrency;
    }

    /**
     * Returns the order status history.
     *
     * @return order status history
     */
    public OrderHistoryDetail[] getHistory() {
        return history;
    }

    /**
     * Returns the order charge detail.
     *
     * @return order charge detail
     */
    public OrderChargeDetail getChargeDetail() {
        return chargeDetail;
    }

    /**
     * Returns the attached orders.
     *
     * @return attached orders
     */
    public AttachedOrderDetail[] getAttachedOrders() {
        return attachedOrders;
    }

    @Override
    public String toString() {
        return "OrderDetail [orderId=" + orderId + ", status=" + status + ", stockName=" + stockName + ", quantity="
                + quantity + ", executedQuantity=" + executedQuantity + ", price=" + price + ", executedPrice="
                + executedPrice + ", submittedAt=" + submittedAt + ", side=" + side + ", symbol=" + symbol
                + ", orderType=" + orderType + ", lastDone=" + lastDone + ", triggerPrice=" + triggerPrice + ", msg="
                + msg + ", tag=" + tag + ", timeInForce=" + timeInForce + ", expireDate=" + expireDate + ", updatedAt="
                + updatedAt + ", triggerAt=" + triggerAt + ", trailingAmount=" + trailingAmount + ", trailingPercent="
                + trailingPercent + ", limitOffset=" + limitOffset + ", triggerStatus=" + triggerStatus + ", currency="
                + currency + ", outsideRth=" + outsideRth + ", limitDepthLevel=" + limitDepthLevel + ", triggerCount="
                + triggerCount + ", monitorPrice=" + monitorPrice + ", remark=" + remark + ", freeStatus=" + freeStatus
                + ", freeAmount=" + freeAmount + ", freeCurrency=" + freeCurrency + ", deductionsStatus="
                + deductionsStatus + ", deductionsAmount=" + deductionsAmount + ", deductionsCurrency="
                + deductionsCurrency + ", platformDeductedStatus=" + platformDeductedStatus
                + ", platformDeductedAmount=" + platformDeductedAmount + ", platformDeductedCurrency="
                + platformDeductedCurrency + ", history=" + Arrays.toString(history) + ", chargeDetail=" + chargeDetail
                + "]";
    }

}