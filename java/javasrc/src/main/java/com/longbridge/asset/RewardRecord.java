package com.longbridge.asset;

import java.math.BigDecimal;
import java.time.OffsetDateTime;

/**
 * A reward record from the reward center. Prize redemption secrets ({@code coupon_secret}), cash card codes ({@code cash_card_code}) and shipping contact details are not modelled.
 */
public class RewardRecord {
    private RewardType rewardType;
    private Integer rewardTypeRaw;
    private String recordId;
    private String activityTitle;
    private String title;
    private String description;
    private String cardType;
    private CouponStatus cardStatus;
    private Integer cardStatusRaw;
    private Integer prizeType;
    private BigDecimal amount;
    private String currency;
    private RewardStatus status;
    private Integer statusRaw;
    private RewardDeliverStatus deliverStatus;
    private Integer deliverStatusRaw;
    private int exitStatus;
    private OffsetDateTime sendTime;
    private OffsetDateTime startTime;
    private OffsetDateTime endTime;
    private OffsetDateTime effectiveTime;
    private OffsetDateTime receiveDeadline;

    /**
     * Returns the reward type: named after the first detail object the server
     * sent (cash, card, prize, in that order), else the server's reward type code.
     *
     * @return the reward type
     */
    public RewardType getRewardType() {
        return rewardType;
    }

    /**
     * The server's numeric code of reward_type, also when it is a code
     * this SDK does not know (reward_type is then Unknown); null when the
     * server sent none
     *
     * @return the raw code
     */
    public Integer getRewardTypeRaw() {
        return rewardTypeRaw;
    }

    /**
     * Returns the record ID.
     *
     * @return the record ID
     */
    public String getRecordId() {
        return recordId;
    }

    /**
     * Returns the title of the activity that granted the reward.
     *
     * @return the title of the activity that granted the reward
     */
    public String getActivityTitle() {
        return activityTitle;
    }

    /**
     * Returns the reward title.
     *
     * @return the reward title
     */
    public String getTitle() {
        return title;
    }

    /**
     * Returns the reward description.
     *
     * @return the reward description
     */
    public String getDescription() {
        return description;
    }

    /**
     * Returns the coupon type (coupon rewards), e.g. {@code commission}.
     *
     * @return the coupon type
     */
    public String getCardType() {
        return cardType;
    }

    /**
     * Returns the coupon status (coupon rewards; {@link CouponStatus#Unknown}
     * otherwise).
     *
     * @return the coupon status
     */
    public CouponStatus getCardStatus() {
        return cardStatus;
    }

    /**
     * The server's numeric code of card_status, also when it is a code
     * this SDK does not know (card_status is then Unknown); null when the
     * server sent none
     *
     * @return the raw code
     */
    public Integer getCardStatusRaw() {
        return cardStatusRaw;
    }

    /**
     * Returns the prize type code (gift rewards; {@code 0} = physical goods), {@code null} when not set.
     *
     * @return the prize type code
     */
    public Integer getPrizeType() {
        return prizeType;
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
     * Returns the reward status.
     *
     * @return the reward status
     */
    public RewardStatus getStatus() {
        return status;
    }

    /**
     * The server's numeric code of status, also when it is a code
     * this SDK does not know (status is then Unknown); null when the
     * server sent none
     *
     * @return the raw code
     */
    public Integer getStatusRaw() {
        return statusRaw;
    }

    /**
     * Returns the delivery status (issued gift rewards only).
     *
     * @return the delivery status
     */
    public RewardDeliverStatus getDeliverStatus() {
        return deliverStatus;
    }

    /**
     * The server's numeric code of deliver_status (e.g. 11 where deliver_status reads AddressFilled), also when it is a code
     * this SDK does not know (deliver_status is then Unknown); null when the
     * server sent none
     *
     * @return the raw code
     */
    public Integer getDeliverStatusRaw() {
        return deliverStatusRaw;
    }

    /**
     * Returns the exit status code, as returned by the server.
     *
     * @return the exit status code, as returned by the server
     */
    public int getExitStatus() {
        return exitStatus;
    }

    /**
     * Returns the time the reward was sent ({@code null} when not set).
     *
     * @return the time the reward was sent
     */
    public OffsetDateTime getSendTime() {
        return sendTime;
    }

    /**
     * Returns the start time ({@code null} when not set).
     *
     * @return the start time
     */
    public OffsetDateTime getStartTime() {
        return startTime;
    }

    /**
     * Returns the end time ({@code null} when not set).
     *
     * @return the end time
     */
    public OffsetDateTime getEndTime() {
        return endTime;
    }

    /**
     * Returns the effective time ({@code null} when not set).
     *
     * @return the effective time
     */
    public OffsetDateTime getEffectiveTime() {
        return effectiveTime;
    }

    /**
     * Returns the deadline to claim the reward ({@code null} when not set).
     *
     * @return the deadline to claim the reward
     */
    public OffsetDateTime getReceiveDeadline() {
        return receiveDeadline;
    }

    @Override
    public String toString() {
        return "RewardRecord [" + "rewardType=" + rewardType + ", " + "rewardTypeRaw=" + rewardTypeRaw + ", " + "recordId=" + recordId + ", " + "activityTitle=" + activityTitle + ", " + "title=" + title + ", " + "description=" + description + ", " + "cardType=" + cardType + ", " + "cardStatus=" + cardStatus + ", " + "cardStatusRaw=" + cardStatusRaw + ", " + "prizeType=" + prizeType + ", " + "amount=" + amount + ", " + "currency=" + currency + ", " + "status=" + status + ", " + "statusRaw=" + statusRaw + ", " + "deliverStatus=" + deliverStatus + ", " + "deliverStatusRaw=" + deliverStatusRaw + ", " + "exitStatus=" + exitStatus + ", " + "sendTime=" + sendTime + ", " + "startTime=" + startTime + ", " + "endTime=" + endTime + ", " + "effectiveTime=" + effectiveTime + ", " + "receiveDeadline=" + receiveDeadline + "]";
    }
}
