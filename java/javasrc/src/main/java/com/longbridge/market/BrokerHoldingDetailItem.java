package com.longbridge.market;

/** One broker's full holding detail with ratio and share count changes. */
public class BrokerHoldingDetailItem {
    /** Broker name. */
    public String name;
    /** Participant number / broker code. */
    public String partiNumber;
    /** Holding ratio changes over various periods. */
    public BrokerHoldingChanges ratio;
    /** Share count changes over various periods. */
    public BrokerHoldingChanges shares;
    /** Whether this is a "strengthening" broker. */
    public boolean strong;
}