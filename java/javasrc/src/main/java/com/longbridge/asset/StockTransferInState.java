package com.longbridge.asset;

/**
 * State of a stock transfer-in application
 *
 * <p>
 * The Java ordinal is not the server code: use {@link #code()} to get the
 * code the server uses for a constant, and {@link #fromCode(int)} for the
 * reverse.
 */
public enum StockTransferInState {
    /** Unknown: an undocumented code (never sent to the server) */
    Unknown(-1),
    /** Submitted (server code 0) */
    Submitted(0),
    /** Cancelled (server code 1) */
    Cancelled(1),
    /** Completed (server code 3) */
    Completed(3),
    /** Processing (server code 5) */
    Processing(5),
    /** Partially completed (server code 7) */
    PartiallyCompleted(7),
    /** Failed (server code 8) */
    Failed(8);

    private final int code;

    StockTransferInState(int code) {
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
    public static StockTransferInState fromCode(int code) {
        for (StockTransferInState value : values()) {
            if (value != Unknown && value.code == code) {
                return value;
            }
        }
        return Unknown;
    }
}
