package com.longbridge.quote;

public interface CandlestickHandler {
    void onCandlestick(String symbol, PushCandlestick event);
}
