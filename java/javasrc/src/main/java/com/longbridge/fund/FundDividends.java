package com.longbridge.fund;

/**
 * FundDividends
 */
public class FundDividends {
    private String currency;
    private FundDividend[] divCashInfos;
    private long lastestDate;
    private String totalDivCash;

    /**
     * Returns currency.
     *
     * @return currency
     */
    public String getCurrency() {
        return currency;
    }

    /**
     * Returns divCashInfos.
     *
     * @return divCashInfos
     */
    public FundDividend[] getDivCashInfos() {
        return divCashInfos;
    }

    /**
     * Returns lastestDate.
     *
     * @return lastestDate
     */
    public long getLastestDate() {
        return lastestDate;
    }

    /**
     * Returns totalDivCash.
     *
     * @return totalDivCash
     */
    public String getTotalDivCash() {
        return totalDivCash;
    }

    @Override
    public String toString() {
        return                 "FundDividends [currency=" + currency +
                ", divCashInfos=" + java.util.Arrays.toString(divCashInfos) +
                ", lastestDate=" + lastestDate +
                ", totalDivCash=" + totalDivCash + "]";
    }
}
