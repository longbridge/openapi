package com.longbridge.fundamental;

/** Options for {@link FundamentalContext#getShareholderDetail}. */
public class ShareholderDetailOptions {
    /** Security symbol, e.g. "AAPL.US" */
    public String symbol;
    /** Shareholder object ID from getShareholderTop */
    public long objectId;
}
