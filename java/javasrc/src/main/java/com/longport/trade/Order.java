package com.longport.trade;

import java.math.BigDecimal;
import java.time.LocalDate;
import java.time.OffsetDateTime;

public class Order {
    private String orderId;
    private OrderStatus status;
    private String stockName;
    private long quantity;
    private long executedQuantity;
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
    private String remark;

    public String getOrderId() {
        return orderId;
    }

    public OrderStatus getStatus() {
        return status;
    }

    public String getStockName() {
        return stockName;
    }

    public long getQuantity() {
        return quantity;
    }

    public long getExecutedQuantity() {
        return executedQuantity;
    }

    public BigDecimal getPrice() {
        return price;
    }

    public BigDecimal getExecutedPrice() {
        return executedPrice;
    }

    public OffsetDateTime getSubmittedAt() {
        return submittedAt;
    }

    public OrderSide getSide() {
        return side;
    }

    public String getSymbol() {
        return symbol;
    }

    public OrderType getOrderType() {
        return orderType;
    }

    public BigDecimal getLastDone() {
        return lastDone;
    }

    public BigDecimal getTriggerPrice() {
        return triggerPrice;
    }

    public String getMsg() {
        return msg;
    }

    public OrderTag getTag() {
        return tag;
    }

    public TimeInForceType getTimeInForce() {
        return timeInForce;
    }

    public LocalDate getExpireDate() {
        return expireDate;
    }

    public OffsetDateTime getUpdatedAt() {
        return updatedAt;
    }

    public OffsetDateTime getTriggerAt() {
        return triggerAt;
    }

    public BigDecimal getTrailingAmount() {
        return trailingAmount;
    }

    public BigDecimal getTrailingPercent() {
        return trailingPercent;
    }

    public BigDecimal getLimitOffset() {
        return limitOffset;
    }

    public TriggerStatus getTriggerStatus() {
        return triggerStatus;
    }

    public String getCurrency() {
        return currency;
    }

    public OutsideRTH getOutsideRth() {
        return outsideRth;
    }

    public String remark() {
        return remark;
    }

    @Override
    public String toString() {
        return "Order [currency=" + currency + ", executedPrice=" + executedPrice + ", executedQuantity="
                + executedQuantity + ", expireDate=" + expireDate + ", lastDone=" + lastDone + ", limitOffset="
                + limitOffset + ", msg=" + msg + ", orderId=" + orderId + ", orderType=" + orderType + ", outsideRth="
                + outsideRth + ", price=" + price + ", quantity=" + quantity + ", side=" + side + ", status=" + status
                + ", stockName=" + stockName + ", submittedAt=" + submittedAt + ", symbol=" + symbol + ", tag=" + tag
                + ", timeInForce=" + timeInForce + ", trailingAmount=" + trailingAmount + ", trailingPercent="
                + trailingPercent + ", triggerAt=" + triggerAt + ", triggerPrice=" + triggerPrice + ", triggerStatus="
                + triggerStatus + ", updatedAt=" + updatedAt + ", remark=" + remark + "]";
    }
}