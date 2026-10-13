package com.longbridge.fund;

/**
 * FundDatedValue
 */
public class FundDatedValue {
    private long date;
    private String value;

    /**
     * Returns date.
     *
     * @return date
     */
    public long getDate() {
        return date;
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
        return                 "FundDatedValue [date=" + date +
                ", value=" + value + "]";
    }
}
