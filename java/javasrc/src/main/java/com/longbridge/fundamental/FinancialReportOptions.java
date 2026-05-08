package com.longbridge.fundamental;

/**
 * Options for {@link FundamentalContext#getFinancialReport}
 */
public class FinancialReportOptions {
    /** Security symbol, set by FundamentalContext internally */
    public String symbol;

    /**
     * Report kind: 0=IncomeStatement, 1=BalanceSheet, 2=CashFlow, 3=All (default)
     */
    public Integer kind;

    /**
     * Report period: 0=Annual, 1=SemiAnnual, 2=Q1, 3=Q2, 4=Q3, 5=QuarterlyFull
     * (null means not specified)
     */
    public Integer period;
}
