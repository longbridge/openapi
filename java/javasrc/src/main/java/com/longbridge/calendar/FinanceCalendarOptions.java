package com.longbridge.calendar;

/** Options for {@link CalendarContext#getFinanceCalendar}. */
public class FinanceCalendarOptions {
    /** Event category filter: 0=Report, 1=Dividend, 2=Split, 3=Ipo, 4=MacroData, 5=Closed (optional). */
    public Integer category;
    /** Start date {@code "YYYY-MM-DD"} of the query window (optional). */
    public String start;
    /** End date {@code "YYYY-MM-DD"} of the query window (optional). */
    public String end;
    /** Market filter, e.g. {@code "HK"} or {@code "US"} (optional). */
    public String market;
}
