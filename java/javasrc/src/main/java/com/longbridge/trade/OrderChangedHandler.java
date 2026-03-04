package com.longbridge.trade;

public interface OrderChangedHandler {
    void onOrderChanged(PushOrderChanged orderChanged);
}
