package com.longbridge.market;

/** Options for {@link MarketContext#getAhPremium}. */
public class AhPremiumOptions {
    /** H-share security symbol to query A/H premium data for, e.g. {@code "700.HK"}. */
    public String symbol;
    /**
     * K-line period. Defaults to Day when null.
     */
    public AhPremiumPeriod period;
    /** Number of K-lines to return (defaults to 100 when null). */
    public Integer count;
}
