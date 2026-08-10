package com.longbridge.trade;

/**
 * Options for querying grid trading trigger history
 */
@SuppressWarnings("unused")
public class GetGridTriggerHistoryOptions {
    private String gridOrderId;
    private Integer page;
    private Integer limit;

    /**
     * Constructs options for querying grid trigger history.
     *
     * @param gridOrderId grid master order ID
     */
    public GetGridTriggerHistoryOptions(String gridOrderId) {
        this.gridOrderId = gridOrderId;
    }

    /**
     * Sets the page number.
     *
     * @param page page number
     * @return this instance for chaining
     */
    public GetGridTriggerHistoryOptions setPage(Integer page) {
        this.page = page;
        return this;
    }

    /**
     * Sets the page size.
     *
     * @param limit page size
     * @return this instance for chaining
     */
    public GetGridTriggerHistoryOptions setLimit(Integer limit) {
        this.limit = limit;
        return this;
    }
}
