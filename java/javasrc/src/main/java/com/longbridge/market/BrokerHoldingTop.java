package com.longbridge.market;

/** Top brokers by net buying and net selling for a security. */
public class BrokerHoldingTop {
    /** Top brokers by net buying. */
    public BrokerHoldingEntry[] buy;
    /** Top brokers by net selling. */
    public BrokerHoldingEntry[] sell;
    /** Last updated timestamp (may be empty). */
    public String updatedAt;
}