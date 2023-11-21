package com.longport.trade;

import java.time.OffsetDateTime;

@SuppressWarnings("unused")
public class GetCashFlowOptions {
    private OffsetDateTime startAt;
    private OffsetDateTime endAt;
    private BalanceType businessType;
    private String symbol;
    private int page;
    private int size;

    public GetCashFlowOptions(OffsetDateTime startAt, OffsetDateTime endAt) {
        this.startAt = startAt;
        this.endAt = endAt;
    }

    public BalanceType getBusinessType() {
        return businessType;
    }

    public GetCashFlowOptions setBusinessType(BalanceType businessType) {
        this.businessType = businessType;
        return this;
    }

    public GetCashFlowOptions setSymbol(String symbol) {
        this.symbol = symbol;
        return this;
    }

    public GetCashFlowOptions setPage(int page) {
        this.page = page;
        return this;
    }

    public GetCashFlowOptions setSize(int size) {
        this.size = size;
        return this;
    }

}
