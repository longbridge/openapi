package com.longbridge.fundamental;

import java.math.BigDecimal;

/** One financial indicator in an operating report. */
public class OperatingIndicator {
    /** Field name key, e.g. {@code "operating_revenue"}. */
    public String fieldName;
    /** Display name, e.g. {@code "营业收入"}. */
    public String indicatorName;
    /** Formatted value, e.g. {@code "8217 亿"}. */
    public String indicatorValue;
    /** Year-over-year change. */
    public BigDecimal yoy;
}