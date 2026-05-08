package com.longbridge.fundamental;

/** A group of sub-indicators under one category indicator for {@link RatingCategory}. */
public class RatingSubIndicatorGroup {
    /** Parent indicator for this group */
    public RatingIndicator indicator;
    /** Leaf sub-indicators */
    public RatingLeafIndicator[] subIndicators;
}
