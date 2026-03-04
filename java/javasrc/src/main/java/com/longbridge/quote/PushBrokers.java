package com.longbridge.quote;

import java.util.Arrays;

public class PushBrokers {
    private Brokers[] askBrokers;
    private Brokers[] bidBrokers;

    public Brokers[] getAskBrokers() {
        return askBrokers;
    }

    public Brokers[] getBidBrokers() {
        return bidBrokers;
    }

    @Override
    public String toString() {
        return "PushBrokers [askBrokers=" + Arrays.toString(askBrokers) + ", bidBrokers=" + Arrays.toString(bidBrokers)
                + "]";
    }
}
