package com.longbridge.trade;

import java.math.BigDecimal;

@SuppressWarnings("unused")
public class ReplaceOrderOptions {
    private String orderId;
    private BigDecimal quantity;
    private BigDecimal price;
    private BigDecimal triggerPrice;
    private BigDecimal limitOffset;
    private BigDecimal trailingAmount;
    private BigDecimal trailingPercent;
    private Integer limitDepthLevel;
    private Integer triggerCount;
    private BigDecimal monitorPrice;
    private String remark;

    public ReplaceOrderOptions(String orderId, BigDecimal quantity) {
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

    public ReplaceOrderOptions setLimitDepthLevel(Integer limitDepthLevel) {
        this.limitDepthLevel = limitDepthLevel;
        return this;
    }

    public ReplaceOrderOptions setTriggerCount(Integer triggerCount) {
        this.triggerCount = triggerCount;
        return this;
    }

    public ReplaceOrderOptions setMonitorPrice(BigDecimal monitorPrice) {
        this.monitorPrice = monitorPrice;
        return this;
    }

    public ReplaceOrderOptions setRemark(String remark) {
        this.remark = remark;
        return this;
    }

}
