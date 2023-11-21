package com.longport.trade;

public class EstimateMaxPurchaseQuantityResponse {
    private long cashMaxQty;
    private long marginMaxQty;

    public long getCashMaxQty() {
        return cashMaxQty;
    }

    public long getMarginMaxQty() {
        return marginMaxQty;
    }

    @Override
    public String toString() {
        return "EstimateMaxPurchaseQuantityResponse [cashMaxQty=" + cashMaxQty + ", marginMaxQty=" + marginMaxQty + "]";
    }
}
