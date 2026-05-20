package com.longbridge.fundamental;

/** Analyst rating distribution counts for a security. */
public class RatingEvaluate {
    /** Number of "Buy" ratings. */
    public int buy;
    /** Number of "Strong Buy" / "Outperform" ratings. */
    public int over;
    /** Number of "Hold" / "Neutral" ratings. */
    public int hold;
    /** Number of "Underperform" ratings. */
    public int under;
    /** Number of "Sell" ratings. */
    public int sell;
    /** Number of "No Opinion" ratings. */
    public int noOpinion;
    /** Total analyst count. */
    public int total;
    /** Window start (unix timestamp string; {@code "0"} means unset). */
    public String startDate;
    /** Window end (unix timestamp string; {@code "0"} means unset). */
    public String endDate;
}