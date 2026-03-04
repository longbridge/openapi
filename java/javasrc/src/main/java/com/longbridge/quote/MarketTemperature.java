package com.longbridge.quote;

import java.time.OffsetDateTime;

public class MarketTemperature {
    private int temperature;
    private String description;
    private int valuation;
    private int sentiment;
    private OffsetDateTime timestamp;

    public int getTemperature() {
        return temperature;
    }

    public String getDescription() {
        return description;
    }

    public int getValuation() {
        return valuation;
    }

    public int getSentiment() {
        return sentiment;
    }

    public OffsetDateTime getTimestamp() {
        return timestamp;
    }

    @Override
    public String toString() {
        return "MarketTemperature [temperature=" + temperature + ", description=" + description + ", valuation="
                + valuation + ", sentiment=" + sentiment + ", timestamp=" + timestamp + "]";
    }
}