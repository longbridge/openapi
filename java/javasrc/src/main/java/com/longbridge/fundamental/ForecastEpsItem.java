package com.longbridge.fundamental;

import java.math.BigDecimal;

/** One EPS forecast snapshot covering a specific forecast window. */
public class ForecastEpsItem {
    /** Median EPS estimate. */
    public BigDecimal forecastEpsMedian;
    /** Mean EPS estimate. */
    public BigDecimal forecastEpsMean;
    /** Lowest EPS estimate. */
    public BigDecimal forecastEpsLowest;
    /** Highest EPS estimate. */
    public BigDecimal forecastEpsHighest;
    /** Total number of forecasting institutions. */
    public int institutionTotal;
    /** Number of institutions that raised their estimate. */
    public int institutionUp;
    /** Number of institutions that lowered their estimate. */
    public int institutionDown;
    /** Forecast window start. */
    public java.time.OffsetDateTime forecastStartDate;
    /** Forecast window end. */
    public java.time.OffsetDateTime forecastEndDate;
}