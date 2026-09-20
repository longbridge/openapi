package com.longbridge.calendar;

import java.util.concurrent.CompletableFuture;
import com.longbridge.*;

/** Financial calendar context */
public class CalendarContext implements AutoCloseable {
    private long raw;

    private long raw() {
        long r = this.raw;
        if (r == 0) {
            throw new IllegalStateException(
                    getClass().getSimpleName() + " has already been closed");
        }
        return r;
    }
    public static CalendarContext create(Config config) {
        CalendarContext ctx = new CalendarContext();
        synchronized (config) { ctx.raw = SdkNative.newCalendarContext(config.getRaw()); }
        return ctx;
    }
    @Override
    public synchronized void close() throws Exception {
        long h = this.raw;
        if (h != 0) {
            this.raw = 0;
            SdkNative.freeCalendarContext(h);
        }
    }

    /** Get financial calendar events */
    public synchronized CompletableFuture<CalendarEventsResponse> getFinanceCalendar(FinanceCalendarOptions opts) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> SdkNative.calendarContextFinanceCalendar(raw(), opts, callback));
    }
}
