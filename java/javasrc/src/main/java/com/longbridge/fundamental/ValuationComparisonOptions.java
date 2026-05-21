package com.longbridge.fundamental;

/** Options for {@link FundamentalContext#getValuationComparison}. */
public class ValuationComparisonOptions {
    /** Primary security symbol, e.g. "AAPL.US" */
    public String symbol;
    /** Currency: "USD", "HKD", or "CNY" */
    public String currency;
    /** Optional peer symbols to compare (up to 4), e.g. ["MSFT.US","GOOGL.US"] */
    public String[] comparisonSymbols;
}
