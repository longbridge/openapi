package com.longbridge.market;

import java.math.BigDecimal;

/** One A/H premium data point. */
public class AhPremiumKline {
    /** A-share price. */
    public BigDecimal aprice;
    /** A-share previous close. */
    public BigDecimal apreclose;
    /** H-share price. */
    public BigDecimal hprice;
    /** H-share previous close. */
    public BigDecimal hpreclose;
    /** CNY/HKD exchange rate. */
    public BigDecimal currencyRate;
    /** A/H premium rate (negative = H-share at premium). */
    public BigDecimal ahpremiumRate;
    /** Price spread. */
    public BigDecimal priceSpread;
    /** Data point timestamp. */
    public java.time.OffsetDateTime timestamp;
}