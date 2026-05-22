package com.longbridge.market;

import com.longbridge.market.TopMoversEvent;

/** Response for {@link MarketContext#getTopMovers}. */
public class TopMoversResponse {
    /** Top mover events */
    public TopMoversEvent[] events;
    /** Pagination cursor (raw JSON); pass to next call for next page */
    public String nextParams;
}
