package com.longport.quote;

import java.math.BigDecimal;
import java.time.OffsetDateTime;

public class CapitalFlowLine {
    private BigDecimal inflow;
    private OffsetDateTime timestamp;

    public BigDecimal getInflow() {
        return inflow;
    }

    public OffsetDateTime getTimestamp() {
        return timestamp;
    }

    @Override
    public String toString() {
        return "CapitalFlowLine [inflow=" + inflow + ", timestamp=" + timestamp + "]";
    }
}
