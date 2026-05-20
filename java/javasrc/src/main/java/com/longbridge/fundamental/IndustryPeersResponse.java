package com.longbridge.fundamental;

/** Response for {@link FundamentalContext#getIndustryPeers}. */
public class IndustryPeersResponse {
    /** Top-level industry node info */
    public IndustryPeersTop top;
    /** Root peer chain node; may be null */
    public IndustryPeerNode chain;
}
