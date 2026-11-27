package com.longbridge.asset;

/**
 * Instrument type a coupon applies to
 *
 * <p>
 * The Java ordinal is not the server code: use {@link #code()} to get the
 * code the server uses for a constant, and {@link #fromCode(int)} for the
 * reverse.
 */
public enum CouponTargetType {
    /** Unknown: an undocumented code (never sent to the server) */
    Unknown(-1),
    /** Stocks (server code 0, {@code StockTrade}) */
    Stock(0),
    /** US stock options (server code 1, {@code UsStockOptions}) */
    UsOption(1),
    /** HK warrants (server code 2, {@code HkStockWarrant}) */
    HkWarrant(2),
    /** ETF (server code 3) */
    Etf(3),
    /** LOF (server code 4) */
    Lof(4);

    private final int code;

    CouponTargetType(int code) {
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
    public static CouponTargetType fromCode(int code) {
        for (CouponTargetType value : values()) {
            if (value != Unknown && value.code == code) {
                return value;
            }
        }
        return Unknown;
    }
}
