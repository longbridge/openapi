package com.longbridge.market;

/** One leaf rank sub-category. Its {@code key} can be passed to {@link MarketContext#getRankList}. */
public class RankSubCategory {
    /** Sub-category key, e.g. {@code "hot_all-us"}. Pass to getRankList. */
    public String key;
    /** Display name, e.g. {@code "美股总热度"} */
    public String name;
    /** Market code, e.g. {@code "US"}, {@code "HK"} */
    public String market;
}
