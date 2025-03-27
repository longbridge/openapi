package com.longport.quote;

public class PushCandlestick {
    private Period period;
    private Candlestick candlestick;
    private boolean isConfirmed;

    public Period getPeriod() {
        return period;
    }

    public Candlestick getCandlestick() {
        return candlestick;
    }

    public boolean isConfirmed() {
        return isConfirmed;
    }

    @Override
    public String toString() {
        return "PushCandlestick [period=" + period + ", candlestick=" + candlestick
                + ", isConfirmed=" + isConfirmed + "]";
    }

}
