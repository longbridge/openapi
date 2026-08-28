package com.longbridge.fundamental;

import java.util.concurrent.CompletableFuture;
import com.longbridge.*;


/**
 * Fundamental data context — financial reports, analyst ratings, dividends,
 * valuation, company overview and more.
 */
public class FundamentalContext implements AutoCloseable {
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
     * Create a FundamentalContext.
     *
     * @param config Config object
     * @return A FundamentalContext object
     */
    public static FundamentalContext create(Config config) {
        FundamentalContext ctx = new FundamentalContext();
        synchronized (config) { ctx.raw = SdkNative.newFundamentalContext(config.getRaw()); }
        return ctx;
    }

    @Override
    public synchronized void close() throws Exception {
        long h = this.raw;
        if (h != 0) {
            this.raw = 0;
            SdkNative.freeFundamentalContext(h);
        }
    }

    /**
     * Get financial reports.
     *
     * @param symbol Security symbol, e.g. "700.HK"
     * @param opts   Options (kind, period); may be null
     * @return JSON string response
     */
    public synchronized CompletableFuture<FinancialReports> getFinancialReport(String symbol, FinancialReportOptions opts)
            throws OpenApiException {
        FinancialReportOptions o = opts != null ? opts : new FinancialReportOptions();
        o.symbol = symbol;
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundamentalContextFinancialReport(raw(), o, callback);
        });
    }

    /**
     * Get analyst ratings (latest + consensus summary).
     *
     * @param symbol Security symbol
     * @return JSON string response
     */
    public synchronized CompletableFuture<InstitutionRating> getInstitutionRating(String symbol)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundamentalContextInstitutionRating(raw(), symbol, callback);
        });
    }

    /**
     * Get historical analyst rating details.
     *
     * @param symbol Security symbol
     * @return JSON string response
     */
    public synchronized CompletableFuture<InstitutionRatingDetail> getInstitutionRatingDetail(String symbol)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundamentalContextInstitutionRatingDetail(raw(), symbol, callback);
        });
    }

    /**
     * Get dividend history.
     *
     * @param symbol Security symbol
     * @return JSON string response
     */
    public synchronized CompletableFuture<DividendList> getDividend(String symbol) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundamentalContextDividend(raw(), symbol, callback);
        });
    }

    /**
     * Get detailed dividend information.
     *
     * @param symbol Security symbol
     * @return JSON string response
     */
    public synchronized CompletableFuture<DividendList> getDividendDetail(String symbol) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundamentalContextDividendDetail(raw(), symbol, callback);
        });
    }

    /**
     * Get EPS forecasts.
     *
     * @param symbol Security symbol
     * @return JSON string response
     */
    public synchronized CompletableFuture<ForecastEps> getForecastEps(String symbol) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundamentalContextForecastEps(raw(), symbol, callback);
        });
    }

    /**
     * Get financial consensus estimates.
     *
     * @param symbol Security symbol
     * @return JSON string response
     */
    public synchronized CompletableFuture<FinancialConsensus> getConsensus(String symbol) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundamentalContextConsensus(raw(), symbol, callback);
        });
    }

    /**
     * Get valuation metrics (PE / PB / PS / dividend yield).
     *
     * @param symbol Security symbol
     * @return JSON string response
     */
    public synchronized CompletableFuture<ValuationData> getValuation(String symbol) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundamentalContextValuation(raw(), symbol, callback);
        });
    }

    /**
     * Get historical valuation data.
     *
     * @param symbol Security symbol
     * @return JSON string response
     */
    public synchronized CompletableFuture<ValuationHistoryResponse> getValuationHistory(String symbol) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundamentalContextValuationHistory(raw(), symbol, callback);
        });
    }

    /**
     * Get industry peer valuation comparison.
     *
     * @param symbol Security symbol
     * @return JSON string response
     */
    public synchronized CompletableFuture<IndustryValuationList> getIndustryValuation(String symbol)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundamentalContextIndustryValuation(raw(), symbol, callback);
        });
    }

    /**
     * Get industry valuation distribution.
     *
     * @param symbol Security symbol
     * @return JSON string response
     */
    public synchronized CompletableFuture<IndustryValuationDist> getIndustryValuationDist(String symbol)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundamentalContextIndustryValuationDist(raw(), symbol, callback);
        });
    }

    /**
     * Get company overview.
     *
     * @param symbol Security symbol
     * @return JSON string response
     */
    public synchronized CompletableFuture<CompanyOverview> getCompany(String symbol) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundamentalContextCompany(raw(), symbol, callback);
        });
    }

    /**
     * Get executive and board member information.
     *
     * @param symbol Security symbol
     * @return JSON string response
     */
    public synchronized CompletableFuture<ExecutiveList> getExecutive(String symbol) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundamentalContextExecutive(raw(), symbol, callback);
        });
    }

    /**
     * Get major shareholders.
     *
     * @param symbol Security symbol
     * @return JSON string response
     */
    public synchronized CompletableFuture<ShareholderList> getShareholder(String symbol) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundamentalContextShareholder(raw(), symbol, callback);
        });
    }

    /**
     * Get fund and ETF holders.
     *
     * @param symbol Security symbol
     * @return JSON string response
     */
    public synchronized CompletableFuture<FundHolders> getFundHolder(String symbol) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundamentalContextFundHolder(raw(), symbol, callback);
        });
    }

    /**
     * Get corporate actions.
     *
     * @param symbol Security symbol
     * @return JSON string response
     */
    public synchronized CompletableFuture<CorpActions> getCorpAction(String symbol) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundamentalContextCorpAction(raw(), symbol, callback);
        });
    }

    /**
     * Get investor relations data.
     *
     * @param symbol Security symbol
     * @return JSON string response
     */
    public synchronized CompletableFuture<InvestRelations> getInvestRelation(String symbol) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundamentalContextInvestRelation(raw(), symbol, callback);
        });
    }

    /**
     * Get operating metrics and financial report summaries.
     *
     * @param symbol Security symbol
     * @return JSON string response
     */
    public synchronized CompletableFuture<OperatingList> getOperating(String symbol) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundamentalContextOperating(raw(), symbol, callback);
        });
    }

    /** Get buyback data. */
    public synchronized CompletableFuture<BuybackData> getBuyback(String symbol) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundamentalContextGetBuyback(raw(), symbol, callback);
        });
    }

    // TODO: temporarily disabled — endpoint not yet open (/v1/quote/ratings)
    // /** Get stock ratings. */
    // public synchronized CompletableFuture<StockRatings> getRatings(String symbol) throws OpenApiException {
    //     return AsyncCallback.executeTask((callback) -> {
    //         SdkNative.fundamentalContextGetRatings(raw(), symbol, callback);
    //     });
    // }

    /** Get business segment breakdowns (latest snapshot). */
    public synchronized CompletableFuture<BusinessSegments> getBusinessSegments(String symbol)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundamentalContextGetBusinessSegments(raw(), symbol, callback);
        });
    }

    /** Get historical business segment breakdowns. */
    public synchronized CompletableFuture<BusinessSegmentsHistory> getBusinessSegmentsHistory(
            BusinessSegmentsHistoryOptions opts) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundamentalContextGetBusinessSegmentsHistory(raw(), opts, callback);
        });
    }

    /** Get historical institutional rating view time-series. */
    public synchronized CompletableFuture<InstitutionRatingViews> getInstitutionRatingViews(String symbol)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundamentalContextGetInstitutionRatingViews(raw(), symbol, callback);
        });
    }

    /** Get industry rank for a market. */
    public synchronized CompletableFuture<IndustryRankResponse> getIndustryRank(IndustryRankOptions opts)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundamentalContextGetIndustryRank(raw(), opts, callback);
        });
    }

    /** Get the industry peer chain for a security or industry. */
    public synchronized CompletableFuture<IndustryPeersResponse> getIndustryPeers(IndustryPeersOptions opts)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundamentalContextGetIndustryPeers(raw(), opts, callback);
        });
    }

    /** Get a financial report snapshot (earnings snapshot). */
    public synchronized CompletableFuture<FinancialReportSnapshot> getFinancialReportSnapshot(
            FinancialReportSnapshotOptions opts) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundamentalContextGetFinancialReportSnapshot(raw(), opts, callback);
        });
    }

    /** Get top 20 major shareholders with multi-period holdings. */
    public synchronized CompletableFuture<ShareholderTopResponse> getShareholderTop(String symbol) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundamentalContextShareholderTop(raw(), symbol, callback);
        });
    }

    /** Get holding history and trade detail for a specific shareholder. */
    public synchronized CompletableFuture<ShareholderDetailResponse> getShareholderDetail(ShareholderDetailOptions opts) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundamentalContextShareholderDetail(raw(), opts, callback);
        });
    }

    /** Get valuation comparison between a symbol and optional peer symbols. */
    public synchronized CompletableFuture<ValuationComparisonResponse> getValuationComparison(ValuationComparisonOptions opts) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundamentalContextValuationComparison(raw(), opts, callback);
        });
    }

    /**
     * List macroeconomic indicators.
     * country: ISO country code string (e.g. "US", "CN", "EU"); pass null for all countries.
     */
    public synchronized CompletableFuture<MacroeconomicIndicatorListResponse> getMacroeconomicIndicators(String country, String keyword, Integer offset, Integer limit) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundamentalContextMacroeconomicIndicators(raw(), country, keyword, offset, limit, callback);
        });
    }

    /**
     * Get historical data for a macroeconomic indicator.
     * startDate and endDate are date strings in "YYYY-MM-DD" format.
     */
    public synchronized CompletableFuture<MacroeconomicResponse> getMacroeconomic(String indicatorCode, String startDate, String endDate, Integer offset, Integer limit) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundamentalContextMacroeconomic(raw(), indicatorCode, startDate, endDate, offset, limit, callback);
        });
    }

    /**
     * List macroeconomic indicators (v2) with optional keyword filter.
     * country: "HK","CN","US","EU","JP","SG" or null for ALL.
     * keyword: optional fuzzy filter on indicator name.
     */
}
