package com.longbridge.sharelist;
import java.util.concurrent.CompletableFuture;
import com.longbridge.*;

/**
 * Community sharelist management context.
 */
public class SharelistContext implements AutoCloseable {
    private long raw;

    /**
     * Create a SharelistContext object.
     *
     * @param config Config object
     * @return A new SharelistContext instance
     */
    public static SharelistContext create(Config config) { SharelistContext ctx = new SharelistContext(); ctx.raw = SdkNative.newSharelistContext(config.getRaw()); return ctx; }

    @Override public void close() throws Exception { SdkNative.freeSharelistContext(raw); }

    /**
     * List the user's own and subscribed sharelists.
     *
     * @param count Maximum number of sharelists to return
     * @return A Future resolving to the sharelist collection
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<SharelistList> list(int count) throws OpenApiException { return AsyncCallback.executeTask((cb) -> SdkNative.sharelistContextList(raw, count, cb)); }

    /**
     * Get sharelist detail including its constituent securities.
     *
     * @param id Sharelist ID
     * @return A Future resolving to the sharelist detail
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<SharelistDetail> detail(long id) throws OpenApiException { return AsyncCallback.executeTask((cb) -> SdkNative.sharelistContextDetail(raw, id, cb)); }

    /**
     * Get popular sharelists.
     *
     * @param count Maximum number of sharelists to return
     * @return A Future resolving to the popular sharelist collection
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<SharelistList> popular(int count) throws OpenApiException { return AsyncCallback.executeTask((cb) -> SdkNative.sharelistContextPopular(raw, count, cb)); }

    /**
     * Create a new sharelist.
     *
     * @param opts Options containing the name and optional description
     * @return A Future resolving to the newly created sharelist detail
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<Void> create(CreateSharelistOptions opts) throws OpenApiException { return AsyncCallback.executeTask((cb) -> SdkNative.sharelistContextCreate(raw, opts, cb)); }

    /**
     * Add securities to a sharelist.
     *
     * @param id      Sharelist ID
     * @param symbols Array of security symbols to add
     * @return A Future that completes when the securities have been added
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<Void> addSecurities(long id, String[] symbols) throws OpenApiException { return AsyncCallback.executeTask((cb) -> SdkNative.sharelistContextAddSecurities(raw, id, symbols, cb)); }

    /**
     * Delete a sharelist.
     *
     * @param id Sharelist ID
     * @return A Future that completes when the sharelist has been deleted
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<Void> delete(long id) throws OpenApiException { return AsyncCallback.executeTask((cb) -> SdkNative.sharelistContextDelete(raw, id, cb)); }

    /**
     * Remove securities from a sharelist.
     *
     * @param id      Sharelist ID
     * @param symbols Array of security symbols to remove
     * @return A Future that completes when the securities have been removed
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<Void> removeSecurities(long id, String[] symbols) throws OpenApiException { return AsyncCallback.executeTask((cb) -> SdkNative.sharelistContextRemoveSecurities(raw, id, symbols, cb)); }

    /**
     * Reorder securities in a sharelist.
     *
     * @param id      Sharelist ID
     * @param symbols Array of security symbols in the desired order
     * @return A Future that completes when the securities have been reordered
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<Void> sortSecurities(long id, String[] symbols) throws OpenApiException { return AsyncCallback.executeTask((cb) -> SdkNative.sharelistContextSortSecurities(raw, id, symbols, cb)); }
}
