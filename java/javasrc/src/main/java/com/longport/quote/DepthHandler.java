
package com.longport.quote;

public interface DepthHandler {
    void onDepth(String symbol, PushDepth event);
}
