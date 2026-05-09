package com.longbridge.alert;

/** Options for {@link AlertContext#add}. */
public class AddAlertOptions {
    /** Security symbol to set the alert on. */
    public String symbol;
    /** Alert condition. */
    public AlertCondition condition;
    /** Trigger value, e.g. {@code "500"} for price or {@code "5"} for percentage. */
    public String triggerValue;
    /** Alert frequency. */
    public AlertFrequency frequency;
}
