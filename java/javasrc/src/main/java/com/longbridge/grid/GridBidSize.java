package com.longbridge.grid;

/**
 * A price-step (bid-size) rule entry from the order-info response
 */
public class GridBidSize {
    private String strProceed;
    private String endProceed;
    private String bidSize;

    /**
     * Returns strProceed.
     *
     * @return strProceed
     */
    public String getStrProceed() {
        return strProceed;
    }

    /**
     * Returns endProceed.
     *
     * @return endProceed
     */
    public String getEndProceed() {
        return endProceed;
    }

    /**
     * Returns bidSize.
     *
     * @return bidSize
     */
    public String getBidSize() {
        return bidSize;
    }

    @Override
    public String toString() {
        return "GridBidSize [strProceed=" + strProceed +
                ", endProceed=" + endProceed +
                ", bidSize=" + bidSize + "]";
    }
}
