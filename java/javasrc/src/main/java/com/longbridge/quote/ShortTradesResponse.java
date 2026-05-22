package com.longbridge.quote;

/** Response for {@link QuoteContext#getShortTrades}. Unified US+HK response. */
public class ShortTradesResponse {
    /** Short trade records. US and HK fields populated depending on market. */
    public ShortTradesItem[] data;
}
