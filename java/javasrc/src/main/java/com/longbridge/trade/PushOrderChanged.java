package com.longbridge.trade;

import java.math.BigDecimal;
import java.time.OffsetDateTime;

/**
 * Real-time order change push event
 */
public class PushOrderChanged {
    private OrderSide side;
    private String stockName;
    private BigDecimal submittedQuantity;
    private String symbol;
    private OrderType orderType;
    private BigDecimal submittedPrice;
    private BigDecimal executedQuantity;
    private BigDecimal executedPrice;
    private String orderId;
    private String currency;
    private OrderStatus status;
    private OffsetDateTime submittedAt;
    private OffsetDateTime updatedAt;
    private BigDecimal triggerPrice;
    private String msg;
    private OrderTag tag;
    private TriggerStatus triggerStatus;
    private OffsetDateTime triggerAt;
    private BigDecimal trailingAmount;
    private BigDecimal trailingPercent;
    private BigDecimal limitOffset;
    private String accountNo;
    private BigDecimal lastShare;
    private BigDecimal lastPrice;
    private String remark;
    private MultiLegInfo multiLeg;

    /**
     * Returns the order side.
     *
     * @return order side
     */
    public OrderSide getSide() {
        return side;
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
     * Returns the submitted quantity.
     *
     * @return submitted quantity
     */
    public BigDecimal getSubmittedQuantity() {
        return submittedQuantity;
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
     * Returns the submitted price.
     *
     * @return submitted price
     */
    public BigDecimal getSubmittedPrice() {
        return submittedPrice;
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
     * Returns the executed price.
     *
     * @return executed price
     */
    public BigDecimal getExecutedPrice() {
        return executedPrice;
    }

    /**
     * Returns the order ID.
     *
     * @return order ID
     */
    public String getOrderId() {
        return orderId;
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
     * Returns the order status.
     *
     * @return order status
     */
    public OrderStatus getStatus() {
        return status;
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
     * Returns the last update time.
     *
     * @return last update time
     */
    public OffsetDateTime getUpdatedAt() {
        return updatedAt;
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
     * Returns the rejection message.
     *
     * @return rejection message
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
     * Returns the trigger status.
     *
     * @return trigger status
     */
    public TriggerStatus getTriggerStatus() {
        return triggerStatus;
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
     * Returns the account number.
     *
     * @return account number
     */
    public String getAccountNo() {
        return accountNo;
    }

    /**
     * Returns the last fill quantity.
     *
     * @return last fill quantity
     */
    public BigDecimal getLastShare() {
        return lastShare;
    }

    /**
     * Returns the last fill price.
     *
     * @return last fill price
     */
    public BigDecimal getLastPrice() {
        return lastPrice;
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
     * Returns the multi-leg strategy information (only present for multi-leg
     * option combination orders).
     *
     * @return multi-leg strategy information, or {@code null}
     */
    public MultiLegInfo getMultiLeg() {
        return multiLeg;
    }

    @Override
    public String toString() {
        return "PushOrderChanged [side=" + side + ", stockName=" + stockName + ", submittedQuantity="
                + submittedQuantity + ", symbol=" + symbol + ", orderType=" + orderType + ", submittedPrice="
                + submittedPrice + ", executedQuantity=" + executedQuantity + ", executedPrice=" + executedPrice
                + ", orderId=" + orderId + ", currency=" + currency + ", status=" + status + ", submittedAt="
                + submittedAt + ", updatedAt=" + updatedAt + ", triggerPrice=" + triggerPrice + ", msg=" + msg
                + ", tag=" + tag + ", triggerStatus=" + triggerStatus + ", triggerAt=" + triggerAt + ", trailingAmount="
                + trailingAmount + ", trailingPercent=" + trailingPercent + ", limitOffset=" + limitOffset
                + ", accountNo=" + accountNo + ", lastShare=" + lastShare + ", lastPrice=" + lastPrice + ", remark="
                + remark + ", multiLeg=" + multiLeg + "]";
    }

}
