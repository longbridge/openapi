package com.longbridge.fundamental;

/** Container for all valuation metrics (PE / PB / PS / dividend yield). */
public class ValuationMetricsData {
    /** Price-to-Earnings ratio history. */
    public ValuationMetricData pe;
    /** Price-to-Book ratio history. */
    public ValuationMetricData pb;
    /** Price-to-Sales ratio history. */
    public ValuationMetricData ps;
    /** Dividend yield history. */
    public ValuationMetricData dvdYld;
}