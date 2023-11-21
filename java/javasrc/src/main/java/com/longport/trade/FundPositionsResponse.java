package com.longport.trade;

import java.util.Arrays;

public class FundPositionsResponse {
    private FundPositionChannel[] channels;

    public FundPositionChannel[] getChannels() {
        return channels;
    }

    @Override
    public String toString() {
        return "FundPositionsResponse [channels=" + Arrays.toString(channels) + "]";
    }
}
