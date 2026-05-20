package com.longbridge.fundamental;

import java.math.BigDecimal;

/** Historical time-series for one valuation metric. */
public class ValuationMetricData {
    /** Human-readable description with current value and percentile. */
    public String desc;
    /** Historical high value. */
    public BigDecimal high;
    /** Historical low value. */
    public BigDecimal low;
    /** Historical median value. */
    public BigDecimal median;
    /** Historical data points. */
    public ValuationPoint[] list;
}