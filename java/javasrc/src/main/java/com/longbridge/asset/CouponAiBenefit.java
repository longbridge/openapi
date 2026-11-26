package com.longbridge.asset;

import java.time.OffsetDateTime;

/**
 * Terms of an AI benefit coupon ({@code ai_benefit})
 */
public class CouponAiBenefit {
    private int benefitDays;
    private OffsetDateTime benefitStart;
    private OffsetDateTime benefitEnd;
    private String benefitTag;

    /**
     * Returns the benefit days, as returned by the server.
     *
     * @return the benefit days, as returned by the server
     */
    public int getBenefitDays() {
        return benefitDays;
    }

    /**
     * Returns the start of the benefit ({@code null} when not set).
     *
     * @return the start of the benefit
     */
    public OffsetDateTime getBenefitStart() {
        return benefitStart;
    }

    /**
     * Returns the end of the benefit ({@code null} when not set).
     *
     * @return the end of the benefit
     */
    public OffsetDateTime getBenefitEnd() {
        return benefitEnd;
    }

    /**
     * Returns the benefit tag, as returned by the server.
     *
     * @return the benefit tag, as returned by the server
     */
    public String getBenefitTag() {
        return benefitTag;
    }

    @Override
    public String toString() {
        return "CouponAiBenefit [" + "benefitDays=" + benefitDays + ", " + "benefitStart=" + benefitStart + ", " + "benefitEnd=" + benefitEnd + ", " + "benefitTag=" + benefitTag + "]";
    }
}
