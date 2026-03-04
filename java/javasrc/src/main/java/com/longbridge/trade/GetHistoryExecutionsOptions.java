package com.longbridge.trade;

import java.time.OffsetDateTime;

@SuppressWarnings("unused")
public class GetHistoryExecutionsOptions {
    private String symbol;
    private OffsetDateTime startAt;
    private OffsetDateTime endAt;

    public GetHistoryExecutionsOptions setSymbol(String symbol) {
        this.symbol = symbol;
        return this;
    }

    public GetHistoryExecutionsOptions setStartAt(OffsetDateTime startAt) {
        this.startAt = startAt;
        return this;
    }

    public GetHistoryExecutionsOptions setEndAt(OffsetDateTime endAt) {
        this.endAt = endAt;
        return this;
    }
}
