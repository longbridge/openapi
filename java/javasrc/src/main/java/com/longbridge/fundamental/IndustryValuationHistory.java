package com.longbridge.fundamental;

import java.math.BigDecimal;

/** Historical valuation snapshot for an industry peer. */
public class IndustryValuationHistory {
    /** Unix timestamp string. */
    public String date;
    /** Price-to-Earnings ratio. */
    public BigDecimal pe;
    /** Price-to-Book ratio. */
    public BigDecimal pb;
    /** Price-to-Sales ratio. */
    public BigDecimal ps;
}