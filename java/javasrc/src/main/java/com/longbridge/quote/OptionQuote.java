package com.longbridge.quote;

import java.math.BigDecimal;
import java.time.OffsetDateTime;
import java.time.LocalDate;

public class OptionQuote {
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
    private BigDecimal strikePrice;
    private BigDecimal contractMultiplier;
    private OptionType contractType;
    private BigDecimal contractSize;
    private OptionDirection direction;
    private BigDecimal historicalVolatility;
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

    public BigDecimal getStrikePrice() {
        return strikePrice;
    }

    public BigDecimal getContractMultiplier() {
        return contractMultiplier;
    }

    public OptionType getContractType() {
        return contractType;
    }

    public BigDecimal getContractSize() {
        return contractSize;
    }

    public OptionDirection getDirection() {
        return direction;
    }

    public BigDecimal getHistoricalVolatility() {
        return historicalVolatility;
    }

    public String getUnderlyingSymbol() {
        return underlyingSymbol;
    }

    @Override
    public String toString() {
        return "OptionQuote [contractMultiplier=" + contractMultiplier + ", contractSize=" + contractSize
                + ", contractType=" + contractType + ", direction=" + direction + ", expiryDate=" + expiryDate
                + ", high=" + high + ", historicalVolatility=" + historicalVolatility + ", impliedVolatility="
                + impliedVolatility + ", lastDone=" + lastDone + ", low=" + low + ", open=" + open + ", openInterest="
                + openInterest + ", prevClose=" + prevClose + ", strikePrice=" + strikePrice + ", symbol=" + symbol
                + ", timestamp=" + timestamp + ", tradeStatus=" + tradeStatus + ", turnover=" + turnover
                + ", underlyingSymbol=" + underlyingSymbol + ", volume=" + volume + "]";
    }
}