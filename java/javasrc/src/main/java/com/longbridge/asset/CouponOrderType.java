package com.longbridge.asset;

/**
 * Which orders of a coupon to list
 */
public enum CouponOrderType {
    /** Orders the coupon is locked to (in progress) */
    Locked,
    /** Orders whose rebate has been settled */
    Settled,
}
