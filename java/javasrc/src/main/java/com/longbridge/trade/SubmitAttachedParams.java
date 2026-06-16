package com.longbridge.trade;

import java.math.BigDecimal;

/** Attached order parameters for submit order */
public class SubmitAttachedParams {
    private AttachedOrderType attachedOrderType;
    private BigDecimal profitTakerPrice;
    private BigDecimal stopLossPrice;
    private TimeInForceType timeInForce;
    private Long expireTime;
    private OrderType activateOrderType;
    private BigDecimal profitTakerSubmitPrice;
    private BigDecimal stopLossSubmitPrice;
    private OutsideRTH activateRth;

    public SubmitAttachedParams(AttachedOrderType attachedOrderType) {
        this.attachedOrderType = attachedOrderType;
    }

    public AttachedOrderType getAttachedOrderType() { return attachedOrderType; }

    public SubmitAttachedParams setProfitTakerPrice(BigDecimal v) { this.profitTakerPrice = v; return this; }

    public BigDecimal getProfitTakerPrice() { return profitTakerPrice; }

    public SubmitAttachedParams setStopLossPrice(BigDecimal v) { this.stopLossPrice = v; return this; }

    public BigDecimal getStopLossPrice() { return stopLossPrice; }

    public SubmitAttachedParams setTimeInForce(TimeInForceType v) { this.timeInForce = v; return this; }

    public TimeInForceType getTimeInForce() { return timeInForce; }

    public SubmitAttachedParams setExpireTime(long v) { this.expireTime = v; return this; }

    public Long getExpireTime() { return expireTime; }

    public SubmitAttachedParams setActivateOrderType(OrderType v) { this.activateOrderType = v; return this; }

    public OrderType getActivateOrderType() { return activateOrderType; }

    public SubmitAttachedParams setProfitTakerSubmitPrice(BigDecimal v) { this.profitTakerSubmitPrice = v; return this; }

    public BigDecimal getProfitTakerSubmitPrice() { return profitTakerSubmitPrice; }

    public SubmitAttachedParams setStopLossSubmitPrice(BigDecimal v) { this.stopLossSubmitPrice = v; return this; }

    public BigDecimal getStopLossSubmitPrice() { return stopLossSubmitPrice; }

    public SubmitAttachedParams setActivateRth(OutsideRTH v) { this.activateRth = v; return this; }

    public OutsideRTH getActivateRth() { return activateRth; }
}
