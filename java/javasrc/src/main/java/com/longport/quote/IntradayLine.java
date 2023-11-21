package com.longport.quote;

import java.math.BigDecimal;
import java.time.OffsetDateTime;

public class IntradayLine {
    private BigDecimal price;
    private OffsetDateTime timestamp;
    private long volume;
    private BigDecimal turnover;
    private BigDecimal avgPrice;

    public BigDecimal getPrice() {
        return price;
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

    public BigDecimal getAvgPrice() {
        return avgPrice;
    }

    @Override
    public String toString() {
        return "IntradayLine [avgPrice=" + avgPrice + ", price=" + price + ", timestamp=" + timestamp + ", turnover="
                + turnover + ", volume=" + volume + "]";
    }
}
