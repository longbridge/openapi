package com.longbridge.market;

import java.math.BigDecimal;

/** One constituent stock of a market index. */
public class ConstituentStock {
    /** Security symbol. */
    public String symbol;
    /** Security name. */
    public String name;
    /** Latest price. */
    public BigDecimal lastDone;
    /** Previous close. */
    public BigDecimal prevClose;
    /** Net capital inflow today. */
    public BigDecimal inflow;
    /** Turnover amount. */
    public BigDecimal balance;
    /** Trading volume (shares). */
    public BigDecimal amount;
    /** Total shares outstanding. */
    public BigDecimal totalShares;
    /** Tags, e.g. {@code ["领涨龙头"]}. */
    public String[] tags;
    /** Brief description. */
    public String intro;
    /** Market, e.g. {@code "HK"}. */
    public String market;
    /** Circulating shares. */
    public BigDecimal circulatingShares;
    /** Whether this is a delayed quote. */
    public boolean delay;
    /** Day change percentage. */
    public BigDecimal chg;
    /** Raw trade status code. */
    public int tradeStatus;
}