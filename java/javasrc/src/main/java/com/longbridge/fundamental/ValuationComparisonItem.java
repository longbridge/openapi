package com.longbridge.fundamental;

/** One security in the valuation comparison. */
public class ValuationComparisonItem {
    /** Symbol, e.g. "AAPL.US" (converted from counter_id) */
    public String symbol;
    public String name;
    public String currency;
    public String marketValue;
    public String priceClose;
    public String pe;
    public String pb;
    public String ps;
    public String roe;
    public String eps;
    public String bps;
    public String dps;
    public String divYld;
    public String assets;
    /** Historical valuation data points */
    public ValuationHistoryPoint[] history;
}
