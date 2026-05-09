package com.longbridge.market;

import java.math.BigDecimal;

/** Summary trade statistics for a security. */
public class TradeStatistics {
    /** Volume-weighted average price. */
    public BigDecimal avgprice;
    /** Total buy volume (shares). */
    public BigDecimal buy;
    /** Total neutral / unknown-direction volume. */
    public BigDecimal neutral;
    /** Previous close price. */
    public BigDecimal preclose;
    /** Total sell volume (shares). */
    public BigDecimal sell;
    /** Data timestamp (unix timestamp string). */
    public String timestamp;
    /** Total trading volume (shares). */
    public BigDecimal totalAmount;
    /** Unix timestamps for the last 5 trading days. */
    public String[] tradeDate;
    /** Total number of trades. */
    public String tradesCount;
}