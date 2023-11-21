package com.longport.quote;

import java.math.BigDecimal;
import java.time.LocalDate;
import java.time.OffsetDateTime;

public class WarrantQuote {
    private String symbol;
    private BigDecimal lastDone;
    private BigDecimal prevClose;
    private BigDecimal open;
    private BigDecimal high;
    private BigDecimal low;
    private OffsetDateTime timestamp;
    private long volume;
    private BigDecimal turnover;
    private TradeStatus tradeStatus;
    private BigDecimal impliedVolatility;
    private long openInterest;
    private LocalDate expiryDate;
    private LocalDate lastTradeDate;
    private BigDecimal outstandingRatio;
    private long outstandingQuantity;
    private BigDecimal conversionRatio;
    private WarrantType category;
    private BigDecimal strikePrice;
    private BigDecimal upperStrikePrice;
    private BigDecimal lowerStrikePrice;
    private BigDecimal callPrice;
    private String underlyingSymbol;

    public String getSymbol() {
        return symbol;
    }

    public BigDecimal getLastDone() {
        return lastDone;
    }

    public BigDecimal getPrevClose() {
        return prevClose;
    }

    public BigDecimal getOpen() {
        return open;
    }

    public BigDecimal getHigh() {
        return high;
    }

    public BigDecimal getLow() {
        return low;
    }

    public OffsetDateTime getTimestamp() {
        return timestamp;
    }

    public long getVolume() {
        return volume;
    }

    public BigDecimal getTurnover() {
        return turnover;
    }

    public TradeStatus getTradeStatus() {
        return tradeStatus;
    }

    public BigDecimal getImpliedVolatility() {
        return impliedVolatility;
    }

    public long getOpenInterest() {
        return openInterest;
    }

    public LocalDate getExpiryDate() {
        return expiryDate;
    }

    public LocalDate getLastTradeDate() {
        return lastTradeDate;
    }

    public BigDecimal getOutstandingRatio() {
        return outstandingRatio;
    }

    public long getOutstandingQuantity() {
        return outstandingQuantity;
    }

    public BigDecimal getConversionRatio() {
        return conversionRatio;
    }

    public WarrantType getCategory() {
        return category;
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

    public BigDecimal getCallPrice() {
        return callPrice;
    }

    public String getUnderlyingSymbol() {
        return underlyingSymbol;
    }

    @Override
    public String toString() {
        return "WarrantQuote [callPrice=" + callPrice + ", category=" + category + ", conversionRatio="
                + conversionRatio + ", expiryDate=" + expiryDate + ", high=" + high + ", impliedVolatility="
                + impliedVolatility + ", lastDone=" + lastDone + ", lastTradeDate=" + lastTradeDate + ", low=" + low
                + ", lowerStrikePrice=" + lowerStrikePrice + ", open=" + open + ", openInterest=" + openInterest
                + ", outstandingQuantity=" + outstandingQuantity + ", outstandingRatio=" + outstandingRatio
                + ", prevClose=" + prevClose + ", strikePrice=" + strikePrice + ", symbol=" + symbol + ", timestamp="
                + timestamp + ", tradeStatus=" + tradeStatus + ", turnover=" + turnover + ", underlyingSymbol="
                + underlyingSymbol + ", upperStrikePrice=" + upperStrikePrice + ", volume=" + volume + "]";
    }
}
