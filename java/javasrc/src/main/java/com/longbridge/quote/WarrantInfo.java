package com.longbridge.quote;

import java.math.BigDecimal;
import java.time.LocalDate;

/**
 * Warrant information from the warrant list.
 */
public class WarrantInfo {
    private String symbol;
    private WarrantType warrantType;
    private String name;
    private BigDecimal lastDone;
    private BigDecimal changeRate;
    private BigDecimal changeValue;
    private long volume;
    private BigDecimal turnover;
    private LocalDate expiryDate;
    private BigDecimal strikePrice;
    private BigDecimal upperStrikePrice;
    private BigDecimal lowerStrikePrice;
    private long outstandingQty;
    private BigDecimal outstandingRatio;
    private BigDecimal premium;
    private BigDecimal itmOtm;
    private BigDecimal impliedVolatility;
    private BigDecimal delta;
    private BigDecimal callPrice;
    private BigDecimal toCallPrice;
    private BigDecimal effectiveLeverage;
    private BigDecimal leverageRatio;
    private BigDecimal conversionRatio;
    private BigDecimal balancePoint;
    private WarrantStatus status;

    /**
     * Returns the security code.
     *
     * @return the security code
     */
    public String getSymbol() {
        return symbol;
    }

    /**
     * Returns the warrant type.
     *
     * @return the warrant type
     */
    public WarrantType getWarrantType() {
        return warrantType;
    }

    /**
     * Returns the warrant name.
     *
     * @return the warrant name
     */
    public String getName() {
        return name;
    }

    /**
     * Returns the latest price.
     *
     * @return the latest price
     */
    public BigDecimal getLastDone() {
        return lastDone;
    }

    /**
     * Returns the change ratio.
     *
     * @return the change ratio
     */
    public BigDecimal getChangeRate() {
        return changeRate;
    }

    /**
     * Returns the change value.
     *
     * @return the change value
     */
    public BigDecimal getChangeValue() {
        return changeValue;
    }

    /**
     * Returns the trading volume.
     *
     * @return the trading volume
     */
    public long getVolume() {
        return volume;
    }

    /**
     * Returns the turnover.
     *
     * @return the turnover
     */
    public BigDecimal getTurnover() {
        return turnover;
    }

    /**
     * Returns the expiry date.
     *
     * @return the expiry date, or {@code null} if the server does not report an
     *         expiry date for this warrant
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
     * Returns the upper bound price (for inline warrants).
     *
     * @return the upper bound price
     */
    public BigDecimal getUpperStrikePrice() {
        return upperStrikePrice;
    }

    /**
     * Returns the lower bound price (for inline warrants).
     *
     * @return the lower bound price
     */
    public BigDecimal getLowerStrikePrice() {
        return lowerStrikePrice;
    }

    /**
     * Returns the outstanding quantity.
     *
     * @return the outstanding quantity
     */
    public long getOutstandingQty() {
        return outstandingQty;
    }

    /**
     * Returns the outstanding ratio.
     *
     * @return the outstanding ratio
     */
    public BigDecimal getOutstandingRatio() {
        return outstandingRatio;
    }

    /**
     * Returns the premium.
     *
     * @return the premium
     */
    public BigDecimal getPremium() {
        return premium;
    }

    /**
     * Returns whether the warrant is in or out of the bound (ITM/OTM).
     *
     * @return the ITM/OTM value
     */
    public BigDecimal getItmOtm() {
        return itmOtm;
    }

    /**
     * Returns the implied volatility.
     *
     * @return the implied volatility
     */
    public BigDecimal getImpliedVolatility() {
        return impliedVolatility;
    }

    /**
     * Returns the delta.
     *
     * @return the delta
     */
    public BigDecimal getDelta() {
        return delta;
    }

    /**
     * Returns the call price.
     *
     * @return the call price
     */
    public BigDecimal getCallPrice() {
        return callPrice;
    }

    /**
     * Returns the price interval from the call price.
     *
     * @return the price interval from the call price
     */
    public BigDecimal getToCallPrice() {
        return toCallPrice;
    }

    /**
     * Returns the effective leverage.
     *
     * @return the effective leverage
     */
    public BigDecimal getEffectiveLeverage() {
        return effectiveLeverage;
    }

    /**
     * Returns the leverage ratio.
     *
     * @return the leverage ratio
     */
    public BigDecimal getLeverageRatio() {
        return leverageRatio;
    }

    /**
     * Returns the conversion ratio.
     *
     * @return the conversion ratio
     */
    public BigDecimal getConversionRatio() {
        return conversionRatio;
    }

    /**
     * Returns the breakeven point.
     *
     * @return the breakeven point
     */
    public BigDecimal getBalancePoint() {
        return balancePoint;
    }

    /**
     * Returns the warrant status.
     *
     * @return the warrant status
     */
    public WarrantStatus getStatus() {
        return status;
    }

    @Override
    public String toString() {
        return "WarrantInfo [symbol=" + symbol + ", warrantType=" + warrantType + ", name=" + name + ", lastDone="
                + lastDone + ", changeRate=" + changeRate + ", changeValue=" + changeValue + ", volume=" + volume
                + ", turnover=" + turnover + ", expiryDate=" + expiryDate + ", strikePrice=" + strikePrice
                + ", upperStrikePrice=" + upperStrikePrice + ", lowerStrikePrice=" + lowerStrikePrice
                + ", outstandingQty=" + outstandingQty + ", outstandingRatio=" + outstandingRatio + ", premium="
                + premium + ", itmOtm=" + itmOtm + ", impliedVolatility=" + impliedVolatility + ", delta=" + delta
                + ", callPrice=" + callPrice + ", toCallPrice=" + toCallPrice + ", effectiveLeverage="
                + effectiveLeverage + ", leverageRatio=" + leverageRatio + ", conversionRatio=" + conversionRatio
                + ", balancePoint=" + balancePoint + ", status=" + status + "]";
    }

}
