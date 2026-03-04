package com.longbridge.quote;

import java.math.BigDecimal;
import java.time.OffsetDateTime;

public class Trade {
    private BigDecimal price;
    private long volume;
    private OffsetDateTime timestamp;
    private String tradeType;
    private TradeDirection direction;
    private TradeSession tradeSession;

    public BigDecimal getPrice() {
        return price;
    }

    public long getVolume() {
        return volume;
    }

    public OffsetDateTime getTimestamp() {
        return timestamp;
    }

    public String getTradeType() {
        return tradeType;
    }

    public TradeDirection getDirection() {
        return direction;
    }

    public TradeSession getTradeSession() {
        return tradeSession;
    }

    @Override
    public String toString() {
        return "Trade [direction=" + direction + ", price=" + price + ", timestamp=" + timestamp + ", tradeType="
                + tradeType + ", tradeSession=" + tradeSession + ", volume=" + volume + "]";
    }
}
