package com.longbridge.calendar;

import java.util.concurrent.CompletableFuture;
import com.longbridge.*;

/** Financial calendar context */
public class CalendarContext implements AutoCloseable {
    private long raw;
    public static CalendarContext create(Config config) {
        CalendarContext ctx = new CalendarContext();
        ctx.raw = SdkNative.newCalendarContext(config.getRaw());
        return ctx;
    }
    @Override public void close() throws Exception { SdkNative.freeCalendarContext(raw); }

    /** Get financial calendar events */
    public CompletableFuture<CalendarEventsResponse> getFinanceCalendar(FinanceCalendarOptions opts) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> SdkNative.calendarContextFinanceCalendar(raw, opts, callback));
    }
}
