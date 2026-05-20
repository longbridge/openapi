package com.longbridge.portfolio;

import java.math.BigDecimal;

/** Detailed P&amp;L breakdown for one asset class. */
public class ProfitDetails {
    /** Current holding market value. */
    public BigDecimal holdingValue;
    /** Total profit/loss. */
    public BigDecimal profit;
    /** Cumulative credited amount. */
    public BigDecimal cumulativeCreditedAmount;
    /** Credit detail entries. */
    public ProfitDetailEntry[] creditedDetails;
    /** Cumulative debited amount. */
    public BigDecimal cumulativeDebitedAmount;
    /** Debit detail entries. */
    public ProfitDetailEntry[] debitedDetails;
    /** Cumulative fee amount. */
    public BigDecimal cumulativeFeeAmount;
    /** Fee detail entries. */
    public ProfitDetailEntry[] feeDetails;
    /** Short position holding value. */
    public BigDecimal shortHoldingValue;
    /** Long position holding value. */
    public BigDecimal longHoldingValue;
    /** Opening position market value at period start. */
    public BigDecimal holdingValueAtBeginning;
    /** Closing position market value at period end. */
    public BigDecimal holdingValueAtEnding;
}
