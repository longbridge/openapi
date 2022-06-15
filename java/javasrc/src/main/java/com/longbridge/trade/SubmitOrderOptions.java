package com.longbridge.trade;

import java.math.BigDecimal;
import java.time.LocalDate;

@SuppressWarnings("unused")
public class SubmitOrderOptions {
    private String symbol;
    private OrderType orderType;
    private OrderSide side;
    private long submittedQuantity;
    private TimeInForceType timeInForce;
    private BigDecimal submittedPrice;
    private BigDecimal triggerPrice;
    private BigDecimal limitOffset;
    private BigDecimal trailingAmount;
    private BigDecimal trailingPercent;
    private LocalDate expireDate;
    private OutsideRTH outsideRth;
    private String remark;

    public SubmitOrderOptions(
            String symbol,
            OrderType orderType,
            OrderSide side,
            long submittedQuantity,
            TimeInForceType timeInForce) {
        this.symbol = symbol;
        this.orderType = orderType;
        this.side = side;
        this.submittedQuantity = submittedQuantity;
        this.timeInForce = timeInForce;
    }

    public void setSubmittedPrice(BigDecimal submittedPrice) {
        this.submittedPrice = submittedPrice;
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

    public void setExpireDate(LocalDate expireDate) {
        this.expireDate = expireDate;
    }

    public void setOutsideRth(OutsideRTH outsideRth) {
        this.outsideRth = outsideRth;
    }

    public void setRemark(String remark) {
        this.remark = remark;
    }
}
