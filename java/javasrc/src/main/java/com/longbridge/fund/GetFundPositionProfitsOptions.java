package com.longbridge.fund;

/**
 * Options for GetFundPositionProfitsOptions.
 */
@SuppressWarnings("unused")
public class GetFundPositionProfitsOptions {
    private String accountChannel;
    private Long aaid;
    private String start;
    private String end;
    private Integer page;
    private Integer size;

    /**
     * Sets accountChannel.
     *
     * @param accountChannel accountChannel
     * @return this instance for chaining
     */
    public GetFundPositionProfitsOptions setAccountChannel(String accountChannel) {
        this.accountChannel = accountChannel;
        return this;
    }

    /**
     * Sets aaid.
     *
     * @param aaid aaid
     * @return this instance for chaining
     */
    public GetFundPositionProfitsOptions setAaid(Long aaid) {
        this.aaid = aaid;
        return this;
    }

    /**
     * Sets start.
     *
     * @param start start
     * @return this instance for chaining
     */
    public GetFundPositionProfitsOptions setStart(String start) {
        this.start = start;
        return this;
    }

    /**
     * Sets end.
     *
     * @param end end
     * @return this instance for chaining
     */
    public GetFundPositionProfitsOptions setEnd(String end) {
        this.end = end;
        return this;
    }

    /**
     * Sets page.
     *
     * @param page page
     * @return this instance for chaining
     */
    public GetFundPositionProfitsOptions setPage(Integer page) {
        this.page = page;
        return this;
    }

    /**
     * Sets size.
     *
     * @param size size
     * @return this instance for chaining
     */
    public GetFundPositionProfitsOptions setSize(Integer size) {
        this.size = size;
        return this;
    }

}
