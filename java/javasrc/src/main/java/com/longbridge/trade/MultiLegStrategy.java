package com.longbridge.trade;

/**
 * Multi-leg strategy
 */
public enum MultiLegStrategy {
    /** Unknown */
    Unknown,
    /** Covered call (covered stock) */
    CoveredCall,
    /** Covered put (covered stock) */
    CoveredPut,
    /** Vertical call spread */
    VerticalCallSpread,
    /** Vertical put spread */
    VerticalPutSpread,
    /** Collar */
    Collar,
    /** Straddle */
    Straddle,
    /** Strangle */
    Strangle,
}
