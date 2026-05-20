package com.longbridge.fundamental;

import java.math.BigDecimal;

/** Historical analyst target price time-series for a security. */
public class InstitutionRatingDetailTarget {
    /** Prediction accuracy ratio (may be null). */
    public BigDecimal dataPercent;
    /** Overall prediction accuracy (may be null). */
    public BigDecimal predictionAccuracy;
    /** Last updated display string. */
    public String updatedAt;
    /** Weekly target price snapshots. */
    public InstitutionRatingDetailTargetItem[] list;
}