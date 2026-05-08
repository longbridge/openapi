package com.longbridge.alert;
import java.util.concurrent.CompletableFuture;
import com.longbridge.*;

/**
 * Price alert management context.
 */
public class AlertContext implements AutoCloseable {
    private long raw;

    /**
     * Create an AlertContext object.
     *
     * @param config Config object
     * @return A new AlertContext instance
     */
    public static AlertContext create(Config config) { AlertContext ctx = new AlertContext(); ctx.raw = SdkNative.newAlertContext(config.getRaw()); return ctx; }

    @Override public void close() throws Exception { SdkNative.freeAlertContext(raw); }

    /**
     * List all price alerts.
     *
     * @return A Future resolving to the list of price alerts
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<AlertList> list() throws OpenApiException { return AsyncCallback.executeTask((cb) -> SdkNative.alertContextList(raw, cb)); }

    /**
     * Add a price alert.
     *
     * @param opts Alert options (symbol, condition, trigger value, frequency)
     * @return A Future that completes when the alert is added
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<Void> add(AddAlertOptions opts) throws OpenApiException { return AsyncCallback.executeTask((cb) -> SdkNative.alertContextAdd(raw, opts, cb)); }

    /**
     * Enable a price alert.
     *
     * @param alertId ID of the alert to enable
     * @return A Future that completes when the alert is enabled
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<Void> enable(String alertId) throws OpenApiException { return AsyncCallback.executeTask((cb) -> SdkNative.alertContextEnable(raw, alertId, cb)); }

    /**
     * Disable a price alert.
     *
     * @param alertId ID of the alert to disable
     * @return A Future that completes when the alert is disabled
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<Void> disable(String alertId) throws OpenApiException { return AsyncCallback.executeTask((cb) -> SdkNative.alertContextDisable(raw, alertId, cb)); }

    /**
     * Delete price alerts.
     *
     * @param opts Options containing the alert IDs to delete
     * @return A Future that completes when the alerts are deleted
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<Void> delete(DeleteAlertOptions opts) throws OpenApiException { return AsyncCallback.executeTask((cb) -> SdkNative.alertContextDelete(raw, opts, cb)); }
}
