package com.longbridge.quote;

import java.time.OffsetDateTime;

public class CapitalDistributionResponse {
    private OffsetDateTime timestamp;
    private CapitalDistribution capitalIn;
    private CapitalDistribution capitalOut;

    public OffsetDateTime getTimestamp() {
        return timestamp;
    }

    public CapitalDistribution getCapitalIn() {
        return capitalIn;
    }

    public CapitalDistribution getCapitalOut() {
        return capitalOut;
    }

    @Override
    public String toString() {
        return "CapitalDistributionResponse [capitalIn=" + capitalIn + ", capitalOut=" + capitalOut + ", timestamp="
                + timestamp + "]";
    }
}
