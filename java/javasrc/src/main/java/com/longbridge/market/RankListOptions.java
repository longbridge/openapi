package com.longbridge.market;

/** Options for {@link MarketContext#getRankList}. */
public class RankListOptions {
    /** Rank category key from getRankCategories, e.g. "ib_hot_all-us" */
    public String key;
    /** Whether to include article content (default: false) */
    public boolean needArticle;
}
