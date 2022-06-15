package com.longbridge.trade;

@SuppressWarnings("unused")
public class GetTodayExecutionsOptions {
    private String symbol;
    private String orderId;

    public void setSymbol(String symbol) {
        this.symbol = symbol;
    }

    public void setOrderId(String orderId) {
        this.orderId = orderId;
    }
}
