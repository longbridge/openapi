package com.longbridge.fundamental;

/** Financial consensus estimates response for a security. */
public class FinancialConsensus {
    /** Per-period consensus reports. */
    public ConsensusReport[] list;
    /** Index into {@code list} of the most recently released period. */
    public int currentIndex;
    /** Reporting currency, e.g. {@code "HKD"}. */
    public String currency;
    /** Available period types, e.g. {@code ["qf", "saf", "af"]}. */
    public String[] optPeriods;
    /** Currently returned period type. */
    public String currentPeriod;
}