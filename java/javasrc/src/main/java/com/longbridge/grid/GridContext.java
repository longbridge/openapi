package com.longbridge.grid;

import java.util.concurrent.CompletableFuture;
import com.longbridge.*;

/**
 * Grid trading order management context.
 */
public class GridContext implements AutoCloseable {
    private long raw;

    private long raw() {
        long r = this.raw;
        if (r == 0) {
            throw new IllegalStateException(
                    getClass().getSimpleName() + " has already been closed");
        }
        return r;
    }

    /**
     * Create a GridContext object.
     *
     * @param config Config object
     * @return A new GridContext instance
     */
    public static GridContext create(Config config) {
        GridContext ctx = new GridContext();
        ctx.raw = SdkNative.newGridContext(config.getRaw());
        return ctx;
    }

    @Override
    public synchronized void close() throws Exception {
        long h = this.raw;
        if (h != 0) {
            this.raw = 0;
            SdkNative.freeGridContext(h);
        }
    }

    /**
     * Submit a grid trading order
     *
     * @param opts Options for this request, not null
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<SubmitGridOrderResponse> submit(SubmitGridOrderOptions opts)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.gridContextSubmit(raw(), opts, callback);
        });
    }

    /**
     * Replace (modify) a grid trading order
     *
     * @param opts Options for this request, not null
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<Void> replace(ReplaceGridOrderOptions opts) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.gridContextReplace(raw(), opts, callback);
        });
    }

    /**
     * Get grid trading orders (paged list)
     *
     * @param opts Options for this request; may be null
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<GridOrdersResponse> list(GetGridOrdersOptions opts) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.gridContextList(raw(), opts, callback);
        });
    }

    /**
     * Query grid trading orders by IDs
     *
     * @param orderIds Grid master order IDs
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<GridOrder[]> listByIds(String[] orderIds) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.gridContextListByIds(raw(), orderIds, callback);
        });
    }

    /**
     * Get grid trading order detail (and paged history)
     *
     * @param opts Options for this request, not null
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<GridOrderDetail> detail(GetGridOrderDetailOptions opts)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.gridContextDetail(raw(), opts, callback);
        });
    }

    /**
     * Get grid trading trigger history
     *
     * @param opts Options for this request, not null
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<GridTriggerHistoryResponse> triggerHistory(GetGridTriggerHistoryOptions opts)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.gridContextTriggerHistory(raw(), opts, callback);
        });
    }

    /**
     * Cancel a grid trading order
     *
     * @param orderId Grid master order ID
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<Void> cancel(String orderId) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.gridContextCancel(raw(), orderId, callback);
        });
    }

    /**
     * Suspend a grid trading order
     *
     * @param orderId Grid master order ID
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<Void> suspend(String orderId) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.gridContextSuspend(raw(), orderId, callback);
        });
    }

    /**
     * Restart a grid trading order
     *
     * @param orderId Grid master order ID
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<Void> restart(String orderId) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.gridContextRestart(raw(), orderId, callback);
        });
    }

    /**
     * Get the security (symbol) info used to build a grid order (lot size,
     * authorization flag, settlement currency, etc.).
     *
     * @param symbol Security symbol (e.g. 700.HK)
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<GridSymbolInfo> symbolInfo(String symbol) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.gridContextSymbolInfo(raw(), symbol, callback);
        });
    }
}
