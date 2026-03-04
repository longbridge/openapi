package com.longbridge.quote;

import java.util.Arrays;

import com.longbridge.Market;

public class MarketTradingSession {
    private Market market;
    private TradingSessionInfo[] tradeSessions;

    public Market getMarket() {
        return market;
    }

    public TradingSessionInfo[] getTradeSessions() {
        return tradeSessions;
    }

    @Override
    public String toString() {
        return "MarketTradingSession [market=" + market + ", tradeSession=" + Arrays.toString(tradeSessions) + "]";
    }
}
