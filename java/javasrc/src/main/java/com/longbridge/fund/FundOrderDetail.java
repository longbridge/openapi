package com.longbridge.fund;

/**
 * FundOrderDetail
 */
public class FundOrderDetail {
    private FundOrderKeyword[] keywords;
    private FundOrderInfo order;
    private FundOrderStage[] stages;

    /**
     * Returns keywords.
     *
     * @return keywords
     */
    public FundOrderKeyword[] getKeywords() {
        return keywords;
    }

    /**
     * Returns order.
     *
     * @return order
     */
    public FundOrderInfo getOrder() {
        return order;
    }

    /**
     * Returns stages.
     *
     * @return stages
     */
    public FundOrderStage[] getStages() {
        return stages;
    }

    @Override
    public String toString() {
        return                 "FundOrderDetail [keywords=" + java.util.Arrays.toString(keywords) +
                ", order=" + order +
                ", stages=" + java.util.Arrays.toString(stages) + "]";
    }
}
