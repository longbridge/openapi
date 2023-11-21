package com.longport.quote;

public interface BrokersHandler {
    void onBrokers(String symbol, PushBrokers event);
}
