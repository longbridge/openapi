package com.longbridge.fund;

/**
 * Options for FundNavRangeOptions.
 */
@SuppressWarnings("unused")
public class FundNavRangeOptions {
    private Integer monthBefore;
    private Integer yearBefore;

    /**
     * Sets monthBefore.
     *
     * @param monthBefore monthBefore
     * @return this instance for chaining
     */
    public FundNavRangeOptions setMonthBefore(Integer monthBefore) {
        this.monthBefore = monthBefore;
        return this;
    }

    /**
     * Sets yearBefore.
     *
     * @param yearBefore yearBefore
     * @return this instance for chaining
     */
    public FundNavRangeOptions setYearBefore(Integer yearBefore) {
        this.yearBefore = yearBefore;
        return this;
    }

}
