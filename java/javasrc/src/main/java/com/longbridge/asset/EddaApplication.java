package com.longbridge.asset;

import java.time.OffsetDateTime;

/**
 * An eDDA authorization application of a bank card
 */
public class EddaApplication {
    private String id;
    private String flowId;
    private String name;
    private String currency;
    private AuthorizationStatus status;
    private Integer statusRaw;
    private String message;
    private OffsetDateTime createdAt;
    private OffsetDateTime updatedAt;

    /**
     * Returns the application ID.
     *
     * @return the application ID
     */
    public String getId() {
        return id;
    }

    /**
     * Returns the authorization flow: {@code hk_edda} (authorized at the bank)
     * or {@code hk_sdda} (authorized in the app).
     *
     * @return the authorization flow
     */
    public String getFlowId() {
        return flowId;
    }

    /**
     * Returns the display name of the authorization.
     *
     * @return the display name
     */
    public String getName() {
        return name;
    }

    /**
     * Returns the authorized currency.
     *
     * @return the currency
     */
    public String getCurrency() {
        return currency;
    }

    /**
     * Returns the application status.
     *
     * @return the application status
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
     * Returns the failure reason (empty unless the application failed).
     *
     * @return the failure reason
     */
    public String getMessage() {
        return message;
    }

    /**
     * Returns the time the application was filed (null when not set).
     *
     * @return the filed time
     */
    public OffsetDateTime getCreatedAt() {
        return createdAt;
    }

    /**
     * Returns the time the application was last updated (null when not set).
     *
     * @return the updated time
     */
    public OffsetDateTime getUpdatedAt() {
        return updatedAt;
    }

    @Override
    public String toString() {
        return "EddaApplication [" + "id=" + id + ", " + "flowId=" + flowId + ", " + "name=" + name + ", " + "currency=" + currency + ", " + "status=" + status + ", " + "statusRaw=" + statusRaw + ", " + "message=" + message + ", " + "createdAt=" + createdAt + ", " + "updatedAt=" + updatedAt + "]";
    }
}
