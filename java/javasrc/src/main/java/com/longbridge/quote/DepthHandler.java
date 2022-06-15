
package com.longbridge.quote;

public interface DepthHandler {
    void onDepth(String symbol, PushDepth depth);
}
