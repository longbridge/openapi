package com.longbridge.fundamental;

/** Options for {@link FundamentalContext#getIndustryRank}. */
public class IndustryRankOptions {
    /** Market code, e.g. "US" */
    public String market;
    /** Indicator (numeric string "0"–"7") */
    public String indicator;
    /** Sort type: "0" (ascending) or "1" (descending) */
    public String sortType;
    /** Maximum number of results */
    public int limit;
}
