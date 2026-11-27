package com.longbridge.asset;

/**
 * Which upstream detail object a reward record came from
 */
public enum RewardDetailKind {
    /** No known detail object (e.g. a task reward); only the reward type is set */
    None,
    /** The {@code cash} object */
    Cash,
    /** The {@code card} (coupon) object */
    Card,
    /** The {@code prize} (gift) object */
    Prize,
}
