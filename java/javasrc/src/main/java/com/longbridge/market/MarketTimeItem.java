package com.longbridge.market;

/** Trading status for one market. */
public class MarketTimeItem {
    /** Market. */
    public com.longbridge.Market market;
    /**
     * Raw trade status code.
     * 101=PreOpen, 102/103/105=Trading, 104=LunchBreak, 106=PostTrading,
     * 108=Closed, 201=PreMarket, 204=PostMarket.
     */
    public int tradeStatus;
    /** Current market time (unix timestamp string). */
    public String timestamp;
    /** Delayed-quote trade status code. */
    public int delayTradeStatus;
    /** Delayed-quote market time (unix timestamp string). */
    public String delayTimestamp;
    /** Sub-status code. */
    public int subStatus;
    /** Delayed-quote sub-status code. */
    public int delaySubStatus;
}