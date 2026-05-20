package com.longbridge.fundamental;

/** Key financial metrics extracted from an operating report. */
public class OperatingFinancial {
    /** Ticker code (may be empty). */
    public String code;
    /** Raw counter ID (may be empty). */
    public String counterId;
    /** Reporting currency. */
    public String currency;
    /** Company name. */
    public String name;
    /** Market region. */
    public String region;
    /** Report period code. */
    public String report;
    /** Report period display text. */
    public String reportTxt;
    /** Financial indicators. */
    public OperatingIndicator[] indicators;
}