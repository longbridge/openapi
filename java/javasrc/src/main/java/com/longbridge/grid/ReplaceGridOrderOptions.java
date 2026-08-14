package com.longbridge.grid;

/**
 * Options for replacing (modifying) a grid trading order
 */
@SuppressWarnings("unused")
public class ReplaceGridOrderOptions {
    private String orderId;
    private GridTradeRule gridTradingRule;

    /**
     * Constructs options for replacing a grid trading order.
     *
     * @param orderId         grid master order ID
     * @param gridTradingRule grid trading rule
     */
    public ReplaceGridOrderOptions(String orderId, GridTradeRule gridTradingRule) {
        this.orderId = orderId;
        this.gridTradingRule = gridTradingRule;
    }

    /**
     * Returns the grid trading rule.
     *
     * @return grid trading rule
     */
    public GridTradeRule getGridTradingRule() {
        return gridTradingRule;
    }
}
