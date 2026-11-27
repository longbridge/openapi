package com.longbridge.asset;

import java.time.OffsetDateTime;

/**
 * Options for the stock transfer-in list
 */
@SuppressWarnings("unused")
public class GetStockTransfersInOptions {
    private String market;
    private StockTransferInState state;
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
    public GetStockTransfersInOptions setMarket(String market) {
        this.market = market;
        return this;
    }

    /**
     * Filters by state. The state is sent to the server as its server code
     * ({@link StockTransferInState#code()}); {@link StockTransferInState#Unknown} clears the filter.
     *
     * @param state state
     * @return this instance for chaining
     */
    public GetStockTransfersInOptions setState(StockTransferInState state) {
        this.state = state;
        return this;
    }

    /**
     * Only applications created at or after this time.
     *
     * @param start start time
     * @return this instance for chaining
     */
    public GetStockTransfersInOptions setStart(OffsetDateTime start) {
        this.start = start;
        return this;
    }

    /**
     * Only applications created at or before this time.
     *
     * @param end end time
     * @return this instance for chaining
     */
    public GetStockTransfersInOptions setEnd(OffsetDateTime end) {
        this.end = end;
        return this;
    }

    /**
     * Sets the page number (default 1).
     *
     * @param page page number
     * @return this instance for chaining
     */
    public GetStockTransfersInOptions setPage(int page) {
        this.page = page;
        return this;
    }

    /**
     * Sets the page size (default 20, max 50).
     *
     * @param size page size
     * @return this instance for chaining
     */
    public GetStockTransfersInOptions setSize(int size) {
        this.size = size;
        return this;
    }
}
