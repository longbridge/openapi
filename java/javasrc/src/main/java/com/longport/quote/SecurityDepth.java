package com.longport.quote;

import java.util.Arrays;

public class SecurityDepth {
    private Depth[] asks;
    private Depth[] bids;

    public Depth[] getAsks() {
        return asks;
    }

    public Depth[] getBids() {
        return bids;
    }

    @Override
    public String toString() {
        return "SecurityDepth [asks=" + Arrays.toString(asks) + ", bids=" + Arrays.toString(bids) + "]";
    }
}
