package com.longbridge.trade;

import java.time.OffsetDateTime;

import com.longbridge.Market;

@SuppressWarnings("unused")
public class GetHistoryOrdersOptions {
    private String symbol;
    private OrderStatus[] status;
    private OrderSide side;
    private Market market;
    private OffsetDateTime startAt;
    private OffsetDateTime endAt;

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

    public void setStartAt(OffsetDateTime startAt) {
        this.startAt = startAt;
    }

    public void setEndAt(OffsetDateTime endAt) {
        this.endAt = endAt;
    }
}
