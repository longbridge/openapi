package com.longport.trade;

import java.math.BigDecimal;
import java.time.LocalDate;
import java.time.OffsetDateTime;
import java.util.Arrays;

public class OrderDetail {
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

    public String getRemark() {
        return remark;
    }

    public CommissionFreeStatus getFreeStatus() {
        return freeStatus;
    }

    public BigDecimal getFreeAmount() {
        return freeAmount;
    }

    public String getFreeCurrency() {
        return freeCurrency;
    }

    public DeductionStatus getDeductionsStatus() {
        return deductionsStatus;
    }

    public BigDecimal getDeductionsAmount() {
        return deductionsAmount;
    }

    public String getDeductionsCurrency() {
        return deductionsCurrency;
    }

    public DeductionStatus getPlatformDeductedStatus() {
        return platformDeductedStatus;
    }

    public BigDecimal getPlatformDeductedAmount() {
        return platformDeductedAmount;
    }

    public String getPlatformDeductedCurrency() {
        return platformDeductedCurrency;
    }

    public OrderHistoryDetail[] getHistory() {
        return history;
    }

    public OrderChargeDetail getChargeDetail() {
        return chargeDetail;
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
                + currency + ", outsideRth=" + outsideRth + ", remark=" + remark + ", freeStatus=" + freeStatus
                + ", freeAmount=" + freeAmount + ", freeCurrency=" + freeCurrency + ", deductionsStatus="
                + deductionsStatus + ", deductionsAmount=" + deductionsAmount + ", deductionsCurrency="
                + deductionsCurrency + ", platformDeductedStatus=" + platformDeductedStatus
                + ", platformDeductedAmount=" + platformDeductedAmount + ", platformDeductedCurrency="
                + platformDeductedCurrency + ", history=" + Arrays.toString(history) + ", chargeDetail=" + chargeDetail
                + "]";
    }

}