package com.longbridge.market;

import java.math.BigDecimal;

/** Changes in a broker's holding or ratio over 1 / 5 / 20 / 60 day periods. */
public class BrokerHoldingChanges {
    /** Current value. */
    public BigDecimal value;
    /** 1-day change. */
    public BigDecimal chg1;
    /** 5-day change. */
    public BigDecimal chg5;
    /** 20-day change. */
    public BigDecimal chg20;
    /** 60-day change. */
    public BigDecimal chg60;
}