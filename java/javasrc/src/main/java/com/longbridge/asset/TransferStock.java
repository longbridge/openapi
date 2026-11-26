package com.longbridge.asset;

import java.math.BigDecimal;

/**
 * One stock in a stock transfer application
 */
public class TransferStock {
    private String code;
    private String name;
    private String nameEn;
    private String market;
    private String counterId;
    private BigDecimal amount;
    private String currency;
    private BigDecimal cost;
    private TransferStockState stateCode;
    private Integer stateCodeRaw;
    private String positionSide;

    /**
     * Returns the stock code.
     *
     * @return the stock code
     */
    public String getCode() {
        return code;
    }

    /**
     * Returns the stock name.
     *
     * @return the stock name
     */
    public String getName() {
        return name;
    }

    /**
     * Returns the stock name in English.
     *
     * @return the stock name in English
     */
    public String getNameEn() {
        return nameEn;
    }

    /**
     * Returns the market.
     *
     * @return the market
     */
    public String getMarket() {
        return market;
    }

    /**
     * Returns the counter ID.
     *
     * @return the counter ID
     */
    public String getCounterId() {
        return counterId;
    }

    /**
     * Returns the quantity ({@code null} when not set).
     *
     * @return the quantity
     */
    public BigDecimal getAmount() {
        return amount;
    }

    /**
     * Returns the currency.
     *
     * @return the currency
     */
    public String getCurrency() {
        return currency;
    }

    /**
     * Returns the cost price ({@code null} when not set).
     *
     * @return the cost price
     */
    public BigDecimal getCost() {
        return cost;
    }

    /**
     * Returns the state code of this stock in the transfer.
     *
     * @return the state code of this stock in the transfer
     */
    public TransferStockState getStateCode() {
        return stateCode;
    }

    /**
     * The server's numeric code of state_code, also when it is a code
     * this SDK does not know (state_code is then Unknown); null when the
     * value is missing or is not a code
     *
     * @return the raw code
     */
    public Integer getStateCodeRaw() {
        return stateCodeRaw;
    }

    /**
     * Returns the position side.
     *
     * @return the position side
     */
    public String getPositionSide() {
        return positionSide;
    }

    @Override
    public String toString() {
        return "TransferStock [" + "code=" + code + ", " + "name=" + name + ", " + "nameEn=" + nameEn + ", " + "market=" + market + ", " + "counterId=" + counterId + ", " + "amount=" + amount + ", " + "currency=" + currency + ", " + "cost=" + cost + ", " + "stateCode=" + stateCode + ", " + "stateCodeRaw=" + stateCodeRaw + ", " + "positionSide=" + positionSide + "]";
    }
}
