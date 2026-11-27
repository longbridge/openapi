package com.longbridge.asset;

import java.time.OffsetDateTime;

/**
 * Options for the withdrawal list
 */
@SuppressWarnings("unused")
public class GetWithdrawalsOptions {
    private Integer page;
    private Integer size;
    private OffsetDateTime start;
    private OffsetDateTime end;

    /**
     * Sets the page number (default 1).
     *
     * @param page page number
     * @return this instance for chaining
     */
    public GetWithdrawalsOptions setPage(int page) {
        this.page = page;
        return this;
    }

    /**
     * Sets the page size (default 20, max 50; larger values are sent as 50,
     * and 0 or a negative value as the default 20).
     *
     * @param size page size
     * @return this instance for chaining
     */
    public GetWithdrawalsOptions setSize(int size) {
        this.size = size;
        return this;
    }

    /**
     * Only withdrawals applied at or after this time.
     *
     * @param start start time
     * @return this instance for chaining
     */
    public GetWithdrawalsOptions setStart(OffsetDateTime start) {
        this.start = start;
        return this;
    }

    /**
     * Only withdrawals applied at or before this time.
     *
     * @param end end time
     * @return this instance for chaining
     */
    public GetWithdrawalsOptions setEnd(OffsetDateTime end) {
        this.end = end;
        return this;
    }
}
