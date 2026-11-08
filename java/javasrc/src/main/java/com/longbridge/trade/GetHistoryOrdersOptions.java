package com.longbridge.trade;

import java.time.OffsetDateTime;

import com.longbridge.Market;

/**
 * Options for querying history orders
 */
@SuppressWarnings("unused")
public class GetHistoryOrdersOptions {
    private String symbol;
    private OrderStatus[] status;
    private OrderSide side;
    private Market market;
    private OffsetDateTime startAt;
    private OffsetDateTime endAt;

    /**
     * Filters by security symbol.
     *
     * @param symbol security symbol
     * @return this instance for chaining
     */
    public GetHistoryOrdersOptions setSymbol(String symbol) {
        this.symbol = symbol;
        return this;
    }

    /**
     * Filters by order status.
     *
     * @param status order statuses
     * @return this instance for chaining
     */
    public GetHistoryOrdersOptions setStatus(OrderStatus[] status) {
        this.status = status;
        return this;
    }

    /**
     * Filters by order side.
     *
     * @param side order side
     * @return this instance for chaining
     */
    public GetHistoryOrdersOptions setSide(OrderSide side) {
        this.side = side;
        return this;
    }

    /**
     * Filters by market.
     *
     * @param market market
     * @return this instance for chaining
     */
    public GetHistoryOrdersOptions setMarket(Market market) {
        this.market = market;
        return this;
    }

    /**
     * Sets the start of the query time range.
     *
     * @param startAt start time
     * @return this instance for chaining
     */
    public GetHistoryOrdersOptions setStartAt(OffsetDateTime startAt) {
        this.startAt = startAt;
        return this;
    }

    /**
     * Sets the end of the query time range.
     *
     * @param endAt end time
     * @return this instance for chaining
     */
    public GetHistoryOrdersOptions setEndAt(OffsetDateTime endAt) {
        this.endAt = endAt;
        return this;
    }

}
