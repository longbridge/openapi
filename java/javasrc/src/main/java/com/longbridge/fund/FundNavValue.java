package com.longbridge.fund;

/**
 * FundNavValue
 */
public class FundNavValue {
    private String change;
    private String changePercent;
    private String changePercentFormat;
    private String counterId;
    private String counterName;
    private String currency;
    private String dateFormat;
    private String isin;
    private long lastUpdateTime;
    private String value;
    private String valueFormat;

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
     * Returns changePercentFormat.
     *
     * @return changePercentFormat
     */
    public String getChangePercentFormat() {
        return changePercentFormat;
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
     * Returns currency.
     *
     * @return currency
     */
    public String getCurrency() {
        return currency;
    }

    /**
     * Returns dateFormat.
     *
     * @return dateFormat
     */
    public String getDateFormat() {
        return dateFormat;
    }

    /**
     * Returns isin.
     *
     * @return isin
     */
    public String getIsin() {
        return isin;
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

    /**
     * Returns valueFormat.
     *
     * @return valueFormat
     */
    public String getValueFormat() {
        return valueFormat;
    }

    @Override
    public String toString() {
        return                 "FundNavValue [change=" + change +
                ", changePercent=" + changePercent +
                ", changePercentFormat=" + changePercentFormat +
                ", counterId=" + counterId +
                ", counterName=" + counterName +
                ", currency=" + currency +
                ", dateFormat=" + dateFormat +
                ", isin=" + isin +
                ", lastUpdateTime=" + lastUpdateTime +
                ", value=" + value +
                ", valueFormat=" + valueFormat + "]";
    }
}
