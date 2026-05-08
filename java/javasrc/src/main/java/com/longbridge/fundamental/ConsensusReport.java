package com.longbridge.fundamental;

/** Consensus report for one fiscal period. */
public class ConsensusReport {
    /** Fiscal year, e.g. {@code 2025}. */
    public int fiscalYear;
    /** Fiscal period code, e.g. {@code "Q4"}. */
    public String fiscalPeriod;
    /** Human-readable period label, e.g. {@code "Q4 FY2025"}. */
    public String periodText;
    /** Per-metric consensus details. */
    public ConsensusDetail[] details;
}