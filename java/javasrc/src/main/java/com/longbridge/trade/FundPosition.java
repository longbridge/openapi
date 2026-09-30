package com.longbridge.trade;

import java.math.BigDecimal;
import java.time.OffsetDateTime;

/**
 * Fund position
 */
public class FundPosition {
    private String counterId;
    private BigDecimal currentNetAssetValue;
    private OffsetDateTime netAssetValueDay;
    private String symbolName;
    private String currency;
    private BigDecimal costNetAssetValue;
    private BigDecimal holdingUnits;

    /**
     * Returns the fund counter id (the ISIN is the last {@code /}-separated segment).
     *
     * @return fund counter id
     */
    public String getCounterId() {
        return counterId;
    }

    /**
     * Returns the current net asset value.
     *
     * @return current net asset value
     */
    public BigDecimal getCurrentNetAssetValue() {
        return currentNetAssetValue;
    }

    /**
     * Returns the date of the net asset value.
     *
     * @return net asset value date
     */
    public OffsetDateTime getNetAssetValueDay() {
        return netAssetValueDay;
    }

    /**
     * Returns the fund name.
     *
     * @return fund name
     */
    public String getSymbolName() {
        return symbolName;
    }

    /**
     * Returns the currency.
     *
     * @return currency
     */
    public String getCurrency() {
        return currency;
    }

    /**
     * Returns the cost net asset value.
     *
     * @return cost net asset value
     */
    public BigDecimal getCostNetAssetValue() {
        return costNetAssetValue;
    }

    /**
     * Returns the holding units.
     *
     * @return holding units
     */
    public BigDecimal getHoldingUnits() {
        return holdingUnits;
    }

    @Override
    public String toString() {
        return "FundPosition [counterId=" + counterId + ", currentNetAssetValue=" + currentNetAssetValue
                + ", netAssetValueDay=" + netAssetValueDay + ", symbolName=" + symbolName + ", currency=" + currency
                + ", costNetAssetValue=" + costNetAssetValue + ", holdingUnits=" + holdingUnits + "]";
    }

}
