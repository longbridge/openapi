package com.longport.trade;

import java.math.BigDecimal;

public class MarginRatio {
    private BigDecimal imFactor;
    private BigDecimal mmFactor;
    private BigDecimal fmFactor;

    public BigDecimal getImFactor() {
        return imFactor;
    }

    public BigDecimal getMmFactor() {
        return mmFactor;
    }

    public BigDecimal getFmFactor() {
        return fmFactor;
    }

    @Override
    public String toString() {
        return "MarginRatio [fmFactor=" + fmFactor + ", imFactor=" + imFactor + ", mmFactor=" + mmFactor + "]";
    }

}
