package com.longbridge.asset;

/**
 * Verification status of a bank card
 *
 * <p>
 * The Java ordinal is not the server code: use {@link #code()} to get the
 * code the server uses for a constant, and {@link #fromCode(int)} for the
 * reverse.
 */
public enum BankCardStatus {
    /** Unknown: an undocumented code (never sent to the server) */
    Unknown(-1),
    /** Not verified (server code 0) */
    Unverified(0),
    /** Under review (server code 1) */
    UnderReview(1),
    /** Verified (server code 2) */
    Verified(2);

    private final int code;

    BankCardStatus(int code) {
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
    public static BankCardStatus fromCode(int code) {
        for (BankCardStatus value : values()) {
            if (value != Unknown && value.code == code) {
                return value;
            }
        }
        return Unknown;
    }
}
