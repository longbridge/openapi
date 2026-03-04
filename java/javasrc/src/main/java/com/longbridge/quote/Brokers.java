package com.longbridge.quote;

import java.util.Arrays;

public class Brokers {
    private int position;
    private int[] brokerIds;

    public int getPosition() {
        return position;
    }

    public int[] getBrokerIds() {
        return brokerIds;
    }

    @Override
    public String toString() {
        return "Brokers [brokerIds=" + Arrays.toString(brokerIds) + ", position=" + position + "]";
    }
}
