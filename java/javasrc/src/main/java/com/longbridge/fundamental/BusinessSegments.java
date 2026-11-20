package com.longbridge.fundamental;

/** Response for {@link FundamentalContext#getBusinessSegments}. */
public class BusinessSegments {
    /** Report date, {@code YYYYMMDD} */
    public String date;
    /** Total revenue */
    public String total;
    /** Reporting currency */
    public String currency;
    /**
     * Fiscal period label, e.g. {@code 2026.Q2}. Follows the issuer's own
     * fiscal calendar, so it does not always match the calendar year of
     * {@code fpEnd}.
     */
    public String reportTxt;
    /** First day of the reported period, {@code YYYY.MM.DD} */
    public String fpStart;
    /** Last day of the reported period, {@code YYYY.MM.DD} */
    public String fpEnd;
    /** Year-on-year change of {@code total}. Empty when the server does not supply one. */
    public String yoy;
    /** Business segment breakdown */
    public BusinessSegmentItem[] business;
    /**
     * Regional breakdown, in the same shape as {@code business}. Empty for
     * issuers that do not report one.
     */
    public BusinessSegmentItem[] regionals;
}
