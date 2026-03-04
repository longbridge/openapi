package com.longbridge.trade;

import java.math.BigDecimal;

class FrozenTransactionFee {
    private String currency;
    private BigDecimal frozenTransactionFee;

    public String getCurrency() {
        return currency;
    }

    public BigDecimal getFrozenTransactionFee() {
        return frozenTransactionFee;
    }

    @Override
    public String toString() {
        return "FrozenTransactionFee [currency=" + currency + ", frozenTransactionFee=" + frozenTransactionFee + "]";
    }
}