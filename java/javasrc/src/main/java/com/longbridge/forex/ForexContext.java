package com.longbridge.forex;

import java.math.BigDecimal;
import java.util.concurrent.CompletableFuture;
import com.longbridge.*;

/**
 * Forex (currency exchange) channel context.
 */
public class ForexContext implements AutoCloseable {
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
     * Create a ForexContext object.
     *
     * @param config Config object
     * @return A new ForexContext instance
     */
    public static ForexContext create(Config config) {
        ForexContext ctx = new ForexContext();
        ctx.raw = SdkNative.newForexContext(config.getRaw());
        return ctx;
    }

    @Override
    public synchronized void close() throws Exception {
        long h = this.raw;
        if (h != 0) {
            this.raw = 0;
            SdkNative.freeForexContext(h);
        }
    }

    /**
     * Get a forex quote.
     *
     * @param from         Convert-out currency (ISO 4217, e.g. {@code USD})
     * @param to           Convert-in currency (e.g. {@code HKD})
     * @param amount       Convert-out amount; may be null
     * @param targetAmount Convert-in amount; may be null
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<ForexQuote> quote(String from, String to, BigDecimal amount,
            BigDecimal targetAmount) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.forexContextQuote(raw(), from, to, amount, targetAmount, callback);
        });
    }

    /**
     * Submit a forex order.
     *
     * <p>
     * Success only means the order was accepted; conversion is asynchronous — poll
     * {@link #order(String)} with the same {@code clientOrderId} for the final state.
     *
     * @param quoteId       Quote id returned by {@link #quote}
     * @param clientOrderId Caller-supplied unique order id
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<Void> submitOrder(String quoteId, String clientOrderId)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.forexContextSubmitOrder(raw(), quoteId, clientOrderId, callback);
        });
    }

    /**
     * Query a forex order by {@code clientOrderId}.
     *
     * @param clientOrderId Caller-supplied order id
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<ForexOrderDetail> order(String clientOrderId) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.forexContextOrder(raw(), clientOrderId, callback);
        });
    }
}
