package com.longbridge.market;

import java.math.BigDecimal;

/** Trade volume at one price level. */
public class TradePriceLevel {
    /** Buy volume at this price. */
    public BigDecimal buyAmount;
    /** Neutral (unknown direction) volume at this price. */
    public BigDecimal neutralAmount;
    /** Price level. */
    public BigDecimal price;
    /** Sell volume at this price. */
    public BigDecimal sellAmount;
}