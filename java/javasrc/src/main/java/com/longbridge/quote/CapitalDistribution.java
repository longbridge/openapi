package com.longbridge.quote;

import java.math.BigDecimal;

public class CapitalDistribution {
    private BigDecimal large;
    private BigDecimal medium;
    private BigDecimal small;

    public BigDecimal getLarge() {
        return large;
    }

    public BigDecimal getMedium() {
        return medium;
    }

    public BigDecimal getSmall() {
        return small;
    }

    @Override
    public String toString() {
        return "CapitalDistribution [large=" + large + ", medium=" + medium + ", small=" + small + "]";
    }
}
