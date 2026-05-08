package com.longbridge.fundamental;

import java.math.BigDecimal;

/** Consensus analyst-rating summary for a security. */
public class InstitutionRatingSummary {
    /** Currency symbol, e.g. {@code "HK$"}. */
    public String ccySymbol;
    /** Change vs previous period. */
    public BigDecimal change;
    /** Simplified rating distribution. */
    public RatingSummaryEvaluate evaluate;
    /** Overall recommendation, e.g. {@code "strong_buy"}. */
    public String recommend;
    /** Consensus target price. */
    public BigDecimal target;
    /** Last updated display string, e.g. {@code "2026 年 5 月 5 日"}. */
    public String updatedAt;
}