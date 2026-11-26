package com.longbridge.asset;

/**
 * Status of a coupon
 */
public enum CouponStatus {
    /** Unknown (an undocumented code) */
    Unknown,
    /** Unused */
    Unused,
    /** Used */
    Used,
    /** Expired */
    Expired,
    /** Being redeemed */
    Redeeming,
    /** In use */
    InUse,
    /** Processing */
    Processing,
}
