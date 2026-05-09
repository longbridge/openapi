package com.longbridge.market;

/** Options for {@link MarketContext#getBrokerHolding}. */
public class BrokerHoldingOptions {
    /** Security symbol to query broker holding for. */
    public String symbol;
    /**
     * Lookback period for net change calculation.
     * Defaults to Rct1 when null.
     */
    public BrokerHoldingPeriod period;
}
