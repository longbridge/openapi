package com.longbridge.asset;

import java.math.BigDecimal;
import java.time.OffsetDateTime;

/**
 * A reward record from the reward center. Redemption secrets, voucher codes and shipping contact details are not modelled.
 */
public class RewardRecord {
    private RewardType rewardType;
    private String recordId;
    private String activityTitle;
    private String title;
    private String description;
    private String cardType;
    private CouponStatus cardStatus;
    private Integer prizeType;
    private BigDecimal amount;
    private String currency;
    private RewardStatus status;
    private RewardDeliverStatus deliverStatus;
    private int exitStatus;
    private OffsetDateTime sendTime;
    private OffsetDateTime startTime;
    private OffsetDateTime endTime;
    private OffsetDateTime effectiveTime;
    private OffsetDateTime receiveDeadline;

    /**
     * Returns the reward type.
     *
     * @return the reward type
     */
    public RewardType getRewardType() {
        return rewardType;
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
     * Returns the delivery status (issued gift rewards only).
     *
     * @return the delivery status
     */
    public RewardDeliverStatus getDeliverStatus() {
        return deliverStatus;
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
        return "RewardRecord [" + "rewardType=" + rewardType + ", " + "recordId=" + recordId + ", " + "activityTitle=" + activityTitle + ", " + "title=" + title + ", " + "description=" + description + ", " + "cardType=" + cardType + ", " + "cardStatus=" + cardStatus + ", " + "prizeType=" + prizeType + ", " + "amount=" + amount + ", " + "currency=" + currency + ", " + "status=" + status + ", " + "deliverStatus=" + deliverStatus + ", " + "exitStatus=" + exitStatus + ", " + "sendTime=" + sendTime + ", " + "startTime=" + startTime + ", " + "endTime=" + endTime + ", " + "effectiveTime=" + effectiveTime + ", " + "receiveDeadline=" + receiveDeadline + "]";
    }
}
