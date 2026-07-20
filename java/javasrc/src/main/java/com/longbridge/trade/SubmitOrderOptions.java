package com.longbridge.trade;

import java.math.BigDecimal;
import java.time.LocalDate;

/**
 * Options for submitting an order
 */
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
    private String clientRequestId;

    /**
     * Constructs options for submitting an order.
     *
     * @param symbol            security symbol
     * @param orderType         order type
     * @param side              order side
     * @param submittedQuantity order quantity
     * @param timeInForce       time-in-force type
     */
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

    /**
     * Sets the submitted price.
     *
     * @param submittedPrice submitted price
     * @return this instance for chaining
     */
    public SubmitOrderOptions setSubmittedPrice(BigDecimal submittedPrice) {
        this.submittedPrice = submittedPrice;
        return this;
    }

    /**
     * Sets the trigger price.
     *
     * @param triggerPrice trigger price
     * @return this instance for chaining
     */
    public SubmitOrderOptions setTriggerPrice(BigDecimal triggerPrice) {
        this.triggerPrice = triggerPrice;
        return this;
    }

    /**
     * Sets the limit offset.
     *
     * @param limitOffset limit offset
     * @return this instance for chaining
     */
    public SubmitOrderOptions setLimitOffset(BigDecimal limitOffset) {
        this.limitOffset = limitOffset;
        return this;
    }

    /**
     * Sets the trailing amount.
     *
     * @param trailingAmount trailing amount
     * @return this instance for chaining
     */
    public SubmitOrderOptions setTrailingAmount(BigDecimal trailingAmount) {
        this.trailingAmount = trailingAmount;
        return this;
    }

    /**
     * Sets the trailing percentage.
     *
     * @param trailingPercent trailing percentage
     * @return this instance for chaining
     */
    public SubmitOrderOptions setTrailingPercent(BigDecimal trailingPercent) {
        this.trailingPercent = trailingPercent;
        return this;
    }

    /**
     * Sets the expiry date (for GoodTilDate orders).
     *
     * @param expireDate expiry date
     * @return this instance for chaining
     */
    public SubmitOrderOptions setExpireDate(LocalDate expireDate) {
        this.expireDate = expireDate;
        return this;
    }

    /**
     * Sets the outside regular trading hours setting.
     *
     * @param outsideRth outside-RTH setting
     * @return this instance for chaining
     */
    public SubmitOrderOptions setOutsideRth(OutsideRTH outsideRth) {
        this.outsideRth = outsideRth;
        return this;
    }

    /**
     * Sets the limit depth level.
     *
     * @param limitDepthLevel limit depth level
     * @return this instance for chaining
     */
    public SubmitOrderOptions setLimitDepthLevel(Integer limitDepthLevel) {
        this.limitDepthLevel = limitDepthLevel;
        return this;
    }

    /**
     * Sets the trigger count.
     *
     * @param triggerCount trigger count
     * @return this instance for chaining
     */
    public SubmitOrderOptions setTriggerCount(Integer triggerCount) {
        this.triggerCount = triggerCount;
        return this;
    }

    /**
     * Sets the monitor price.
     *
     * @param monitorPrice monitor price
     * @return this instance for chaining
     */
    public SubmitOrderOptions setMonitorPrice(BigDecimal monitorPrice) {
        this.monitorPrice = monitorPrice;
        return this;
    }

    /**
     * Sets the order remark.
     *
     * @param remark remark
     * @return this instance for chaining
     */
    public SubmitOrderOptions setRemark(String remark) {
        this.remark = remark;
        return this;
    }

    /**
     * Sets the idempotent request ID.
     * If not specified, idempotency control is skipped.
     * The server caches this ID for 10 minutes to prevent duplicate orders.
     *
     * @param clientRequestId unique request identifier (e.g., UUID)
     * @return this instance for chaining
     */
    public SubmitOrderOptions setClientRequestId(String clientRequestId) {
        this.clientRequestId = clientRequestId;
        return this;
    }
}
