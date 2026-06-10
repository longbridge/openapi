package com.longbridge.fundamental;

/** One historical data point for a macroeconomic indicator. */
public class Macrodata {
    /** Statistical period (e.g. 2024-Q1, 2024-03). */
    public String period;
    public String releaseAt;
    public String actualValue;
    public String previousValue;
    public String forecastValue;
    public String revisedValue;
    public String nextReleaseAt;
    public MultiLanguageText unit;
    public MultiLanguageText unitPrefix;
}
