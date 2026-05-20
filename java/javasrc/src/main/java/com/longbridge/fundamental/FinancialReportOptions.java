package com.longbridge.fundamental;

/**
 * Options for {@link FundamentalContext#getFinancialReport}
 */
public class FinancialReportOptions {
    /** Security symbol, set by FundamentalContext internally */
    public String symbol;

    /**
     * Report kind (default: All).
     */
    public FinancialReportKind kind;

    /**
     * Report period (null means not specified).
     */
    public FinancialReportPeriod period;
}
