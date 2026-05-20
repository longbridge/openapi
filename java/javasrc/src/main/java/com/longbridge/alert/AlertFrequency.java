package com.longbridge.alert;

/** Alert notification frequency. */
public enum AlertFrequency {
    /** Trigger at most once per day */
    Daily,
    /** Trigger every time the condition is met */
    EveryTime,
    /** Trigger only the first time */
    Once,
}
