package com.longbridge.fund;

/**
 * FundAssetAllocationItem
 */
public class FundAssetAllocationItem {
    private String code;
    private String counterId;
    private String name;
    private String positionRatio;

    /**
     * Returns code.
     *
     * @return code
     */
    public String getCode() {
        return code;
    }

    /**
     * Returns counterId.
     *
     * @return counterId
     */
    public String getCounterId() {
        return counterId;
    }

    /**
     * Returns name.
     *
     * @return name
     */
    public String getName() {
        return name;
    }

    /**
     * Returns positionRatio.
     *
     * @return positionRatio
     */
    public String getPositionRatio() {
        return positionRatio;
    }

    @Override
    public String toString() {
        return                 "FundAssetAllocationItem [code=" + code +
                ", counterId=" + counterId +
                ", name=" + name +
                ", positionRatio=" + positionRatio + "]";
    }
}
