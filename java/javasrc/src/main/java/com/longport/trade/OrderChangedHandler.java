package com.longport.trade;

public interface OrderChangedHandler {
    void onOrderChanged(PushOrderChanged orderChanged);
}
