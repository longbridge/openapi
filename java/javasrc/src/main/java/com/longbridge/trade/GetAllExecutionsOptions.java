package com.longbridge.trade;

import java.time.OffsetDateTime;

/**
 * Options for querying all executions
 */
@SuppressWarnings("unused")
public class GetAllExecutionsOptions {
    private String symbol;
    private String orderId;
    private OffsetDateTime startAt;
    private OffsetDateTime endAt;
    private Long page;

    /**
     * Filters by security symbol.
     *
     * @param symbol security symbol
     * @return this instance for chaining
     */
    public GetAllExecutionsOptions setSymbol(String symbol) {
        this.symbol = symbol;
        return this;
    }

    /**
     * Filters by order ID.
     *
     * @param orderId order ID
     * @return this instance for chaining
     */
    public GetAllExecutionsOptions setOrderId(String orderId) {
        this.orderId = orderId;
        return this;
    }

    /**
     * Sets the start of the query time range.
     *
     * @param startAt start time
     * @return this instance for chaining
     */
    public GetAllExecutionsOptions setStartAt(OffsetDateTime startAt) {
        this.startAt = startAt;
        return this;
    }

    /**
     * Sets the end of the query time range.
     *
     * @param endAt end time
     * @return this instance for chaining
     */
    public GetAllExecutionsOptions setEndAt(OffsetDateTime endAt) {
        this.endAt = endAt;
        return this;
    }

    /**
     * Sets the page number (starting from 1).
     *
     * @param page page number
     * @return this instance for chaining
     */
    public GetAllExecutionsOptions setPage(long page) {
        this.page = page;
        return this;
    }
}
