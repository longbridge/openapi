package com.longbridge.calendar;

/** Response for {@link CalendarContext#getFinanceCalendar}. */
public class CalendarEventsResponse {
    /** Start date of the query window. */
    public String date;
    /** Per-day event groups. */
    public CalendarDateGroup[] list;
}
