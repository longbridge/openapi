package com.longbridge.asset;

/**
 * State of a withdrawal record
 *
 * <p>
 * The Java ordinal is not the server code: use {@link #code()} to get the
 * code the server uses for a constant, and {@link #fromCode(int)} for the
 * reverse.
 */
public enum WithdrawalState {
    /** Unknown: an undocumented code (never sent to the server) */
    Unknown(-1),
    /** Submitted (server code 0) */
    Submitted(0),
    /** Processing (server code 1) */
    Processing(1),
    /** Rejected (server code 2) */
    Rejected(2),
    /** Cancelled (server code 3) */
    Cancelled(3),
    /** Remitted (server code 4) */
    Remitted(4),
    /** Failed (server code 5) */
    Failed(5),
    /** Refunded (server code 6) */
    Refunded(6),
    /** Approved, awaiting payout (server code 7) */
    ApprovedAwaitingPayout(7),
    /** Large-amount review (server code 8) */
    LargeAmountReview(8),
    /** Change pending approval (server code 9) */
    ChangePendingApproval(9),
    /** Deletion pending approval (server code 10) */
    DeletionPendingApproval(10),
    /** Deleted (server code 11) */
    Deleted(11),
    /** Rejection pending approval (server code 12) */
    RejectionPendingApproval(12),
    /** Awaiting confirmation (server code 15) */
    AwaitingConfirmation(15);

    private final int code;

    WithdrawalState(int code) {
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
    public static WithdrawalState fromCode(int code) {
        for (WithdrawalState value : values()) {
            if (value != Unknown && value.code == code) {
                return value;
            }
        }
        return Unknown;
    }
}
