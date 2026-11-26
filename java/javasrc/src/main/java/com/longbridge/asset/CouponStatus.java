package com.longbridge.asset;

/**
 * Status of a coupon
 *
 * <p>
 * The Java ordinal is not the server code: use {@link #code()} to get the
 * code the server uses for a constant, and {@link #fromCode(int)} for the
 * reverse.
 */
public enum CouponStatus {
    /** Unknown: an undocumented code (never sent to the server) */
    Unknown(-1),
    /** Unused (server code 0) */
    Unused(0),
    /** Used (server code 1) */
    Used(1),
    /** Expired (server code 2) */
    Expired(2),
    /** Being redeemed (server code 3) */
    Redeeming(3),
    /** In use (server code 4) */
    InUse(4),
    /** Processing (server code 5) */
    Processing(5);

    private final int code;

    CouponStatus(int code) {
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
    public static CouponStatus fromCode(int code) {
        for (CouponStatus value : values()) {
            if (value != Unknown && value.code == code) {
                return value;
            }
        }
        return Unknown;
    }
}
