package com.longbridge.quote;

import java.math.BigDecimal;
import java.time.LocalDate;

public class SecurityCalcIndex {
    /// Security code
    private String symbol;
    /// Latest price
    private BigDecimal lastDone;
    /// Change value
    private BigDecimal changeValue;
    /// Change ratio
    private BigDecimal changeRate;
    /// Volume
    private long volume;
    /// Turnover
    private BigDecimal turnover;
    /// Year-to-date change ratio
    private BigDecimal ytdChangeRate;
    /// Turnover rate
    private BigDecimal turnoverRate;
    /// Total market value
    private BigDecimal totalMarketValue;
    /// Capital flow
    private BigDecimal capitalFlow;
    /// Amplitude
    private BigDecimal amplitude;
    /// Volume ratio
    private BigDecimal volumeRatio;
    /// PE (TTM)
    private BigDecimal peTtmRatio;
    /// PB
    private BigDecimal pbRatio;
    /// Dividend ratio (TTM)
    private BigDecimal dividendRatioTtm;
    /// Five days change ratio
    private BigDecimal fiveDayChangeRate;
    /// Ten days change ratio
    private BigDecimal tenDayChangeRate;
    /// Half year change ratio
    private BigDecimal halfYearChangeRate;
    /// Five minutes change ratio
    private BigDecimal fiveMinutesChangeRate;
    /// Expiry date
    private LocalDate expiryDate;
    /// Strike price
    private BigDecimal strikePrice;
    /// Upper bound price
    private BigDecimal upperStrikePrice;
    /// Lower bound price
    private BigDecimal lowerStrikePrice;
    /// Outstanding quantity
    private long outstandingQty;
    /// Outstanding ratio
    private BigDecimal outstandingRatio;
    /// Premium
    private BigDecimal premium;
    /// In/out of the bound
    private BigDecimal itmOtm;
    /// Implied volatility
    private BigDecimal impliedVolatility;
    /// Warrant delta
    private BigDecimal warrantDelta;
    /// Call price
    private BigDecimal callPrice;
    /// Price interval from the call price
    private BigDecimal toCallPrice;
    /// Effective leverage
    private BigDecimal effectiveLeverage;
    /// Leverage ratio
    private BigDecimal leverageRatio;
    /// Conversion ratio
    private BigDecimal conversionRatio;
    /// Breakeven point
    private BigDecimal balancePoint;
    /// Open interest
    private long openInterest;
    /// Delta
    private BigDecimal delta;
    /// Gamma
    private BigDecimal gamma;
    /// Theta
    private BigDecimal theta;
    /// Vega
    private BigDecimal vega;
    /// Rho
    private BigDecimal rho;

    public String getSymbol() {
        return symbol;
    }

    public BigDecimal getLastDone() {
        return lastDone;
    }

    public BigDecimal getChangeValue() {
        return changeValue;
    }

    public BigDecimal getChangeRate() {
        return changeRate;
    }

    public long getVolume() {
        return volume;
    }

    public BigDecimal getTurnover() {
        return turnover;
    }

    public BigDecimal getYtdChangeRate() {
        return ytdChangeRate;
    }

    public BigDecimal getTurnoverRate() {
        return turnoverRate;
    }

    public BigDecimal getTotalMarketValue() {
        return totalMarketValue;
    }

    public BigDecimal getCapitalFlow() {
        return capitalFlow;
    }

    public BigDecimal getAmplitude() {
        return amplitude;
    }

    public BigDecimal getVolumeRatio() {
        return volumeRatio;
    }

    public BigDecimal getPeTtmRatio() {
        return peTtmRatio;
    }

    public BigDecimal getPbRatio() {
        return pbRatio;
    }

    public BigDecimal getDividendRatioTtm() {
        return dividendRatioTtm;
    }

    public BigDecimal getFiveDayChangeRate() {
        return fiveDayChangeRate;
    }

    public BigDecimal getTenDayChangeRate() {
        return tenDayChangeRate;
    }

    public BigDecimal getHalfYearChangeRate() {
        return halfYearChangeRate;
    }

    public BigDecimal getFiveMinutesChangeRate() {
        return fiveMinutesChangeRate;
    }

    public LocalDate getExpiryDate() {
        return expiryDate;
    }

    public BigDecimal getStrikePrice() {
        return strikePrice;
    }

    public BigDecimal getUpperStrikePrice() {
        return upperStrikePrice;
    }

    public BigDecimal getLowerStrikePrice() {
        return lowerStrikePrice;
    }

    public long getOutstandingQty() {
        return outstandingQty;
    }

    public BigDecimal getOutstandingRatio() {
        return outstandingRatio;
    }

    public BigDecimal getPremium() {
        return premium;
    }

    public BigDecimal getItmOtm() {
        return itmOtm;
    }

    public BigDecimal getImpliedVolatility() {
        return impliedVolatility;
    }

    public BigDecimal getWarrantDelta() {
        return warrantDelta;
    }

    public BigDecimal getCallPrice() {
        return callPrice;
    }

    public BigDecimal getToCallPrice() {
        return toCallPrice;
    }

    public BigDecimal getEffectiveLeverage() {
        return effectiveLeverage;
    }

    public BigDecimal getLeverageRatio() {
        return leverageRatio;
    }

    public BigDecimal getConversionRatio() {
        return conversionRatio;
    }

    public BigDecimal getBalancePoint() {
        return balancePoint;
    }

    public long getOpenInterest() {
        return openInterest;
    }

    public BigDecimal getDelta() {
        return delta;
    }

    public BigDecimal getGamma() {
        return gamma;
    }

    public BigDecimal getTheta() {
        return theta;
    }

    public BigDecimal getVega() {
        return vega;
    }

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
