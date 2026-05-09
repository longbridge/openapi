package com.longbridge.fundamental;

/** A security in an institutional shareholder's cross-holdings. */
public class ShareholderStock {
    /** Security symbol of the cross-held stock. */
    public String symbol;
    /** Ticker code, e.g. {@code "BLK"}. */
    public String code;
    /** Market, e.g. {@code "US"}. */
    public String market;
    /** Day change percentage, e.g. {@code "-0.32%"}. */
    public String chg;
}