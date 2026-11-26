package com.longbridge.asset;

/**
 * Witness state of a bank card
 *
 * <p>
 * The Java ordinal is not the server code: use {@link #code()} to get the
 * code the server uses for a constant, and {@link #fromCode(int)} for the
 * reverse.
 */
public enum BankCardWitnessState {
    /** Unknown: an undocumented code (never sent to the server) */
    Unknown(-1),
    /** Witnessing not required (server code 1) */
    NotRequired(1),
    /** Witnessing pending (server code 2) */
    Pending(2),
    /** Witnessed (server code 3) */
    Witnessed(3);

    private final int code;

    BankCardWitnessState(int code) {
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
    public static BankCardWitnessState fromCode(int code) {
        for (BankCardWitnessState value : values()) {
            if (value != Unknown && value.code == code) {
                return value;
            }
        }
        return Unknown;
    }
}
