package com.longbridge.asset;

/**
 * State of a withdrawal record
 */
public enum WithdrawalState {
    /** Unknown (an undocumented code) */
    Unknown,
    /** Submitted */
    Submitted,
    /** Processing */
    Processing,
    /** Rejected */
    Rejected,
    /** Cancelled */
    Cancelled,
    /** Remitted */
    Remitted,
    /** Failed */
    Failed,
    /** Refunded */
    Refunded,
    /** Approved, awaiting payout */
    ApprovedAwaitingPayout,
    /** Large-amount review */
    LargeAmountReview,
    /** Change pending approval */
    ChangePendingApproval,
    /** Deletion pending approval */
    DeletionPendingApproval,
    /** Deleted */
    Deleted,
    /** Rejection pending approval */
    RejectionPendingApproval,
    /** Awaiting confirmation */
    AwaitingConfirmation,
}
