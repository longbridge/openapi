package com.longbridge.trade;

import java.math.BigDecimal;

/** Attached order parameters for replace order */
public class ReplaceAttachedParams {
    private AttachedOrderType attachedOrderType;
    private BigDecimal profitTakerPrice;
    private BigDecimal stopLossPrice;
    private TimeInForceType timeInForce;
    private Long expireTime;
    private OrderType activateOrderType;
    private BigDecimal profitTakerSubmitPrice;
    private BigDecimal stopLossSubmitPrice;
    private OutsideRTH activateRth;
    private Long profitTakerId;
    private Long stopLossId;
    private Boolean cancelAllAttached;
    private Long mainId;
    private BigDecimal quantity;
    private BigDecimal marketPrice;

    public ReplaceAttachedParams(AttachedOrderType attachedOrderType) {
        this.attachedOrderType = attachedOrderType;
    }

    public AttachedOrderType getAttachedOrderType() { return attachedOrderType; }

    public ReplaceAttachedParams setProfitTakerPrice(BigDecimal v) { this.profitTakerPrice = v; return this; }

    public BigDecimal getProfitTakerPrice() { return profitTakerPrice; }

    public ReplaceAttachedParams setStopLossPrice(BigDecimal v) { this.stopLossPrice = v; return this; }

    public BigDecimal getStopLossPrice() { return stopLossPrice; }

    public ReplaceAttachedParams setTimeInForce(TimeInForceType v) { this.timeInForce = v; return this; }

    public TimeInForceType getTimeInForce() { return timeInForce; }

    public ReplaceAttachedParams setExpireTime(long v) { this.expireTime = v; return this; }

    public Long getExpireTime() { return expireTime; }

    public ReplaceAttachedParams setActivateOrderType(OrderType v) { this.activateOrderType = v; return this; }

    public OrderType getActivateOrderType() { return activateOrderType; }

    public ReplaceAttachedParams setProfitTakerSubmitPrice(BigDecimal v) { this.profitTakerSubmitPrice = v; return this; }

    public BigDecimal getProfitTakerSubmitPrice() { return profitTakerSubmitPrice; }

    public ReplaceAttachedParams setStopLossSubmitPrice(BigDecimal v) { this.stopLossSubmitPrice = v; return this; }

    public BigDecimal getStopLossSubmitPrice() { return stopLossSubmitPrice; }

    public ReplaceAttachedParams setActivateRth(OutsideRTH v) { this.activateRth = v; return this; }

    public OutsideRTH getActivateRth() { return activateRth; }

    public ReplaceAttachedParams setProfitTakerId(long v) { this.profitTakerId = v; return this; }

    public Long getProfitTakerId() { return profitTakerId; }

    public ReplaceAttachedParams setStopLossId(long v) { this.stopLossId = v; return this; }

    public Long getStopLossId() { return stopLossId; }

    public ReplaceAttachedParams setCancelAllAttached(boolean v) { this.cancelAllAttached = v; return this; }

    public Boolean getCancelAllAttached() { return cancelAllAttached; }

    public ReplaceAttachedParams setMainId(long v) { this.mainId = v; return this; }

    public Long getMainId() { return mainId; }

    public ReplaceAttachedParams setQuantity(BigDecimal v) { this.quantity = v; return this; }

    public BigDecimal getQuantity() { return quantity; }

    public ReplaceAttachedParams setMarketPrice(BigDecimal v) { this.marketPrice = v; return this; }

    public BigDecimal getMarketPrice() { return marketPrice; }
}
