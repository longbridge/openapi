package com.longbridge.fundamental;

/** Latest analyst-rating snapshot for a security. */
public class InstitutionRatingLatest {
    /** Rating distribution counts and date range. */
    public RatingEvaluate evaluate;
    /** Target price range. */
    public RatingTarget target;
    /** Industry classification ID. */
    public long industryId;
    /** Industry name. */
    public String industryName;
    /** Rank of this security within the industry (1 = highest). */
    public int industryRank;
    /** Total number of securities in the industry. */
    public int industryTotal;
    /** Mean analyst count in the industry. */
    public int industryMean;
    /** Median analyst count in the industry. */
    public int industryMedian;
}