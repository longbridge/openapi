package com.longbridge.quote;

public interface BrokersHandler {
    void onBrokers(String symbol, PushBrokers event);
}
