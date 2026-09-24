package com.longbridge.fund;

/**
 * Options for GetFundOrdersOptions.
 */
@SuppressWarnings("unused")
public class GetFundOrdersOptions {
    private String[] symbols;
    private String actions;
    private String states;
    private String currency;
    private Long start;
    private Long end;
    private Integer page;
    private Integer size;

    /**
     * Sets symbols.
     *
     * @param symbols symbols
     * @return this instance for chaining
     */
    public GetFundOrdersOptions setSymbols(String[] symbols) {
        this.symbols = symbols;
        return this;
    }

    /**
     * Sets actions.
     *
     * @param actions actions
     * @return this instance for chaining
     */
    public GetFundOrdersOptions setActions(String actions) {
        this.actions = actions;
        return this;
    }

    /**
     * Sets states.
     *
     * @param states states
     * @return this instance for chaining
     */
    public GetFundOrdersOptions setStates(String states) {
        this.states = states;
        return this;
    }

    /**
     * Sets currency.
     *
     * @param currency currency
     * @return this instance for chaining
     */
    public GetFundOrdersOptions setCurrency(String currency) {
        this.currency = currency;
        return this;
    }

    /**
     * Sets start.
     *
     * @param start start
     * @return this instance for chaining
     */
    public GetFundOrdersOptions setStart(Long start) {
        this.start = start;
        return this;
    }

    /**
     * Sets end.
     *
     * @param end end
     * @return this instance for chaining
     */
    public GetFundOrdersOptions setEnd(Long end) {
        this.end = end;
        return this;
    }

    /**
     * Sets page.
     *
     * @param page page
     * @return this instance for chaining
     */
    public GetFundOrdersOptions setPage(Integer page) {
        this.page = page;
        return this;
    }

    /**
     * Sets size.
     *
     * @param size size
     * @return this instance for chaining
     */
    public GetFundOrdersOptions setSize(Integer size) {
        this.size = size;
        return this;
    }

}
