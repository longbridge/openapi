package com.longbridge.quote;

/** Request for {@link QuoteContext#updatePinned}. */
public class UpdatePinnedRequest {
    /** Whether to add or remove the pinned securities */
    public PinnedMode mode;
    /** Security symbols to pin or unpin */
    public String[] symbols;
}
