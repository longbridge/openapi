package com.longbridge.trade;

import java.math.BigDecimal;

/**
 * Cash balance information for a single currency
 */
public class CashInfo {
    private BigDecimal withdrawCash;
    private BigDecimal availableCash;
    private BigDecimal frozenCash;
    private BigDecimal settlingCash;
    private String currency;
    private BigDecimal redemptionCash;

    /**
     * Returns the withdrawable cash amount.
     *
     * @return withdrawable cash
     */
    public BigDecimal getWithdrawCash() {
        return withdrawCash;
    }

    /**
     * Returns the available cash amount.
     *
     * @return available cash
     */
    public BigDecimal getAvailableCash() {
        return availableCash;
    }

    /**
     * Returns the frozen cash amount.
     *
     * @return frozen cash
     */
    public BigDecimal getFrozenCash() {
        return frozenCash;
    }

    /**
     * Returns the settling cash amount.
     *
     * @return settling cash
     */
    public BigDecimal getSettlingCash() {
        return settlingCash;
    }

    /**
     * Returns the currency.
     *
     * @return currency
     */
    public String getCurrency() {
        return currency;
    }

    /**
     * Returns the redeemable cash amount.
     *
     * @return redeemable cash
     */
    public BigDecimal getRedemptionCash() {
        return redemptionCash;
    }

    @Override
    public String toString() {
        return "CashInfo [availableCash=" + availableCash + ", currency=" + currency + ", frozenCash=" + frozenCash
                + ", redemptionCash=" + redemptionCash + ", settlingCash=" + settlingCash + ", withdrawCash="
                + withdrawCash + "]";
    }
}
