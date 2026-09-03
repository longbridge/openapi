package com.longbridge.quote;

import java.math.BigDecimal;
import java.time.LocalDate;

/**
 * Calculated indexes for a security.
 */
public class SecurityCalcIndex {
    private String symbol;
    private BigDecimal lastDone;
    private BigDecimal changeValue;
    private BigDecimal changeRate;
    private long volume;
    private BigDecimal turnover;
    private BigDecimal ytdChangeRate;
    private BigDecimal turnoverRate;
    private BigDecimal totalMarketValue;
    private BigDecimal capitalFlow;
    private BigDecimal amplitude;
    private BigDecimal volumeRatio;
    private BigDecimal peTtmRatio;
    private BigDecimal pbRatio;
    private BigDecimal dividendRatioTtm;
    private BigDecimal fiveDayChangeRate;
    private BigDecimal tenDayChangeRate;
    private BigDecimal halfYearChangeRate;
    private BigDecimal fiveMinutesChangeRate;
    private LocalDate expiryDate;
    private BigDecimal strikePrice;
    private BigDecimal upperStrikePrice;
    private BigDecimal lowerStrikePrice;
    private long outstandingQty;
    private BigDecimal outstandingRatio;
    private BigDecimal premium;
    private BigDecimal itmOtm;
    private BigDecimal impliedVolatility;
    private BigDecimal warrantDelta;
    private BigDecimal callPrice;
    private BigDecimal toCallPrice;
    private BigDecimal effectiveLeverage;
    private BigDecimal leverageRatio;
    private BigDecimal conversionRatio;
    private BigDecimal balancePoint;
    private long openInterest;
    private BigDecimal delta;
    private BigDecimal gamma;
    private BigDecimal theta;
    private BigDecimal vega;
    private BigDecimal rho;

    /**
     * Returns the security code.
     *
     * @return the security code
     */
    public String getSymbol() {
        return symbol;
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
     * Returns the change value.
     *
     * @return the change value
     */
    public BigDecimal getChangeValue() {
        return changeValue;
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
     * Returns the volume.
     *
     * @return the volume
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
     * Returns the year-to-date change ratio.
     *
     * @return the year-to-date change ratio
     */
    public BigDecimal getYtdChangeRate() {
        return ytdChangeRate;
    }

    /**
     * Returns the turnover rate.
     *
     * @return the turnover rate
     */
    public BigDecimal getTurnoverRate() {
        return turnoverRate;
    }

    /**
     * Returns the total market value.
     *
     * @return the total market value
     */
    public BigDecimal getTotalMarketValue() {
        return totalMarketValue;
    }

    /**
     * Returns the capital flow.
     *
     * @return the capital flow
     */
    public BigDecimal getCapitalFlow() {
        return capitalFlow;
    }

    /**
     * Returns the amplitude.
     *
     * @return the amplitude
     */
    public BigDecimal getAmplitude() {
        return amplitude;
    }

    /**
     * Returns the volume ratio.
     *
     * @return the volume ratio
     */
    public BigDecimal getVolumeRatio() {
        return volumeRatio;
    }

    /**
     * Returns the PE (TTM).
     *
     * @return the PE (TTM)
     */
    public BigDecimal getPeTtmRatio() {
        return peTtmRatio;
    }

    /**
     * Returns the PB.
     *
     * @return the PB
     */
    public BigDecimal getPbRatio() {
        return pbRatio;
    }

    /**
     * Returns the dividend ratio (TTM).
     *
     * @return the dividend ratio (TTM)
     */
    public BigDecimal getDividendRatioTtm() {
        return dividendRatioTtm;
    }

    /**
     * Returns the five days change ratio.
     *
     * @return the five days change ratio
     */
    public BigDecimal getFiveDayChangeRate() {
        return fiveDayChangeRate;
    }

    /**
     * Returns the ten days change ratio.
     *
     * @return the ten days change ratio
     */
    public BigDecimal getTenDayChangeRate() {
        return tenDayChangeRate;
    }

    /**
     * Returns the half year change ratio.
     *
     * @return the half year change ratio
     */
    public BigDecimal getHalfYearChangeRate() {
        return halfYearChangeRate;
    }

    /**
     * Returns the five minutes change ratio.
     *
     * @return the five minutes change ratio
     */
    public BigDecimal getFiveMinutesChangeRate() {
        return fiveMinutesChangeRate;
    }

    /**
     * Returns the expiry date.
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
     * Returns the upper bound price.
     *
     * @return the upper bound price
     */
    public BigDecimal getUpperStrikePrice() {
        return upperStrikePrice;
    }

    /**
     * Returns the lower bound price.
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
     * Returns the in/out of the bound value.
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
     * Returns the warrant delta.
     *
     * @return the warrant delta
     */
    public BigDecimal getWarrantDelta() {
        return warrantDelta;
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
     * Returns the open interest.
     *
     * @return the open interest
     */
    public long getOpenInterest() {
        return openInterest;
    }

    /**
     * Returns the delta. Measures the expected change in option price for a $1
     * move in the underlying asset price.
     *
     * @return the delta
     */
    public BigDecimal getDelta() {
        return delta;
    }

    /**
     * Returns the gamma. Measures the expected change in Delta for a $1 move in
     * the underlying asset price.
     *
     * @return the gamma
     */
    public BigDecimal getGamma() {
        return gamma;
    }

    /**
     * Returns the theta. Measures the expected change in option price as one day
     * passes; the raw value has been divided by 365 to convert to a daily value,
     * representing the impact of one day's time decay on the option price.
     *
     * @return the theta
     */
    public BigDecimal getTheta() {
        return theta;
    }

    /**
     * Returns the vega. Measures the expected change in option price when implied
     * volatility (IV) moves by 1 (i.e. 100%); divide the raw value by 100 to get
     * the expected price change per 1% move in IV.
     *
     * @return the vega
     */
    public BigDecimal getVega() {
        return vega;
    }

    /**
     * Returns the rho. Measures the expected change in option price when the
     * risk-free interest rate moves by 1 (i.e. 100%); divide the raw value by 100
     * to get the expected price change per 1% move in the interest rate.
     *
     * @return the rho
     */
    public BigDecimal getRho() {
        return rho;
    }

    @Override
    public String toString() {
        return "SecurityCalcIndex [symbol=" + symbol + ", lastDone=" + lastDone + ", changeValue=" + changeValue
                + ", changeRate=" + changeRate + ", volume=" + volume + ", turnover=" + turnover + ", ytdChangeRate="
                + ytdChangeRate + ", turnoverRate=" + turnoverRate + ", totalMarketValue=" + totalMarketValue
                + ", capitalFlow=" + capitalFlow + ", amplitude=" + amplitude + ", volumeRatio=" + volumeRatio
                + ", peTtmRatio=" + peTtmRatio + ", pbRatio=" + pbRatio + ", dividendRatioTtm=" + dividendRatioTtm
                + ", fiveDayChangeRate=" + fiveDayChangeRate + ", tenDayChangeRate=" + tenDayChangeRate
                + ", halfYearChangeRate=" + halfYearChangeRate + ", fiveMinutesChangeRate=" + fiveMinutesChangeRate
                + ", expiryDate=" + expiryDate + ", strikePrice=" + strikePrice + ", upperStrikePrice="
                + upperStrikePrice + ", lowerStrikePrice=" + lowerStrikePrice + ", outstandingQty=" + outstandingQty
                + ", outstandingRatio=" + outstandingRatio + ", premium=" + premium + ", itmOtm=" + itmOtm
                + ", impliedVolatility=" + impliedVolatility + ", warrantDelta=" + warrantDelta + ", callPrice="
                + callPrice + ", toCallPrice=" + toCallPrice + ", effectiveLeverage=" + effectiveLeverage
                + ", leverageRatio=" + leverageRatio + ", conversionRatio=" + conversionRatio + ", balancePoint="
                + balancePoint + ", openInterest=" + openInterest + ", delta=" + delta + ", gamma=" + gamma + ", theta="
                + theta + ", vega=" + vega + ", rho=" + rho + "]";
    }

}
