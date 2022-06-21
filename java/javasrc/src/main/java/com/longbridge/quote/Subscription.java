package com.longbridge.quote;

import java.util.Arrays;

public class Subscription {
    private String symbol;
    private int subTypes;
    private Period[] candlesticks;

    public String getSymbol() {
        return symbol;
    }

    public int getSubTypes() {
        return subTypes;
    }

    public Period[] getCandlesticks() {
        return candlesticks;
    }

    @Override
    public String toString() {
        return "Subscription [candlesticks=" + Arrays.toString(candlesticks) + ", subTypes=" + subTypes + ", symbol="
                + symbol + "]";
    }
}
