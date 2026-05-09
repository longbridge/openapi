package com.longbridge.fundamental;

/** One weekly analyst rating distribution snapshot. */
public class InstitutionRatingDetailEvaluateItem {
    /** Number of "Buy" ratings. */
    public int buy;
    /** Date in {@code "2021/05/14"} format. */
    public String date;
    /** Number of "Hold" ratings. */
    public int hold;
    /** Number of "Sell" ratings. */
    public int sell;
    /** Number of "Strong Buy" / "Outperform" ratings. */
    public int strongBuy;
    /** Number of "No Opinion" ratings. */
    public int noOpinion;
    /** Number of "Underperform" ratings. */
    public int under;
}