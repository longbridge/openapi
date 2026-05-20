package com.longbridge.market;

/** Market anomaly alerts response for a security. */
public class AnomalyResponse {
    /** Whether anomaly alerts are globally disabled. */
    public boolean allOff;
    /** List of market anomaly events. */
    public AnomalyItem[] changes;
}