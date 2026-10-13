package com.longbridge.calendar;

/** Options for {@link CalendarContext#getFinanceCalendar}. */
public class FinanceCalendarOptions {
    /** Event category filter (optional). */
    public CalendarCategory category;
    /** Start date {@code "YYYY-MM-DD"} of the query window (optional). */
    public String start;
    /** End date {@code "YYYY-MM-DD"} of the query window (optional). */
    public String end;
    /** Market filter, e.g. {@code "HK"} or {@code "US"} (optional). */
    public String market;
    /** Maximum number of events per page; server default when {@code null} (optional). */
    public Integer count;
    /** Number of events to skip from the start of the window (optional). */
    public Integer offset;
    /** Direction to page from the {@code nextDate} cursor (optional). */
    public CalendarPageDirection next;
}
