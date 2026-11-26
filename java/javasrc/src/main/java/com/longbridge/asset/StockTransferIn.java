package com.longbridge.asset;

import java.time.OffsetDateTime;
import java.util.Arrays;

/**
 * A stock transfer-in application (another broker to this account). Holder name, contact details and proof document keys are not modelled.
 */
public class StockTransferIn {
    private String id;
    private String orderId;
    private String market;
    private String broker;
    private String brokerId;
    private String brokerEn;
    private String brokerCode;
    private String account;
    private String state;
    private StockTransferInState stateCode;
    private String appState;
    private boolean canModify;
    private OffsetDateTime createdAt;
    private OffsetDateTime updatedAt;
    private OffsetDateTime processAt;
    private OffsetDateTime execAt;
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
     * Returns the ID of the other broker.
     *
     * @return the ID of the other broker
     */
    public String getBrokerId() {
        return brokerId;
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
     * Returns the state code.
     *
     * @return the state code
     */
    public StockTransferInState getStateCode() {
        return stateCode;
    }

    /**
     * Returns the app-side state: {@code noticed} means the user tapped "I
     * have notified the broker" in the app (the user's own confirmation, not a
     * Longbridge action); {@code default} otherwise.
     *
     * @return the app-side state
     */
    public String getAppState() {
        return appState;
    }

    /**
     * Returns whether the user confirmed in the app that the other broker was
     * notified ({@link #getAppState()} is {@code noticed}).
     *
     * @return whether the user notified the other broker
     */
    public boolean isUserNotifiedBroker() {
        return "noticed".equals(appState);
    }

    /**
     * Returns whether the application can still be cancelled or changed.
     *
     * @return whether the application can still be cancelled or changed
     */
    public boolean isCanModify() {
        return canModify;
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
     * Returns the remark.
     *
     * @return the remark
     */
    public String getRemark() {
        return remark;
    }

    /**
     * Returns the reason (e.g. for a failure).
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
        return "StockTransferIn [" + "id=" + id + ", " + "orderId=" + orderId + ", " + "market=" + market + ", " + "broker=" + broker + ", " + "brokerId=" + brokerId + ", " + "brokerEn=" + brokerEn + ", " + "brokerCode=" + brokerCode + ", " + "account=" + account + ", " + "state=" + state + ", " + "stateCode=" + stateCode + ", " + "appState=" + appState + ", " + "canModify=" + canModify + ", " + "createdAt=" + createdAt + ", " + "updatedAt=" + updatedAt + ", " + "processAt=" + processAt + ", " + "execAt=" + execAt + ", " + "remark=" + remark + ", " + "reason=" + reason + ", " + "stocks=" + Arrays.toString(stocks) + "]";
    }
}
