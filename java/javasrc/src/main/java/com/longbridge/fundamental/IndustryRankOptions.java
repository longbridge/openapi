package com.longbridge.fundamental;

import com.longbridge.Market;

/** Options for {@link FundamentalContext#getIndustryRank}. */
public class IndustryRankOptions {
    /** Market */
    public Market market;
    /** Ranking indicator */
    public IndustryRankIndicator indicator;
    /** Sort mode */
    public IndustryRankSortType sortType;
    /** Number of results to return; the server defaults to 20 when 0 */
    public int limit;
}
