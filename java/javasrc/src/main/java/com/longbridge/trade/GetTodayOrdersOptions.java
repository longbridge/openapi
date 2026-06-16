package com.longbridge.trade;

import com.longbridge.Market;

/**
 * Options for querying today's orders
 */
@SuppressWarnings("unused")
public class GetTodayOrdersOptions {
    private String symbol;
    private OrderStatus[] status;
    private OrderSide side;
    private Market market;
    private String orderId;
    private Boolean isAttached;

    /**
     * Filters by security symbol.
     *
     * @param symbol security symbol
     * @return this instance for chaining
     */
    public GetTodayOrdersOptions setSymbol(String symbol) {
        this.symbol = symbol;
        return this;
    }

    /**
     * Filters by order status.
     *
     * @param status order statuses
     * @return this instance for chaining
     */
    public GetTodayOrdersOptions setStatus(OrderStatus[] status) {
        this.status = status;
        return this;
    }

    /**
     * Filters by order side.
     *
     * @param side order side
     * @return this instance for chaining
     */
    public GetTodayOrdersOptions setSide(OrderSide side) {
        this.side = side;
        return this;
    }

    /**
     * Filters by market.
     *
     * @param market market
     * @return this instance for chaining
     */
    public GetTodayOrdersOptions setMarket(Market market) {
        this.market = market;
        return this;
    }

    /**
     * Filters by order ID.
     *
     * @param orderId order ID
     * @return this instance for chaining
     */
    public GetTodayOrdersOptions setOrderId(String orderId) {
        this.orderId = orderId;
        return this;
    }

    /**
     * Filters to include only attached orders.
     *
     * @return this instance for chaining
     */
    public GetTodayOrdersOptions setIsAttached() {
        this.isAttached = true;
        return this;
    }

    /**
     * Returns the is-attached filter.
     *
     * @return is-attached filter
     */
    public Boolean getIsAttached() {
        return isAttached;
    }

}
