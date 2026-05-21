package com.longbridge.market;

/** One top-movers event. */
public class TopMoversEvent {
    /** Event timestamp in RFC 3339 format */
    public String timestamp;
    /** Alert reason description */
    public String alertReason;
    /** Alert type code */
    public long alertType;
    /** Stock information */
    public TopMoversStock stock;
    /** Associated news post as JSON string (may be null) */
    public String post;
}
