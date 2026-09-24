package com.longbridge.fund;

/**
 * Options for GetFundAnalysisOptions.
 */
@SuppressWarnings("unused")
public class GetFundAnalysisOptions {
    private Integer period;

    /**
     * Sets period.
     *
     * @param period period
     * @return this instance for chaining
     */
    public GetFundAnalysisOptions setPeriod(Integer period) {
        this.period = period;
        return this;
    }

}
