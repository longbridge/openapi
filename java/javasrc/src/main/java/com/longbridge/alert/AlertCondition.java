package com.longbridge.alert;

/** Alert trigger condition. */
public enum AlertCondition {
    /** Price rises above the trigger value */
    PriceRise,
    /** Price falls below the trigger value */
    PriceFall,
    /** Price rises by the given percentage */
    PercentRise,
    /** Price falls by the given percentage */
    PercentFall,
}
