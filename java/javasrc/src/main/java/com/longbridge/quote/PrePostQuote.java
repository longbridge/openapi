package com.longbridge.quote;

import java.math.BigDecimal;
import java.time.OffsetDateTime;

public class PrePostQuote {
    private BigDecimal lastDone;
    private OffsetDateTime timestamp;
    private long volume;
    private BigDecimal turnover;
    private BigDecimal high;
    private BigDecimal low;
    private BigDecimal prevClose;

    public BigDecimal getLastDone() {
        return lastDone;
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

    public BigDecimal getHigh() {
        return high;
    }

    public BigDecimal getLow() {
        return low;
    }

    public BigDecimal getPrevClose() {
        return prevClose;
    }

    @Override
    public String toString() {
        return "PrePostQuote [high=" + high + ", lastDone=" + lastDone + ", low=" + low + ", prevClose=" + prevClose
                + ", timestamp=" + timestamp + ", turnover=" + turnover + ", volume=" + volume + "]";
    }
}
