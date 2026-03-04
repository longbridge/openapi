package com.longbridge.trade;

import java.util.Arrays;

public class FundPositionChannel {
    private String accountChannel;
    private FundPosition[] positions;

    public String getAccountChannel() {
        return accountChannel;
    }

    public FundPosition[] getPositions() {
        return positions;
    }

    @Override
    public String toString() {
        return "FundPositionChannel [accountChannel=" + accountChannel + ", positions=" + Arrays.toString(positions)
                + "]";
    }
}
