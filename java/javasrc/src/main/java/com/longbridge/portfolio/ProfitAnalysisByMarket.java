package com.longbridge.portfolio;

import java.math.BigDecimal;

/** Response for {@link PortfolioContext#getProfitAnalysisByMarket}. */
public class ProfitAnalysisByMarket {
    /** Total P&amp;L across all returned items */
    public BigDecimal profit;
    /** Whether more pages are available */
    public boolean hasMore;
    /** Per-security P&amp;L items */
    public ProfitAnalysisByMarketItem[] stockItems;
}
