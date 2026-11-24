package com.longbridge.fund;

/**
 * Options for GetFundsOptions.
 */
@SuppressWarnings("unused")
public class GetFundsOptions {
    private String filter;
    private long[] quickIds;
    private String[] timeInterval;

    /**
     * Sets filter.
     *
     * @param filter server-defined filter object as a JSON string; invalid
     *               JSON makes {@code FundContext.funds} throw a
     *               {@code RuntimeException} ({@code invalid filter JSON: ...})
     *               synchronously from the call itself, not through the
     *               returned future, instead of being ignored
     * @return this instance for chaining
     */
    public GetFundsOptions setFilter(String filter) {
        this.filter = filter;
        return this;
    }

    /**
     * Sets quickIds.
     *
     * @param quickIds quickIds
     * @return this instance for chaining
     */
    public GetFundsOptions setQuickIds(long[] quickIds) {
        this.quickIds = quickIds;
        return this;
    }

    /**
     * Sets timeInterval.
     *
     * @param timeInterval timeInterval
     * @return this instance for chaining
     */
    public GetFundsOptions setTimeInterval(String[] timeInterval) {
        this.timeInterval = timeInterval;
        return this;
    }

}
