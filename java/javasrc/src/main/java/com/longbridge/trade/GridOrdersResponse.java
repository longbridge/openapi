package com.longbridge.trade;

import java.util.Arrays;

/**
 * Response containing a page of grid trading orders
 */
public class GridOrdersResponse {
    private GridOrder[] gridOrder;
    private boolean hasMore;

    /**
     * Returns gridOrder.
     *
     * @return gridOrder
     */
    public GridOrder[] getGridOrder() {
        return gridOrder;
    }

    /**
     * Returns hasMore.
     *
     * @return hasMore
     */
    public boolean getHasMore() {
        return hasMore;
    }

    @Override
    public String toString() {
        return "GridOrdersResponse [gridOrder=" + Arrays.toString(gridOrder) +
                ", hasMore=" + hasMore + "]";
    }
}
