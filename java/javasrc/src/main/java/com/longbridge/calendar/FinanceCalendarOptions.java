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
}
