package com.longbridge.asset;

import java.util.Arrays;

/**
 * One page of deposit records
 * <p>
 * The server also returns a total, but it follows the page size rather than
 * the record count, so it is not modelled; see {@link #getHasMore()}.
 */
public class DepositsResponse {
    private Deposit[] items;
    private boolean hasMore;

    /**
     * Returns the deposit records, newest first.
     *
     * @return the deposit records
     */
    public Deposit[] getItems() {
        return items;
    }

    /**
     * Returns whether a full page came back (as many records as the page size
     * sent, at most 20), so the next page may hold more.
     *
     * @return whether the next page may hold more
     */
    public boolean getHasMore() {
        return hasMore;
    }

    @Override
    public String toString() {
        return "DepositsResponse [" + "items=" + Arrays.toString(items) + ", " + "hasMore=" + hasMore + "]";
    }
}
