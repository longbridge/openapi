package com.longbridge.trade;

import java.math.BigDecimal;

@SuppressWarnings("unused")
public class ReplaceOrderOptions {
    private String orderId;
    private long quantity;
    private BigDecimal price;
    private BigDecimal triggerPrice;
    private BigDecimal limitOffset;
    private BigDecimal trailingAmount;
    private BigDecimal trailingPercent;
    private String remark;

    public ReplaceOrderOptions(String orderId, long quantity) {
        this.orderId = orderId;
        this.quantity = quantity;
    }

    public void setPrice(BigDecimal price) {
        this.price = price;
    }

    public void setTriggerPrice(BigDecimal triggerPrice) {
        this.triggerPrice = triggerPrice;
    }

    public void setLimitOffset(BigDecimal limitOffset) {
        this.limitOffset = limitOffset;
    }

    public void setTrailingAmount(BigDecimal trailingAmount) {
        this.trailingAmount = trailingAmount;
    }

    public void setTrailingPercent(BigDecimal trailingPercent) {
        this.trailingPercent = trailingPercent;
    }

    public void setRemark(String remark) {
        this.remark = remark;
    }

}
