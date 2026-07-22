package com.longbridge.trade;

import java.math.BigDecimal;

/**
 * Options for replacing an order
 */
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
    private ReplaceAttachedParams attachedParams;

    /**
     * Constructs options for replacing an order.
     *
     * @param orderId  the ID of the order to replace
     * @param quantity new order quantity
     */
    public ReplaceOrderOptions(String orderId, BigDecimal quantity) {
        this.orderId = orderId;
        this.quantity = quantity;
    }

    /**
     * Sets the new order price.
     *
     * @param price new order price
     * @return this instance for chaining
     */
    public ReplaceOrderOptions setPrice(BigDecimal price) {
        this.price = price;
        return this;
    }

    /**
     * Sets the new trigger price.
     *
     * @param triggerPrice new trigger price
     * @return this instance for chaining
     */
    public ReplaceOrderOptions setTriggerPrice(BigDecimal triggerPrice) {
        this.triggerPrice = triggerPrice;
        return this;
    }

    /**
     * Sets the new limit offset.
     *
     * @param limitOffset new limit offset
     * @return this instance for chaining
     */
    public ReplaceOrderOptions setLimitOffset(BigDecimal limitOffset) {
        this.limitOffset = limitOffset;
        return this;
    }

    /**
     * Sets the new trailing amount.
     *
     * @param trailingAmount new trailing amount
     * @return this instance for chaining
     */
    public ReplaceOrderOptions setTrailingAmount(BigDecimal trailingAmount) {
        this.trailingAmount = trailingAmount;
        return this;
    }

    /**
     * Sets the new trailing percentage.
     *
     * @param trailingPercent new trailing percentage
     * @return this instance for chaining
     */
    public ReplaceOrderOptions setTrailingPercent(BigDecimal trailingPercent) {
        this.trailingPercent = trailingPercent;
        return this;
    }

    /**
     * Sets the new limit depth level.
     *
     * @param limitDepthLevel new limit depth level
     * @return this instance for chaining
     */
    public ReplaceOrderOptions setLimitDepthLevel(Integer limitDepthLevel) {
        this.limitDepthLevel = limitDepthLevel;
        return this;
    }

    /**
     * Sets the new trigger count.
     *
     * @param triggerCount new trigger count
     * @return this instance for chaining
     */
    public ReplaceOrderOptions setTriggerCount(Integer triggerCount) {
        this.triggerCount = triggerCount;
        return this;
    }

    /**
     * Sets the new monitor price.
     *
     * @param monitorPrice new monitor price
     * @return this instance for chaining
     */
    public ReplaceOrderOptions setMonitorPrice(BigDecimal monitorPrice) {
        this.monitorPrice = monitorPrice;
        return this;
    }

    /**
     * Sets the order remark.
     *
     * @param remark remark
     * @return this instance for chaining
     */
    public ReplaceOrderOptions setRemark(String remark) {
        this.remark = remark;
        return this;
    }

    /**
     * Sets the attached order parameters.
     *
     * @param attachedParams attached order parameters
     * @return this instance for chaining
     */
    public ReplaceOrderOptions setAttachedParams(ReplaceAttachedParams attachedParams) {
        this.attachedParams = attachedParams;
        return this;
    }

    /**
     * Returns the attached order parameters.
     *
     * @return attached order parameters
     */
    public ReplaceAttachedParams getAttachedParams() {
        return attachedParams;
    }

}
