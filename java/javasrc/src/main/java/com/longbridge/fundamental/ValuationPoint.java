package com.longbridge.fundamental;

import java.math.BigDecimal;

/** One valuation data point in a historical time-series. */
public class ValuationPoint {
    /** Date of the data point. */
    public java.time.OffsetDateTime timestamp;
    /** Metric value. */
    public BigDecimal value;
}