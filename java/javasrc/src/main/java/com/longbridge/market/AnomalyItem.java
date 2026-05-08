package com.longbridge.market;

/** One market anomaly event, e.g. a large block trade or margin buying surge. */
public class AnomalyItem {
    /** Security symbol. */
    public String symbol;
    /** Security name. */
    public String name;
    /** Anomaly type name, e.g. {@code "大宗交易"}, {@code "融资买入"}. */
    public String alertName;
    /** Time of the anomaly (unix timestamp in milliseconds). */
    public long alertTime;
    /** Change values associated with the anomaly. */
    public String[] changeValues;
    /** Sentiment direction: 1 = positive/up, 2 = negative/down. */
    public int emotion;
}