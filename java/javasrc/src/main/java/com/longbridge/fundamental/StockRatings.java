package com.longbridge.fundamental;

/** Response for {@link FundamentalContext#getRatings}. */
public class StockRatings {
    /** Style display name */
    public String styleTxtName;
    /** Scale display name */
    public String scaleTxtName;
    /** Report period display text */
    public String reportPeriodTxt;
    /** Composite score (JSON string; may be int, float, or null) */
    public String multiScore;
    /** Composite score letter grade */
    public String multiLetter;
    /** Score change vs previous period */
    public int multiScoreChange;
    /** Industry name */
    public String industryName;
    /** Industry rank (JSON string) */
    public String industryRank;
    /** Total securities in the industry (JSON string) */
    public String industryTotal;
    /** Industry mean score (JSON string) */
    public String industryMeanScore;
    /** Industry median score (JSON string) */
    public String industryMedianScore;
    /** Detailed rating categories */
    public RatingCategory[] ratings;
}
