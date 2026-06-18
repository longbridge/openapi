package com.longbridge.market;

/** Trading status for one market. */
public class MarketTimeItem {
    /** Market. */
    public com.longbridge.Market market;
    /**
     * Raw market trade status code.
     * See the market status definition for the complete code table.
     */
    public int tradeStatus;
    /** Current market time (unix timestamp string). */
    public String timestamp;
    /** Delayed-quote market trade status code. */
    public int delayTradeStatus;
    /** Delayed-quote market time (unix timestamp string). */
    public String delayTimestamp;
    /** Sub-status code. */
    public int subStatus;
    /** Delayed-quote sub-status code. */
    public int delaySubStatus;
}
