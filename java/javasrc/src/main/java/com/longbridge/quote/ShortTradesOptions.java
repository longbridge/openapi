package com.longbridge.quote;

/** Options for {@link QuoteContext#getShortTrades}. */
public class ShortTradesOptions {
    /** Security symbol (US or HK), e.g. "AAPL.US" or "700.HK" */
    public String symbol;
    /** Number of records to return (1-100, default 20) */
    public int count;
}
