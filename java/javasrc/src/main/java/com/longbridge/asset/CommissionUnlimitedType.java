package com.longbridge.asset;

/**
 * Kind of commission coupon (the server's {@code unlimited} code). The
 * meanings are the ones the Longbridge app gives the codes; the proto names
 * are given in brackets.
 *
 * <p>
 * The Java ordinal is not the server code: use {@link #code()} to get the
 * code the server uses for a constant, and {@link #fromCode(int)} for the
 * reverse.
 */
public enum CommissionUnlimitedType {
    /** Unknown: an undocumented code (never sent to the server) */
    Unknown(-1),
    /** Time-limited free commission ({@code Proportionate}, server code 0) */
    Proportionate(0),
    /** Commission rebate ({@code Full}, server code 1) */
    Full(1),
    /** Commission deduction ({@code Lifetime}, server code 2) */
    Lifetime(2),
    /** Per-use card with a fixed amount per use: amount / usage count ({@code ByAmountPer}, server code 6) */
    ByAmountPer(6),
    /** Per-use discount card, capped at the amount limit per use ({@code ByRatePer}, server code 7) */
    ByRatePer(7);

    private final int code;

    CommissionUnlimitedType(int code) {
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
    public static CommissionUnlimitedType fromCode(int code) {
        for (CommissionUnlimitedType value : values()) {
            if (value != Unknown && value.code == code) {
                return value;
            }
        }
        return Unknown;
    }
}
