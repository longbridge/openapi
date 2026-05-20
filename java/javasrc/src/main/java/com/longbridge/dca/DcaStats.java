package com.longbridge.dca;

import java.math.BigDecimal;

/** Response for {@link DcaContext#stats}. */
public class DcaStats {
    /** Number of active plans. */
    public String activeCount;
    /** Number of finished plans. */
    public String finishedCount;
    /** Number of suspended plans. */
    public String suspendedCount;
    /** Nearest upcoming plans. */
    public DcaPlan[] nearestPlans;
    /** Days until next investment. */
    public String restDays;
    /** Total invested amount. */
    public BigDecimal totalAmount;
    /** Total profit/loss. */
    public BigDecimal totalProfit;
}
