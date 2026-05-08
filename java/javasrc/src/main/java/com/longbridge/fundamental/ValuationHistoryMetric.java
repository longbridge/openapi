package com.longbridge.fundamental;

import java.math.BigDecimal;

/** Historical data for one valuation metric including statistical bounds. */
public class ValuationHistoryMetric {
    /** Human-readable description. */
    public String desc;
    /** Historical high over the period. */
    public BigDecimal high;
    /** Historical low over the period. */
    public BigDecimal low;
    /** Historical median over the period. */
    public BigDecimal median;
    /** Historical data points. */
    public ValuationPoint[] list;
}