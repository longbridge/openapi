package com.longbridge.quote;

import java.math.BigDecimal;
import java.time.OffsetDateTime;

public class Candlestick {
    private BigDecimal close;
    private BigDecimal open;
    private BigDecimal low;
    private BigDecimal high;
    private long volume;
    private BigDecimal turnover;
    private OffsetDateTime timestamp;
    private TradeSession tradeSession;

    public BigDecimal getClose() {
        return close;
    }

    public BigDecimal getOpen() {
        return open;
    }

    public BigDecimal getLow() {
        return low;
    }

    public BigDecimal getHigh() {
        return high;
    }

    public long getVolume() {
        return volume;
    }

    public BigDecimal getTurnover() {
        return turnover;
    }

    public OffsetDateTime getTimestamp() {
        return timestamp;
    }

    public TradeSession getTradeSession() {
        return tradeSession;
    }

    @Override
    public String toString() {
        return "Candlestick [close=" + close + ", open=" + open + ", low=" + low + ", high=" + high + ", volume="
                + volume + ", turnover=" + turnover + ", timestamp=" + timestamp + ", tradeSession=" + tradeSession
                + "]";
    }

}
