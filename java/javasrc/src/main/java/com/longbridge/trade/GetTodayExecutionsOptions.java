package com.longbridge.trade;

@SuppressWarnings("unused")
public class GetTodayExecutionsOptions {
    private String symbol;
    private String orderId;

    public GetTodayExecutionsOptions setSymbol(String symbol) {
        this.symbol = symbol;
        return this;
    }

    public GetTodayExecutionsOptions setOrderId(String orderId) {
        this.orderId = orderId;
        return this;
    }
}
