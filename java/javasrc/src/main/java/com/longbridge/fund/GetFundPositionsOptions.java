package com.longbridge.fund;

/**
 * Options for GetFundPositionsOptions.
 */
@SuppressWarnings("unused")
public class GetFundPositionsOptions {
    private String accountChannel;
    private Long aaid;

    /**
     * Sets accountChannel.
     *
     * @param accountChannel accountChannel
     * @return this instance for chaining
     */
    public GetFundPositionsOptions setAccountChannel(String accountChannel) {
        this.accountChannel = accountChannel;
        return this;
    }

    /**
     * Sets aaid.
     *
     * @param aaid aaid
     * @return this instance for chaining
     */
    public GetFundPositionsOptions setAaid(Long aaid) {
        this.aaid = aaid;
        return this;
    }

}
