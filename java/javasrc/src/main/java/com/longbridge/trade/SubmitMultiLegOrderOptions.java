package com.longbridge.trade;

import java.math.BigDecimal;

/**
 * Options for submitting a multi-leg option combination order
 */
@SuppressWarnings("unused")
public class SubmitMultiLegOrderOptions {
    private OrderSide side;
    private OrderType orderType;
    private BigDecimal submittedQuantity;
    private MultiLegStrategy strategy;
    private SubmitMultiLegOrderLeg[] legs;
    private BigDecimal submittedPrice;
    private String remark;
    private String clientRequestId;

    /**
     * Constructs options for submitting a multi-leg option combination order.
     *
     * @param side              order side
     * @param orderType         order type
     * @param submittedQuantity submitted quantity (number of combinations)
     * @param strategy          multi-leg strategy
     * @param legs              legs of the combination order
     */
    public SubmitMultiLegOrderOptions(
            OrderSide side,
            OrderType orderType,
            BigDecimal submittedQuantity,
            MultiLegStrategy strategy,
            SubmitMultiLegOrderLeg[] legs) {
        this.side = side;
        this.orderType = orderType;
        this.submittedQuantity = submittedQuantity;
        this.strategy = strategy;
        this.legs = legs;
    }

    /**
     * Sets the submitted price (required for limit order types such as
     * {@code LO}).
     *
     * @param submittedPrice submitted price
     * @return this instance for chaining
     */
    public SubmitMultiLegOrderOptions setSubmittedPrice(BigDecimal submittedPrice) {
        this.submittedPrice = submittedPrice;
        return this;
    }

    /**
     * Sets the remark (maximum 255 characters).
     *
     * @param remark remark
     * @return this instance for chaining
     */
    public SubmitMultiLegOrderOptions setRemark(String remark) {
        this.remark = remark;
        return this;
    }

    /**
     * Sets the client request ID for idempotency control. If not specified,
     * idempotency control is skipped. The server caches this ID for 10 minutes.
     *
     * @param clientRequestId client request ID
     * @return this instance for chaining
     */
    public SubmitMultiLegOrderOptions setClientRequestId(String clientRequestId) {
        this.clientRequestId = clientRequestId;
        return this;
    }
}
