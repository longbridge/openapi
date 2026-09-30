package com.longbridge.fund;

/**
 * Options for GetFundTransactionsOptions.
 */
@SuppressWarnings("unused")
public class GetFundTransactionsOptions {
    private String accountChannel;
    private String businessType;
    private String category;
    private String currencies;
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
    public GetFundTransactionsOptions setAccountChannel(String accountChannel) {
        this.accountChannel = accountChannel;
        return this;
    }

    /**
     * Sets businessType.
     *
     * @param businessType businessType
     * @return this instance for chaining
     */
    public GetFundTransactionsOptions setBusinessType(String businessType) {
        this.businessType = businessType;
        return this;
    }

    /**
     * Sets category.
     *
     * @param category category
     * @return this instance for chaining
     */
    public GetFundTransactionsOptions setCategory(String category) {
        this.category = category;
        return this;
    }

    /**
     * Sets currencies.
     *
     * @param currencies currencies
     * @return this instance for chaining
     */
    public GetFundTransactionsOptions setCurrencies(String currencies) {
        this.currencies = currencies;
        return this;
    }

    /**
     * Sets start.
     *
     * @param start start
     * @return this instance for chaining
     */
    public GetFundTransactionsOptions setStart(Long start) {
        this.start = start;
        return this;
    }

    /**
     * Sets end.
     *
     * @param end end
     * @return this instance for chaining
     */
    public GetFundTransactionsOptions setEnd(Long end) {
        this.end = end;
        return this;
    }

    /**
     * Sets page.
     *
     * @param page page
     * @return this instance for chaining
     */
    public GetFundTransactionsOptions setPage(Integer page) {
        this.page = page;
        return this;
    }

    /**
     * Sets size.
     *
     * @param size size
     * @return this instance for chaining
     */
    public GetFundTransactionsOptions setSize(Integer size) {
        this.size = size;
        return this;
    }

}
