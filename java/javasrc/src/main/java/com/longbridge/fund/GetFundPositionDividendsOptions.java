package com.longbridge.fund;

/**
 * Options for GetFundPositionDividendsOptions.
 */
@SuppressWarnings("unused")
public class GetFundPositionDividendsOptions {
    private String accountChannel;
    private Long aaid;
    private String currency;
    private Long start;
    private Long end;
    private Integer page;
    private Integer size;

    /**
     * Sets accountChannel.
     *
     * @param accountChannel accountChannel
     * @return this instance for chaining
     */
    public GetFundPositionDividendsOptions setAccountChannel(String accountChannel) {
        this.accountChannel = accountChannel;
        return this;
    }

    /**
     * Sets aaid.
     *
     * @param aaid aaid
     * @return this instance for chaining
     */
    public GetFundPositionDividendsOptions setAaid(Long aaid) {
        this.aaid = aaid;
        return this;
    }

    /**
     * Sets currency.
     *
     * @param currency currency
     * @return this instance for chaining
     */
    public GetFundPositionDividendsOptions setCurrency(String currency) {
        this.currency = currency;
        return this;
    }

    /**
     * Sets start.
     *
     * @param start start
     * @return this instance for chaining
     */
    public GetFundPositionDividendsOptions setStart(Long start) {
        this.start = start;
        return this;
    }

    /**
     * Sets end.
     *
     * @param end end
     * @return this instance for chaining
     */
    public GetFundPositionDividendsOptions setEnd(Long end) {
        this.end = end;
        return this;
    }

    /**
     * Sets page.
     *
     * @param page page
     * @return this instance for chaining
     */
    public GetFundPositionDividendsOptions setPage(Integer page) {
        this.page = page;
        return this;
    }

    /**
     * Sets size.
     *
     * @param size size
     * @return this instance for chaining
     */
    public GetFundPositionDividendsOptions setSize(Integer size) {
        this.size = size;
        return this;
    }

}
