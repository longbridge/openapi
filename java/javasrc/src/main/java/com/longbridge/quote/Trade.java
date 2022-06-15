package com.longbridge.quote;

import java.math.BigDecimal;
import java.time.OffsetDateTime;

public class Trade {
    private BigDecimal price;
    private long volume;
    private OffsetDateTime timestamp;
    private String tradeType;
    private TradeDirection direction;
    private TradeSession trade_sessions;

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

    public TradeSession getTrade_sessions() {
        return trade_sessions;
    }
}
