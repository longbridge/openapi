package com.longbridge.quote;

import java.util.Arrays;

public class SecurityBrokers {
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
        return "SecurityBrokers [askBrokers=" + Arrays.toString(askBrokers) + ", bidBrokers="
                + Arrays.toString(bidBrokers) + "]";
    }
}
