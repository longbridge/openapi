package com.longbridge.calendar;

/** Events for one calendar date. */
public class CalendarDateGroup {
    /** Date string, e.g. {@code "2025-05-02"}. */
    public String date;
    /** Total event count for this date. */
    public int count;
    /** Event details. */
    public CalendarEventInfo[] infos;
}
