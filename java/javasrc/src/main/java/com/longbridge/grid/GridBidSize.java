package com.longbridge.grid;

import java.math.BigDecimal;

/**
 * A price-step (bid-size) rule entry from the order-info response
 */
public class GridBidSize {
    private BigDecimal strProceed;
    private BigDecimal endProceed;
    private BigDecimal bidSize;

    /**
     * Returns strProceed.
     *
     * @return strProceed
     */
    public BigDecimal getStrProceed() {
        return strProceed;
    }

    /**
     * Returns endProceed.
     *
     * @return endProceed
     */
    public BigDecimal getEndProceed() {
        return endProceed;
    }

    /**
     * Returns bidSize.
     *
     * @return bidSize
     */
    public BigDecimal getBidSize() {
        return bidSize;
    }

    @Override
    public String toString() {
        return "GridBidSize [strProceed=" + strProceed +
                ", endProceed=" + endProceed +
                ", bidSize=" + bidSize + "]";
    }
}
