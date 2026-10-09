package com.longbridge.trade;

import java.math.BigDecimal;

/**
 * Options for estimating a multi-leg option combination's maximum tradable
 * quantity and margin impact.
 */
@SuppressWarnings("unused")
public class EstimateMultiLegAvailableQuantityOptions {
    private OrderSide side;
    private OrderType orderType;
    private BigDecimal submittedQuantity;
    private MultiLegStrategy strategy;
    private EstimateMultiLegOrderLeg[] legs;
    private BigDecimal submittedPrice;

    /**
     * Constructs options for a multi-leg estimate.
     *
     * @param side              order side of the combination
     * @param orderType         order type
     * @param submittedQuantity submitted quantity (number of combinations)
     * @param strategy          multi-leg strategy
     * @param legs              legs of the combination
     */
    public EstimateMultiLegAvailableQuantityOptions(
            OrderSide side,
            OrderType orderType,
            BigDecimal submittedQuantity,
            MultiLegStrategy strategy,
            EstimateMultiLegOrderLeg[] legs) {
        this.side = side;
        this.orderType = orderType;
        this.submittedQuantity = submittedQuantity;
        this.strategy = strategy;
        this.legs = legs;
    }

    /**
     * Sets the submitted price (required for limit order types such as
     * {@code LO}).
     *
     * @param submittedPrice submitted price
     * @return this instance for chaining
     */
    public EstimateMultiLegAvailableQuantityOptions setSubmittedPrice(BigDecimal submittedPrice) {
        this.submittedPrice = submittedPrice;
        return this;
    }

    /**
     * Returns the order side.
     *
     * @return order side
     */
    public OrderSide getSide() {
        return side;
    }

    /**
     * Returns the order type.
     *
     * @return order type
     */
    public OrderType getOrderType() {
        return orderType;
    }

    /**
     * Returns the submitted quantity.
     *
     * @return submitted quantity
     */
    public BigDecimal getSubmittedQuantity() {
        return submittedQuantity;
    }

    /**
     * Returns the multi-leg strategy.
     *
     * @return multi-leg strategy
     */
    public MultiLegStrategy getStrategy() {
        return strategy;
    }

    /**
     * Returns the legs of the combination.
     *
     * @return legs
     */
    public EstimateMultiLegOrderLeg[] getLegs() {
        return legs;
    }

    /**
     * Returns the submitted price.
     *
     * @return submitted price
     */
    public BigDecimal getSubmittedPrice() {
        return submittedPrice;
    }
}
