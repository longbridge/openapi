package com.longbridge.trade;

import java.util.concurrent.CompletableFuture;

import com.longbridge.*;

/**
 * Trade context
 */
public class TradeContext implements AutoCloseable {
    private long raw;

    /**
     * Create a TradeContext object
     * 
     * @param config Config object
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public static CompletableFuture<TradeContext> create(Config config)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.newTradeContext(config.getRaw(), callback);
        });
    }

    @Override
    public void close() throws Exception {
        SdkNative.freeTradeContext(raw);
    }

    /**
     * Set order changed event callback, after receiving the order changed event, it
     * will call back to this handler.
     * 
     * @param handler A order changed handler
     */
    public void setOnOrderChange(OrderChangedHandler handler) {
        SdkNative.tradeContextSetOnOrderChanged(this.raw, handler);
    }

    /**
     * Subscribe
     * 
     * @param topics Topics
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<Void> subscribe(TopicType[] topics) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.tradeContextSubscribe(this.raw, topics, callback);
        });
    }

    /**
     * Unsubscribe
     * 
     * @param topics Topics
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<Void> unsubscribe(TopicType[] topics) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.tradeContextUnsubscribe(this.raw, topics, callback);
        });
    }

    /**
     * Get history executions
     * 
     * @param opts Options for this request
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<Execution[]> getHistoryExecutions(GetHistoryExecutionsOptions opts)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.tradeContextHistoryExecutions(this.raw, opts, callback);
        });
    }

    /**
     * Get today executions
     * 
     * @param opts Options for this request
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<Execution[]> getTodayExecutions(GetTodayExecutionsOptions opts) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.tradeContextTodayExecutions(this.raw, opts, callback);
        });
    }

    /**
     * Get history orders
     * 
     * @param opts Options for this request
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<Order[]> getHistoryOrders(GetHistoryOrdersOptions opts) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.tradeContextHistoryOrders(this.raw, opts, callback);
        });
    }

    /**
     * Get today orders
     * 
     * @param opts Options for this request
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<Order[]> getTodayOrders(GetTodayOrdersOptions opts) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.tradeContextTodayOrders(this.raw, opts, callback);
        });
    }

    /**
     * Replace order
     * 
     * @param opts Options for this request
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<Void> replaceOrder(ReplaceOrderOptions opts) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.tradeContextReplaceOrder(this.raw, opts, callback);
        });
    }

    /**
     * Submit order
     * 
     * @param opts Options for this request
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<SubmitOrderResponse> submitOrder(SubmitOrderOptions opts) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.tradeContextSubmitOrder(this.raw, opts, callback);
        });
    }

    /**
     * Cancel order
     * 
     * @param orderId Order ID
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<Void> cancelOrder(String orderId) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.tradeContextCancelOrder(this.raw, orderId, callback);
        });
    }

    /**
     * Get account balance
     * 
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<AccountBalance[]> getAccountBalance() throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.tradeContextAccountBalance(this.raw, callback);
        });
    }

    /**
     * Get cash flow
     * 
     * @param opts Options for this request
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<CashFlow[]> getCashFlow(GetCashFlowOptions opts) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.tradeContextCashFlow(this.raw, opts, callback);
        });
    }

    /**
     * Get fund positions
     * 
     * @param opts Options for this request
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<FundPositionsResponse> getFundPositions(GetFundPositionsOptions opts)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.tradeContextFundPositions(this.raw, opts, callback);
        });
    }

    /**
     * Get stock positions
     * 
     * @param opts Options for this request
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<StockPositionsResponse> getStockPositions(GetStockPositionsOptions opts)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.tradeContextStockPositions(this.raw, opts, callback);
        });
    }
}
