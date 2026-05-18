package com.longbridge.fundamental;

/** One historical rating distribution snapshot. */
public class InstitutionRatingViewItem {
    /** Date as unix timestamp string */
    public String date;
    /** Number of Buy ratings */
    public String buy;
    /** Number of Outperform ratings */
    public String over;
    /** Number of Hold ratings */
    public String hold;
    /** Number of Underperform ratings */
    public String under;
    /** Number of Sell ratings */
    public String sell;
    /** Total analyst count */
    public String total;
}
