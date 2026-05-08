package com.longbridge.market;

/** Options for {@link MarketContext#getAhPremium}. */
public class AhPremiumOptions {
    /** H-share security symbol to query A/H premium data for, e.g. {@code "700.HK"}. */
    public String symbol;
    /**
     * K-line period.
     * 0=1-minute, 1=5-minute, 2=15-minute, 3=30-minute, 4=60-minute,
     * 5=daily, 6=weekly, 7=monthly, 8=yearly.
     * Defaults to 5 (daily) when null.
     */
    public Integer period;
    /** Number of K-lines to return (defaults to 100 when null). */
    public Integer count;
}
