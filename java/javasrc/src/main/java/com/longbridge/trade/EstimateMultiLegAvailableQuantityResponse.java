package com.longbridge.trade;

import java.math.BigDecimal;

/**
 * Estimated tradable quantity and margin impact of a multi-leg option
 * combination.
 */
public class EstimateMultiLegAvailableQuantityResponse {
    private BigDecimal maxOpenQty;
    private BigDecimal unitMargin;
    private BigDecimal initialMarginChange;
    private BigDecimal maintenanceMarginChange;

    /**
     * Returns the maximum open quantity of the combination.
     *
     * @return maximum open quantity
     */
    public BigDecimal getMaxOpenQty() {
        return maxOpenQty;
    }

    /**
     * Returns the margin required per combination unit.
     *
     * @return unit margin
     */
    public BigDecimal getUnitMargin() {
        return unitMargin;
    }

    /**
     * Returns the change of the initial margin after placing the order.
     *
     * @return initial margin change
     */
    public BigDecimal getInitialMarginChange() {
        return initialMarginChange;
    }

    /**
     * Returns the change of the maintenance margin after placing the order.
     *
     * @return maintenance margin change
     */
    public BigDecimal getMaintenanceMarginChange() {
        return maintenanceMarginChange;
    }

    @Override
    public String toString() {
        return "EstimateMultiLegAvailableQuantityResponse [maxOpenQty=" + maxOpenQty + ", unitMargin=" + unitMargin
                + ", initialMarginChange=" + initialMarginChange + ", maintenanceMarginChange="
                + maintenanceMarginChange + "]";
    }
}
