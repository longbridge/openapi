package com.longbridge.trade;

import java.util.concurrent.CompletableFuture;

import com.longbridge.*;

public class TradeContext implements AutoCloseable {
    private long raw;

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

    public void setOnOrderChange(OrderChangedHandler handler) {
        SdkNative.tradeContextSetOnOrderChanged(this.raw, handler);
    }

    public CompletableFuture<Void> subscribe(TopicType[] topics) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.tradeContextSubscribe(this.raw, topics, callback);
        });
    }

    public CompletableFuture<Void> unsubscribe(TopicType[] topics) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.tradeContextUnsubscribe(this.raw, topics, callback);
        });
    }

    public CompletableFuture<Execution[]> historyExecutions(GetHistoryExecutionsOptions opts) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.tradeContextHistoryExecutions(this.raw, opts, callback);
        });
    }

    public CompletableFuture<Execution[]> todayExecutions(GetTodayExecutionsOptions opts) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.tradeContextTodayExecutions(this.raw, opts, callback);
        });
    }

    public CompletableFuture<Order[]> historyOrders(GetHistoryOrdersOptions opts) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.tradeContextHistoryOrders(this.raw, opts, callback);
        });
    }

    public CompletableFuture<Order[]> todayOrders(GetTodayOrdersOptions opts) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.tradeContextTodayOrders(this.raw, opts, callback);
        });
    }

    public CompletableFuture<Void> replaceOrder(ReplaceOrderOptions opts) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.tradeContextReplaceOrder(this.raw, opts, callback);
        });
    }

    public CompletableFuture<SubmitOrderResponse> submitOrder(SubmitOrderOptions opts) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.tradeContextSubmitOrder(this.raw, opts, callback);
        });
    }

    public CompletableFuture<Void> cancelOrder(String orderIdId) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.tradeContextCancelOrder(this.raw, orderIdId, callback);
        });
    }

    public CompletableFuture<AccountBalance[]> accountBalance() throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.tradeContextAccountBalance(this.raw, callback);
        });
    }

    public CompletableFuture<CashFlow[]> cashFlow(GetCashFlowOptions opts) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.tradeContextCashFlow(this.raw, opts, callback);
        });
    }

    public CompletableFuture<FundPositionsResponse> fundPositions(GetFundPositionsOptions opts)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.tradeContextFundPositions(this.raw, opts, callback);
        });
    }

    public CompletableFuture<StockPositionsResponse> stockPositions(GetStockPositionsOptions opts)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.tradeContextStockPositions(this.raw, opts, callback);
        });
    }
}
