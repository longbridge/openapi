package com.longbridge.quote;

import java.math.BigDecimal;
import java.time.OffsetDateTime;

public class SecurityQuote {
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
    private PrePostQuote preMarketQuote;
    private PrePostQuote postMarketQuote;
    private PrePostQuote overnightQuote;

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

    public PrePostQuote getPreMarketQuote() {
        return preMarketQuote;
    }

    public PrePostQuote getPostMarketQuote() {
        return postMarketQuote;
    }

    public PrePostQuote getOvernightQuote() {
        return overnightQuote;
    }

    @Override
    public String toString() {
        return "SecurityQuote [symbol=" + symbol + ", lastDone=" + lastDone + ", prevClose=" + prevClose + ", open="
                + open + ", high=" + high + ", low=" + low + ", timestamp=" + timestamp + ", volume=" + volume
                + ", turnover=" + turnover + ", tradeStatus=" + tradeStatus + ", preMarketQuote=" + preMarketQuote
                + ", postMarketQuote=" + postMarketQuote + ", overnightQuote=" + overnightQuote + "]";
    }

}