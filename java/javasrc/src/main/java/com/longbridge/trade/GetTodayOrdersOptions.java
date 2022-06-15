package com.longbridge.trade;

import com.longbridge.Market;

@SuppressWarnings("unused")
public class GetTodayOrdersOptions {
    private String symbol;
    private OrderStatus[] status;
    private OrderSide side;
    private Market market;

    public void setSymbol(String symbol) {
        this.symbol = symbol;
    }

    public void setStatus(OrderStatus[] status) {
        this.status = status;
    }

    public void setSide(OrderSide side) {
        this.side = side;
    }

    public void setMarket(Market market) {
        this.market = market;
    }

}
