package com.longbridge.asset;

import java.time.OffsetDateTime;

/**
 * Options for the deposit list
 */
@SuppressWarnings("unused")
public class GetDepositsOptions {
    private Integer page;
    private Integer size;
    private DepositState[] states;
    private String[] currencies;
    private OffsetDateTime start;
    private OffsetDateTime end;

    /**
     * Sets the page number (default 1).
     *
     * @param page page number
     * @return this instance for chaining
     */
    public GetDepositsOptions setPage(int page) {
        this.page = page;
        return this;
    }

    /**
     * Sets the page size (max 20; larger values are sent as 20, because the
     * deposit list returns a short 10-record page for them).
     *
     * @param size page size
     * @return this instance for chaining
     */
    public GetDepositsOptions setSize(int size) {
        this.size = size;
        return this;
    }

    /**
     * Filters by states ({@link DepositState#Unknown} is ignored).
     *
     * @param states deposit states
     * @return this instance for chaining
     */
    public GetDepositsOptions setStates(DepositState[] states) {
        this.states = states;
        return this;
    }

    /**
     * Filters by currencies (e.g. {@code HKD}, {@code USD}).
     *
     * @param currencies currencies
     * @return this instance for chaining
     */
    public GetDepositsOptions setCurrencies(String[] currencies) {
        this.currencies = currencies;
        return this;
    }

    /**
     * Only deposits applied at or after this time.
     *
     * @param start start time
     * @return this instance for chaining
     */
    public GetDepositsOptions setStart(OffsetDateTime start) {
        this.start = start;
        return this;
    }

    /**
     * Only deposits applied at or before this time.
     *
     * @param end end time
     * @return this instance for chaining
     */
    public GetDepositsOptions setEnd(OffsetDateTime end) {
        this.end = end;
        return this;
    }
}
