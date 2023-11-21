package com.longport.trade;

import com.longport.Market;

@SuppressWarnings("unused")
public class GetTodayOrdersOptions {
    private String symbol;
    private OrderStatus[] status;
    private OrderSide side;
    private Market market;
    private String orderId;

    public GetTodayOrdersOptions setSymbol(String symbol) {
        this.symbol = symbol;
        return this;
    }

    public GetTodayOrdersOptions setStatus(OrderStatus[] status) {
        this.status = status;
        return this;
    }

    public GetTodayOrdersOptions setSide(OrderSide side) {
        this.side = side;
        return this;
    }

    public GetTodayOrdersOptions setMarket(Market market) {
        this.market = market;
        return this;
    }

    public GetTodayOrdersOptions setOrderId(String orderId) {
        this.orderId = orderId;
        return this;
    }

}
