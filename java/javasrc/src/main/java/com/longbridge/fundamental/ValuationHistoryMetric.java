package com.longbridge.fundamental;

import java.math.BigDecimal;

/** Historical data for one valuation metric including statistical bounds. */
public class ValuationHistoryMetric {
    /** Human-readable description. */
    public String desc;
    /** Upper bound of the fair-value band (not the period maximum; the series can rise above it). */
    public BigDecimal high;
    /** Lower bound of the fair-value band (not the period minimum; the series can fall below it). */
    public BigDecimal low;
    /** Historical median over the period. */
    public BigDecimal median;
    /** Historical data points. */
    public ValuationPoint[] list;
}