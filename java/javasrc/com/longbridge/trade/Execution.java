package com.longbridge.trade;

import java.math.BigDecimal;
import java.time.OffsetDateTime;

public class Execution {
    private String orderId;
    private String tradeId;
    private String symbol;
    private OffsetDateTime tradeDoneAt;
    private long quantity;
    private BigDecimal price;

    public String getOrderId() {
        return orderId;
    }

    public String getTradeId() {
        return tradeId;
    }

    public String getSymbol() {
        return symbol;
    }

    public OffsetDateTime getTradeDoneAt() {
        return tradeDoneAt;
    }

    public long getQuantity() {
        return quantity;
    }

    public BigDecimal getPrice() {
        return price;
    }
}
