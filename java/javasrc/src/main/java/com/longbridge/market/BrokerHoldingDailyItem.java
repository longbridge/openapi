package com.longbridge.market;

import java.math.BigDecimal;

/** One day's broker holding record. */
public class BrokerHoldingDailyItem {
    /** Date in {@code "2026.05.05"} format. */
    public String date;
    /** Total shares held. */
    public BigDecimal holding;
    /** Holding ratio. */
    public BigDecimal ratio;
    /** Change vs previous day. */
    public BigDecimal chg;
}