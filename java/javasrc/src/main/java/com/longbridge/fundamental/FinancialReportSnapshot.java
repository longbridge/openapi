package com.longbridge.fundamental;

/** Response for {@link FundamentalContext#getFinancialReportSnapshot}. */
public class FinancialReportSnapshot {
    /** Company name */
    public String name;
    /** Ticker code */
    public String ticker;
    /** Fiscal period start date */
    public String fpStart;
    /** Fiscal period end date */
    public String fpEnd;
    /** Reporting currency */
    public String currency;
    /** Report description */
    public String reportDesc;
    /** Forecast revenue; may be null */
    public SnapshotForecastMetric foRevenue;
    /** Forecast EBIT; may be null */
    public SnapshotForecastMetric foEbit;
    /** Forecast EPS; may be null */
    public SnapshotForecastMetric foEps;
    /** Reported revenue; may be null */
    public SnapshotReportedMetric frRevenue;
    /** Reported net profit; may be null */
    public SnapshotReportedMetric frProfit;
    /** Reported operating cash flow; may be null */
    public SnapshotReportedMetric frOperateCash;
    /** Reported investing cash flow; may be null */
    public SnapshotReportedMetric frInvestCash;
    /** Reported financing cash flow; may be null */
    public SnapshotReportedMetric frFinanceCash;
    /** Reported total assets; may be null */
    public SnapshotReportedMetric frTotalAssets;
    /** Reported total liabilities; may be null */
    public SnapshotReportedMetric frTotalLiability;
    /** ROE TTM */
    public String frRoeTtm;
    /** Profit margin */
    public String frProfitMargin;
    /** Profit margin TTM */
    public String frProfitMarginTtm;
    /** Asset turnover TTM */
    public String frAssetTurnTtm;
    /** Leverage TTM */
    public String frLeverageTtm;
    /** Debt-to-assets ratio */
    public String frDebtAssetsRatio;
}
