package com.longport.quote;

import java.math.BigDecimal;
import java.time.LocalDate;

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

    public String getSymbol() {
        return symbol;
    }

    public WarrantType getWarrantType() {
        return warrantType;
    }

    public String getName() {
        return name;
    }

    public BigDecimal getLastDone() {
        return lastDone;
    }

    public BigDecimal getChangeRate() {
        return changeRate;
    }

    public BigDecimal getChangeValue() {
        return changeValue;
    }

    public long getVolume() {
        return volume;
    }

    public BigDecimal getTurnover() {
        return turnover;
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

    public BigDecimal getDelta() {
        return delta;
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
