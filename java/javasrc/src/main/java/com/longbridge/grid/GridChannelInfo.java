package com.longbridge.grid;

import java.util.Arrays;

/**
 * Channel / authorization info nested in the symbol-info response
 */
public class GridChannelInfo {
    private boolean strategyGranted;
    private boolean supportRth;
    private String currency;
    private String[] settlementCurrency;

    /**
     * Returns strategyGranted.
     *
     * @return strategyGranted
     */
    public boolean getStrategyGranted() {
        return strategyGranted;
    }

    /**
     * Returns supportRth.
     *
     * @return supportRth
     */
    public boolean getSupportRth() {
        return supportRth;
    }

    /**
     * Returns currency.
     *
     * @return currency
     */
    public String getCurrency() {
        return currency;
    }

    /**
     * Returns settlementCurrency.
     *
     * @return settlementCurrency
     */
    public String[] getSettlementCurrency() {
        return settlementCurrency;
    }

    @Override
    public String toString() {
        return "GridChannelInfo [strategyGranted=" + strategyGranted +
                ", supportRth=" + supportRth +
                ", currency=" + currency +
                ", settlementCurrency=" + Arrays.toString(settlementCurrency) + "]";
    }
}
