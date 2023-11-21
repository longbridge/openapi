package com.longport.trade;

import java.math.BigDecimal;

@SuppressWarnings("unused")
public class EstimateMaxPurchaseQuantityOptions {
    private String symbol;
    private OrderType orderType;
    private OrderSide side;
    private BigDecimal price;
    private String currency;
    private String orderId;

    public EstimateMaxPurchaseQuantityOptions(String symbol, OrderType orderType, OrderSide side) {
        this.symbol = symbol;
        this.orderType = orderType;
        this.side = side;
    }

    public EstimateMaxPurchaseQuantityOptions setPrice(BigDecimal price) {
        this.price = price;
        return this;
    }

    public EstimateMaxPurchaseQuantityOptions setCurrency(String currency) {
        this.currency = currency;
        return this;
    }

    public EstimateMaxPurchaseQuantityOptions setOrderId(String orderId) {
        this.orderId = orderId;
        return this;
    }
}
