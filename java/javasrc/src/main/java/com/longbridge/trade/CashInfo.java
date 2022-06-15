package com.longbridge.trade;

import java.math.BigDecimal;

public class CashInfo {
    private BigDecimal withdrawCash;
    private BigDecimal availableCash;
    private BigDecimal frozenCash;
    private BigDecimal settlingCash;
    private String currency;

    public BigDecimal getWithdrawCash() {
        return withdrawCash;
    }

    public BigDecimal getAvailableCash() {
        return availableCash;
    }

    public BigDecimal getFrozenCash() {
        return frozenCash;
    }

    public BigDecimal getSettlingCash() {
        return settlingCash;
    }

    public String getCurrency() {
        return currency;
    }
}
