package com.longbridge.trade;

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

    public void setBusinessType(BalanceType businessType) {
        this.businessType = businessType;
    }

    public void setSymbol(String symbol) {
        this.symbol = symbol;
    }

    public void setPage(int page) {
        this.page = page;
    }

    public void setSize(int size) {
        this.size = size;
    }

}
