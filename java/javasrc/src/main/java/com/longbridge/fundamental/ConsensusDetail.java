package com.longbridge.fundamental;

import java.math.BigDecimal;

/** Consensus estimate for one financial metric within a fiscal period. */
public class ConsensusDetail {
    /** Metric key, e.g. {@code "revenue"}, {@code "eps"}. */
    public String key;
    /** Display name. */
    public String name;
    /** Metric description. */
    public String description;
    /** Actual reported value (null if not yet released). */
    public BigDecimal actual;
    /** Consensus estimate value. */
    public BigDecimal estimate;
    /** Actual minus estimate. */
    public BigDecimal compValue;
    /** Beat/miss description, e.g. {@code "超出预期"}. */
    public String compDesc;
    /** Comparison result code for colour coding. */
    public String comp;
    /** Whether the actual results have been published. */
    public boolean isReleased;
}