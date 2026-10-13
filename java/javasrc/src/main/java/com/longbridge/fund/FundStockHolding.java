package com.longbridge.fund;

/**
 * FundStockHolding
 */
public class FundStockHolding {
    private String code;
    private String counterId;
    private String currency;
    private String name;
    private String positionRatio;
    private String reportDate;

    /**
     * Returns code.
     *
     * @return code
     */
    public String getCode() {
        return code;
    }

    /**
     * Returns counterId.
     *
     * @return counterId
     */
    public String getCounterId() {
        return counterId;
    }

    /**
     * Returns currency.
     *
     * @return currency
     */
    public String getCurrency() {
        return currency;
    }

    /**
     * Returns name.
     *
     * @return name
     */
    public String getName() {
        return name;
    }

    /**
     * Returns positionRatio.
     *
     * @return positionRatio
     */
    public String getPositionRatio() {
        return positionRatio;
    }

    /**
     * Returns reportDate.
     *
     * @return reportDate
     */
    public String getReportDate() {
        return reportDate;
    }

    @Override
    public String toString() {
        return                 "FundStockHolding [code=" + code +
                ", counterId=" + counterId +
                ", currency=" + currency +
                ", name=" + name +
                ", positionRatio=" + positionRatio +
                ", reportDate=" + reportDate + "]";
    }
}
