package com.longbridge.asset;

import java.util.concurrent.CompletableFuture;

import com.longbridge.*;

/**
 * Statement context for querying and downloading account statements
 */
public class StatementContext implements AutoCloseable {
    private long raw;

    /**
     * Create a StatementContext object
     *
     * @param config Config object
     * @return A StatementContext object
     */
    public static StatementContext create(Config config) {
        StatementContext ctx = new StatementContext();
        ctx.raw = SdkNative.newStatementContext(config.getRaw());
        return ctx;
    }

    @Override
    public void close() throws Exception {
        SdkNative.freeStatementContext(raw);
    }

    /**
     * Get statement data list
     *
     * @param opts Query options (statementType, startDate, limit); may be null
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<Object> getStatements(GetStatementListOptions opts)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.statementContextStatements(raw, opts, callback);
        });
    }

    /**
     * Get statement data download URL
     *
     * @param fileKey File key obtained from getStatements
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<Object> getStatementDownloadUrl(String fileKey)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.statementContextDownloadUrl(raw, fileKey, callback);
        });
    }
}
