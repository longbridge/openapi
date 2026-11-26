package com.longbridge.asset;

import java.time.OffsetDateTime;

/**
 * Options for the stock transfer-out list
 */
@SuppressWarnings("unused")
public class GetStockTransfersOutOptions {
    private String market;
    private StockTransferOutState state;
    private OffsetDateTime start;
    private OffsetDateTime end;
    private Integer page;
    private Integer size;

    /**
     * Filters by market ({@code HK}, {@code US}, {@code CN}, {@code SG}).
     *
     * @param market market
     * @return this instance for chaining
     */
    public GetStockTransfersOutOptions setMarket(String market) {
        this.market = market;
        return this;
    }

    /**
     * Filters by state. The state is sent to the server as its server code
     * ({@link StockTransferOutState#code()}); {@link StockTransferOutState#Unknown} clears the filter.
     *
     * @param state state
     * @return this instance for chaining
     */
    public GetStockTransfersOutOptions setState(StockTransferOutState state) {
        this.state = state;
        return this;
    }

    /**
     * Only applications created at or after this time.
     *
     * @param start start time
     * @return this instance for chaining
     */
    public GetStockTransfersOutOptions setStart(OffsetDateTime start) {
        this.start = start;
        return this;
    }

    /**
     * Only applications created at or before this time.
     *
     * @param end end time
     * @return this instance for chaining
     */
    public GetStockTransfersOutOptions setEnd(OffsetDateTime end) {
        this.end = end;
        return this;
    }

    /**
     * Sets the page number (default 1).
     *
     * @param page page number
     * @return this instance for chaining
     */
    public GetStockTransfersOutOptions setPage(int page) {
        this.page = page;
        return this;
    }

    /**
     * Sets the page size (default 20, max 50).
     *
     * @param size page size
     * @return this instance for chaining
     */
    public GetStockTransfersOutOptions setSize(int size) {
        this.size = size;
        return this;
    }
}
