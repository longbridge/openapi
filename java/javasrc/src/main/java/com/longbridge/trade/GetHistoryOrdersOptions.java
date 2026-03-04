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

    public GetHistoryOrdersOptions setSymbol(String symbol) {
        this.symbol = symbol;
        return this;
    }

    public GetHistoryOrdersOptions setStatus(OrderStatus[] status) {
        this.status = status;
        return this;
    }

    public GetHistoryOrdersOptions setSide(OrderSide side) {
        this.side = side;
        return this;
    }

    public GetHistoryOrdersOptions setMarket(Market market) {
        this.market = market;
        return this;
    }

    public GetHistoryOrdersOptions setStartAt(OffsetDateTime startAt) {
        this.startAt = startAt;
        return this;
    }

    public GetHistoryOrdersOptions setEndAt(OffsetDateTime endAt) {
        this.endAt = endAt;
        return this;
    }
}
