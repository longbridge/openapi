package com.longport.quote;

public interface CandlestickHandler {
    void onCandlestick(String symbol, PushCandlestick event);
}
