package com.longbridge.portfolio;

/** Options for {@link PortfolioContext#getProfitAnalysisFlows}. */
public class ProfitAnalysisFlowsOptions {
    /** Security symbol (required), e.g. "700.HK" */
    public String symbol;
    /** Page number (1-based, default 1) */
    public Integer page;
    /** Page size (default 20) */
    public Integer size;
    /** Whether to include outside-RTH flows (default false) */
    public boolean includeOutsideRth;
    /** Start date "YYYY-MM-DD" (optional) */
    public String start;
    /** End date "YYYY-MM-DD" (optional) */
    public String end;
}
