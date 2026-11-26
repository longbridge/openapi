package com.longbridge.asset;

/**
 * State of a stock transfer-out application
 *
 * <p>
 * The Java ordinal is not the server code: use {@link #code()} to get the
 * code the server uses for a constant, and {@link #fromCode(int)} for the
 * reverse.
 */
public enum StockTransferOutState {
    /** Unknown: an undocumented code (never sent to the server) */
    Unknown(-1),
    /** Submitted (the only state in which the application can still be cancelled) (server code 0) */
    Submitted(0),
    /** Processing (server code 1) */
    Processing(1),
    /** Executing (server code 2) */
    Executing(2),
    /** Completed (server code 3) */
    Completed(3),
    /** Cancelled (server code 4) */
    Cancelled(4),
    /** Partially completed (server code 5) */
    PartiallyCompleted(5),
    /** Failed (server code 6) */
    Failed(6),
    /** Rejected (server code 7) */
    Rejected(7);

    private final int code;

    StockTransferOutState(int code) {
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
    public static StockTransferOutState fromCode(int code) {
        for (StockTransferOutState value : values()) {
            if (value != Unknown && value.code == code) {
                return value;
            }
        }
        return Unknown;
    }
}
