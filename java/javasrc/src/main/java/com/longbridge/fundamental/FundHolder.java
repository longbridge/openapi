package com.longbridge.fundamental;

import java.math.BigDecimal;

/** A fund or ETF that holds the queried security. */
public class FundHolder {
    /** Fund/ETF ticker code, e.g. {@code "513050"}. */
    public String code;
    /** Fund/ETF symbol, e.g. {@code "513050.SH"}. */
    public String symbol;
    /** Reporting currency, e.g. {@code "CNY"}. */
    public String currency;
    /** Fund/ETF full name. */
    public String name;
    /** Position ratio as a percentage. */
    public BigDecimal positionRatio;
    /** Report date, e.g. {@code "2025.12.31"}. */
    public String reportDate;
}