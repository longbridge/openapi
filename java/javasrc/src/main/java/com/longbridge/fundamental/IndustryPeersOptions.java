package com.longbridge.fundamental;

/** Options for {@link FundamentalContext#getIndustryPeers}. */
public class IndustryPeersOptions {
    /** Symbol (e.g. "AAPL.US") or industry counter ID */
    public String counterId;
    /** Market code, e.g. "US" */
    public String market;
    /** Industry ID, or null */
    public String industryId;
}
