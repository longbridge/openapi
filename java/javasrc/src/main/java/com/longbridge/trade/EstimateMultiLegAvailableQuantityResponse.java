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
     * Returns the maximum number of combinations the account can open. The
     * server returns {@code Long.MAX_VALUE} (9223372036854775807) when the legs
     * do not form the given strategy; treat that as no usable estimate.
     *
     * @return maximum open quantity
     */
    public BigDecimal getMaxOpenQty() {
        return maxOpenQty;
    }

    /**
     * Returns the funds required per combination unit: the net debit at current
     * market prices for debit (premium-paying) combinations, or the margin
     * requirement for credit / short combinations.
     *
     * @return unit margin
     */
    public BigDecimal getUnitMargin() {
        return unitMargin;
    }

    /**
     * Returns the change of the initial margin after placing
     * {@code submittedQuantity} combinations (after minus before); {@code 0} for debit
     * combinations, which consume cash instead.
     *
     * @return initial margin change
     */
    public BigDecimal getInitialMarginChange() {
        return initialMarginChange;
    }

    /**
     * Returns the change of the maintenance margin after placing
     * {@code submittedQuantity} combinations (after minus before); {@code 0} for debit
     * combinations, which consume cash instead.
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
