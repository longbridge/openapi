package com.longbridge.portfolio;

/** P&amp;L summary for one asset category. */
public class ProfitSummaryInfo {
    /** Asset type. */
    public AssetType assetType;
    /** Security with the maximum profit. */
    public String profitMax;
    /** Name of the max-profit security. */
    public String profitMaxName;
    /** Security with the maximum loss. */
    public String lossMax;
    /** Name of the max-loss security. */
    public String lossMaxName;
}
