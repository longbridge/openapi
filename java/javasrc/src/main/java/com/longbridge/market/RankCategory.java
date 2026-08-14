package com.longbridge.market;

/** A top-level rank category grouping one or more sub-categories. */
public class RankCategory {
    /** Top-level key, e.g. {@code "hot"} */
    public String key;
    /** Display name, e.g. {@code "热度排行"} */
    public String name;
    /** Sub-categories */
    public RankSubCategory[] subCategories;
}
