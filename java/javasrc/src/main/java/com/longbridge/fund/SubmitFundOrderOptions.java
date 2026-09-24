package com.longbridge.fund;

/**
 * Options for SubmitFundOrderOptions.
 */
@SuppressWarnings("unused")
public class SubmitFundOrderOptions {
    private String symbol;
    private String action;
    private String currency;
    private String amount;
    private String units;
    private Integer dividendOption;
    private String fee;
    private Boolean isSellAll;
    private String remark;
    private Integer tradeMethod;

    /**
     * Creates a new SubmitFundOrderOptions.
     *
     * @param symbol symbol
     * @param action action
     * @param currency currency
     */
    public SubmitFundOrderOptions(String symbol, String action, String currency) {
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
    public SubmitFundOrderOptions setAmount(String amount) {
        this.amount = amount;
        return this;
    }

    /**
     * Sets units.
     *
     * @param units units
     * @return this instance for chaining
     */
    public SubmitFundOrderOptions setUnits(String units) {
        this.units = units;
        return this;
    }

    /**
     * Sets dividendOption.
     *
     * @param dividendOption dividendOption
     * @return this instance for chaining
     */
    public SubmitFundOrderOptions setDividendOption(Integer dividendOption) {
        this.dividendOption = dividendOption;
        return this;
    }

    /**
     * Sets fee.
     *
     * @param fee fee
     * @return this instance for chaining
     */
    public SubmitFundOrderOptions setFee(String fee) {
        this.fee = fee;
        return this;
    }

    /**
     * Sets isSellAll.
     *
     * @param isSellAll isSellAll
     * @return this instance for chaining
     */
    public SubmitFundOrderOptions setIsSellAll(Boolean isSellAll) {
        this.isSellAll = isSellAll;
        return this;
    }

    /**
     * Sets remark.
     *
     * @param remark remark
     * @return this instance for chaining
     */
    public SubmitFundOrderOptions setRemark(String remark) {
        this.remark = remark;
        return this;
    }

    /**
     * Sets tradeMethod.
     *
     * @param tradeMethod tradeMethod
     * @return this instance for chaining
     */
    public SubmitFundOrderOptions setTradeMethod(Integer tradeMethod) {
        this.tradeMethod = tradeMethod;
        return this;
    }

}
