package com.longbridge.market;

/** Options for {@link MarketContext#getTopMovers}. */
public class TopMoversOptions {
    /**
     * Market list, e.g. {@code ["HK", "US", "CN", "SG"]}.
     * Pass {@code null} or an empty array to return all markets.
     */
    public String[] markets;

    /**
     * Sort order.
     * 0 = time (newest first), 1 = price change, 2 = hotness (default).
     * Pass {@code null} to use the server default.
     */
    public Integer sort;

    /**
     * Target date in {@code "YYYY-MM-DD"} format.
     * Pass {@code null} to return the latest data.
     */
    public String date;

    /**
     * Maximum number of results to return.
     * Pass {@code null} to use the server default (20).
     */
    public Integer limit;
}
