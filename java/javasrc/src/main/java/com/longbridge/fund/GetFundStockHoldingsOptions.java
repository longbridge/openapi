package com.longbridge.fund;

/**
 * Options for GetFundStockHoldingsOptions.
 */
@SuppressWarnings("unused")
public class GetFundStockHoldingsOptions {
    private Integer limit;

    /**
     * Sets limit.
     *
     * @param limit limit
     * @return this instance for chaining
     */
    public GetFundStockHoldingsOptions setLimit(Integer limit) {
        this.limit = limit;
        return this;
    }

}
