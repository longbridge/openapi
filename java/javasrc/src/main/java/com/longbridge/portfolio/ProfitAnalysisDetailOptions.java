package com.longbridge.portfolio;

/** Options for {@link PortfolioContext#getProfitAnalysisDetail}. */
public class ProfitAnalysisDetailOptions {
    /** Security symbol to query detail for. */
    public String symbol;
    /** Start date {@code "YYYY-MM-DD"} of the analysis period. */
    public String start;
    /** End date {@code "YYYY-MM-DD"} of the analysis period. */
    public String end;
}
