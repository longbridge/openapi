package com.longbridge.fundamental;

import java.math.BigDecimal;

/** One weekly analyst target price snapshot. */
public class InstitutionRatingDetailTargetItem {
    /** Average target price. */
    public BigDecimal avgTarget;
    /** Date in {@code "2021/05/16"} format. */
    public String date;
    /** Highest target price. */
    public BigDecimal maxTarget;
    /** Lowest target price. */
    public BigDecimal minTarget;
    /** Whether the stock price reached the target. */
    public boolean meet;
    /** Actual stock price at this date. */
    public BigDecimal price;
    /** Unix timestamp string. */
    public String timestamp;
}