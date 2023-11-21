package com.longport.trade;

import java.math.BigDecimal;

import com.longport.Market;

public class StockPosition {
    private String symbol;
    private String symbolName;
    private long quantity;
    private long availableQuantity;
    private String currency;
    private BigDecimal costPrice;
    private Market market;

    public String getSymbol() {
        return symbol;
    }

    public String getSymbolName() {
        return symbolName;
    }

    public long getQuantity() {
        return quantity;
    }

    public long getAvailableQuantity() {
        return availableQuantity;
    }

    public String getCurrency() {
        return currency;
    }

    public BigDecimal getCostPrice() {
        return costPrice;
    }

    public Market getMarket() {
        return market;
    }

    @Override
    public String toString() {
        return "StockPosition [availableQuantity=" + availableQuantity + ", costPrice=" + costPrice + ", currency="
                + currency + ", market=" + market + ", quantity=" + quantity + ", symbol=" + symbol + ", symbolName="
                + symbolName + "]";
    }
}
