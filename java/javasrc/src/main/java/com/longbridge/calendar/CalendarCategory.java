package com.longbridge.calendar;

/** Financial calendar event category. */
public enum CalendarCategory {
    /** Earnings reports */
    Report,
    /** Dividend announcements */
    Dividend,
    /** Stock splits */
    Split,
    /** Initial public offerings */
    Ipo,
    /** Macro-economic data releases */
    MacroData,
    /** Market closure days */
    Closed,
    /** Shareholder / analyst meetings */
    Meeting,
    /** Stock consolidations / mergers */
    Merge,
}
