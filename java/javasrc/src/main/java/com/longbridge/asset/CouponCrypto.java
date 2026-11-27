package com.longbridge.asset;

import java.util.Arrays;

/**
 * Terms of a crypto coupon ({@code crypto})
 */
public class CouponCrypto {
    private CouponCryptoFallback fallback;
    private CouponCryptoCounter[] optionalCounters;

    /**
     * Returns the fallback.
     *
     * @return the fallback
     */
    public CouponCryptoFallback getFallback() {
        return fallback;
    }

    /**
     * Returns the optional counters.
     *
     * @return the optional counters
     */
    public CouponCryptoCounter[] getOptionalCounters() {
        return optionalCounters;
    }

    @Override
    public String toString() {
        return "CouponCrypto [" + "fallback=" + fallback + ", " + "optionalCounters=" + Arrays.toString(optionalCounters) + "]";
    }
}
