package com.longbridge.asset;

/**
 * State of a deposit record
 *
 * <p>
 * The Java ordinal is not the server code: use {@link #code()} to get the
 * code the server uses for a constant, and {@link #fromCode(int)} for the
 * reverse.
 */
public enum DepositState {
    /** Unknown: an undocumented code (never sent to the server) */
    Unknown(-1),
    /** Processing (server code 0) */
    Processing(0),
    /** Credited to the account (server code 1) */
    Credited(1),
    /** Failed (server code 2) */
    Failed(2),
    /** Cancelled (server code 3) */
    Cancelled(3),
    /** Invalidated (server code 4) */
    Invalidated(4),
    /** Rejected (server code 5) */
    Rejected(5);

    private final int code;

    DepositState(int code) {
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
    public static DepositState fromCode(int code) {
        for (DepositState value : values()) {
            if (value != Unknown && value.code == code) {
                return value;
            }
        }
        return Unknown;
    }
}
