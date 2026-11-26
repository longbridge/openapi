package com.longbridge.asset;

/**
 * Delivery status of an issued prize
 *
 * <p>
 * The Java ordinal is not the server code: use {@link #code()} to get the
 * code the server uses for a constant, and {@link #fromCode(int)} for the
 * reverse.
 */
public enum RewardDeliverStatus {
    /** Unknown: an undocumented code, or no delivery status (never sent to the server) */
    Unknown(-1),
    /** Not activated (server code 0) */
    NotActivated(0),
    /** Activated, or shipping (server code 1) */
    ActivatedOrShipping(1),
    /** Consumed (server code 2) */
    Consumed(2),
    /** E-voucher activating (server code 5) */
    EVoucherActivating(5),
    /** E-voucher expired before activation (server code 6) */
    EVoucherExpired(6),
    /** E-voucher invalid (server code 7) */
    EVoucherInvalid(7),
    /** Shipping address missing (server code 8) */
    AddressMissing(8),
    /** Shipping address filled (server code 9; the server also uses 11) */
    AddressFilled(9),
    /** Shipping (server code 10) */
    Shipping(10);

    private final int code;

    RewardDeliverStatus(int code) {
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
    public static RewardDeliverStatus fromCode(int code) {
        if (code == 11) {
            return AddressFilled;
        }
        for (RewardDeliverStatus value : values()) {
            if (value != Unknown && value.code == code) {
                return value;
            }
        }
        return Unknown;
    }
}
