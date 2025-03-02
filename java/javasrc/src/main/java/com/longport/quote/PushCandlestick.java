package com.longport.quote;

public class PushCandlestick {
    private TradeSession tradeSession;
    private Period period;
    private Candlestick candlestick;
    private boolean isConfirmed;

    public TradeSession getTradeSession() {
        return tradeSession;
    }

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
        return "PushCandlestick [tradeSession=" + tradeSession + ", period=" + period + ", candlestick=" + candlestick
                + ", isConfirmed=" + isConfirmed + "]";
    }

}
