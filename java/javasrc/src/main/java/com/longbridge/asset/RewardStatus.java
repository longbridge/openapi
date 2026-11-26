package com.longbridge.asset;

/**
 * Status of a reward record
 *
 * <p>
 * The Java ordinal is not the server code: use {@link #code()} to get the
 * code the server uses for a constant, and {@link #fromCode(int)} for the
 * reverse.
 */
public enum RewardStatus {
    /** Unknown: an undocumented code (never sent to the server) */
    Unknown(-1),
    /** Upcoming (server code 0) */
    Upcoming(0),
    /** Cancelled (server code 1) */
    Cancelled(1),
    /** The user exited the activity (server code 2) */
    ExitedActivity(2),
    /** Not claimed yet (server code 8) */
    Unclaimed(8),
    /** Being issued (server code 10) */
    Issuing(10),
    /** Issued (server code 12) */
    Issued(12),
    /** Issuing failed (server code 13) */
    Failed(13),
    /** Exchanged (server code 16) */
    Exchanged(16),
    /** Expired (server code 17) */
    Expired(17),
    /** Refunded (server code 18) */
    Refunded(18);

    private final int code;

    RewardStatus(int code) {
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
    public static RewardStatus fromCode(int code) {
        for (RewardStatus value : values()) {
            if (value != Unknown && value.code == code) {
                return value;
            }
        }
        return Unknown;
    }
}
