package com.longbridge.asset;

import java.math.BigDecimal;
import java.time.OffsetDateTime;

/**
 * A withdrawal record
 */
public class Withdrawal {
    private String id;
    private String orderId;
    private OffsetDateTime createdAt;
    private String currency;
    private BigDecimal amount;
    private BigDecimal fee;
    private BigDecimal settlementAmount;
    private WithdrawalState stateCode;
    private Integer stateCodeRaw;
    private String rawState;
    private String stateDescription;
    private String bankAccount;
    private String displayBankAccount;
    private String userBankSwiftCode;
    private String accountChannel;

    /**
     * Returns the record ID.
     *
     * @return the record ID
     */
    public String getId() {
        return id;
    }

    /**
     * Returns the order ID.
     *
     * @return the order ID
     */
    public String getOrderId() {
        return orderId;
    }

    /**
     * Returns the created time.
     *
     * @return the created time
     */
    public OffsetDateTime getCreatedAt() {
        return createdAt;
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
     * Returns the applied amount.
     *
     * @return the applied amount
     */
    public BigDecimal getAmount() {
        return amount;
    }

    /**
     * Returns the fee.
     *
     * @return the fee
     */
    public BigDecimal getFee() {
        return fee;
    }

    /**
     * Returns the amount paid out.
     *
     * @return the amount paid out
     */
    public BigDecimal getSettlementAmount() {
        return settlementAmount;
    }

    /**
     * Returns the state code.
     *
     * @return the state code
     */
    public WithdrawalState getStateCode() {
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
     * Returns the raw state, as returned by the server.
     *
     * @return the raw state, as returned by the server
     */
    public String getRawState() {
        return rawState;
    }

    /**
     * Returns the state display text, as returned by the server.
     *
     * @return the state display text, as returned by the server
     */
    public String getStateDescription() {
        return stateDescription;
    }

    /**
     * Returns the receiving bank account number, as returned by the server (not masked).
     *
     * @return the receiving bank account number, as returned by the server
     */
    public String getBankAccount() {
        return bankAccount;
    }

    /**
     * Returns the receiving bank account, as displayed in the app.
     *
     * @return the receiving bank account, as displayed in the app
     */
    public String getDisplayBankAccount() {
        return displayBankAccount;
    }

    /**
     * Returns the SWIFT code of the receiving bank.
     *
     * @return the SWIFT code of the receiving bank
     */
    public String getUserBankSwiftCode() {
        return userBankSwiftCode;
    }

    /**
     * Returns the account channel.
     *
     * @return the account channel
     */
    public String getAccountChannel() {
        return accountChannel;
    }

    @Override
    public String toString() {
        return "Withdrawal [" + "id=" + id + ", " + "orderId=" + orderId + ", " + "createdAt=" + createdAt + ", " + "currency=" + currency + ", " + "amount=" + amount + ", " + "fee=" + fee + ", " + "settlementAmount=" + settlementAmount + ", " + "stateCode=" + stateCode + ", " + "stateCodeRaw=" + stateCodeRaw + ", " + "rawState=" + rawState + ", " + "stateDescription=" + stateDescription + ", " + "bankAccount=" + bankAccount + ", " + "displayBankAccount=" + displayBankAccount + ", " + "userBankSwiftCode=" + userBankSwiftCode + ", " + "accountChannel=" + accountChannel + "]";
    }
}
