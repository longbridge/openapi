package com.longbridge.asset;

/**
 * Binding state of a bank card
 *
 * <p>
 * The Java ordinal is not the server code: use {@link #code()} to get the
 * code the server uses for a constant, and {@link #fromCode(int)} for the
 * reverse.
 */
public enum BankCardBindState {
    /** Unknown: an undocumented code (never sent to the server) */
    Unknown(-1),
    /** Binding in progress (server code 0) */
    Binding(0),
    /** Bound (server code 1) */
    Bound(1),
    /** Binding failed (server code 2) */
    Failed(2);

    private final int code;

    BankCardBindState(int code) {
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
    public static BankCardBindState fromCode(int code) {
        for (BankCardBindState value : values()) {
            if (value != Unknown && value.code == code) {
                return value;
            }
        }
        return Unknown;
    }
}
