package com.longbridge.quote;

import java.util.Arrays;

public class HistoryMarketTemperatureResponse {
    private Granularity granularity;
    private MarketTemperature[] records;

    public Granularity getGranularity() {
        return granularity;
    }

    public MarketTemperature[] getRecords() {
        return records;
    }

    @Override
    public String toString() {
        return "HistoryMarketTemperatureResponse [granularity=" + granularity + ", records=" + Arrays.toString(records)
                + "]";
    }

}
