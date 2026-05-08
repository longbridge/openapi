package com.longbridge.portfolio;

import java.math.BigDecimal;

/** Account-level P&amp;L summary. */
public class ProfitAnalysisSummary {
    /** Account currency. */
    public String currency;
    /** Current total asset value. */
    public BigDecimal currentTotalAsset;
    /** Query start date string. */
    public String startDate;
    /** Query end date string. */
    public String endDate;
    /** Start time (unix timestamp string). */
    public String startTime;
    /** End time (unix timestamp string). */
    public String endTime;
    /** Ending asset value. */
    public BigDecimal endingAssetValue;
    /** Initial asset value. */
    public BigDecimal initialAssetValue;
    /** Total invested amount. */
    public BigDecimal investAmount;
    /** Whether any trades occurred. */
    public boolean isTraded;
    /** Total profit/loss. */
    public BigDecimal sumProfit;
    /** Total profit/loss rate. */
    public BigDecimal sumProfitRate;
    /** Per-asset-type P&amp;L breakdown. */
    public ProfitSummaryBreakdown profits;
}
