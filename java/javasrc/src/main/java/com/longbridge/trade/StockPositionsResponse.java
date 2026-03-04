package com.longbridge.trade;

import java.util.Arrays;

public class StockPositionsResponse {
    private StockPositionChannel[] channels;

    public StockPositionChannel[] getChannels() {
        return channels;
    }

    @Override
    public String toString() {
        return "StockPositionsResponse [channels=" + Arrays.toString(channels) + "]";
    }
}
