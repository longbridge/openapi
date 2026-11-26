package com.longbridge.asset;

import java.math.BigDecimal;
import java.time.OffsetDateTime;

/**
 * A deposit record. Receipt images are not modelled.
 */
public class Deposit {
    private String id;
    private OffsetDateTime createdAt;
    private OffsetDateTime updatedAt;
    private String currency;
    private BigDecimal amount;
    private BigDecimal receivableAmount;
    private String state;
    private DepositState stateCode;
    private Integer stateCodeRaw;
    private int subStateCode;
    private String bankOperationTypeId;
    private String bankOperationTypeName;
    private String referenceNo;
    private boolean canCancel;
    private String userBankSwiftCode;
    private long voucherCount;

    /**
     * Returns the record ID.
     *
     * @return the record ID
     */
    public String getId() {
        return id;
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
     * Returns the updated time.
     *
     * @return the updated time
     */
    public OffsetDateTime getUpdatedAt() {
        return updatedAt;
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
     * Returns the notified (applied) amount.
     *
     * @return the notified
     */
    public BigDecimal getAmount() {
        return amount;
    }

    /**
     * Returns the received amount ({@code 0} = not received yet).
     *
     * @return the received amount
     */
    public BigDecimal getReceivableAmount() {
        return receivableAmount;
    }

    /**
     * Returns the state text, as returned by the server.
     *
     * @return the state text, as returned by the server
     */
    public String getState() {
        return state;
    }

    /**
     * Returns the state code.
     *
     * @return the state code
     */
    public DepositState getStateCode() {
        return stateCode;
    }

    /**
     * The server's numeric code of state_code, also when it is a code
     * this SDK does not know (state_code is then Unknown); null when the
     * server sent none
     *
     * @return the raw code
     */
    public Integer getStateCodeRaw() {
        return stateCodeRaw;
    }

    /**
     * Returns the sub-state code, as returned by the server.
     *
     * @return the sub-state code, as returned by the server
     */
    public int getSubStateCode() {
        return subStateCode;
    }

    /**
     * Returns the deposit method ID.
     *
     * @return the deposit method ID
     */
    public String getBankOperationTypeId() {
        return bankOperationTypeId;
    }

    /**
     * Returns the deposit method name.
     *
     * @return the deposit method name
     */
    public String getBankOperationTypeName() {
        return bankOperationTypeName;
    }

    /**
     * Returns the reference number.
     *
     * @return the reference number
     */
    public String getReferenceNo() {
        return referenceNo;
    }

    /**
     * Returns whether the deposit can still be cancelled.
     *
     * @return whether the deposit can still be cancelled
     */
    public boolean isCanCancel() {
        return canCancel;
    }

    /**
     * Returns the SWIFT code of the paying bank.
     *
     * @return the SWIFT code of the paying bank
     */
    public String getUserBankSwiftCode() {
        return userBankSwiftCode;
    }

    /**
     * Returns the number of receipts uploaded with the deposit.
     *
     * @return the number of receipts
     */
    public long getVoucherCount() {
        return voucherCount;
    }

    @Override
    public String toString() {
        return "Deposit [" + "id=" + id + ", " + "createdAt=" + createdAt + ", " + "updatedAt=" + updatedAt + ", " + "currency=" + currency + ", " + "amount=" + amount + ", " + "receivableAmount=" + receivableAmount + ", " + "state=" + state + ", " + "stateCode=" + stateCode + ", " + "stateCodeRaw=" + stateCodeRaw + ", " + "subStateCode=" + subStateCode + ", " + "bankOperationTypeId=" + bankOperationTypeId + ", " + "bankOperationTypeName=" + bankOperationTypeName + ", " + "referenceNo=" + referenceNo + ", " + "canCancel=" + canCancel + ", " + "userBankSwiftCode=" + userBankSwiftCode + ", " + "voucherCount=" + voucherCount + "]";
    }
}
