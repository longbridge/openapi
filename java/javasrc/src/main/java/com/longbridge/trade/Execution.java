package com.longbridge.trade;

import java.math.BigDecimal;
import java.time.OffsetDateTime;

/**
 * Order execution (fill)
 */
public class Execution {
    private String orderId;
    private String tradeId;
    private String symbol;
    private OffsetDateTime tradeDoneAt;
    private BigDecimal quantity;
    private BigDecimal price;
    private OrderSide side;

    /**
     * Returns the order ID.
     *
     * @return order ID
     */
    public String getOrderId() {
        return orderId;
    }

    /**
     * Returns the trade ID.
     *
     * @return trade ID
     */
    public String getTradeId() {
        return tradeId;
    }

    /**
     * Returns the security symbol.
     *
     * @return security symbol
     */
    public String getSymbol() {
        return symbol;
    }

    /**
     * Returns the time the trade was done.
     *
     * @return trade done time
     */
    public OffsetDateTime getTradeDoneAt() {
        return tradeDoneAt;
    }

    /**
     * Returns the executed quantity.
     *
     * @return executed quantity
     */
    public BigDecimal getQuantity() {
        return quantity;
    }

    /**
     * Returns the executed price.
     *
     * @return executed price
     */
    public BigDecimal getPrice() {
        return price;
    }

    /**
     * Returns the order side.
     *
     * @return order side
     */
    public OrderSide getSide() {
        return side;
    }

    @Override
    public String toString() {
        return "Execution [orderId=" + orderId + ", price=" + price + ", quantity=" + quantity + ", side=" + side
                + ", symbol=" + symbol + ", tradeDoneAt=" + tradeDoneAt + ", tradeId=" + tradeId + "]";
    }
}
