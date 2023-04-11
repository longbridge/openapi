package com.longbridge.trade;

import java.util.Arrays;

public class OrderChargeItem {
    private ChargeCategoryCode code;
    private String name;
    private OrderChargeFee[] fees;

    public ChargeCategoryCode getCode() {
        return code;
    }

    public String getName() {
        return name;
    }

    public OrderChargeFee[] getFees() {
        return fees;
    }

    @Override
    public String toString() {
        return "OrderChargeItem [code=" + code + ", name=" + name + ", fees=" + Arrays.toString(fees) + "]";
    }
}
