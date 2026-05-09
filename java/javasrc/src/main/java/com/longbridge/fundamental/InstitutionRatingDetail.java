package com.longbridge.fundamental;

/** Detailed historical analyst rating data for a security. */
public class InstitutionRatingDetail {
    /** Currency symbol, e.g. {@code "HK$"}. */
    public String ccySymbol;
    /** Historical rating distribution time-series. */
    public InstitutionRatingDetailEvaluate evaluate;
    /** Historical target price time-series. */
    public InstitutionRatingDetailTarget target;
}