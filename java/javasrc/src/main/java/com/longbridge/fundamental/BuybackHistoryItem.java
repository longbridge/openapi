package com.longbridge.fundamental;

import java.math.BigDecimal;

/** Historical annual buyback data point for {@link BuybackData}. */
public class BuybackHistoryItem {
    /** Fiscal year label, e.g. "FY2024" */
    public String fiscalYear;
    /** Fiscal year date range string */
    public String fiscalYearRange;
    /** Net buyback amount; may be null */
    public BigDecimal netBuyback;
    /** Net buyback yield; may be null */
    public BigDecimal netBuybackYield;
    /** Year-over-year net buyback growth rate; may be null */
    public BigDecimal netBuybackGrowthRate;
    /** Reporting currency */
    public String currency;
}
