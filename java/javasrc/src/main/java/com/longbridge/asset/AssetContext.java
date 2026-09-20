package com.longbridge.asset;

import java.util.concurrent.CompletableFuture;

import com.longbridge.*;

/**
 * Asset context for querying and downloading account statements
 */
public class AssetContext implements AutoCloseable {
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
     * Create a AssetContext object
     *
     * @param config Config object
     * @return A AssetContext object
     */
    public static AssetContext create(Config config) {
        AssetContext ctx = new AssetContext();
        synchronized (config) { ctx.raw = SdkNative.newAssetContext(config.getRaw()); }
        return ctx;
    }

    @Override
    public synchronized void close() throws Exception {
        long h = this.raw;
        if (h != 0) {
            this.raw = 0;
            SdkNative.freeAssetContext(h);
        }
    }

    /**
     * Get statement data list
     *
     * @param opts Query options (statementType, startDate, limit); may be null
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public synchronized CompletableFuture<Object> getStatements(GetStatementListOptions opts)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.assetContextStatements(raw(), opts, callback);
        });
    }

    /**
     * Get statement data download URL
     *
     * @param fileKey File key obtained from getStatements
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public synchronized CompletableFuture<Object> getStatementDownloadUrl(String fileKey)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.assetContextDownloadUrl(raw(), fileKey, callback);
        });
    }
}
