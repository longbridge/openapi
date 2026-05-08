package com.longbridge.fundamental;

import java.math.BigDecimal;

/** Distribution statistics for one valuation metric within an industry. */
public class ValuationDist {
    /** Minimum value in the industry. */
    public BigDecimal low;
    /** Maximum value in the industry. */
    public BigDecimal high;
    /** Median value in the industry. */
    public BigDecimal median;
    /** Current value of the queried security. */
    public BigDecimal value;
    /** Percentile ranking (0–1 range). */
    public BigDecimal ranking;
    /** Ordinal rank index (1-based). */
    public String rankIndex;
    /** Total number of securities in the industry. */
    public String rankTotal;
}