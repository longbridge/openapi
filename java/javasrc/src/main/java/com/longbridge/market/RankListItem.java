package com.longbridge.market;

/** One item in the popularity rank list. */
public class RankListItem {
    /** Symbol, e.g. "MU.US" (converted from counter_id) */
    public String symbol;
    /** Ticker code */
    public String code;
    /** Security name */
    public String name;
    /** Latest price */
    public String lastDone;
    /** Price change ratio (decimal) */
    public String chg;
    /** Absolute price change */
    public String change;
    /** Net inflow */
    public String inflow;
    /** Market cap */
    public String marketCap;
    /** Industry name */
    public String industry;
    /** Pre/post market price */
    public String prePostPrice;
    /** Pre/post market change */
    public String prePostChg;
    /** Amplitude */
    public String amplitude;
    /** 5-day change */
    public String fiveDayChg;
    /** Turnover rate */
    public String turnoverRate;
    /** Volume ratio */
    public String volumeRate;
    /** P/B ratio TTM */
    public String pbTtm;
}
