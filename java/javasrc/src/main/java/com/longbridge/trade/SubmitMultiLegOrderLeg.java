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
     * @param ratioQuantity leg ratio quantity
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
