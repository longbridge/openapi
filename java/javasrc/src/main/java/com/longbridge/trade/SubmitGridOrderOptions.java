package com.longbridge.trade;

/**
 * Options for submitting a grid trading order
 */
@SuppressWarnings("unused")
public class SubmitGridOrderOptions {
    private String symbol;
    private String settlementCurrency;
    private GridTradeRule gridTradingRule;

    /**
     * Constructs options for submitting a grid trading order.
     *
     * @param symbol             security symbol (e.g. 700.HK)
     * @param settlementCurrency settlement currency
     * @param gridTradingRule    grid trading rule
     */
    public SubmitGridOrderOptions(String symbol, String settlementCurrency, GridTradeRule gridTradingRule) {
        this.symbol = symbol;
        this.settlementCurrency = settlementCurrency;
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
