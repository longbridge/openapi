package com.longbridge.fundamental;

import java.math.BigDecimal;

/** Valuation data for one peer security within an industry. */
public class IndustryValuationItem {
    /** Security symbol, e.g. {@code "700.HK"}. */
    public String symbol;
    /** Company name. */
    public String name;
    /** Reporting currency. */
    public String currency;
    /** Total assets. */
    public BigDecimal assets;
    /** Book value per share. */
    public BigDecimal bps;
    /** Earnings per share. */
    public BigDecimal eps;
    /** Dividends per share. */
    public BigDecimal dps;
    /** Dividend yield. */
    public BigDecimal divYld;
    /** Dividend payout ratio. */
    public BigDecimal divPayoutRatio;
    /** 5-year average dividends per share. */
    public BigDecimal fiveYAvgDps;
    /** Current PE ratio. */
    public BigDecimal pe;
    /** Historical PE/PB/PS snapshots. */
    public IndustryValuationHistory[] history;
}