package com.longbridge.market;

/** Trade statistics response including summary and per-price-level breakdown. */
public class TradeStatsResponse {
    /** Summary statistics. */
    public TradeStatistics statistics;
    /** Per-price-level trade volume breakdown. */
    public TradePriceLevel[] trades;
}