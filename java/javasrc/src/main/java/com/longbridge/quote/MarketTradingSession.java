package com.longbridge.quote;

import com.longbridge.Market;

public class MarketTradingSession {
    private Market market;
    private TradingSessionInfo[] tradeSession;

    public Market getMarket() {
        return market;
    }

    public TradingSessionInfo[] getTradeSession() {
        return tradeSession;
    }
}
