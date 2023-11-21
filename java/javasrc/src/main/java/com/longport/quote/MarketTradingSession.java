package com.longport.quote;

import java.util.Arrays;

import com.longport.Market;

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
