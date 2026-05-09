package com.longbridge.portfolio;

import java.math.BigDecimal;

/** One profit-analysis flow record for {@link ProfitAnalysisFlowsResponse}. */
public class FlowItem {
    /** Execution date string, e.g. "2024-01-15" */
    public String executedDate;
    /** Execution timestamp (JSON string; may be int or string) */
    public String executedTimestamp;
    /** Security code / ticker */
    public String code;
    /** Direction of the flow. */
    public FlowDirection direction;
    /** Executed quantity; may be null */
    public BigDecimal executedQuantity;
    /** Executed price; may be null */
    public BigDecimal executedPrice;
    /** Executed cost; may be null */
    public BigDecimal executedCost;
    /** Human-readable description */
    public String describe;
}
