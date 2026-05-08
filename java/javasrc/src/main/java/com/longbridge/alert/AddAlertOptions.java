package com.longbridge.alert;

/** Options for {@link AlertContext#addAlert}. */
public class AddAlertOptions {
    /** Security symbol to set the alert on. */
    public String symbol;
    /** Alert condition: 1=price_rise, 2=price_fall, 3=pct_rise, 4=pct_fall. */
    public Integer condition;
    /** Trigger value, e.g. {@code "500"} for price or {@code "5"} for percentage. */
    public String triggerValue;
    /** Alert frequency: 1=daily, 2=every_time, 3=once. */
    public Integer frequency;
}
