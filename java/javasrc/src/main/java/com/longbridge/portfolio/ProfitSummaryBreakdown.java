package com.longbridge.portfolio;

import java.math.BigDecimal;

/** P&amp;L breakdown by asset type. */
public class ProfitSummaryBreakdown {
    /** Stock P&amp;L. */
    public BigDecimal stock;
    /** Fund P&amp;L. */
    public BigDecimal fund;
    /** Crypto P&amp;L. */
    public BigDecimal crypto;
    /** Money market fund P&amp;L. */
    public BigDecimal mmf;
    /** Other P&amp;L. */
    public BigDecimal other;
    /** Cumulative transaction amount. */
    public BigDecimal cumulativeTransactionAmount;
    /** Total number of orders. */
    public String tradeOrderNum;
    /** Total number of traded securities. */
    public String tradeStockNum;
    /** IPO P&amp;L. */
    public BigDecimal ipo;
    /** IPO hits. */
    public int ipoHit;
    /** IPO subscriptions. */
    public int ipoSubscription;
    /** Per-category summary info. */
    public ProfitSummaryInfo[] summaryInfo;
}
