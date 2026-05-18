package com.longbridge.fundamental;

/** Options for {@link FundamentalContext#getFinancialReportSnapshot}. */
public class FinancialReportSnapshotOptions {
    /** Security symbol */
    public String symbol;
    /** Report type: "qf", "saf", "af", or null */
    public String report;
    /** Fiscal year (e.g. 2023), or null */
    public Integer fiscalYear;
    /** Fiscal period string, or null */
    public String fiscalPeriod;
}
