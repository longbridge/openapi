package com.longbridge.quote;

/**
 * One short-position record, unified for US and HK markets.
 * US-specific fields are empty for HK records and vice versa.
 */
public class ShortPositionsItem {
    /** Trading date in RFC 3339 format, e.g. "2022-03-15T04:00:00Z" */
    public String timestamp;
    /** Short ratio */
    public String rate;
    /** Closing price */
    public String close;
    /** [US] Number of short shares outstanding */
    public String currentSharesShort;
    /** [US] Average daily share volume */
    public String avgDailyShareVolume;
    /** [US] Days-to-cover ratio */
    public String daysToCover;
    /** [HK] Short sale amount (HKD) */
    public String amount;
    /** [HK] Short position balance */
    public String balance;
    /** [HK] Closing price (HK naming) */
    public String cost;
}
