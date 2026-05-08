package com.longbridge.market;

import java.math.BigDecimal;

/** One broker entry in a top net-buying or net-selling list. */
public class BrokerHoldingEntry {
    /** Broker name. */
    public String name;
    /** Participant number / broker code. */
    public String partiNumber;
    /** Net change in shares held. */
    public BigDecimal chg;
    /** Whether this is a "strengthening" broker. */
    public boolean strong;
}