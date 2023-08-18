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
     * <pre>
     * {@code
     * import com.longbridge.*;
     * import com.longbridge.trade.*;
     * import java.math.BigDecimal;
     * 
     * class Main {
     *     public static void main(String[] args) throws Exception {
     *         try (Config config = Config.fromEnv(); TradeContext ctx = TradeContext.create(config).get()) {
     *             ctx.setOnOrderChange((order_changed) -> {
     *                 System.out.println(order_changed);
     *             });
     *             ctx.subscribe(new TopicType[] { TopicType.Private }).get();
     * 
     *             SubmitOrderOptions opts = new SubmitOrderOptions("700.HK",
     *                     OrderType.LO,
     *                     OrderSide.Buy,
     *                     200,
     *                     TimeInForceType.Day).setSubmittedPrice(new BigDecimal(50));
     *             SubmitOrderResponse resp = ctx.submitOrder(opts).get();
     *             System.out.println(resp);
     *             Thread.sleep(3000);
     *         }
     *     }
     * }
     * }
     * </pre>
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
     * <pre>
     * {@code
     * import com.longbridge.*;
     * import com.longbridge.trade.*;
     * import java.time.*;
     * 
     * class Main {
     *     public static void main(String[] args) throws Exception {
     *         try (Config config = Config.fromEnv(); TradeContext ctx = TradeContext.create(config).get()) {
     *             GetHistoryExecutionsOptions opts = new GetHistoryExecutionsOptions().setSymbol("700.HK")
     *                     .setStartAt(OffsetDateTime.of(2022, 5, 9, 0, 0, 0, 0, ZoneOffset.UTC))
     *                     .setEndAt(OffsetDateTime.of(2022, 5, 12, 0, 0, 0, 0, ZoneOffset.UTC));
     *             Execution[] resp = ctx.getHistoryExecutions(opts).get();
     *             for (Execution obj : resp) {
     *                 System.out.println(obj);
     *             }
     *         }
     *     }
     * }
     * }
     * </pre>
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
     * <pre>
     * {@code
     * import com.longbridge.*;
     * import com.longbridge.trade.*;
     * 
     * class Main {
     *     public static void main(String[] args) throws Exception {
     *         try (Config config = Config.fromEnv(); TradeContext ctx = TradeContext.create(config).get()) {
     *             GetTodayExecutionsOptions opts = new GetTodayExecutionsOptions().setSymbol("700.HK");
     *             Execution[] resp = ctx.getTodayExecutions(opts).get();
     *             for (Execution obj : resp) {
     *                 System.out.println(obj);
     *             }
     *         }
     *     }
     * }
     * 
     * }
     * </pre>
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
     * <pre>
     * {@code
     * import com.longbridge.*;
     * import com.longbridge.trade.*;
     * import java.time.*;
     * 
     * class Main {
     *     public static void main(String[] args) throws Exception {
     *         try (Config config = Config.fromEnv(); TradeContext ctx = TradeContext.create(config).get()) {
     *             GetHistoryOrdersOptions opts = new GetHistoryOrdersOptions().setSymbol("700.HK")
     *                     .setStatus(new OrderStatus[] { OrderStatus.Filled, OrderStatus.New })
     *                     .setSide(OrderSide.Buy)
     *                     .setMarket(Market.HK)
     *                     .setStartAt(OffsetDateTime.of(2022, 5, 9, 0, 0, 0, 0, ZoneOffset.UTC))
     *                     .setStartAt(OffsetDateTime.of(2022, 5, 12, 0, 0, 0, 0, ZoneOffset.UTC));
     *             Order[] resp = ctx.getHistoryOrders(opts).get();
     *             for (Order obj : resp) {
     *                 System.out.println(obj);
     *             }
     *         }
     *     }
     * }
     * }
     * </pre>
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
     * <pre>
     * {@code
     * import com.longbridge.*;
     * import com.longbridge.trade.*;
     * 
     * class Main {
     *     public static void main(String[] args) throws Exception {
     *         try (Config config = Config.fromEnv(); TradeContext ctx = TradeContext.create(config).get()) {
     *             GetTodayOrdersOptions opts = new GetTodayOrdersOptions().setSymbol("700.HK")
     *                     .setStatus(new OrderStatus[] { OrderStatus.Filled, OrderStatus.New })
     *                     .setSide(OrderSide.Buy)
     *                     .setMarket(Market.HK);
     *             Order[] resp = ctx.getTodayOrders(opts).get();
     *             for (Order obj : resp) {
     *                 System.out.println(obj);
     *             }
     *         }
     *     }
     * }
     * }
     * </pre>
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
     * <pre>
     * {@code
     * import com.longbridge.*;
     * import com.longbridge.trade.*;
     * import java.math.BigDecimal;
     * 
     * class Main {
     *     public static void main(String[] args) throws Exception {
     *         try (Config config = Config.fromEnv(); TradeContext ctx = TradeContext.create(config).get()) {
     *             ReplaceOrderOptions opts = new ReplaceOrderOptions("709043056541253632", 100)
     *                     .setPrice(new BigDecimal(300));
     *             ctx.replaceOrder(opts).get();
     *         }
     *     }
     * }
     * }
     * </pre>
     * 
     * @param opts Options for this request, not null
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
     * <pre>
     * {@code
     * import com.longbridge.*;
     * import com.longbridge.trade.*;
     * import java.math.BigDecimal;
     * 
     * class Main {
     *     public static void main(String[] args) throws Exception {
     *         try (Config config = Config.fromEnv(); TradeContext ctx = TradeContext.create(config).get()) {
     *             SubmitOrderOptions opts = new SubmitOrderOptions(
     *                     "700.HK",
     *                     OrderType.LO,
     *                     OrderSide.Buy,
     *                     200,
     *                     TimeInForceType.Day).setSubmittedPrice(new BigDecimal(50));
     *             SubmitOrderResponse resp = ctx.submitOrder(opts).get();
     *             System.out.println(resp);
     *         }
     *     }
     * }
     * }
     * </pre>
     * 
     * @param opts Options for this request, not null
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
     * <pre>
     * {@code
     * import com.longbridge.*;
     * import com.longbridge.trade.*;
     * 
     * class Main {
     *     public static void main(String[] args) throws Exception {
     *         try (Config config = Config.fromEnv(); TradeContext ctx = TradeContext.create(config).get()) {
     *             ctx.cancelOrder("709043056541253632").get();
     *         }
     *     }
     * }
     * }
     * </pre>
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
     * Get account balance with currency
     * 
     * <pre>
     * {@code
     * import com.longbridge.*;
     * import com.longbridge.trade.*;
     * 
     * class Main {
     *     public static void main(String[] args) throws Exception {
     *         try (Config config = Config.fromEnv(); TradeContext ctx = TradeContext.create(config).get()) {
     *             AccountBalance[] resp = ctx.getAccountBalance("HKD").get();
     *             for (AccountBalance obj : resp) {
     *                 System.out.println(obj);
     *             }
     *         }
     *     }
     * }
     * }
     * </pre>
     * 
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<AccountBalance[]> getAccountBalance(String currency) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.tradeContextAccountBalance(this.raw, currency, callback);
        });
    }

    /**
     * Get account balance
     * 
     * <pre>
     * {@code
     * import com.longbridge.*;
     * import com.longbridge.trade.*;
     * 
     * class Main {
     *     public static void main(String[] args) throws Exception {
     *         try (Config config = Config.fromEnv(); TradeContext ctx = TradeContext.create(config).get()) {
     *             AccountBalance[] resp = ctx.getAccountBalance().get();
     *             for (AccountBalance obj : resp) {
     *                 System.out.println(obj);
     *             }
     *         }
     *     }
     * }
     * }
     * </pre>
     * 
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<AccountBalance[]> getAccountBalance() throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.tradeContextAccountBalance(this.raw, null, callback);
        });
    }

    /**
     * Get cash flow
     * 
     * <pre>
     * {@code
     * import com.longbridge.*;
     * import com.longbridge.trade.*;
     * import java.time.*;
     * 
     * class Main {
     *     public static void main(String[] args) throws Exception {
     *         try (Config config = Config.fromEnv(); TradeContext ctx = TradeContext.create(config).get()) {
     *             GetCashFlowOptions opts = new GetCashFlowOptions(
     *                     OffsetDateTime.of(2022, 5, 9, 0, 0, 0, 0, ZoneOffset.UTC),
     *                     OffsetDateTime.of(2022, 5, 12, 0, 0, 0, 0, ZoneOffset.UTC));
     *             CashFlow[] resp = ctx.getCashFlow(opts).get();
     *             for (CashFlow obj : resp) {
     *                 System.out.println(obj);
     *             }
     *         }
     *     }
     * }
     * }
     * </pre>
     * 
     * @param opts Options for this request, not null
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
     * <pre>
     * {@code
     * import com.longbridge.*;
     * import com.longbridge.trade.*;
     * 
     * class Main {
     *     public static void main(String[] args) throws Exception {
     *         try (Config config = Config.fromEnv(); TradeContext ctx = TradeContext.create(config).get()) {
     *             FundPositionsResponse resp = ctx.getFundPositions(null).get();
     *             System.out.println(resp);
     *         }
     *     }
     * }
     * }
     * </pre>
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
     * <pre>
     * {@code
     * import com.longbridge.*;
     * import com.longbridge.trade.*;
     * 
     * class Main {
     *     public static void main(String[] args) throws Exception {
     *         try (Config config = Config.fromEnv(); TradeContext ctx = TradeContext.create(config).get()) {
     *             StockPositionsResponse resp = ctx.getStockPositions(null).get();
     *             System.out.println(resp);
     *         }
     *     }
     * }
     * }
     * </pre>
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

    /**
     * Get margin ratio
     * 
     * <pre>
     * {@code
     * import com.longbridge.*;
     * import com.longbridge.trade.*;
     * 
     * class Main {
     *     public static void main(String[] args) throws Exception {
     *         try (Config config = Config.fromEnv(); TradeContext ctx = TradeContext.create(config).get()) {
     *             StockPositionsResponse resp = ctx.getMarginRatio("700.HK").get();
     *             System.out.println(resp);
     *         }
     *     }
     * }
     * }
     * </pre>
     * 
     * @param symbol Security symbol
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<MarginRatio> getMarginRatio(String symbol)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.tradeContextMarginRatio(this.raw, symbol, callback);
        });
    }

    /**
     * Get order detail
     * 
     * <pre>
     * {@code
     * import com.longbridge.*;
     * import com.longbridge.trade.*;
     * 
     * class Main {
     *     public static void main(String[] args) throws Exception {
     *         try (Config config = Config.fromEnv(); TradeContext ctx = TradeContext.create(config).get()) {
     *             OrderDetail detail = ctx.getOrderDetail("701276261045858304").get();
     *             System.out.println(resp);
     *         }
     *     }
     * }
     * }
     * </pre>
     * 
     * @param orderId Order id
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<OrderDetail> getOrderDetail(String orderId)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.tradeContextOrderDetail(this.raw, orderId, callback);
        });
    }

    /**
     * Estimating the maximum purchase quantity for Hong Kong and US stocks,
     * warrants, and options
     * 
     * @param opts Options for this request
     * @return A Future representing the result of the operation
     * @throws OpenApiException
     */
    public CompletableFuture<EstimateMaxPurchaseQuantityResponse> getEstimateMaxPurchaseQuantity(
            EstimateMaxPurchaseQuantityOptions opts)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.tradeContextEstimateMaxPurchaseQuantity(this.raw, opts, callback);
        });
    }
}
