package com.longport.quote;

public interface TradesHandler {
    void onTrades(String symbol, PushTrades event);
}
