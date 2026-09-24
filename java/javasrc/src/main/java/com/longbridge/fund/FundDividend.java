package com.longbridge.fund;

/**
 * FundDividend
 */
public class FundDividend {
    private String amount;
    private String counterId;
    private String currency;
    private long date;
    private String divMethod;
    private String name;

    /**
     * Returns amount.
     *
     * @return amount
     */
    public String getAmount() {
        return amount;
    }

    /**
     * Returns counterId.
     *
     * @return counterId
     */
    public String getCounterId() {
        return counterId;
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
     * Returns date.
     *
     * @return date
     */
    public long getDate() {
        return date;
    }

    /**
     * Returns divMethod.
     *
     * @return divMethod
     */
    public String getDivMethod() {
        return divMethod;
    }

    /**
     * Returns name.
     *
     * @return name
     */
    public String getName() {
        return name;
    }

    @Override
    public String toString() {
        return                 "FundDividend [amount=" + amount +
                ", counterId=" + counterId +
                ", currency=" + currency +
                ", date=" + date +
                ", divMethod=" + divMethod +
                ", name=" + name + "]";
    }
}
