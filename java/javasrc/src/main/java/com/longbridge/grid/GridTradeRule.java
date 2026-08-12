package com.longbridge.grid;

import java.math.BigDecimal;

/**
 * Grid trading rule.
 *
 * <p>The {@linkplain #GridTradeRule(BigDecimal, BigDecimal, BigDecimal, GridTrigger,
 * BigDecimal, BigDecimal, BigDecimal, GridTimeInForce) full constructor} takes the
 * fields a valid grid order requires; optional parameters are set through the
 * chained setters. A no-arg constructor is also available for setter-only use.
 */
@SuppressWarnings("unused")
public class GridTradeRule {
    private BigDecimal submittedBasePrice;
    private BigDecimal upperLimitPrice;
    private BigDecimal lowerLimitPrice;
    private TriggerPriceType triggerPriceType;
    private BigDecimal triggerSpreadUp;
    private BigDecimal triggerSpreadDown;
    private BigDecimal triggerPercentUp;
    private BigDecimal triggerPercentDown;
    private Boolean multipleTrigger;
    private GridTimeInForce timeInForce;
    private BigDecimal upperLimitQuantity;
    private BigDecimal lowerLimitQuantity;
    private Long expireTime;
    private GridLimitEvent upperLimitEvent;
    private GridLimitEvent lowerLimitEvent;
    private Integer triggerSellDepth;
    private Integer triggerBuyDepth;
    private BigDecimal triggerQuantity;
    private Boolean supportShortsell;
    private Integer rth;
    private String gridOrderTypeUp;
    private String gridOrderTypeDown;

    /**
     * Creates an empty rule; populate it through the chained setters.
     */
    public GridTradeRule() {
    }

    /**
     * Creates a rule with the fields a valid grid order requires. The gateway
     * still validates business rules, but this makes the minimum field set
     * visible in the signature instead of leaving every field optional.
     *
     * @param basePrice     base price the grid is anchored to
     * @param upperPrice    upper price bound
     * @param lowerPrice    lower price bound
     * @param trigger       up/down trigger thresholds (percent or spread)
     * @param quantity      quantity per trigger
     * @param upperQuantity quantity handled when the upper bound is reached
     * @param lowerQuantity quantity handled when the lower bound is reached
     * @param timeInForce   time in force
     */
    public GridTradeRule(BigDecimal basePrice, BigDecimal upperPrice, BigDecimal lowerPrice,
            GridTrigger trigger, BigDecimal quantity, BigDecimal upperQuantity,
            BigDecimal lowerQuantity, GridTimeInForce timeInForce) {
        this.submittedBasePrice = basePrice;
        this.upperLimitPrice = upperPrice;
        this.lowerLimitPrice = lowerPrice;
        this.triggerQuantity = quantity;
        this.upperLimitQuantity = upperQuantity;
        this.lowerLimitQuantity = lowerQuantity;
        this.timeInForce = timeInForce;
        this.triggerPriceType = trigger.getType();
        if (trigger.getType() == TriggerPriceType.Percent) {
            this.triggerPercentUp = trigger.getUp();
            this.triggerPercentDown = trigger.getDown();
        } else {
            this.triggerSpreadUp = trigger.getUp();
            this.triggerSpreadDown = trigger.getDown();
        }
    }

    /**
     * Sets the actions taken at the upper / lower bounds.
     *
     * @param upper action at the upper bound
     * @param lower action at the lower bound
     * @return this instance for chaining
     */
    public GridTradeRule limitEvents(GridLimitEvent upper, GridLimitEvent lower) {
        this.upperLimitEvent = upper;
        this.lowerLimitEvent = lower;
        return this;
    }

    /**
     * Sets the sell / buy order-book depths (0 = use the order type).
     *
     * @param sell sell-side depth
     * @param buy  buy-side depth
     * @return this instance for chaining
     */
    public GridTradeRule depths(int sell, int buy) {
        this.triggerSellDepth = sell;
        this.triggerBuyDepth = buy;
        return this;
    }

    /**
     * Sets the sell / buy order types (GMO / GLO / GTG).
     *
     * @param up   sell-side order type
     * @param down buy-side order type
     * @return this instance for chaining
     */
    public GridTradeRule orderTypes(String up, String down) {
        this.gridOrderTypeUp = up;
        this.gridOrderTypeDown = down;
        return this;
    }

    /**
     * Allows a single grid level to trigger multiple times.
     *
     * @param value multiple-trigger flag
     * @return this instance for chaining
     */
    public GridTradeRule multipleTrigger(boolean value) {
        this.multipleTrigger = value;
        return this;
    }

    /**
     * Allows short selling.
     *
     * @param value short-sell flag
     * @return this instance for chaining
     */
    public GridTradeRule supportShortsell(boolean value) {
        this.supportShortsell = value;
        return this;
    }

    /**
     * Sets the regular-trading-hours flag (0 / 1 / 2).
     *
     * @param value regular trading hours flag
     * @return this instance for chaining
     */
    public GridTradeRule rth(int value) {
        this.rth = value;
        return this;
    }

    /**
     * Sets the expiry time (unix seconds), used with a GTD time-in-force.
     *
     * @param unixSeconds expiry time (unix seconds)
     * @return this instance for chaining
     */
    public GridTradeRule expireTime(long unixSeconds) {
        this.expireTime = unixSeconds;
        return this;
    }

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
     * Sets the trigger price type.
     *
     * @param triggerPriceType trigger price type
     * @return this instance for chaining
     */
    public GridTradeRule setTriggerPriceType(TriggerPriceType triggerPriceType) {
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
     * Sets the time in force.
     *
     * @param timeInForce time in force
     * @return this instance for chaining
     */
    public GridTradeRule setTimeInForce(GridTimeInForce timeInForce) {
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
     * Sets the action when the upper bound is reached.
     *
     * @param upperLimitEvent action
     * @return this instance for chaining
     */
    public GridTradeRule setUpperLimitEvent(GridLimitEvent upperLimitEvent) {
        this.upperLimitEvent = upperLimitEvent;
        return this;
    }

    /**
     * Sets the action when the lower bound is reached.
     *
     * @param lowerLimitEvent action
     * @return this instance for chaining
     */
    public GridTradeRule setLowerLimitEvent(GridLimitEvent lowerLimitEvent) {
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
