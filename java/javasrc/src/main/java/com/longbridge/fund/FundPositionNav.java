package com.longbridge.fund;

/**
 * FundPositionNav
 */
public class FundPositionNav {
    private String change;
    private String changePercent;
    private String counterId;
    private String counterName;
    private long lastUpdateTime;
    private String value;

    /**
     * Returns change.
     *
     * @return change
     */
    public String getChange() {
        return change;
    }

    /**
     * Returns changePercent.
     *
     * @return changePercent
     */
    public String getChangePercent() {
        return changePercent;
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
     * Returns counterName.
     *
     * @return counterName
     */
    public String getCounterName() {
        return counterName;
    }

    /**
     * Returns lastUpdateTime.
     *
     * @return lastUpdateTime
     */
    public long getLastUpdateTime() {
        return lastUpdateTime;
    }

    /**
     * Returns value.
     *
     * @return value
     */
    public String getValue() {
        return value;
    }

    @Override
    public String toString() {
        return                 "FundPositionNav [change=" + change +
                ", changePercent=" + changePercent +
                ", counterId=" + counterId +
                ", counterName=" + counterName +
                ", lastUpdateTime=" + lastUpdateTime +
                ", value=" + value + "]";
    }
}
