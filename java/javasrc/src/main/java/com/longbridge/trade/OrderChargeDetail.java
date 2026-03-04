package com.longbridge.trade;

import java.math.BigDecimal;
import java.util.Arrays;

public class OrderChargeDetail {
    private BigDecimal totalAmount;
    private String currency;
    private OrderChargeItem[] items;

    public BigDecimal getTotalAmount() {
        return totalAmount;
    }

    public String getCurrency() {
        return currency;
    }

    public OrderChargeItem[] getItems() {
        return items;
    }

    @Override
    public String toString() {
        return "OrderChargeDetail [totalAmount=" + totalAmount + ", currency=" + currency + ", items="
                + Arrays.toString(items) + "]";
    }
}
