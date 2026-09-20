package com.longbridge.grid;

/**
 * Options for querying grid trading order detail (and paged history)
 */
@SuppressWarnings("unused")
public class GetGridOrderDetailOptions {
    private String orderId;
    private String historyId;
    private Integer limit;

    /**
     * Constructs options for querying grid order detail.
     *
     * @param orderId grid master order ID
     */
    public GetGridOrderDetailOptions(String orderId) {
        this.orderId = orderId;
    }

    /**
     * Sets the history cursor for paging through the trigger history.
     *
     * @param historyId history cursor
     * @return this instance for chaining
     */
    public GetGridOrderDetailOptions setHistoryId(String historyId) {
        this.historyId = historyId;
        return this;
    }

    /**
     * Sets the page size.
     *
     * @param limit page size
     * @return this instance for chaining
     */
    public GetGridOrderDetailOptions setLimit(Integer limit) {
        this.limit = limit;
        return this;
    }
}
