package com.longbridge.trade;

import java.math.BigDecimal;
import java.time.OffsetDateTime;

public class PushOrderChanged {
    private OrderSide side;
    private String stockName;
    private long submittedQuantity;
    private String symbol;
    private OrderType orderType;
    private BigDecimal submittedPrice;
    private long executedQuantity;
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

    public OrderSide getSide() {
        return side;
    }

    public String getStockName() {
        return stockName;
    }

    public long getSubmittedQuantity() {
        return submittedQuantity;
    }

    public String getSymbol() {
        return symbol;
    }

    public OrderType getOrderType() {
        return orderType;
    }

    public BigDecimal getSubmittedPrice() {
        return submittedPrice;
    }

    public long getExecutedQuantity() {
        return executedQuantity;
    }

    public BigDecimal getExecutedPrice() {
        return executedPrice;
    }

    public String getOrderId() {
        return orderId;
    }

    public String getCurrency() {
        return currency;
    }

    public OrderStatus getStatus() {
        return status;
    }

    public OffsetDateTime getSubmittedAt() {
        return submittedAt;
    }

    public OffsetDateTime getUpdatedAt() {
        return updatedAt;
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

    public TriggerStatus getTriggerStatus() {
        return triggerStatus;
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

    public String getAccountNo() {
        return accountNo;
    }

    public BigDecimal getLastPrice() {
        return lastPrice;
    }

    public BigDecimal getLastShare() {
        return lastShare;
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
                + ", accountNo=" + accountNo + ", lastShare=" + lastShare + ", lastPrice=" + lastPrice + "]";
    }

}
