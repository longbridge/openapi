package com.longbridge.asset;

/**
 * Status of a bank-card authorization or of an eDDA application
 *
 * <p>
 * The Java ordinal is not the server code: use {@link #code()} to get the
 * code the server uses for a constant, and {@link #fromCode(int)} for the
 * reverse.
 */
public enum AuthorizationStatus {
    /** Unknown: an undocumented code (never sent to the server) */
    Unknown(-1),
    /** Not authorized (server code 0) */
    Unauthorized(0),
    /** Authorized (server code 1) */
    Authorized(1),
    /** Authorization in progress (server code 2) */
    Authorizing(2),
    /** Authorization failed (server code 3) */
    Failed(3);

    private final int code;

    AuthorizationStatus(int code) {
        this.code = code;
    }

    /**
     * Returns the server code of this constant ({@code -1} for
     * {@link #Unknown}, which is not a server code).
     *
     * @return the server code
     */
    public int code() {
        return code;
    }

    /**
     * Returns the constant for a server code.
     *
     * @param code server code
     * @return the constant, or {@link #Unknown} for an undocumented code
     */
    public static AuthorizationStatus fromCode(int code) {
        for (AuthorizationStatus value : values()) {
            if (value != Unknown && value.code == code) {
                return value;
            }
        }
        return Unknown;
    }
}
