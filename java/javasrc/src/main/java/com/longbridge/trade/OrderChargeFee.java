package com.longbridge.trade;

import java.math.BigDecimal;

public class OrderChargeFee {
    private String code;
    private String name;
    private BigDecimal amount;
    private String currency;

    public String getCode() {
        return code;
    }

    public String getName() {
        return name;
    }

    public BigDecimal getAmount() {
        return amount;
    }

    public String getCurrency() {
        return currency;
    }

    @Override
    public String toString() {
        return "OrderChargeFee [code=" + code + ", name=" + name + ", amount=" + amount + ", currency=" + currency
                + "]";
    }
}
