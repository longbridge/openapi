package com.longbridge.quote;

import java.math.BigDecimal;

public class Depth {
    private int position;
    private BigDecimal price;
    private long volume;
    private long orderNum;

    public int getPosition() {
        return position;
    }

    public BigDecimal getPrice() {
        return price;
    }

    public long getVolume() {
        return volume;
    }

    public long getOrderNum() {
        return orderNum;
    }
}
