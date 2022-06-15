package com.longbridge.trade;

import java.math.BigDecimal;
import java.time.OffsetDateTime;

public class CashFlow {
    private String transactionFlowName;
    private CashFlowDirection direction;
    private BalanceType businessType;
    private BigDecimal balance;
    private String currency;
    private OffsetDateTime businessTime;
    private String symbol;
    private String description;

    public String getTransactionFlowName() {
        return transactionFlowName;
    }

    public CashFlowDirection getDirection() {
        return direction;
    }

    public BalanceType getBusinessType() {
        return businessType;
    }

    public BigDecimal getBalance() {
        return balance;
    }

    public String getCurrency() {
        return currency;
    }

    public OffsetDateTime getBusinessTime() {
        return businessTime;
    }

    public String getSymbol() {
        return symbol;
    }

    public String getDescription() {
        return description;
    }
}
