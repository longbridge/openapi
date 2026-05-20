package com.longbridge.alert;

/** One price alert. */
public class AlertItem {
    /** Alert ID. */
    public String id;
    /** Condition: "1"=price_rise, "2"=price_fall, "3"=pct_rise, "4"=pct_fall. */
    public String indicatorId;
    /** Whether the alert is active. */
    public boolean enabled;
    /** Frequency: 1=daily, 2=every_time, 3=once. */
    public int frequency;
    /** Scope. */
    public int scope;
    /** Display text. */
    public String text;
    /** Trigger state flags. */
    public int[] state;
    /** Trigger value map, e.g. {@code {"price":"500"}} or {@code {"chg":"5"}}. */
    public String valueMap;
}
