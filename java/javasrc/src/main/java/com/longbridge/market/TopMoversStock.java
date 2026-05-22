package com.longbridge.market;

/** Stock information in a top-movers event. */
public class TopMoversStock {
    /** Symbol, e.g. "TSLA.US" */
    public String symbol;
    /** Ticker code */
    public String code;
    /** Security name */
    public String name;
    /** Full name */
    public String fullName;
    /** Price change (decimal ratio) */
    public String change;
    /** Latest price */
    public String lastDone;
    /** Market */
    public String market;
    /** Labels / tags */
    public String[] labels;
    /** Logo URL */
    public String logo;
}
