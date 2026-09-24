package com.longbridge.fund;

/**
 * Options for ValidateFundOrderOptions.
 */
@SuppressWarnings("unused")
public class ValidateFundOrderOptions {
    private String symbol;
    private String action;
    private String currency;
    private String amount;
    private String units;
    private Integer dividendOption;
    private Integer fundSource;
    private String accountChannel;

    /**
     * Creates a new ValidateFundOrderOptions.
     *
     * @param symbol symbol
     * @param action action
     * @param currency currency
     */
    public ValidateFundOrderOptions(String symbol, String action, String currency) {
        this.symbol = symbol;
        this.action = action;
        this.currency = currency;
    }

    /**
     * Sets amount.
     *
     * @param amount amount
     * @return this instance for chaining
     */
    public ValidateFundOrderOptions setAmount(String amount) {
        this.amount = amount;
        return this;
    }

    /**
     * Sets units.
     *
     * @param units units
     * @return this instance for chaining
     */
    public ValidateFundOrderOptions setUnits(String units) {
        this.units = units;
        return this;
    }

    /**
     * Sets dividendOption.
     *
     * @param dividendOption dividendOption
     * @return this instance for chaining
     */
    public ValidateFundOrderOptions setDividendOption(Integer dividendOption) {
        this.dividendOption = dividendOption;
        return this;
    }

    /**
     * Sets fundSource.
     *
     * @param fundSource fundSource
     * @return this instance for chaining
     */
    public ValidateFundOrderOptions setFundSource(Integer fundSource) {
        this.fundSource = fundSource;
        return this;
    }

    /**
     * Sets accountChannel.
     *
     * @param accountChannel accountChannel
     * @return this instance for chaining
     */
    public ValidateFundOrderOptions setAccountChannel(String accountChannel) {
        this.accountChannel = accountChannel;
        return this;
    }

}
