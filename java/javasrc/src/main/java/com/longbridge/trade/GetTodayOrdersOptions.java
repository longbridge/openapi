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
     * Indicate that the provided order ID is an attached order ID.
     * When set, the server looks up the order whose attached sub-order matches
     * the given order ID, rather than treating it as a regular order ID.
     *
     * @return this instance for chaining
     */
    public GetTodayOrdersOptions setIsAttached() {
        this.isAttached = true;
        return this;
    }

    /**
     * Returns whether the order ID is treated as an attached order ID.
     *
     * @return is-attached flag
     */
    public Boolean getIsAttached() {
        return isAttached;
    }

}
