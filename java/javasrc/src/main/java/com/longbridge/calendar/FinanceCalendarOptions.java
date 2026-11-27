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
    /**
     * Maximum number of events per date; server default when {@code null} (optional).
     * A date group holding fewer events than its {@code count} was truncated: call again
     * with {@code start} = {@code end} = that date and {@code offset} set to the events
     * already received.
     */
    public Integer count;
    /** Number of events to skip within a date (optional). */
    public Integer offset;
    /** Direction to page from the {@code nextDate} cursor (optional). */
    public CalendarPageDirection next;
}
