package com.longbridge.alert;
import java.util.concurrent.CompletableFuture;
import com.longbridge.*;

/**
 * Price alert management context.
 */
public class AlertContext implements AutoCloseable {
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
     * Create an AlertContext object.
     *
     * @param config Config object
     * @return A new AlertContext instance
     */
    public static AlertContext create(Config config) { AlertContext ctx = new AlertContext(); synchronized (config) { ctx.raw = SdkNative.newAlertContext(config.getRaw()); } return ctx; }

    @Override
    public synchronized void close() throws Exception {
        long h = this.raw;
        if (h != 0) {
            this.raw = 0;
            SdkNative.freeAlertContext(h);
        }
    }

    /**
     * List all price alerts.
     *
     * @return A Future resolving to the list of price alerts
     * @throws OpenApiException If an error occurs
     */
    public synchronized CompletableFuture<AlertList> list() throws OpenApiException { return AsyncCallback.executeTask((cb) -> SdkNative.alertContextList(raw(), cb)); }

    /**
     * Add a price alert.
     *
     * @param opts Alert options (symbol, condition, trigger value, frequency)
     * @return A Future that completes when the alert is added
     * @throws OpenApiException If an error occurs
     */
    public synchronized CompletableFuture<Void> add(AddAlertOptions opts) throws OpenApiException { return AsyncCallback.executeTask((cb) -> SdkNative.alertContextAdd(raw(), opts, cb)); }

    /**
     * Update a price alert (e.g. enable/disable or change its trigger).
     *
     * <p>Typically obtained from {@link #list()}; set {@code item.enabled} to
     * {@code true} to re-enable or {@code false} to disable, then pass it here.
     *
     * @param item The alert item to update
     * @return A Future that completes when the alert is updated
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<Void> update(AlertItem item) throws OpenApiException { return AsyncCallback.executeTask((cb) -> SdkNative.alertContextUpdate(raw(), item, cb)); }

    /**
     * Delete price alerts.
     *
     * @param opts Options containing the alert IDs to delete
     * @return A Future that completes when the alerts are deleted
     * @throws OpenApiException If an error occurs
     */
    public synchronized CompletableFuture<Void> delete(DeleteAlertOptions opts) throws OpenApiException { return AsyncCallback.executeTask((cb) -> SdkNative.alertContextDelete(raw(), opts, cb)); }
}
