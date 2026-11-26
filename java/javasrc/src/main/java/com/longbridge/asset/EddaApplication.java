package com.longbridge.asset;

/**
 * An eDDA authorization application of a bank card
 */
public class EddaApplication {
    private String id;
    private AuthorizationStatus status;
    private String message;

    /**
     * Returns the application ID.
     *
     * @return the application ID
     */
    public String getId() {
        return id;
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
     * Returns the failure reason (empty unless the application failed).
     *
     * @return the failure reason
     */
    public String getMessage() {
        return message;
    }

    @Override
    public String toString() {
        return "EddaApplication [" + "id=" + id + ", " + "status=" + status + ", " + "message=" + message + "]";
    }
}
