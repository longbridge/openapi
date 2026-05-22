package com.longbridge.market;

/** Response for {@link MarketContext#getRankList}. */
public class RankListResponse {
    /** Whether the response is delayed */
    public boolean bmp;
    /** Ranked securities list */
    public RankListItem[] lists;
}
