package com.longport.trade;

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

    public ReplaceOrderOptions setPrice(BigDecimal price) {
        this.price = price;
        return this;
    }

    public ReplaceOrderOptions setTriggerPrice(BigDecimal triggerPrice) {
        this.triggerPrice = triggerPrice;
        return this;
    }

    public ReplaceOrderOptions setLimitOffset(BigDecimal limitOffset) {
        this.limitOffset = limitOffset;
        return this;
    }

    public ReplaceOrderOptions setTrailingAmount(BigDecimal trailingAmount) {
        this.trailingAmount = trailingAmount;
        return this;
    }

    public ReplaceOrderOptions setTrailingPercent(BigDecimal trailingPercent) {
        this.trailingPercent = trailingPercent;
        return this;
    }

    public ReplaceOrderOptions setRemark(String remark) {
        this.remark = remark;
        return this;
    }

}
