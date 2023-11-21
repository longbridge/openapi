package com.longport.trade;

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

    @Override
    public String toString() {
        return "AccountBalance [cashInfos=" + Arrays.toString(cashInfos) + ", currency=" + currency + ", initMargin="
                + initMargin + ", maintenanceMargin=" + maintenanceMargin + ", marginCall=" + marginCall
                + ", maxFinanceAmount=" + maxFinanceAmount + ", netAssets=" + netAssets + ", remainingFinanceAmount="
                + remainingFinanceAmount + ", riskLevel=" + riskLevel + ", totalCash=" + totalCash + "]";
    }
}
