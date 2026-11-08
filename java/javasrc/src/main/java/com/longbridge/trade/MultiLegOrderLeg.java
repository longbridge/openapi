package com.longbridge.trade;

import java.math.BigDecimal;
import java.time.LocalDate;

/**
 * A leg of a multi-leg combination order
 */
public class MultiLegOrderLeg {
    private String symbol;
    private OrderSide side;
    private MultiLegPosition position;
    private BigDecimal ratioQuantity;
    private BigDecimal strikePrice;
    private LocalDate expireDate;
    private ContractDirection contractDirection;

    /**
     * Returns the option symbol, in `ticker.region` format.
     *
     * @return option symbol
     */
    public String getSymbol() {
        return symbol;
    }

    /**
     * Returns the order side.
     *
     * @return order side
     */
    public OrderSide getSide() {
        return side;
    }

    /**
     * Returns the position direction.
     *
     * @return position direction
     */
    public MultiLegPosition getPosition() {
        return position;
    }

    /**
     * Returns the leg ratio quantity.
     *
     * @return leg ratio quantity
     */
    public BigDecimal getRatioQuantity() {
        return ratioQuantity;
    }

    /**
     * Returns the strike price.
     *
     * @return strike price
     */
    public BigDecimal getStrikePrice() {
        return strikePrice;
    }

    /**
     * Returns the option expiry date.
     *
     * @return option expiry date
     */
    public LocalDate getExpireDate() {
        return expireDate;
    }

    /**
     * Returns the contract type.
     *
     * @return contract type
     */
    public ContractDirection getContractDirection() {
        return contractDirection;
    }

    @Override
    public String toString() {
        return "MultiLegOrderLeg [symbol=" + symbol + ", side=" + side + ", position=" + position
                + ", ratioQuantity=" + ratioQuantity + ", strikePrice=" + strikePrice + ", expireDate=" + expireDate
                + ", contractDirection=" + contractDirection + "]";
    }
}
