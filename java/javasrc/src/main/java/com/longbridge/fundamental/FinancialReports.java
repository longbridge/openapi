package com.longbridge.fundamental;

/**
 * Raw financial report data for a security.
 *
 * <p>The {@code list} field contains raw JSON with top-level keys such as
 * {@code "IS"} (income statement), {@code "BS"} (balance sheet), and
 * {@code "CF"} (cash flow statement).
 */
public class FinancialReports {
    /** Raw nested financial data as a JSON string. */
    public String list;
}