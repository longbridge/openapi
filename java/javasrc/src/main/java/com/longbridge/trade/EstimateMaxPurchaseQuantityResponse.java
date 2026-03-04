package com.longbridge.trade;

import java.math.BigDecimal;

public class EstimateMaxPurchaseQuantityResponse {
    private BigDecimal cashMaxQty;
    private BigDecimal marginMaxQty;

    public BigDecimal getCashMaxQty() {
        return cashMaxQty;
    }

    public BigDecimal getMarginMaxQty() {
        return marginMaxQty;
    }

    @Override
    public String toString() {
        return "EstimateMaxPurchaseQuantityResponse [cashMaxQty=" + cashMaxQty + ", marginMaxQty=" + marginMaxQty + "]";
    }
}
