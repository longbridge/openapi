package com.longbridge.portfolio;

import java.math.BigDecimal;

/** Response for {@link PortfolioContext#getProfitAnalysisByMarket}. */
public class ProfitAnalysisByMarket {
    /** Total P&L across all returned items */
    public BigDecimal profit;
    /** Whether more pages are available */
    public boolean hasMore;
    /** Per-security P&L items */
    public ProfitAnalysisByMarketItem[] stockItems;
}
