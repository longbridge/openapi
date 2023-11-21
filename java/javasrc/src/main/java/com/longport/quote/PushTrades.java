package com.longport.quote;

import java.util.Arrays;

public class PushTrades {
    private Trade[] trades;

    public Trade[] getTrades() {
        return trades;
    }

    @Override
    public String toString() {
        return "PushTrades [trades=" + Arrays.toString(trades) + "]";
    }
}
