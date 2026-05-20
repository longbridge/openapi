package com.longbridge.portfolio;

import java.math.BigDecimal;

/** Response for {@link PortfolioContext#getProfitAnalysisDetail}. */
public class ProfitAnalysisDetail {
    /** Total profit/loss. */
    public BigDecimal profit;
    /** Underlying stock P&amp;L details. */
    public ProfitDetails underlyingDetails;
    /** Derivative P&amp;L details. */
    public ProfitDetails derivativePnlDetails;
    /** Security name. */
    public String name;
    /** Last updated time (unix timestamp string). */
    public String updatedAt;
    /** Last updated date string. */
    public String updatedDate;
    /** Currency. */
    public String currency;
    /** Default detail tab: 0 = underlying, 1 = derivative. */
    public int defaultTag;
    /** Query start time (unix timestamp string). */
    public String start;
    /** Query end time (unix timestamp string). */
    public String end;
    /** Query start date string. */
    public String startDate;
    /** Query end date string. */
    public String endDate;
}
