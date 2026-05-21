package com.longbridge.quote;

/** Response for {@link QuoteContext#getShortPositions}. Unified US+HK response. */
public class ShortPositionsResponse {
    /** Short position records. US and HK fields populated depending on market. */
    public ShortPositionsItem[] data;
}