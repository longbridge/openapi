package com.longbridge.sharelist;

/** Response for {@link SharelistContext#list} and {@link SharelistContext#popular}. */
public class SharelistList {
    /** User's own and followed sharelists. */
    public SharelistInfo[] sharelists;
    /** Subscribed sharelists (may be absent in popular response). */
    public SharelistInfo[] subscribedSharelists;
    /** Pagination cursor for the subscribed list. */
    public String tailMark;
}
