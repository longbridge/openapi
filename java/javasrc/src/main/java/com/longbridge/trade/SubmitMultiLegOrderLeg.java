package com.longbridge.trade;

import java.math.BigDecimal;

/**
 * A leg of a multi-leg combination order to submit
 */
@SuppressWarnings("unused")
public class SubmitMultiLegOrderLeg {
    private String symbol;
    private BigDecimal ratioQuantity;

    /**
     * Constructs a leg of a multi-leg combination order.
     *
     * @param symbol        option symbol, in `ticker.region` format (e.g.
     *                      {@code QQQ260731C764000.US})
     * @param ratioQuantity leg ratio quantity; must be a positive number. The
     *                      direction of each leg is implied by the strategy
     *                      together with the order side, not by the sign of
     *                      this value; a negative or zero ratio is rejected by
     *                      the server with {@code 602001}
     */
    public SubmitMultiLegOrderLeg(String symbol, BigDecimal ratioQuantity) {
        this.symbol = symbol;
        this.ratioQuantity = ratioQuantity;
    }

    /**
     * Returns the option symbol.
     *
     * @return option symbol
     */
    public String getSymbol() {
        return symbol;
    }

    /**
     * Returns the leg ratio quantity.
     *
     * @return leg ratio quantity
     */
    public BigDecimal getRatioQuantity() {
        return ratioQuantity;
    }
}
