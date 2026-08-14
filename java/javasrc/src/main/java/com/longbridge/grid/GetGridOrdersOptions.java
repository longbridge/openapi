package com.longbridge.grid;

import com.longbridge.Market;

/**
 * Options for querying grid trading orders (paged list)
 */
@SuppressWarnings("unused")
public class GetGridOrdersOptions {
    private Integer page;
    private Integer limit;
    private Market market;
    private String status;
    private String symbol;
    private String sortBy;
    private String sortOrder;

    /**
     * Sets the page number.
     *
     * @param page page number
     * @return this instance for chaining
     */
    public GetGridOrdersOptions setPage(Integer page) {
        this.page = page;
        return this;
    }

    /**
     * Sets the page size.
     *
     * @param limit page size
     * @return this instance for chaining
     */
    public GetGridOrdersOptions setLimit(Integer limit) {
        this.limit = limit;
        return this;
    }

    /**
     * Sets the market filter.
     *
     * @param market market
     * @return this instance for chaining
     */
    public GetGridOrdersOptions setMarket(Market market) {
        this.market = market;
        return this;
    }

    /**
     * Sets the status filter (comma-joined, e.g. {@code Performing,Suspended}).
     *
     * @param status status filter
     * @return this instance for chaining
     */
    public GetGridOrdersOptions setStatus(String status) {
        this.status = status;
        return this;
    }

    /**
     * Sets the security symbol filter (e.g. 700.HK).
     *
     * @param symbol security symbol
     * @return this instance for chaining
     */
    public GetGridOrdersOptions setSymbol(String symbol) {
        this.symbol = symbol;
        return this;
    }

    /**
     * Sets the sort field.
     *
     * @param sortBy sort field
     * @return this instance for chaining
     */
    public GetGridOrdersOptions setSortBy(String sortBy) {
        this.sortBy = sortBy;
        return this;
    }

    /**
     * Sets the sort order.
     *
     * @param sortOrder sort order
     * @return this instance for chaining
     */
    public GetGridOrdersOptions setSortOrder(String sortOrder) {
        this.sortOrder = sortOrder;
        return this;
    }
}
