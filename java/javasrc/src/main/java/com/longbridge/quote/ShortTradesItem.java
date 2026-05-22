package com.longbridge.quote;

/**
 * One short-trade record, unified for US and HK markets.
 */
public class ShortTradesItem {
    /** Trading date in RFC 3339 format */
    public String timestamp;
    /** Short ratio */
    public String rate;
    /** Closing price */
    public String close;
    /** [US] NASDAQ short sale volume */
    public String nusAmount;
    /** [US] NYSE short sale volume */
    public String nyAmount;
    /** [US] Total trading volume */
    public String totalAmount;
    /** [HK] Short sale turnover amount (HKD) */
    public String amount;
    /** [HK] Short position balance */
    public String balance;
}
