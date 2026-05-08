package com.longbridge.market;

/** Full broker holding detail list for a security. */
public class BrokerHoldingDetail {
    /** Full list of broker holdings. */
    public BrokerHoldingDetailItem[] list;
    /** Last updated timestamp (may be empty). */
    public String updatedAt;
}