package com.longbridge.screener;

/** Options for {@link ScreenerContext#search}. */
public class ScreenerSearchOptions {
    /** Market: "US", "HK", "CN", or "SG" */
    public String market;
    /** Strategy ID (optional; null for custom filter mode) */
    public Long strategyId;
    /** Page number (1-indexed, default 1) */
    public int page = 1;
    /** Page size (default 20) */
    public int size = 20;
}
