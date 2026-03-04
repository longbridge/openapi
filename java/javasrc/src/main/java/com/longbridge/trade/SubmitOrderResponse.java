package com.longbridge.trade;

public class SubmitOrderResponse {
    private String orderId;

    public String getOrderId() {
        return orderId;
    }

    @Override
    public String toString() {
        return "SubmitOrderResponse [orderId=" + orderId + "]";
    }
}
