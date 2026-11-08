package com.longbridge.trade;

/**
 * Callback interface for grid trading order change push events
 */
public interface GridOrderChangedHandler {
    /**
     * Called when a grid trading order status change is received.
     *
     * @param gridOrderChanged grid order change event
     */
    void onGridOrderChanged(PushGridOrderChanged gridOrderChanged);
}
