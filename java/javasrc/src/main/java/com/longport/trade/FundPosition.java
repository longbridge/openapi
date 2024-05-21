package com.longport.trade;

import java.math.BigDecimal;
import java.time.OffsetDateTime;

public class FundPosition {
    private String symbol;
    private BigDecimal currentNetAssetValue;
    private OffsetDateTime netAssetValueDay;
    private String symbolName;
    private String currency;
    private BigDecimal costNetAssetValue;
    private BigDecimal holdingUnits;

    public String getSymbol() {
        return symbol;
    }

    public BigDecimal getCurrentNetAssetValue() {
        return currentNetAssetValue;
    }

    public OffsetDateTime getNetAssetValueDay() {
        return netAssetValueDay;
    }

    public String getSymbolName() {
        return symbolName;
    }

    public String getCurrency() {
        return currency;
    }

    public BigDecimal getCostNetAssetValue() {
        return costNetAssetValue;
    }

    public BigDecimal getHoldingUnits() {
        return holdingUnits;
    }

    @Override
    public String toString() {
        return "FundPosition [symbol=" + symbol + ", currentNetAssetValue=" + currentNetAssetValue
                + ", netAssetValueDay=" + netAssetValueDay + ", symbolName=" + symbolName + ", currency=" + currency
                + ", costNetAssetValue=" + costNetAssetValue + ", holdingUnits=" + holdingUnits + "]";
    }

}
