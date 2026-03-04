package com.longbridge.trade;

import java.math.BigDecimal;
import java.time.LocalDate;

@SuppressWarnings("unused")
public class SubmitOrderOptions {
    private String symbol;
    private OrderType orderType;
    private OrderSide side;
    private BigDecimal submittedQuantity;
    private TimeInForceType timeInForce;
    private BigDecimal submittedPrice;
    private BigDecimal triggerPrice;
    private BigDecimal limitOffset;
    private BigDecimal trailingAmount;
    private BigDecimal trailingPercent;
    private LocalDate expireDate;
    private OutsideRTH outsideRth;
    private Integer limitDepthLevel;
    private Integer triggerCount;
    private BigDecimal monitorPrice;
    private String remark;

    public SubmitOrderOptions(
            String symbol,
            OrderType orderType,
            OrderSide side,
            BigDecimal submittedQuantity,
            TimeInForceType timeInForce) {
        this.symbol = symbol;
        this.orderType = orderType;
        this.side = side;
        this.submittedQuantity = submittedQuantity;
        this.timeInForce = timeInForce;
    }

    public SubmitOrderOptions setSubmittedPrice(BigDecimal submittedPrice) {
        this.submittedPrice = submittedPrice;
        return this;
    }

    public SubmitOrderOptions setTriggerPrice(BigDecimal triggerPrice) {
        this.triggerPrice = triggerPrice;
        return this;
    }

    public SubmitOrderOptions setLimitOffset(BigDecimal limitOffset) {
        this.limitOffset = limitOffset;
        return this;
    }

    public SubmitOrderOptions setTrailingAmount(BigDecimal trailingAmount) {
        this.trailingAmount = trailingAmount;
        return this;
    }

    public SubmitOrderOptions setTrailingPercent(BigDecimal trailingPercent) {
        this.trailingPercent = trailingPercent;
        return this;
    }

    public SubmitOrderOptions setExpireDate(LocalDate expireDate) {
        this.expireDate = expireDate;
        return this;
    }

    public SubmitOrderOptions setOutsideRth(OutsideRTH outsideRth) {
        this.outsideRth = outsideRth;
        return this;
    }

    public SubmitOrderOptions setLimitDepthLevel(Integer limitDepthLevel) {
        this.limitDepthLevel = limitDepthLevel;
        return this;
    }

    public SubmitOrderOptions setTriggerCount(Integer triggerCount) {
        this.triggerCount = triggerCount;
        return this;
    }

    public SubmitOrderOptions setMonitorPrice(BigDecimal monitorPrice) {
        this.monitorPrice = monitorPrice;
        return this;
    }

    public SubmitOrderOptions setRemark(String remark) {
        this.remark = remark;
        return this;
    }
}
