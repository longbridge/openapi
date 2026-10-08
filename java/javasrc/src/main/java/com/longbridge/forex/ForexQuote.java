package com.longbridge.forex;

import java.math.BigDecimal;

/**
 * A forex quote.
 */
public class ForexQuote {
    private String quoteId;
    private BigDecimal rate;
    private long expireAt;
    private String ccyPair;

    /**
     * Returns the quote id, used when submitting the order.
     *
     * @return quoteId
     */
    public String getQuoteId() {
        return quoteId;
    }

    /**
     * Returns the customer execution rate (standard currency-pair terms).
     *
     * @return rate
     */
    public BigDecimal getRate() {
        return rate;
    }

    /**
     * Returns the quote expiry, Unix milliseconds.
     *
     * @return expireAt
     */
    public long getExpireAt() {
        return expireAt;
    }

    /**
     * Returns the standard currency pair for {@code rate}, format {@code BASE/QUOTE}.
     *
     * @return ccyPair
     */
    public String getCcyPair() {
        return ccyPair;
    }

    @Override
    public String toString() {
        return "ForexQuote [quoteId=" + quoteId +
                ", rate=" + rate +
                ", expireAt=" + expireAt +
                ", ccyPair=" + ccyPair + "]";
    }
}
