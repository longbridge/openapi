package com.longbridge.fundamental;

/** Metadata for one macroeconomic indicator. */
public class MacrodataIndicator {
    /** External vendor code (input to getEconomicIndicator). */
    public String indicatorCode;
    public String sourceOrg;
    public String country;
    public MultiLanguageText name;
    public String adjustmentFactor;
    /** Release periodicity (e.g. monthly / quarterly). */
    public String periodicity;
    public String category;
    public MultiLanguageText describe;
    /** Importance — higher is more important. */
    public int importance;
    /** Start date of data coverage (unix timestamp string). */
    public String startDate;
}
