package com.longbridge.fundamental;

import java.math.BigDecimal;

/** Historical time-series for one valuation metric. */
public class ValuationMetricData {
    /** Human-readable description with current value and percentile. */
    public String desc;
    /** Upper bound of the fair-value band (not the period maximum; the series can rise above it). */
    public BigDecimal high;
    /** Lower bound of the fair-value band (not the period minimum; the series can fall below it). */
    public BigDecimal low;
    /** Historical median value. */
    public BigDecimal median;
    /** Historical data points. */
    public ValuationPoint[] list;
}