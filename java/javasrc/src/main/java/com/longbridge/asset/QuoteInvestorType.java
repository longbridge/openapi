package com.longbridge.asset;

/**
 * Investor type a quote coupon applies to
 *
 * <p>
 * The Java ordinal is not the server code: use {@link #code()} to get the
 * code the server uses for a constant, and {@link #fromCode(int)} for the
 * reverse.
 */
public enum QuoteInvestorType {
    /** Unknown: an undocumented code (never sent to the server) */
    Unknown(-1),
    /** Any investor (server code 0, {@code INVESTOR_TYPE_ANY}) */
    Any(0),
    /** Non-professional investors (server code 1, {@code INVESTOR_TYPE_NON_PROFESSIONAL}) */
    NonProfessional(1),
    /** Professional investors (server code 2, {@code INVESTOR_TYPE_PROFESSIONAL}) */
    Professional(2);

    private final int code;

    QuoteInvestorType(int code) {
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
    public static QuoteInvestorType fromCode(int code) {
        for (QuoteInvestorType value : values()) {
            if (value != Unknown && value.code == code) {
                return value;
            }
        }
        return Unknown;
    }
}
