package com.longbridge.trade;

import java.math.BigDecimal;
import java.util.Arrays;

public class AccountBalance {
    private BigDecimal totalCash;
    private BigDecimal maxFinanceAmount;
    private BigDecimal remainingFinanceAmount;
    private int riskLevel;
    private BigDecimal marginCall;
    private String currency;
    private CashInfo[] cashInfos;
    private BigDecimal netAssets;
    private BigDecimal initMargin;
    private BigDecimal maintenanceMargin;
    private BigDecimal buyPower;
    private FrozenTransactionFee[] frozenTransactionFees;

    public BigDecimal getTotalCash() {
        return totalCash;
    }

    public BigDecimal getMaxFinanceAmount() {
        return maxFinanceAmount;
    }

    public BigDecimal getRemainingFinanceAmount() {
        return remainingFinanceAmount;
    }

    public int getRiskLevel() {
        return riskLevel;
    }

    public BigDecimal getMarginCall() {
        return marginCall;
    }

    public String getCurrency() {
        return currency;
    }

    public CashInfo[] getCashInfos() {
        return cashInfos;
    }

    public BigDecimal getNetAssets() {
        return netAssets;
    }

    public BigDecimal getInitMargin() {
        return initMargin;
    }

    public BigDecimal getMaintenanceMargin() {
        return maintenanceMargin;
    }

    public BigDecimal getBuyPower() {
        return buyPower;
    }

    public FrozenTransactionFee[] getFrozenTransactionFees() {
        return frozenTransactionFees;
    }

    @Override
    public String toString() {
        return "AccountBalance [totalCash=" + totalCash + ", maxFinanceAmount=" + maxFinanceAmount
                + ", remainingFinanceAmount=" + remainingFinanceAmount + ", riskLevel=" + riskLevel + ", marginCall="
                + marginCall + ", currency=" + currency + ", cashInfos=" + Arrays.toString(cashInfos) + ", netAssets="
                + netAssets + ", initMargin=" + initMargin + ", maintenanceMargin=" + maintenanceMargin + ", buyPower="
                + buyPower + ", frozenTransactionFees=" + Arrays.toString(frozenTransactionFees) + "]";
    }
}
