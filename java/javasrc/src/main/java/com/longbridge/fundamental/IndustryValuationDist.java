package com.longbridge.fundamental;

/** Valuation ratio distributions for an industry, used for percentile ranking. */
public class IndustryValuationDist {
    /** PE ratio distribution within the industry. */
    public ValuationDist pe;
    /** PB ratio distribution within the industry. */
    public ValuationDist pb;
    /** PS ratio distribution within the industry. */
    public ValuationDist ps;
}