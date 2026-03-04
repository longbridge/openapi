package com.longbridge.trade;

import java.util.Arrays;

public class StockPositionChannel {
    private String accountChannel;
    private StockPosition[] positions;

    public String getAccountChannel() {
        return accountChannel;
    }

    public StockPosition[] getPositions() {
        return positions;
    }

    @Override
    public String toString() {
        return "StockPositionChannel [accountChannel=" + accountChannel + ", positions=" + Arrays.toString(positions)
                + "]";
    }
}
