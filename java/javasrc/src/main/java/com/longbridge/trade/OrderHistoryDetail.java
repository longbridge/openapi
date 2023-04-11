package com.longbridge.trade;

import java.math.BigDecimal;
import java.time.OffsetDateTime;

public class OrderHistoryDetail {
    private BigDecimal price;
    private long quantity;
    private OrderStatus status;
    private String msg;
    private OffsetDateTime time;

    public BigDecimal getPrice() {
        return price;
    }

    public long getQuantity() {
        return quantity;
    }

    public OrderStatus getStatus() {
        return status;
    }

    public String getMsg() {
        return msg;
    }

    public OffsetDateTime getTime() {
        return time;
    }

    @Override
    public String toString() {
        return "OrderHistoryDetail [price=" + price + ", quantity=" + quantity + ", status=" + status + ", msg=" + msg
                + ", time=" + time + "]";
    }

}
