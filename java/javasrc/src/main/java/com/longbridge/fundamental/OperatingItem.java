package com.longbridge.fundamental;

/** One operating summary report (annual or quarterly). */
public class OperatingItem {
    /** Internal report ID. */
    public String id;
    /** Report period code, e.g. {@code "af"} (annual), {@code "qf"} (quarterly). */
    public String report;
    /** Report title, e.g. {@code "2025 财年年报"}. */
    public String title;
    /** Management discussion text. */
    public String txt;
    /** Whether this is the most recent report. */
    public boolean latest;
    /** URL to the full community report page. */
    public String webUrl;
    /** Key financial metrics extracted from the report. */
    public OperatingFinancial financial;
    /** Keyword tags (usually empty). */
    public String[] keywords;
}