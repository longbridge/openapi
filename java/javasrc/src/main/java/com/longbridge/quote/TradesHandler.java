package com.longbridge.quote;

public interface TradesHandler {
    void onTrades(String symbol, PushTrades event);
}
