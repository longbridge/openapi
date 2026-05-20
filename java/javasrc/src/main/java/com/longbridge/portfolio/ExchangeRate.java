package com.longbridge.portfolio;

/** One currency exchange rate. */
public class ExchangeRate {
    /** Average rate (base_currency / other_currency). */
    public double averageRate;
    /** Base currency, e.g. {@code "USD"}. */
    public String baseCurrency;
    /** Bid rate. */
    public double bidRate;
    /** Offer rate. */
    public double offerRate;
    /** Other currency, e.g. {@code "HKD"}. */
    public String otherCurrency;
}
