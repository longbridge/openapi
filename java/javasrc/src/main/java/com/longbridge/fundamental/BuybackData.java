package com.longbridge.fundamental;

/** Response for {@link FundamentalContext#getBuyback}. */
public class BuybackData {
    /** Most recent buyback summary (TTM); may be null */
    public RecentBuybacks recentBuybacks;
    /** Historical annual buyback data */
    public BuybackHistoryItem[] buybackHistory;
    /** Buyback payout and cash-flow ratios */
    public BuybackRatios[] buybackRatios;
}
