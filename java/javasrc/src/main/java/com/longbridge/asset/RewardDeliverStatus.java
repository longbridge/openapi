package com.longbridge.asset;

/**
 * Delivery status of an issued prize
 */
public enum RewardDeliverStatus {
    /** Unknown (an undocumented code, or no delivery status) */
    Unknown,
    /** Not activated */
    NotActivated,
    /** Activated, or shipping */
    ActivatedOrShipping,
    /** Consumed */
    Consumed,
    /** E-voucher activating */
    EVoucherActivating,
    /** E-voucher expired before activation */
    EVoucherExpired,
    /** E-voucher invalid */
    EVoucherInvalid,
    /** Shipping address missing */
    AddressMissing,
    /** Shipping address filled */
    AddressFilled,
    /** Shipping */
    Shipping,
}
