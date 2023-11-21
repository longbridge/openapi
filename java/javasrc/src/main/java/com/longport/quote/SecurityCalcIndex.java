package com.longport.quote;

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
    private double changeRate;
    /// Volume
    private long volume;
    /// Turnover
    private BigDecimal turnover;
    /// Year-to-date change ratio
    private double ytdChangeRate;
    /// Turnover rate
    private double turnoverRate;
    /// Total market value
    private BigDecimal totalMarketValue;
    /// Capital flow
    private BigDecimal capitalFlow;
    /// Amplitude
    private double amplitude;
    /// Volume ratio
    private double volumeRatio;
    /// PE (TTM)
    private double peTtmRatio;
    /// PB
    private double pbRatio;
    /// Dividend ratio (TTM)
    private double dividendRatioTtm;
    /// Five days change ratio
    private double fiveDayChangeRate;
    /// Ten days change ratio
    private double tenDayChangeRate;
    /// Half year change ratio
    private double halfYearChangeRate;
    /// Five minutes change ratio
    private double fiveMinutesChangeRate;
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
    private double outstandingRatio;
    /// Premium
    private double premium;
    /// In/out of the bound
    private double itmOtm;
    /// Implied volatility
    private double impliedVolatility;
    /// Warrant delta
    private double warrantDelta;
    /// Call price
    private BigDecimal callPrice;
    /// Price interval from the call price
    private BigDecimal toCallPrice;
    /// Effective leverage
    private double effectiveLeverage;
    /// Leverage ratio
    private double leverageRatio;
    /// Conversion ratio
    private double conversionRatio;
    /// Breakeven point
    private double balancePoint;
    /// Open interest
    private long openInterest;
    /// Delta
    private double delta;
    /// Gamma
    private double gamma;
    /// Theta
    private double theta;
    /// Vega
    private double vega;
    /// Rho
    private double rho;

    public String getSymbol() {
        return symbol;
    }

    public BigDecimal getLastDone() {
        return lastDone;
    }

    public BigDecimal getChangeValue() {
        return changeValue;
    }

    public double getChangeRate() {
        return changeRate;
    }

    public long getVolume() {
        return volume;
    }

    public BigDecimal getTurnover() {
        return turnover;
    }

    public double getYtdChangeRate() {
        return ytdChangeRate;
    }

    public double getTurnoverRate() {
        return turnoverRate;
    }

    public BigDecimal getTotalMarketValue() {
        return totalMarketValue;
    }

    public BigDecimal getCapitalFlow() {
        return capitalFlow;
    }

    public double getAmplitude() {
        return amplitude;
    }

    public double getVolumeRatio() {
        return volumeRatio;
    }

    public double getPeTtmRatio() {
        return peTtmRatio;
    }

    public double getPbRatio() {
        return pbRatio;
    }

    public double getDividendRatioTtm() {
        return dividendRatioTtm;
    }

    public double getFiveDayChangeRate() {
        return fiveDayChangeRate;
    }

    public double getTenDayChangeRate() {
        return tenDayChangeRate;
    }

    public double getHalfYearChangeRate() {
        return halfYearChangeRate;
    }

    public double getFiveMinutesChangeRate() {
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

    public double getOutstandingRatio() {
        return outstandingRatio;
    }

    public double getPremium() {
        return premium;
    }

    public double getItmOtm() {
        return itmOtm;
    }

    public double getImpliedVolatility() {
        return impliedVolatility;
    }

    public double getWarrantDelta() {
        return warrantDelta;
    }

    public BigDecimal getCallPrice() {
        return callPrice;
    }

    public BigDecimal getToCallPrice() {
        return toCallPrice;
    }

    public double getEffectiveLeverage() {
        return effectiveLeverage;
    }

    public double getLeverageRatio() {
        return leverageRatio;
    }

    public double getConversionRatio() {
        return conversionRatio;
    }

    public double getBalancePoint() {
        return balancePoint;
    }

    public long getOpenInterest() {
        return openInterest;
    }

    public double getDelta() {
        return delta;
    }

    public double getGamma() {
        return gamma;
    }

    public double getTheta() {
        return theta;
    }

    public double getVega() {
        return vega;
    }

    public double getRho() {
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
