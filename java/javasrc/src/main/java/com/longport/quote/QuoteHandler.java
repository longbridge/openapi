package com.longport.quote;

public interface QuoteHandler {
    void onQuote(String symbol, PushQuote event);
}
