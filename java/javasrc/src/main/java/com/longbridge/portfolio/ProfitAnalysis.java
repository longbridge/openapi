package com.longbridge.portfolio;

/** Combined response for {@link PortfolioContext#getProfitAnalysis}. */
public class ProfitAnalysis {
    /** Account-level P&amp;L summary. */
    public ProfitAnalysisSummary summary;
    /** Per-security P&amp;L breakdown. */
    public ProfitAnalysisSublist sublist;
}
