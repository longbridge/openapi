package com.longport.quote;

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

    @Override
    public String toString() {
        return "StrikePriceInfo [callSymbol=" + callSymbol + ", price=" + price + ", putSymbol=" + putSymbol
                + ", standard=" + standard + "]";
    }
}
