package com.longbridge.fund;

/**
 * FundAssetAllocation
 */
public class FundAssetAllocation {
    private int assetType;
    private FundAssetAllocationItem[] lists;
    private String reportDate;

    /**
     * Returns assetType.
     *
     * @return assetType
     */
    public int getAssetType() {
        return assetType;
    }

    /**
     * Returns lists.
     *
     * @return lists
     */
    public FundAssetAllocationItem[] getLists() {
        return lists;
    }

    /**
     * Returns reportDate.
     *
     * @return reportDate
     */
    public String getReportDate() {
        return reportDate;
    }

    @Override
    public String toString() {
        return                 "FundAssetAllocation [assetType=" + assetType +
                ", lists=" + java.util.Arrays.toString(lists) +
                ", reportDate=" + reportDate + "]";
    }
}
