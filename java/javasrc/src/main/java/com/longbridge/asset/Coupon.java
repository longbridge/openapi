package com.longbridge.asset;

import java.math.BigDecimal;
import java.time.OffsetDateTime;
import java.util.Arrays;

/**
 * A coupon (reward card)
 */
public class Coupon {
    private String cardId;
    private String cardType;
    private String title;
    private String titleSec;
    private String description;
    private String detail;
    private CouponStatus cardStatus;
    private Integer cardStatusRaw;
    private OffsetDateTime startTime;
    private OffsetDateTime endTime;
    private boolean noExpiry;
    private OffsetDateTime activateTime;
    private BigDecimal amount;
    private String currency;
    private String availableTradeSession;
    private String[] tradeSessions;
    private String subscriptionId;
    private CouponAiBenefit aiBenefit;
    private CouponCash cash;
    private CouponCoffee coffeeCoupon;
    private CouponCommission commission;
    private CouponCrypto crypto;
    private CouponFinanceInterest financeInterest;
    private CouponFundDeduct fundDeduct;
    private CouponQuote fundamental;
    private CouponIpo ipo;
    private CouponIpoCommission ipoCommission;
    private CouponIpoInterest ipoInterest;
    private CouponPlatformFeeDeduct platformFeeDeduct;
    private CouponQuote quote;
    private CouponStock stock;
    private CouponStockFragment stockFragment;

    /**
     * Returns the coupon ID (32 hex characters; pass to coupon_orders).
     *
     * @return the coupon ID
     */
    public String getCardId() {
        return cardId;
    }

    /**
     * Returns the coupon type, e.g. {@code commission}, {@code platform_fee_deduct}, {@code stock}, {@code quote}, {@code cash}.
     *
     * @return the coupon type, e.g. {@code commission}, {@code platform_fee_deduct}, {@code stock}, {@code quote}, {@code cash}
     */
    public String getCardType() {
        return cardType;
    }

    /**
     * Returns the title.
     *
     * @return the title
     */
    public String getTitle() {
        return title;
    }

    /**
     * Returns the secondary title.
     *
     * @return the secondary title
     */
    public String getTitleSec() {
        return titleSec;
    }

    /**
     * Returns the description.
     *
     * @return the description
     */
    public String getDescription() {
        return description;
    }

    /**
     * Returns the terms and details.
     *
     * @return the terms and details
     */
    public String getDetail() {
        return detail;
    }

    /**
     * Returns the status.
     *
     * @return the status
     */
    public CouponStatus getCardStatus() {
        return cardStatus;
    }

    /**
     * The server's numeric code of card_status, also when it is a code
     * this SDK does not know (card_status is then Unknown); null when the
     * value is missing or is not a code
     *
     * @return the raw code
     */
    public Integer getCardStatusRaw() {
        return cardStatusRaw;
    }

    /**
     * Returns the start of validity ({@code null} when not set).
     *
     * @return the start of validity
     */
    public OffsetDateTime getStartTime() {
        return startTime;
    }

    /**
     * Returns the end of validity (none when not set or when the coupon never expires; see {@code no_expiry}).
     *
     * @return the end of validity
     */
    public OffsetDateTime getEndTime() {
        return endTime;
    }

    /**
     * Returns the whether the coupon never expires (the server's end time is in or after 2100).
     *
     * @return the whether the coupon never expires
     */
    public boolean getNoExpiry() {
        return noExpiry;
    }

    /**
     * Returns the activation time (none when not activated).
     *
     * @return the activation time
     */
    public OffsetDateTime getActivateTime() {
        return activateTime;
    }

    /**
     * Returns the amount ({@code null} when not set).
     *
     * @return the amount
     */
    public BigDecimal getAmount() {
        return amount;
    }

    /**
     * Returns the currency.
     *
     * @return the currency
     */
    public String getCurrency() {
        return currency;
    }

    /**
     * Returns the trading sessions the coupon applies to, as display text.
     *
     * @return the trading sessions the coupon applies to, as display text
     */
    public String getAvailableTradeSession() {
        return availableTradeSession;
    }

    /**
     * Returns the trading sessions the coupon applies to, as returned by the server.
     *
     * @return the trading sessions the coupon applies to, as returned by the server
     */
    public String[] getTradeSessions() {
        return tradeSessions;
    }

    /**
     * Returns the subscription ID, as returned by the server.
     *
     * @return the subscription ID, as returned by the server
     */
    public String getSubscriptionId() {
        return subscriptionId;
    }

    /**
     * Returns the terms of an AI benefit coupon ({@code null} for other coupon types).
     *
     * @return the terms of an AI benefit coupon
     */
    public CouponAiBenefit getAiBenefit() {
        return aiBenefit;
    }

    /**
     * Returns the terms of a cash coupon ({@code null} for other coupon types).
     *
     * @return the terms of a cash coupon
     */
    public CouponCash getCash() {
        return cash;
    }

    /**
     * Returns the terms of a coffee coupon ({@code null} for other coupon types).
     *
     * @return the terms of a coffee coupon
     */
    public CouponCoffee getCoffeeCoupon() {
        return coffeeCoupon;
    }

    /**
     * Returns the terms of a commission coupon ({@code null} for other coupon types).
     *
     * @return the terms of a commission coupon
     */
    public CouponCommission getCommission() {
        return commission;
    }

    /**
     * Returns the terms of a crypto coupon ({@code null} for other coupon types).
     *
     * @return the terms of a crypto coupon
     */
    public CouponCrypto getCrypto() {
        return crypto;
    }

    /**
     * Returns the terms of a finance interest coupon ({@code null} for other coupon types).
     *
     * @return the terms of a finance interest coupon
     */
    public CouponFinanceInterest getFinanceInterest() {
        return financeInterest;
    }

    /**
     * Returns the terms of a fund deduction coupon ({@code null} for other coupon types).
     *
     * @return the terms of a fund deduction coupon
     */
    public CouponFundDeduct getFundDeduct() {
        return fundDeduct;
    }

    /**
     * Returns the terms of a fundamental-data coupon ({@code null} for other coupon types).
     *
     * @return the terms of a fundamental-data coupon
     */
    public CouponQuote getFundamental() {
        return fundamental;
    }

    /**
     * Returns the terms of an IPO coupon ({@code null} for other coupon types).
     *
     * @return the terms of an IPO coupon
     */
    public CouponIpo getIpo() {
        return ipo;
    }

    /**
     * Returns the terms of an IPO commission coupon ({@code null} for other coupon types).
     *
     * @return the terms of an IPO commission coupon
     */
    public CouponIpoCommission getIpoCommission() {
        return ipoCommission;
    }

    /**
     * Returns the terms of an IPO interest coupon ({@code null} for other coupon types).
     *
     * @return the terms of an IPO interest coupon
     */
    public CouponIpoInterest getIpoInterest() {
        return ipoInterest;
    }

    /**
     * Returns the terms of a platform fee deduction coupon ({@code null} for other coupon types).
     *
     * @return the terms of a platform fee deduction coupon
     */
    public CouponPlatformFeeDeduct getPlatformFeeDeduct() {
        return platformFeeDeduct;
    }

    /**
     * Returns the terms of a quote (market data) coupon ({@code null} for other coupon types).
     *
     * @return the terms of a quote
     */
    public CouponQuote getQuote() {
        return quote;
    }

    /**
     * Returns the terms of a stock coupon ({@code null} for other coupon types).
     *
     * @return the terms of a stock coupon
     */
    public CouponStock getStock() {
        return stock;
    }

    /**
     * Returns the terms of a stock fragment coupon ({@code null} for other coupon types).
     *
     * @return the terms of a stock fragment coupon
     */
    public CouponStockFragment getStockFragment() {
        return stockFragment;
    }

    @Override
    public String toString() {
        return "Coupon [" + "cardId=" + cardId + ", " + "cardType=" + cardType + ", " + "title=" + title + ", " + "titleSec=" + titleSec + ", " + "description=" + description + ", " + "detail=" + detail + ", " + "cardStatus=" + cardStatus + ", " + "cardStatusRaw=" + cardStatusRaw + ", " + "startTime=" + startTime + ", " + "endTime=" + endTime + ", " + "noExpiry=" + noExpiry + ", " + "activateTime=" + activateTime + ", " + "amount=" + amount + ", " + "currency=" + currency + ", " + "availableTradeSession=" + availableTradeSession + ", " + "tradeSessions=" + Arrays.toString(tradeSessions) + ", " + "subscriptionId=" + subscriptionId + ", " + "aiBenefit=" + aiBenefit + ", " + "cash=" + cash + ", " + "coffeeCoupon=" + coffeeCoupon + ", " + "commission=" + commission + ", " + "crypto=" + crypto + ", " + "financeInterest=" + financeInterest + ", " + "fundDeduct=" + fundDeduct + ", " + "fundamental=" + fundamental + ", " + "ipo=" + ipo + ", " + "ipoCommission=" + ipoCommission + ", " + "ipoInterest=" + ipoInterest + ", " + "platformFeeDeduct=" + platformFeeDeduct + ", " + "quote=" + quote + ", " + "stock=" + stock + ", " + "stockFragment=" + stockFragment + "]";
    }
}
