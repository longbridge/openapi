package com.longbridge.portfolio;

import java.math.BigDecimal;

/** P&amp;L for one security. */
public class ProfitAnalysisItem {
    /** Security name. */
    public String name;
    /** Market. */
    public String market;
    /** Whether still holding. */
    public boolean isHolding;
    /** Profit/loss amount. */
    public BigDecimal profit;
    /** Profit/loss rate. */
    public BigDecimal profitRate;
    /** Number of completed trades. */
    public long clearanceTimes;
    /** Asset type. */
    public AssetType itemType;
    /** Currency. */
    public String currency;
    /** Security symbol. */
    public String symbol;
    /** Holding period display string. */
    public String holdingPeriod;
    /** Ticker code. */
    public String securityCode;
    /** ISIN (for funds). */
    public String isin;
    /** Underlying stock P&amp;L. */
    public BigDecimal underlyingProfit;
    /** Derivatives P&amp;L. */
    public BigDecimal derivativesProfit;
    /** P&amp;L in order currency. */
    public BigDecimal orderProfit;
}
