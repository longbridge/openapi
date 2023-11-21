package com.longport.quote;

import java.math.BigDecimal;
import java.time.OffsetDateTime;

public class PushQuote {
    private BigDecimal lastDone;
    private BigDecimal open;
    private BigDecimal high;
    private BigDecimal low;
    private OffsetDateTime timestamp;
    private long volume;
    private BigDecimal turnover;
    private TradeStatus tradeStatus;
    private TradeSession tradeSession;

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

    public TradeSession getTradeSession() {
        return tradeSession;
    }

    @Override
    public String toString() {
        return "PushQuote [high=" + high + ", lastDone=" + lastDone + ", low=" + low + ", open=" + open + ", timestamp="
                + timestamp + ", tradeSession=" + tradeSession + ", tradeStatus=" + tradeStatus + ", turnover="
                + turnover + ", volume=" + volume + "]";
    }

}
