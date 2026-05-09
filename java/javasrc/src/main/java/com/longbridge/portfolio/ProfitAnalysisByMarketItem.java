package com.longbridge.portfolio;

import java.math.BigDecimal;

/** One security entry in a by-market P&amp;L response. */
public class ProfitAnalysisByMarketItem {
    /** Security symbol (ticker code) */
    public String code;
    /** Security name */
    public String name;
    /** Market, e.g. {@code "HK"} or {@code "US"} */
    public String market;
    /** Profit/loss amount */
    public BigDecimal profit;
}
