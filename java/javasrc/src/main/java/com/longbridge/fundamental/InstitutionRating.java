package com.longbridge.fundamental;

/** Combined analyst-rating response for a security. */
public class InstitutionRating {
    /** Latest snapshot of analyst ratings. */
    public InstitutionRatingLatest latest;
    /** Consensus summary of analyst ratings. */
    public InstitutionRatingSummary summary;
}