package com.longbridge.trade;

import java.math.BigDecimal;

/**
 * Grid trading rule. Every field is optional; leave a field null to omit it.
 */
@SuppressWarnings("unused")
public class GridTradeRule {
    private BigDecimal submittedBasePrice;
    private BigDecimal upperLimitPrice;
    private BigDecimal lowerLimitPrice;
    private Integer triggerPriceType;
    private BigDecimal triggerSpreadUp;
    private BigDecimal triggerSpreadDown;
    private BigDecimal triggerPercentUp;
    private BigDecimal triggerPercentDown;
    private Boolean multipleTrigger;
    private Integer timeInForce;
    private BigDecimal upperLimitQuantity;
    private BigDecimal lowerLimitQuantity;
    private Long expireTime;
    private Integer upperLimitEvent;
    private Integer lowerLimitEvent;
    private Integer triggerSellDepth;
    private Integer triggerBuyDepth;
    private BigDecimal triggerQuantity;
    private Boolean supportShortsell;
    private Integer rth;
    private String gridOrderTypeUp;
    private String gridOrderTypeDown;

    /**
     * Sets the base price the grid is anchored to.
     *
     * @param submittedBasePrice base price
     * @return this instance for chaining
     */
    public GridTradeRule setSubmittedBasePrice(BigDecimal submittedBasePrice) {
        this.submittedBasePrice = submittedBasePrice;
        return this;
    }

    /**
     * Sets the upper price bound.
     *
     * @param upperLimitPrice upper price bound
     * @return this instance for chaining
     */
    public GridTradeRule setUpperLimitPrice(BigDecimal upperLimitPrice) {
        this.upperLimitPrice = upperLimitPrice;
        return this;
    }

    /**
     * Sets the lower price bound.
     *
     * @param lowerLimitPrice lower price bound
     * @return this instance for chaining
     */
    public GridTradeRule setLowerLimitPrice(BigDecimal lowerLimitPrice) {
        this.lowerLimitPrice = lowerLimitPrice;
        return this;
    }

    /**
     * Sets the trigger price type (only 1 / 2 allowed).
     *
     * @param triggerPriceType trigger price type
     * @return this instance for chaining
     */
    public GridTradeRule setTriggerPriceType(Integer triggerPriceType) {
        this.triggerPriceType = triggerPriceType;
        return this;
    }

    /**
     * Sets the upward trigger spread (absolute).
     *
     * @param triggerSpreadUp upward trigger spread
     * @return this instance for chaining
     */
    public GridTradeRule setTriggerSpreadUp(BigDecimal triggerSpreadUp) {
        this.triggerSpreadUp = triggerSpreadUp;
        return this;
    }

    /**
     * Sets the downward trigger spread (absolute).
     *
     * @param triggerSpreadDown downward trigger spread
     * @return this instance for chaining
     */
    public GridTradeRule setTriggerSpreadDown(BigDecimal triggerSpreadDown) {
        this.triggerSpreadDown = triggerSpreadDown;
        return this;
    }

    /**
     * Sets the upward trigger percent.
     *
     * @param triggerPercentUp upward trigger percent
     * @return this instance for chaining
     */
    public GridTradeRule setTriggerPercentUp(BigDecimal triggerPercentUp) {
        this.triggerPercentUp = triggerPercentUp;
        return this;
    }

    /**
     * Sets the downward trigger percent.
     *
     * @param triggerPercentDown downward trigger percent
     * @return this instance for chaining
     */
    public GridTradeRule setTriggerPercentDown(BigDecimal triggerPercentDown) {
        this.triggerPercentDown = triggerPercentDown;
        return this;
    }

    /**
     * Sets whether a single grid level may trigger multiple times.
     *
     * @param multipleTrigger multiple-trigger flag
     * @return this instance for chaining
     */
    public GridTradeRule setMultipleTrigger(Boolean multipleTrigger) {
        this.multipleTrigger = multipleTrigger;
        return this;
    }

    /**
     * Sets the time in force (0 = Day, 1 = GTC, 6 = GTD).
     *
     * @param timeInForce time in force
     * @return this instance for chaining
     */
    public GridTradeRule setTimeInForce(Integer timeInForce) {
        this.timeInForce = timeInForce;
        return this;
    }

    /**
     * Sets the quantity handled when the upper bound is reached.
     *
     * @param upperLimitQuantity quantity
     * @return this instance for chaining
     */
    public GridTradeRule setUpperLimitQuantity(BigDecimal upperLimitQuantity) {
        this.upperLimitQuantity = upperLimitQuantity;
        return this;
    }

    /**
     * Sets the quantity handled when the lower bound is reached.
     *
     * @param lowerLimitQuantity quantity
     * @return this instance for chaining
     */
    public GridTradeRule setLowerLimitQuantity(BigDecimal lowerLimitQuantity) {
        this.lowerLimitQuantity = lowerLimitQuantity;
        return this;
    }

    /**
     * Sets the expiry time (unix seconds), used with GTD.
     *
     * @param expireTime expiry time (unix seconds)
     * @return this instance for chaining
     */
    public GridTradeRule setExpireTime(Long expireTime) {
        this.expireTime = expireTime;
        return this;
    }

    /**
     * Sets the action when the upper bound is reached (only 1 / 2 allowed).
     *
     * @param upperLimitEvent action code
     * @return this instance for chaining
     */
    public GridTradeRule setUpperLimitEvent(Integer upperLimitEvent) {
        this.upperLimitEvent = upperLimitEvent;
        return this;
    }

    /**
     * Sets the action when the lower bound is reached (only 1 / 2 allowed).
     *
     * @param lowerLimitEvent action code
     * @return this instance for chaining
     */
    public GridTradeRule setLowerLimitEvent(Integer lowerLimitEvent) {
        this.lowerLimitEvent = lowerLimitEvent;
        return this;
    }

    /**
     * Sets the sell-side order-book depth (-5..5, 0 = use gridOrderTypeUp).
     *
     * @param triggerSellDepth sell-side depth
     * @return this instance for chaining
     */
    public GridTradeRule setTriggerSellDepth(Integer triggerSellDepth) {
        this.triggerSellDepth = triggerSellDepth;
        return this;
    }

    /**
     * Sets the buy-side order-book depth (-5..5, 0 = use gridOrderTypeDown).
     *
     * @param triggerBuyDepth buy-side depth
     * @return this instance for chaining
     */
    public GridTradeRule setTriggerBuyDepth(Integer triggerBuyDepth) {
        this.triggerBuyDepth = triggerBuyDepth;
        return this;
    }

    /**
     * Sets the quantity per trigger.
     *
     * @param triggerQuantity quantity per trigger
     * @return this instance for chaining
     */
    public GridTradeRule setTriggerQuantity(BigDecimal triggerQuantity) {
        this.triggerQuantity = triggerQuantity;
        return this;
    }

    /**
     * Sets whether short selling is allowed.
     *
     * @param supportShortsell short-sell flag
     * @return this instance for chaining
     */
    public GridTradeRule setSupportShortsell(Boolean supportShortsell) {
        this.supportShortsell = supportShortsell;
        return this;
    }

    /**
     * Sets the regular trading hours flag (0 / 1 / 2).
     *
     * @param rth regular trading hours flag
     * @return this instance for chaining
     */
    public GridTradeRule setRth(Integer rth) {
        this.rth = rth;
        return this;
    }

    /**
     * Sets the sell-side order type when depth is 0 (GMO / GLO / GTG).
     *
     * @param gridOrderTypeUp sell-side order type
     * @return this instance for chaining
     */
    public GridTradeRule setGridOrderTypeUp(String gridOrderTypeUp) {
        this.gridOrderTypeUp = gridOrderTypeUp;
        return this;
    }

    /**
     * Sets the buy-side order type when depth is 0 (GMO / GLO / GTG).
     *
     * @param gridOrderTypeDown buy-side order type
     * @return this instance for chaining
     */
    public GridTradeRule setGridOrderTypeDown(String gridOrderTypeDown) {
        this.gridOrderTypeDown = gridOrderTypeDown;
        return this;
    }
}
