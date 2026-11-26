package com.longbridge.asset;

import java.time.OffsetDateTime;
import java.util.Arrays;

/**
 * A linked bank card. The card holder's name and address and ID document keys are not modelled.
 */
public class BankCard {
    private String id;
    private String bankId;
    private String bank;
    private String bankEn;
    private String bankCode;
    private String account;
    private String accountType;
    private String country;
    private String region;
    private String regionName;
    private String swiftCode;
    private String remark;
    private BankCardStatus status;
    private BankCardWitnessState witnessState;
    private boolean supportWitness;
    private BankCardBindState bindState;
    private String bindTip;
    private String withdrawFee;
    private OffsetDateTime createdAt;
    private BankCardAuthorization[] authList;

    /**
     * Returns the bank card ID (pass to {@link AssetContext#getEddaApplications}).
     *
     * @return the bank card ID
     */
    public String getId() {
        return id;
    }

    /**
     * Returns the bank ID.
     *
     * @return the bank ID
     */
    public String getBankId() {
        return bankId;
    }

    /**
     * Returns the bank name.
     *
     * @return the bank name
     */
    public String getBank() {
        return bank;
    }

    /**
     * Returns the bank name in English.
     *
     * @return the bank name in English
     */
    public String getBankEn() {
        return bankEn;
    }

    /**
     * Returns the bank code.
     *
     * @return the bank code
     */
    public String getBankCode() {
        return bankCode;
    }

    /**
     * Returns the bank account number, as returned by the server (not masked).
     *
     * @return the bank account number, as returned by the server
     */
    public String getAccount() {
        return account;
    }

    /**
     * Returns the account type.
     *
     * @return the account type
     */
    public String getAccountType() {
        return accountType;
    }

    /**
     * Returns the country or region of the bank (e.g. {@code Hong Kong}; not a currency).
     *
     * @return the country or region of the bank
     */
    public String getCountry() {
        return country;
    }

    /**
     * Returns the region code.
     *
     * @return the region code
     */
    public String getRegion() {
        return region;
    }

    /**
     * Returns the region name.
     *
     * @return the region name
     */
    public String getRegionName() {
        return regionName;
    }

    /**
     * Returns the SWIFT code.
     *
     * @return the SWIFT code
     */
    public String getSwiftCode() {
        return swiftCode;
    }

    /**
     * Returns the remark.
     *
     * @return the remark
     */
    public String getRemark() {
        return remark;
    }

    /**
     * Returns the verification status.
     *
     * @return the verification status
     */
    public BankCardStatus getStatus() {
        return status;
    }

    /**
     * Returns the witness state.
     *
     * @return the witness state
     */
    public BankCardWitnessState getWitnessState() {
        return witnessState;
    }

    /**
     * Returns whether the card supports witnessing.
     *
     * @return whether the card supports witnessing
     */
    public boolean isSupportWitness() {
        return supportWitness;
    }

    /**
     * Returns the binding state.
     *
     * @return the binding state
     */
    public BankCardBindState getBindState() {
        return bindState;
    }

    /**
     * Returns the binding hint text.
     *
     * @return the binding hint text
     */
    public String getBindTip() {
        return bindTip;
    }

    /**
     * Returns the withdrawal fee text, as returned by the server.
     *
     * @return the withdrawal fee text, as returned by the server
     */
    public String getWithdrawFee() {
        return withdrawFee;
    }

    /**
     * Returns the time the card was added ({@code null} when not set).
     *
     * @return the time the card was added
     */
    public OffsetDateTime getCreatedAt() {
        return createdAt;
    }

    /**
     * Returns the authorizations attached to the card.
     *
     * @return the authorizations attached to the card
     */
    public BankCardAuthorization[] getAuthList() {
        return authList;
    }

    @Override
    public String toString() {
        return "BankCard [" + "id=" + id + ", " + "bankId=" + bankId + ", " + "bank=" + bank + ", " + "bankEn=" + bankEn + ", " + "bankCode=" + bankCode + ", " + "account=" + account + ", " + "accountType=" + accountType + ", " + "country=" + country + ", " + "region=" + region + ", " + "regionName=" + regionName + ", " + "swiftCode=" + swiftCode + ", " + "remark=" + remark + ", " + "status=" + status + ", " + "witnessState=" + witnessState + ", " + "supportWitness=" + supportWitness + ", " + "bindState=" + bindState + ", " + "bindTip=" + bindTip + ", " + "withdrawFee=" + withdrawFee + ", " + "createdAt=" + createdAt + ", " + "authList=" + Arrays.toString(authList) + "]";
    }
}
