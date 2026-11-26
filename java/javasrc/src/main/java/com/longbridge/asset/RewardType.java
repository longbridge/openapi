package com.longbridge.asset;

/**
 * Type of a reward record
 *
 * <p>
 * The Java ordinal is not the server code: use {@link #code()} to get the
 * code the server uses for a constant, and {@link #fromCode(int)} for the
 * reverse.
 */
public enum RewardType {
    /** Unknown: an undocumented code (never sent to the server) */
    Unknown(-1),
    /** Cash reward (server code 0) */
    Cash(0),
    /** Gift (prize) (server code 1) */
    Gift(1),
    /** Coupon (server code 2) */
    Coupon(2),
    /** Task reward (server code 3) */
    Task(3);

    private final int code;

    RewardType(int code) {
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
    public static RewardType fromCode(int code) {
        for (RewardType value : values()) {
            if (value != Unknown && value.code == code) {
                return value;
            }
        }
        return Unknown;
    }
}
