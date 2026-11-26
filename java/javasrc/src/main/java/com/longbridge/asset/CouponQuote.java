package com.longbridge.asset;

import java.time.OffsetDateTime;
import java.util.Arrays;

/**
 * Terms of a quote (market data) or fundamental-data coupon ({@code quote} / {@code fundamental})
 */
public class CouponQuote {
    private OffsetDateTime currentEndTime;
    private boolean inapplicable;
    private QuoteInvestorType investorType;
    private Integer investorTypeRaw;
    private boolean manualActivate;
    private String[] markets;
    private int memberInvestorType;
    private String pairSpuNo;
    private long quoteDuration;
    private String quoteMarket;
    private String[] quoteTags;
    private boolean superposition;

    /**
     * Returns the current end time ({@code null} when not set).
     *
     * @return the current end time
     */
    public OffsetDateTime getCurrentEndTime() {
        return currentEndTime;
    }

    /**
     * Returns the whether the coupon is inapplicable, as returned by the server.
     *
     * @return the whether the coupon is inapplicable, as returned by the server
     */
    public boolean getInapplicable() {
        return inapplicable;
    }

    /**
     * Returns the investor type the coupon applies to.
     *
     * @return the investor type the coupon applies to
     */
    public QuoteInvestorType getInvestorType() {
        return investorType;
    }

    /**
     * The server's numeric code of investor_type, also when it is a code
     * this SDK does not know (investor_type is then Unknown); null when the
     * server sent none
     *
     * @return the raw code
     */
    public Integer getInvestorTypeRaw() {
        return investorTypeRaw;
    }

    /**
     * Returns the whether the coupon is activated manually.
     *
     * @return the whether the coupon is activated manually
     */
    public boolean getManualActivate() {
        return manualActivate;
    }

    /**
     * Returns the markets.
     *
     * @return the markets
     */
    public String[] getMarkets() {
        return markets;
    }

    /**
     * Returns the investor type of the member, as returned by the server.
     *
     * @return the investor type of the member, as returned by the server
     */
    public int getMemberInvestorType() {
        return memberInvestorType;
    }

    /**
     * Returns the paired SPU number, as returned by the server.
     *
     * @return the paired SPU number, as returned by the server
     */
    public String getPairSpuNo() {
        return pairSpuNo;
    }

    /**
     * Returns the quote duration in seconds (the app reads 200 years or more as lifetime).
     *
     * @return the quote duration in seconds (the app reads 200 years or more as lifetime)
     */
    public long getQuoteDuration() {
        return quoteDuration;
    }

    /**
     * Returns the quote market.
     *
     * @return the quote market
     */
    public String getQuoteMarket() {
        return quoteMarket;
    }

    /**
     * Returns the quote tags.
     *
     * @return the quote tags
     */
    public String[] getQuoteTags() {
        return quoteTags;
    }

    /**
     * Returns the whether the coupon can be combined with other coupons.
     *
     * @return the whether the coupon can be combined with other coupons
     */
    public boolean getSuperposition() {
        return superposition;
    }

    @Override
    public String toString() {
        return "CouponQuote [" + "currentEndTime=" + currentEndTime + ", " + "inapplicable=" + inapplicable + ", " + "investorType=" + investorType + ", " + "investorTypeRaw=" + investorTypeRaw + ", " + "manualActivate=" + manualActivate + ", " + "markets=" + Arrays.toString(markets) + ", " + "memberInvestorType=" + memberInvestorType + ", " + "pairSpuNo=" + pairSpuNo + ", " + "quoteDuration=" + quoteDuration + ", " + "quoteMarket=" + quoteMarket + ", " + "quoteTags=" + Arrays.toString(quoteTags) + ", " + "superposition=" + superposition + "]";
    }
}
