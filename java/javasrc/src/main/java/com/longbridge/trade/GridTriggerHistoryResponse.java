package com.longbridge.trade;

import java.util.Arrays;

/**
 * Response containing a page of grid trigger history entries
 */
public class GridTriggerHistoryResponse {
    private TriggerOrder[] triggerOrders;
    private boolean hasMore;

    /**
     * Returns triggerOrders.
     *
     * @return triggerOrders
     */
    public TriggerOrder[] getTriggerOrders() {
        return triggerOrders;
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
        return "GridTriggerHistoryResponse [triggerOrders=" + Arrays.toString(triggerOrders) +
                ", hasMore=" + hasMore + "]";
    }
}
