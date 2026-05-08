package com.longbridge.sharelist;

import java.math.BigDecimal;

/** Stock in a sharelist. */
public class SharelistStock {
    /** Security symbol. */
    public String symbol;
    /** Security name. */
    public String name;
    /** Market, e.g. {@code "HK"}. */
    public String market;
    /** Ticker code. */
    public String code;
    /** Brief description. */
    public String intro;
    /** Unread change log category. */
    public String unreadChangeLogCategory;
    /** Day change percentage. */
    public BigDecimal change;
    /** Latest price. */
    public BigDecimal lastDone;
    /** Trade status code. */
    public Integer tradeStatus;
    /** Whether delayed quote. */
    public boolean latency;
}
