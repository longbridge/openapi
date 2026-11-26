package com.longbridge.asset;

import java.math.BigDecimal;
import java.time.OffsetDateTime;

/**
 * An authorization attached to a bank card (eDDA, bank securities account, FPS, ...)
 */
public class BankCardAuthorization {
    private String flowId;
    private String name;
    private String currency;
    private AuthorizationStatus status;
    private Integer statusRaw;
    private String message;
    private boolean isSupport;
    private BigDecimal maxAmount;
    private boolean maxAmountUnlimited;
    private Long dailyMaxCount;
    private boolean dailyMaxCountUnlimited;
    private OffsetDateTime expiryDate;
    private boolean longTermValid;
    private boolean docReviewing;
    private boolean noPriorDepositReviewing;

    /**
     * Returns the authorization flow: {@code hk_edda} / {@code hk_sdda} (eDDA), {@code hk_bank_sec}, {@code hk_fps}, ....
     *
     * @return the authorization flow: {@code hk_edda} / {@code hk_sdda}
     */
    public String getFlowId() {
        return flowId;
    }

    /**
     * Returns the display name of the authorization.
     *
     * @return the display name of the authorization
     */
    public String getName() {
        return name;
    }

    /**
     * Returns the authorized currency.
     *
     * @return the authorized currency
     */
    public String getCurrency() {
        return currency;
    }

    /**
     * Returns the authorization status.
     *
     * @return the authorization status
     */
    public AuthorizationStatus getStatus() {
        return status;
    }

    /**
     * The server's numeric code of status, also when it is a code
     * this SDK does not know (status is then Unknown); null when the
     * value is missing or is not a code
     *
     * @return the raw code
     */
    public Integer getStatusRaw() {
        return statusRaw;
    }

    /**
     * Returns the failure reason (empty unless the authorization failed).
     *
     * @return the failure reason
     */
    public String getMessage() {
        return message;
    }

    /**
     * Returns whether this authorization is supported for the card.
     *
     * @return whether this authorization is supported for the card
     */
    public boolean isSupport() {
        return isSupport;
    }

    /**
     * Returns the maximum amount per debit ({@code null} when not set).
     *
     * @return the maximum amount per debit
     */
    public BigDecimal getMaxAmount() {
        return maxAmount;
    }

    /**
     * Returns whether the per-debit amount is unlimited.
     *
     * @return whether the per-debit amount is unlimited
     */
    public boolean isMaxAmountUnlimited() {
        return maxAmountUnlimited;
    }

    /**
     * Returns the maximum number of debits per day ({@code null} when not set).
     *
     * @return the maximum number of debits per day
     */
    public Long getDailyMaxCount() {
        return dailyMaxCount;
    }

    /**
     * Returns whether the daily debit count is unlimited.
     *
     * @return whether the daily debit count is unlimited
     */
    public boolean isDailyMaxCountUnlimited() {
        return dailyMaxCountUnlimited;
    }

    /**
     * Returns the expiry date ({@code null} when not set and for a long-term
     * authorization: the server's 9999-12-31 "never expires" sentinel is
     * {@code null}; see {@link #isLongTermValid()}).
     *
     * @return the expiry date
     */
    public OffsetDateTime getExpiryDate() {
        return expiryDate;
    }

    /**
     * Returns whether the authorization never expires.
     *
     * @return whether the authorization never expires
     */
    public boolean isLongTermValid() {
        return longTermValid;
    }

    /**
     * Returns whether submitted documents are under review.
     *
     * @return whether submitted documents are under review
     */
    public boolean isDocReviewing() {
        return docReviewing;
    }

    /**
     * Returns whether the "no prior deposit" check is under review.
     *
     * @return whether the "no prior deposit" check is under review
     */
    public boolean isNoPriorDepositReviewing() {
        return noPriorDepositReviewing;
    }

    @Override
    public String toString() {
        return "BankCardAuthorization [" + "flowId=" + flowId + ", " + "name=" + name + ", " + "currency=" + currency + ", " + "status=" + status + ", " + "statusRaw=" + statusRaw + ", " + "message=" + message + ", " + "isSupport=" + isSupport + ", " + "maxAmount=" + maxAmount + ", " + "maxAmountUnlimited=" + maxAmountUnlimited + ", " + "dailyMaxCount=" + dailyMaxCount + ", " + "dailyMaxCountUnlimited=" + dailyMaxCountUnlimited + ", " + "expiryDate=" + expiryDate + ", " + "longTermValid=" + longTermValid + ", " + "docReviewing=" + docReviewing + ", " + "noPriorDepositReviewing=" + noPriorDepositReviewing + "]";
    }
}
