package com.longbridge.dca;

import java.math.BigDecimal;

/** One DCA execution record. */
public class DcaHistoryRecord {
    /** Execution time. */
    public String createdAt;
    /** Associated order ID. */
    public String orderId;
    /** Status. */
    public String status;
    /** Action type. */
    public String action;
    /** Order type. */
    public String orderType;
    /** Executed quantity. */
    public BigDecimal executedQty;
    /** Executed price. */
    public BigDecimal executedPrice;
    /** Executed amount. */
    public BigDecimal executedAmount;
    /** Rejection reason (if any). */
    public String rejectedReason;
    /** Security symbol. */
    public String symbol;
}
