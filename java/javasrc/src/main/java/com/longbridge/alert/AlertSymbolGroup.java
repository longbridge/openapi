package com.longbridge.alert;

import java.math.BigDecimal;

/** Alert items for one security. */
public class AlertSymbolGroup {
    /** Security symbol. */
    public String symbol;
    /** Ticker code (without market). */
    public String code;
    /** Market, e.g. {@code "HK"}. */
    public String market;
    /** Security name. */
    public String name;
    /** Latest price. */
    public BigDecimal price;
    /** Day change amount. */
    public BigDecimal chg;
    /** Day change percentage. */
    public BigDecimal pChg;
    /** Product type (may be empty). */
    public String product;
    /** Alert items. */
    public AlertItem[] indicators;
}
