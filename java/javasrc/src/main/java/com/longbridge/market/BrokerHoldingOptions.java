package com.longbridge.market;

/** Options for {@link MarketContext#getBrokerHolding}. */
public class BrokerHoldingOptions {
    /** Security symbol to query broker holding for. */
    public String symbol;
    /**
     * Lookback period for net change calculation.
     * 0 = 1 day (rct_1), 1 = 5 days (rct_5), 2 = 20 days (rct_20), 3 = 60 days (rct_60).
     * Defaults to 0 (1 day) when null.
     */
    public Integer period;
}
