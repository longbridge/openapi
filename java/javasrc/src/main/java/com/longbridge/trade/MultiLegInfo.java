package com.longbridge.trade;

import java.util.Arrays;

/**
 * Multi-leg strategy information
 */
public class MultiLegInfo {
    private MultiLegStrategy strategy;
    private String strategyName;
    private String multilegId;
    private String code;
    private MultiLegOrderLeg[] legs = new MultiLegOrderLeg[0];

    /**
     * Returns the multi-leg strategy.
     *
     * @return multi-leg strategy
     */
    public MultiLegStrategy getStrategy() {
        return strategy;
    }

    /**
     * Returns the strategy name.
     *
     * @return strategy name
     */
    public String getStrategyName() {
        return strategyName;
    }

    /**
     * Returns the multi-leg combination ID.
     *
     * @return multi-leg combination ID
     */
    public String getMultilegId() {
        return multilegId;
    }

    /**
     * Returns the multi-leg combination code.
     *
     * @return multi-leg combination code
     */
    public String getCode() {
        return code;
    }

    /**
     * Returns the legs of the combination order.
     *
     * @return legs of the combination order
     */
    public MultiLegOrderLeg[] getLegs() {
        return legs;
    }

    @Override
    public String toString() {
        return "MultiLegInfo [strategy=" + strategy + ", strategyName=" + strategyName + ", multilegId=" + multilegId
                + ", code=" + code + ", legs=" + Arrays.toString(legs) + "]";
    }
}
