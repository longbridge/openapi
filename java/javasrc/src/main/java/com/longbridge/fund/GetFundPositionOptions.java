package com.longbridge.fund;

/**
 * Options for GetFundPositionOptions.
 */
@SuppressWarnings("unused")
public class GetFundPositionOptions {
    private String accountChannel;
    private Long aaid;
    private String start;
    private String end;

    /**
     * Sets accountChannel.
     *
     * @param accountChannel accountChannel
     * @return this instance for chaining
     */
    public GetFundPositionOptions setAccountChannel(String accountChannel) {
        this.accountChannel = accountChannel;
        return this;
    }

    /**
     * Sets aaid.
     *
     * @param aaid aaid
     * @return this instance for chaining
     */
    public GetFundPositionOptions setAaid(Long aaid) {
        this.aaid = aaid;
        return this;
    }

    /**
     * Sets start.
     *
     * @param start start
     * @return this instance for chaining
     */
    public GetFundPositionOptions setStart(String start) {
        this.start = start;
        return this;
    }

    /**
     * Sets end.
     *
     * @param end end
     * @return this instance for chaining
     */
    public GetFundPositionOptions setEnd(String end) {
        this.end = end;
        return this;
    }

}
