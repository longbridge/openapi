package com.longbridge.quote;

public interface QuoteHandler {
    void onQuote(String symbol, PushQuote event);
}
