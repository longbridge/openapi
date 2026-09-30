package com.longbridge.fund;

/**
 * Options for FundPageOptions.
 */
@SuppressWarnings("unused")
public class FundPageOptions {
    private Integer page;
    private Integer size;

    /**
     * Sets page.
     *
     * @param page page
     * @return this instance for chaining
     */
    public FundPageOptions setPage(Integer page) {
        this.page = page;
        return this;
    }

    /**
     * Sets size.
     *
     * @param size size
     * @return this instance for chaining
     */
    public FundPageOptions setSize(Integer size) {
        this.size = size;
        return this;
    }

}
