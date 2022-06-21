package com.longbridge.quote;

public class PushCandlestick {
    private Period period;
    private Candlestick candlestick;

    public Period getPeriod() {
        return period;
    }

    public Candlestick getCandlestick() {
        return candlestick;
    }

    @Override
    public String toString() {
        return "PushCandlestick [candlestick=" + candlestick + ", period=" + period + "]";
    }
}
