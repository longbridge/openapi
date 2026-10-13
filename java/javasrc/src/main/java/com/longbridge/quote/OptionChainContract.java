package com.longbridge.quote;

import java.math.BigDecimal;
import java.time.LocalDate;

/**
 * A single option contract of an option chain.
 *
 * <p>
 * Every contract is an independent entry: calls and puts are not paired, so a
 * strike price that is listed on one side only yields a single entry.
 */
public class OptionChainContract {
    private String symbol;
    private LocalDate expiryDate;
    private BigDecimal strikePrice;
    private OptionDirection direction;
    private OptionExpiryCycleType optionType;
    private OptionStandardAttr standardAttr;
    private int daysToExpiry;

    /**
     * Returns the option contract code, in {@code ticker.region} format.
     *
     * @return the option contract code
     */
    public String getSymbol() {
        return symbol;
    }

    /**
     * Returns the expiry date, in US Eastern time.
     *
     * @return the expiry date
     */
    public LocalDate getExpiryDate() {
        return expiryDate;
    }

    /**
     * Returns the strike price.
     *
     * @return the strike price
     */
    public BigDecimal getStrikePrice() {
        return strikePrice;
    }

    /**
     * Returns the contract direction.
     *
     * @return the contract direction
     */
    public OptionDirection getDirection() {
        return direction;
    }

    /**
     * Returns the special expiration cycle of the contract.
     *
     * @return the expiration cycle
     */
    public OptionExpiryCycleType getOptionType() {
        return optionType;
    }

    /**
     * Returns whether the contract is a legacy contract left over from a
     * corporate action.
     *
     * @return the standard attribute
     */
    public OptionStandardAttr getStandardAttr() {
        return standardAttr;
    }

    /**
     * Returns the number of days remaining until the option expires, updated
     * daily at midnight ET.
     *
     * <p>
     * The value is {@code 0} for options expiring today, and negative for
     * already-expired options.
     *
     * @return the number of days remaining until expiry
     */
    public int getDaysToExpiry() {
        return daysToExpiry;
    }

    @Override
    public String toString() {
        return "OptionChainContract [symbol=" + symbol + ", expiryDate=" + expiryDate + ", strikePrice=" + strikePrice
                + ", direction=" + direction + ", optionType=" + optionType + ", standardAttr=" + standardAttr
                + ", daysToExpiry=" + daysToExpiry + "]";
    }
}
