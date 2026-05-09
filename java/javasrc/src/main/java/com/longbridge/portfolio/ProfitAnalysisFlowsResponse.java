package com.longbridge.portfolio;

/** Response for {@link PortfolioContext#getProfitAnalysisFlows}. */
public class ProfitAnalysisFlowsResponse {
    /** Paginated list of flow items */
    public FlowItem[] flowsList;
    /** Whether there are more pages */
    public boolean hasMore;
}
