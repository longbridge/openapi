package com.longbridge.asset;

/**
 * Status of a reward record
 */
public enum RewardStatus {
    /** Unknown (an undocumented code) */
    Unknown,
    /** Upcoming */
    Upcoming,
    /** Cancelled */
    Cancelled,
    /** The user exited the activity */
    ExitedActivity,
    /** Not claimed yet */
    Unclaimed,
    /** Being issued */
    Issuing,
    /** Issued */
    Issued,
    /** Issuing failed */
    Failed,
    /** Exchanged */
    Exchanged,
    /** Expired */
    Expired,
    /** Refunded */
    Refunded,
}
