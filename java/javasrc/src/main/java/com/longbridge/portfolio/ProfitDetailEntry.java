package com.longbridge.portfolio;

import java.math.BigDecimal;

/** One P&amp;L detail line item (credit, debit, or fee). */
public class ProfitDetailEntry {
    /** Description. */
    public String describe;
    /** Amount. */
    public BigDecimal amount;
}
