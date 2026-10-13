package com.longbridge.fund;

/**
 * FundOrderSubmitResponse
 */
public class FundOrderSubmitResponse {
    private String action;
    private String amount;
    private String counterId;
    private long createdAt;
    private String fundName;
    private long id;
    private String msg;
    private int status;
    private String units;

    /**
     * Returns action.
     *
     * @return action
     */
    public String getAction() {
        return action;
    }

    /**
     * Returns amount.
     *
     * @return amount
     */
    public String getAmount() {
        return amount;
    }

    /**
     * Returns counterId.
     *
     * @return counterId
     */
    public String getCounterId() {
        return counterId;
    }

    /**
     * Returns createdAt.
     *
     * @return createdAt
     */
    public long getCreatedAt() {
        return createdAt;
    }

    /**
     * Returns fundName.
     *
     * @return fundName
     */
    public String getFundName() {
        return fundName;
    }

    /**
     * Returns id.
     *
     * @return id
     */
    public long getId() {
        return id;
    }

    /**
     * Returns msg.
     *
     * @return msg
     */
    public String getMsg() {
        return msg;
    }

    /**
     * Returns status.
     *
     * @return status
     */
    public int getStatus() {
        return status;
    }

    /**
     * Returns units.
     *
     * @return units
     */
    public String getUnits() {
        return units;
    }

    @Override
    public String toString() {
        return                 "FundOrderSubmitResponse [action=" + action +
                ", amount=" + amount +
                ", counterId=" + counterId +
                ", createdAt=" + createdAt +
                ", fundName=" + fundName +
                ", id=" + id +
                ", msg=" + msg +
                ", status=" + status +
                ", units=" + units + "]";
    }
}
