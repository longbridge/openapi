package com.longbridge.trade;

import java.time.OffsetDateTime;

@SuppressWarnings("unused")
public class GetHistoryExecutionsOptions {
    private String symbol;
    private OffsetDateTime startAt;
    private OffsetDateTime endAt;

    public void setSymbol(String symbol) {
        this.symbol = symbol;
    }

    public void setStartAt(OffsetDateTime startAt) {
        this.startAt = startAt;
    }

    public void setEndAt(OffsetDateTime endAt) {
        this.endAt = endAt;
    }
}
