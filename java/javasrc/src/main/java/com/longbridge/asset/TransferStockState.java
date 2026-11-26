package com.longbridge.asset;

/**
 * State of one stock inside a stock transfer application
 *
 * <p>
 * The Java ordinal is not the server code: use {@link #code()} to get the
 * code the server uses for a constant, and {@link #fromCode(int)} for the
 * reverse.
 */
public enum TransferStockState {
    /** Unknown: an undocumented code (never sent to the server) */
    Unknown(-1),
    /** Processing (server code 0) */
    Processing(0),
    /** Succeeded (server code 1) */
    Succeeded(1),
    /** Failed (server code 2) */
    Failed(2);

    private final int code;

    TransferStockState(int code) {
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
    public static TransferStockState fromCode(int code) {
        for (TransferStockState value : values()) {
            if (value != Unknown && value.code == code) {
                return value;
            }
        }
        return Unknown;
    }
}
