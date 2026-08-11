package com.longbridge.grid;

/**
 * Response from submitting a grid trading order
 */
public class SubmitGridOrderResponse {
    private String orderId;

    /**
     * Returns orderId.
     *
     * @return orderId
     */
    public String getOrderId() {
        return orderId;
    }

    @Override
    public String toString() {
        return "SubmitGridOrderResponse [orderId=" + orderId + "]";
    }
}
