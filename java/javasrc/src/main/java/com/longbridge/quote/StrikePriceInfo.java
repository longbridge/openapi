package com.longbridge.quote;

import java.math.BigDecimal;

public class StrikePriceInfo {
    private BigDecimal price;
    private String callSymbol;
    private String putSymbol;
    private boolean standard;

    public BigDecimal getPrice() {
        return price;
    }

    public String getCallSymbol() {
        return callSymbol;
    }

    public String getPutSymbol() {
        return putSymbol;
    }

    public boolean isStandard() {
        return standard;
    }
}
