package com.longport.quote;

import java.math.BigDecimal;
import java.time.OffsetDateTime;

public class RealtimeQuote {
    private String symbol;
    private BigDecimal lastDone;
    private BigDecimal open;
    private BigDecimal high;
    private BigDecimal low;
    private OffsetDateTime timestamp;
    private long volume;
    private BigDecimal turnover;
    private TradeStatus tradeStatus;

    public String getSymbol() {
        return symbol;
    }

    public BigDecimal getLastDone() {
        return lastDone;
    }

    public BigDecimal getOpen() {
        return open;
    }

    public BigDecimal getHigh() {
        return high;
    }

    public BigDecimal getLow() {
        return low;
    }

    public OffsetDateTime getTimestamp() {
        return timestamp;
    }

    public long getVolume() {
        return volume;
    }

    public BigDecimal getTurnover() {
        return turnover;
    }

    public TradeStatus getTradeStatus() {
        return tradeStatus;
    }

    @Override
    public String toString() {
        return "RealtimeQuote [high=" + high + ", lastDone=" + lastDone + ", low=" + low + ", open=" + open
                + ", symbol=" + symbol + ", timestamp=" + timestamp + ", tradeStatus=" + tradeStatus + ", turnover="
                + turnover + ", volume=" + volume + "]";
    }
}
