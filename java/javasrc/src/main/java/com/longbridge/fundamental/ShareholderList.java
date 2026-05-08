package com.longbridge.fundamental;

/** Response containing major shareholders of a security. */
public class ShareholderList {
    /** List of major shareholders. */
    public Shareholder[] shareholderList;
    /** Link to the full shareholder page. */
    public String forwardUrl;
    /** Total number of shareholders returned. */
    public int total;
}