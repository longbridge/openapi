package com.longbridge.fundamental;

/** Response for {@link FundamentalContext#getRatings}. */
public class StockRatings {
    /** Style display name */
    public String styleTxtName;
    /** Scale display name */
    public String scaleTxtName;
    /** Report period display text */
    public String reportPeriodTxt;
    /** Composite score; may be null when not rated */
    public Double multiScore;
    /** Composite score letter grade */
    public String multiLetter;
    /** Score change vs previous period */
    public int multiScoreChange;
    /** Industry name */
    public String industryName;
    /** Industry rank; may be null */
    public Long industryRank;
    /** Total securities in the industry; may be null */
    public Long industryTotal;
    /** Industry mean score; may be null */
    public Double industryMeanScore;
    /** Industry median score; may be null */
    public Double industryMedianScore;
    /** Detailed rating categories */
    public RatingCategory[] ratings;
}
