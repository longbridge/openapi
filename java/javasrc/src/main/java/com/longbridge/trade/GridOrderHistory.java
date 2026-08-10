package com.longbridge.trade;

import java.time.OffsetDateTime;

/**
 * A grid order lifecycle-history entry
 */
public class GridOrderHistory {
    private String historyId;
    private OffsetDateTime createdAt;
    private String status;
    private String suspendReason;
    private String reason;

    /**
     * Returns historyId.
     *
     * @return historyId
     */
    public String getHistoryId() {
        return historyId;
    }

    /**
     * Returns createdAt.
     *
     * @return createdAt
     */
    public OffsetDateTime getCreatedAt() {
        return createdAt;
    }

    /**
     * Returns status.
     *
     * @return status
     */
    public String getStatus() {
        return status;
    }

    /**
     * Returns suspendReason.
     *
     * @return suspendReason
     */
    public String getSuspendReason() {
        return suspendReason;
    }

    /**
     * Returns reason.
     *
     * @return reason
     */
    public String getReason() {
        return reason;
    }

    @Override
    public String toString() {
        return "GridOrderHistory [historyId=" + historyId +
                ", createdAt=" + createdAt +
                ", status=" + status +
                ", suspendReason=" + suspendReason +
                ", reason=" + reason + "]";
    }
}
