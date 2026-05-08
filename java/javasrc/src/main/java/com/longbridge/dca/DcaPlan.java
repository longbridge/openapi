package com.longbridge.dca;

import java.math.BigDecimal;

/** One DCA (dollar-cost averaging) investment plan. */
public class DcaPlan {
    /** Plan ID. */
    public String planId;
    /** Status: {@code "Active"}, {@code "Suspended"}, or {@code "Finished"}. */
    public String status;
    /** Security symbol. */
    public String symbol;
    /** Member ID. */
    public String memberId;
    /** Account ID. */
    public String aaid;
    /** Account channel. */
    public String accountChannel;
    /** Display account. */
    public String displayAccount;
    /** Market. */
    public String market;
    /** Investment amount per period. */
    public BigDecimal perInvestAmount;
    /** Frequency: {@code "Daily"}, {@code "Weekly"}, or {@code "Monthly"}. */
    public String investFrequency;
    /** Day of week for weekly plans, e.g. {@code "Mon"}. */
    public String investDayOfWeek;
    /** Day of month for monthly plans. */
    public String investDayOfMonth;
    /** Whether margin finance is allowed. */
    public boolean allowMarginFinance;
    /** Reminder time. */
    public String alterHours;
    /** Creation time. */
    public String createdAt;
    /** Last updated time. */
    public String updatedAt;
    /** Next investment date. */
    public String nextTrdDate;
    /** Security name. */
    public String stockName;
    /** Cumulative invested amount. */
    public BigDecimal cumAmount;
    /** Number of completed investment periods. */
    public long issueNumber;
    /** Average cost. */
    public BigDecimal averageCost;
    /** Cumulative profit/loss. */
    public BigDecimal cumProfit;
}
