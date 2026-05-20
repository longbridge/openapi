package com.longbridge.calendar;

/** One financial calendar event. */
public class CalendarEventInfo {
    /** Security symbol. */
    public String symbol;
    /** Market, e.g. {@code "HK"}. */
    public String market;
    /** Event content description. */
    public String content;
    /** Security name. */
    public String counterName;
    /** Date type label, e.g. {@code "盘前"}. */
    public String dateType;
    /** Event date string, e.g. {@code "2025.05.02"}. */
    public String date;
    /** Chart UID (may be empty). */
    public String chartUid;
    /** Structured data key-value pairs. */
    public CalendarDataKv[] dataKv;
    /** Event type code, e.g. {@code "financial"}. */
    public String eventType;
    /** Event datetime (unix timestamp string). */
    public String datetime;
    /** Icon URL. */
    public String icon;
    /** Importance star rating (0–3). */
    public int star;
    /** Associated live stream (usually null). */
    public String live;
    /** Internal event ID. */
    public String id;
    /** Financial market session time string. */
    public String financialMarketTime;
    /** Currency. */
    public String currency;
    /** Extended data (structure varies by event type). */
    public String ext;
    /** Activity type code. */
    public String activityType;
}
