package com.longbridge.trade;

import java.math.BigDecimal;

public class FundPosition {
    private String symbol;
    private BigDecimal currentNetAssetValue;
    private BigDecimal netAssetValueDay;
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

    public BigDecimal getNetAssetValueDay() {
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
}
