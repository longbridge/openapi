package com.longbridge.asset;

import java.time.OffsetDateTime;

/**
 * Options for the stock transfer-in and transfer-out lists
 */
@SuppressWarnings("unused")
public class GetStockTransfersOptions {
    private String market;
    private Integer state;
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
    public GetStockTransfersOptions setMarket(String market) {
        this.market = market;
        return this;
    }

    /**
     * Filters by state code: a {@link StockTransferInState} code for the
     * transfer-in list (0 submitted, 1 cancelled, 3 completed, 5 processing, 7
     * partially completed, 8 failed), a {@link StockTransferOutState} code for
     * the transfer-out list (0 submitted, 1 processing, 2 executing, 3
     * completed, 4 cancelled, 5 partially completed, 6 failed, 7 rejected).
     *
     * @param state state code
     * @return this instance for chaining
     */
    public GetStockTransfersOptions setState(int state) {
        this.state = state;
        return this;
    }

    /**
     * Only applications created at or after this time.
     *
     * @param start start time
     * @return this instance for chaining
     */
    public GetStockTransfersOptions setStart(OffsetDateTime start) {
        this.start = start;
        return this;
    }

    /**
     * Only applications created at or before this time.
     *
     * @param end end time
     * @return this instance for chaining
     */
    public GetStockTransfersOptions setEnd(OffsetDateTime end) {
        this.end = end;
        return this;
    }

    /**
     * Sets the page number (default 1).
     *
     * @param page page number
     * @return this instance for chaining
     */
    public GetStockTransfersOptions setPage(int page) {
        this.page = page;
        return this;
    }

    /**
     * Sets the page size (default 20, max 50).
     *
     * @param size page size
     * @return this instance for chaining
     */
    public GetStockTransfersOptions setSize(int size) {
        this.size = size;
        return this;
    }
}
