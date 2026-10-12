package com.longbridge.trade;

/**
 * A leg of a multi-leg combination for the pre-trade estimate.
 */
@SuppressWarnings("unused")
public class EstimateMultiLegOrderLeg {
    private String symbol;
    private OrderSide side;

    /**
     * Constructs a leg of the combination.
     *
     * @param symbol option or underlying-stock symbol, in `ticker.region`
     *               format (e.g. {@code QQQ260731C764000.US})
     */
    public EstimateMultiLegOrderLeg(String symbol) {
        this.symbol = symbol;
    }

    /**
     * Sets the leg order side (reserved; not in use for now).
     *
     * @param side leg order side
     * @return this instance for chaining
     */
    public EstimateMultiLegOrderLeg setSide(OrderSide side) {
        this.side = side;
        return this;
    }

    /**
     * Returns the leg symbol.
     *
     * @return leg symbol
     */
    public String getSymbol() {
        return symbol;
    }

    /**
     * Returns the leg order side (reserved).
     *
     * @return leg order side
     */
    public OrderSide getSide() {
        return side;
    }
}
