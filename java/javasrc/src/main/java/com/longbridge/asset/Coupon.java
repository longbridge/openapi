package com.longbridge.asset;

import java.math.BigDecimal;
import java.time.OffsetDateTime;

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
    private OffsetDateTime startTime;
    private OffsetDateTime endTime;
    private OffsetDateTime activateTime;
    private BigDecimal amount;
    private String currency;
    private String availableTradeSession;
    private CouponCommission commission;

    /**
     * Returns the coupon ID (32 hex characters; pass to {@link AssetContext#getCouponOrders}).
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
     * Returns the start of validity ({@code null} when not set).
     *
     * @return the start of validity
     */
    public OffsetDateTime getStartTime() {
        return startTime;
    }

    /**
     * Returns the end of validity ({@code null} when not set); a date after 2100 means the coupon never expires.
     *
     * @return the end of validity
     */
    public OffsetDateTime getEndTime() {
        return endTime;
    }

    /**
     * Returns the activation time ({@code null} when not activated).
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
     * Returns the commission terms (commission coupons only; empty otherwise).
     *
     * @return the commission terms
     */
    public CouponCommission getCommission() {
        return commission;
    }

    @Override
    public String toString() {
        return "Coupon [" + "cardId=" + cardId + ", " + "cardType=" + cardType + ", " + "title=" + title + ", " + "titleSec=" + titleSec + ", " + "description=" + description + ", " + "detail=" + detail + ", " + "cardStatus=" + cardStatus + ", " + "startTime=" + startTime + ", " + "endTime=" + endTime + ", " + "activateTime=" + activateTime + ", " + "amount=" + amount + ", " + "currency=" + currency + ", " + "availableTradeSession=" + availableTradeSession + ", " + "commission=" + commission + "]";
    }
}
