package com.longbridge.fundamental;

import java.math.BigDecimal;

/** TTM buyback summary for {@link BuybackData}. */
public class RecentBuybacks {
    /** Reporting currency */
    public String currency;
    /** Net buyback amount TTM; may be null */
    public BigDecimal netBuybackTtm;
    /** Net buyback yield TTM; may be null */
    public BigDecimal netBuybackYieldTtm;
}
