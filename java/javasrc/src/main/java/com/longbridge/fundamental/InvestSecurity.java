package com.longbridge.fundamental;

import java.math.BigDecimal;

/** A security in which the queried company has an investment stake. */
public class InvestSecurity {
    /** Internal company ID (string form; may be {@code "0"}). */
    public String companyId;
    /** Company name (locale-aware). */
    public String companyName;
    /** Company name in English. */
    public String companyNameEn;
    /** Company name in Simplified Chinese. */
    public String companyNameZhcn;
    /** Security symbol of the invested company. */
    public String symbol;
    /** Reporting currency. */
    public String currency;
    /** Percentage of shares held. */
    public BigDecimal percentOfShares;
    /** Shareholder rank, e.g. {@code "1"} = largest shareholder. */
    public String sharesRank;
    /** Market value of the holding. */
    public BigDecimal sharesValue;
}