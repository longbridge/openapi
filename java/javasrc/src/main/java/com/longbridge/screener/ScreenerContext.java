package com.longbridge.screener;

import java.util.concurrent.CompletableFuture;
import com.longbridge.*;

/**
 * Screener context — stock screener strategies, search, and indicator metadata.
 */
public class ScreenerContext implements AutoCloseable {
    private long raw;

    public static ScreenerContext create(Config config) {
        ScreenerContext ctx = new ScreenerContext();
        ctx.raw = SdkNative.newScreenerContext(config.getRaw());
        return ctx;
    }

    @Override
    public void close() throws Exception {
        SdkNative.freeScreenerContext(raw);
    }

    /** Get platform-preset screener strategies for the given market (default "US"). */
    public CompletableFuture<ScreenerRecommendStrategiesResponse> getRecommendStrategies(String market) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> SdkNative.screenerContextRecommendStrategies(raw, market, callback));
    }

    /** Get platform-preset screener strategies (defaults to US market). */
    public CompletableFuture<ScreenerRecommendStrategiesResponse> getRecommendStrategies() throws OpenApiException {
        return getRecommendStrategies("US");
    }

    /** Get the current user's saved screener strategies for the given market (default "US"). */
    public CompletableFuture<ScreenerUserStrategiesResponse> getUserStrategies(String market) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> SdkNative.screenerContextUserStrategies(raw, market, callback));
    }

    /** Get the current user's saved screener strategies (defaults to US market). */
    public CompletableFuture<ScreenerUserStrategiesResponse> getUserStrategies() throws OpenApiException {
        return getUserStrategies("US");
    }

    /** Get detail for one screener strategy by ID. */
    public CompletableFuture<ScreenerStrategyResponse> getStrategy(ScreenerStrategyOptions opts) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> SdkNative.screenerContextStrategy(raw, opts, callback));
    }

    /** Search / screen securities using a strategy ID or custom filters. */
    public CompletableFuture<ScreenerSearchResponse> search(ScreenerSearchOptions opts) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> SdkNative.screenerContextSearch(raw, opts, callback));
    }

    /** Get all available screener indicator definitions. */
    public CompletableFuture<ScreenerIndicatorsResponse> getIndicators() throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> SdkNative.screenerContextIndicators(raw, callback));
    }
}
