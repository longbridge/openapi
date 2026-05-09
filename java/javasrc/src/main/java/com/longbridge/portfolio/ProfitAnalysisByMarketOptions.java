package com.longbridge.portfolio;
/** Options for {@link PortfolioContext#getProfitAnalysisByMarket}. */
public class ProfitAnalysisByMarketOptions {
    /** Market filter, e.g. {@code "HK"} or {@code "US"} (optional) */
    public String market;
    /** Start date {@code "YYYY-MM-DD"} (optional) */
    public String start;
    /** End date {@code "YYYY-MM-DD"} (optional) */
    public String end;
    /** Currency filter (optional) */
    public String currency;
    /** Page number (1-based, default 1) */
    public Integer page;
    /** Page size (default 20) */
    public Integer size;
}
