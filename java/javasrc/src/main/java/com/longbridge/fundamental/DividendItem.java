package com.longbridge.fundamental;

/** A single dividend / distribution event. */
public class DividendItem {
    /** Security symbol, e.g. {@code "700.HK"}. */
    public String symbol;
    /** Internal record ID (may be absent in dividend_detail response). */
    public String id;
    /** Human-readable description, e.g. {@code "每股派息 5.3 HKD"}. */
    public String desc;
    /** Record / book-close date, e.g. {@code "2026.05.18"}. */
    public String recordDate;
    /** Ex-dividend date, e.g. {@code "2026.05.15"}. */
    public String exDate;
    /** Payment date, e.g. {@code "2026.06.01"}. */
    public String paymentDate;
}