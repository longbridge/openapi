package com.longbridge.asset;

import java.time.OffsetDateTime;
import java.util.Arrays;

/**
 * A stock transfer-out application (this account to another broker). Holder name, contact details and proof document keys are not modelled.
 */
public class StockTransferOut {
    private String id;
    private String orderId;
    private String market;
    private String broker;
    private String brokerEn;
    private String brokerCode;
    private String account;
    private String state;
    private StockTransferOutState stateCode;
    private OffsetDateTime createdAt;
    private OffsetDateTime updatedAt;
    private OffsetDateTime processAt;
    private OffsetDateTime execAt;
    private OffsetDateTime agreeAt;
    private OffsetDateTime refuseAt;
    private OffsetDateTime cancelAt;
    private String remark;
    private String reason;
    private TransferStock[] stocks;

    /**
     * Returns the record ID.
     *
     * @return the record ID
     */
    public String getId() {
        return id;
    }

    /**
     * Returns the order ID.
     *
     * @return the order ID
     */
    public String getOrderId() {
        return orderId;
    }

    /**
     * Returns the market.
     *
     * @return the market
     */
    public String getMarket() {
        return market;
    }

    /**
     * Returns the name of the other broker.
     *
     * @return the name of the other broker
     */
    public String getBroker() {
        return broker;
    }

    /**
     * Returns the English name of the other broker.
     *
     * @return the English name of the other broker
     */
    public String getBrokerEn() {
        return brokerEn;
    }

    /**
     * Returns the broker code of the other broker.
     *
     * @return the broker code of the other broker
     */
    public String getBrokerCode() {
        return brokerCode;
    }

    /**
     * Returns the account number at the other broker, as returned by the server (not masked).
     *
     * @return the account number at the other broker, as returned by the server
     */
    public String getAccount() {
        return account;
    }

    /**
     * Returns the state text, as returned by the server.
     *
     * @return the state text, as returned by the server
     */
    public String getState() {
        return state;
    }

    /**
     * Returns the state code; only {@link StockTransferOutState#Submitted} applications can be cancelled.
     *
     * @return the state code
     */
    public StockTransferOutState getStateCode() {
        return stateCode;
    }

    /**
     * Returns the created time.
     *
     * @return the created time
     */
    public OffsetDateTime getCreatedAt() {
        return createdAt;
    }

    /**
     * Returns the updated time.
     *
     * @return the updated time
     */
    public OffsetDateTime getUpdatedAt() {
        return updatedAt;
    }

    /**
     * Returns the processing start time ({@code null} when not set).
     *
     * @return the processing start time
     */
    public OffsetDateTime getProcessAt() {
        return processAt;
    }

    /**
     * Returns the execution time ({@code null} when not executed).
     *
     * @return the execution time
     */
    public OffsetDateTime getExecAt() {
        return execAt;
    }

    /**
     * Returns the time the other broker agreed ({@code null} when not set).
     *
     * @return the time the other broker agreed
     */
    public OffsetDateTime getAgreeAt() {
        return agreeAt;
    }

    /**
     * Returns the time the application was refused ({@code null} when not set).
     *
     * @return the time the application was refused
     */
    public OffsetDateTime getRefuseAt() {
        return refuseAt;
    }

    /**
     * Returns the time the application was cancelled ({@code null} when not set).
     *
     * @return the time the application was cancelled
     */
    public OffsetDateTime getCancelAt() {
        return cancelAt;
    }

    /**
     * Returns the remark.
     *
     * @return the remark
     */
    public String getRemark() {
        return remark;
    }

    /**
     * Returns the reason (e.g. for a rejection).
     *
     * @return the reason
     */
    public String getReason() {
        return reason;
    }

    /**
     * Returns the stocks in the application.
     *
     * @return the stocks in the application
     */
    public TransferStock[] getStocks() {
        return stocks;
    }

    @Override
    public String toString() {
        return "StockTransferOut [" + "id=" + id + ", " + "orderId=" + orderId + ", " + "market=" + market + ", " + "broker=" + broker + ", " + "brokerEn=" + brokerEn + ", " + "brokerCode=" + brokerCode + ", " + "account=" + account + ", " + "state=" + state + ", " + "stateCode=" + stateCode + ", " + "createdAt=" + createdAt + ", " + "updatedAt=" + updatedAt + ", " + "processAt=" + processAt + ", " + "execAt=" + execAt + ", " + "agreeAt=" + agreeAt + ", " + "refuseAt=" + refuseAt + ", " + "cancelAt=" + cancelAt + ", " + "remark=" + remark + ", " + "reason=" + reason + ", " + "stocks=" + Arrays.toString(stocks) + "]";
    }
}
