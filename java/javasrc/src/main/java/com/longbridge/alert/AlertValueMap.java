package com.longbridge.alert;

import java.math.BigDecimal;

/**
 * Trigger value of a price alert.
 *
 * <p>Exactly one field is populated, depending on the alert condition:
 * {@code price} for absolute-price alerts, {@code chg} for percentage-change
 * alerts.
 */
public class AlertValueMap {
    /** Absolute price threshold, e.g. {@code 500} (null if not set). */
    public BigDecimal price;
    /** Percentage-change threshold, e.g. {@code 5} (null if not set). */
    public Double chg;
}
