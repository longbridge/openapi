package com.longbridge.dca;
import java.util.concurrent.CompletableFuture;
import com.longbridge.*;

/**
 * Dollar-cost averaging (DCA) plan management context.
 */
public class DcaContext implements AutoCloseable {
    private long raw;

    /**
     * Create a DcaContext object.
     *
     * @param config Config object
     * @return A new DcaContext instance
     */
    public static DcaContext create(Config config) { DcaContext ctx = new DcaContext(); ctx.raw = SdkNative.newDcaContext(config.getRaw()); return ctx; }

    @Override public void close() throws Exception { SdkNative.freeDcaContext(raw); }

    /**
     * Create a new DCA plan.
     *
     * @param opts Creation options (symbol, amount, frequency, etc.)
     * @return A Future resolving to the updated plan list
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<DcaCreateResult> createDca(DcaCreateOptions opts) throws OpenApiException { return AsyncCallback.executeTask((cb) -> SdkNative.dcaContextCreate(raw, opts, cb)); }

    /**
     * Update an existing DCA plan.
     *
     * @param opts Update options (planId, amount, frequency, etc.)
     * @return A Future resolving to the result containing the plan ID
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<DcaCreateResult> updateDca(DcaUpdateOptions opts) throws OpenApiException { return AsyncCallback.executeTask((cb) -> SdkNative.dcaContextUpdate(raw, opts, cb)); }

    /**
     * List DCA plans, optionally filtered by status and/or symbol.
     *
     * @param opts Query options (status, symbol)
     * @return A Future resolving to the list of DCA plans
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<DcaList> list(DcaListOptions opts) throws OpenApiException { return AsyncCallback.executeTask((cb) -> SdkNative.dcaContextList(raw, opts, cb)); }

    /**
     * Get DCA statistics, optionally scoped to a single security.
     *
     * @param symbol Security symbol, or {@code null} for all securities
     * @return A Future resolving to DCA statistics
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<DcaStats> stats(String symbol) throws OpenApiException { return AsyncCallback.executeTask((cb) -> SdkNative.dcaContextStats(raw, symbol, cb)); }

    /**
     * Check DCA support for a batch of securities.
     *
     * @param symbols Array of security symbols to check
     * @return A Future resolving to the support status list
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<DcaSupportList> checkSupport(String[] symbols) throws OpenApiException { return AsyncCallback.executeTask((cb) -> SdkNative.dcaContextCheckSupport(raw, symbols, cb)); }

    /**
     * Get execution history for a DCA plan.
     *
     * @param opts Query options (planId, page, limit)
     * @return A Future resolving to the execution history
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<DcaHistoryResponse> history(DcaHistoryOptions opts) throws OpenApiException { return AsyncCallback.executeTask((cb) -> SdkNative.dcaContextHistory(raw, opts, cb)); }

    /**
     * Pause a DCA plan.
     *
     * @param planId ID of the plan to pause
     * @return A Future resolving to the updated plan list
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<Void> pause(String planId) throws OpenApiException { return AsyncCallback.executeTask((cb) -> SdkNative.dcaContextPause(raw, planId, cb)); }

    /**
     * Resume a suspended DCA plan.
     *
     * @param planId ID of the plan to resume
     * @return A Future that completes when the plan is resumed
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<Void> resume(String planId) throws OpenApiException { return AsyncCallback.executeTask((cb) -> SdkNative.dcaContextResume(raw, planId, cb)); }

    /**
     * Stop (permanently finish) a DCA plan.
     *
     * @param planId ID of the plan to stop
     * @return A Future that completes when the plan is stopped
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<Void> stop(String planId) throws OpenApiException { return AsyncCallback.executeTask((cb) -> SdkNative.dcaContextStop(raw, planId, cb)); }

    /**
     * Calculate the next projected trade date for a DCA plan with the given schedule parameters.
     *
     * @param opts Calculation options (symbol, frequency, dayOfWeek, dayOfMonth)
     * @return A Future resolving to the calculated date result
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<DcaCalcDateResult> calcDate(DcaCalcDateOptions opts) throws OpenApiException { return AsyncCallback.executeTask((cb) -> SdkNative.dcaContextCalcDate(raw, opts, cb)); }

    /**
     * Update the advance reminder hours for DCA execution notifications.
     * {@code hours} must be one of {@code "1"}, {@code "6"}, or {@code "12"}.
     *
     * @param hours Number of hours before execution to send the reminder
     * @return A Future that completes when the reminder setting is updated
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<Void> setReminder(String hours) throws OpenApiException { return AsyncCallback.executeTask((cb) -> SdkNative.dcaContextSetReminder(raw, hours, cb)); }
}
