package com.longbridge.market;

/** Options for {@link MarketContext#getBrokerHoldingDaily}. */
public class BrokerHoldingDailyOptions {
    /** Security symbol to query daily broker holding history for. */
    public String symbol;
    /** Broker participant number to filter results to a specific broker. */
    public String brokerId;
}
