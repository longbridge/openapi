package com.longbridge.quote;

import java.time.LocalTime;

public class TradingSessionInfo {
    private LocalTime beginTime;
    private LocalTime endTime;
    private TradeSession tradeSession;

    public LocalTime getBeginTime() {
        return beginTime;
    }

    public LocalTime getEndTime() {
        return endTime;
    }

    public TradeSession getTradeSession() {
        return tradeSession;
    }
}
