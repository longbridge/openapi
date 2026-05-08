package com.longbridge.portfolio;

import java.util.concurrent.CompletableFuture;
import com.longbridge.*;

/** Portfolio analytics context — exchange rates, P&amp;L analysis. */
public class PortfolioContext implements AutoCloseable {
    private long raw;
    public static PortfolioContext create(Config config) {
        PortfolioContext ctx = new PortfolioContext();
        ctx.raw = SdkNative.newPortfolioContext(config.getRaw());
        return ctx;
    }
    @Override public void close() throws Exception { SdkNative.freePortfolioContext(raw); }

    /**
     * Get exchange rates for supported currencies.
     *
     * @return A Future resolving to the current exchange rates
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<ExchangeRates> getExchangeRate() throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> SdkNative.portfolioContextExchangeRate(raw, callback));
    }

    /**
     * Get portfolio P&amp;L analysis (summary and per-security breakdown).
     *
     * @param opts Date range options (start, end)
     * @return A Future resolving to the profit/loss analysis
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<ProfitAnalysis> getProfitAnalysis(ProfitAnalysisOptions opts) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> SdkNative.portfolioContextProfitAnalysis(raw, opts, callback));
    }

    /**
     * Get P&amp;L detail for a specific security.
     *
     * @param opts Query options (symbol, start date, end date)
     * @return A Future resolving to the security-level profit/loss detail
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<ProfitAnalysisDetail> getProfitAnalysisDetail(ProfitAnalysisDetailOptions opts) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> SdkNative.portfolioContextProfitAnalysisDetail(raw, opts, callback));
    }

    /**
     * Get paginated P&amp;L analysis filtered by market.
     *
     * @param opts Query options (market, start, end, currency, page, size)
     * @return A Future resolving to the paginated market-level analysis
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<ProfitAnalysisByMarket> getProfitAnalysisByMarket(ProfitAnalysisByMarketOptions opts) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> SdkNative.portfolioContextProfitAnalysisByMarket(raw, opts, callback));
    }

    /**
     * Get paginated profit-analysis flow records for a security.
     *
     * @param opts Query options (symbol, page, size, includeOutsideRth, start, end)
     * @return A Future resolving to the flow records and pagination flag
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<ProfitAnalysisFlowsResponse> getProfitAnalysisFlows(ProfitAnalysisFlowsOptions opts) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> SdkNative.portfolioContextProfitAnalysisFlows(raw, opts, callback));
    }
}
