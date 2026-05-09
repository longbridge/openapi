package com.longbridge.fundamental;

import java.math.BigDecimal;

/** Analyst target price range for a security. */
public class RatingTarget {
    /** Highest price target. */
    public BigDecimal highestPrice;
    /** Lowest price target. */
    public BigDecimal lowestPrice;
    /** Previous close price. */
    public BigDecimal prevClose;
    /** Window start (unix timestamp string). */
    public String startDate;
    /** Window end (unix timestamp string). */
    public String endDate;
}