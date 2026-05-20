package com.longbridge.fundamental;

/** One rating category (e.g. growth, profitability) for {@link StockRatings}. */
public class RatingCategory {
    /** Category type code */
    public int kind;
    /** Sub-indicator groups within this category */
    public RatingSubIndicatorGroup[] subIndicators;
}
