package com.longbridge.asset;

/**
 * IPO subscription type an IPO commission coupon applies to
 *
 * <p>
 * The Java ordinal is not the server code: use {@link #code()} to get the
 * code the server uses for a constant, and {@link #fromCode(int)} for the
 * reverse.
 */
public enum IpoCouponTargetType {
    /** Unknown: unset or an undocumented code (never sent to the server) */
    Unknown(-1),
    /** Financing (margin) IPO subscription (server code 1, {@code FinancingIPO}) */
    Financing(1),
    /** Cash IPO subscription (server code 2, {@code CashIPO}) */
    Cash(2);

    private final int code;

    IpoCouponTargetType(int code) {
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
    public static IpoCouponTargetType fromCode(int code) {
        for (IpoCouponTargetType value : values()) {
            if (value != Unknown && value.code == code) {
                return value;
            }
        }
        return Unknown;
    }
}
