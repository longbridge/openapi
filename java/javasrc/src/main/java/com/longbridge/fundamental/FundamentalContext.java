package com.longbridge.fundamental;

import java.util.concurrent.CompletableFuture;
import com.longbridge.*;


/**
 * Fundamental data context — financial reports, analyst ratings, dividends,
 * valuation, company overview and more.
 */
public class FundamentalContext implements AutoCloseable {
    private long raw;

    /**
     * Create a FundamentalContext.
     *
     * @param config Config object
     * @return A FundamentalContext object
     */
    public static FundamentalContext create(Config config) {
        FundamentalContext ctx = new FundamentalContext();
        ctx.raw = SdkNative.newFundamentalContext(config.getRaw());
        return ctx;
    }

    @Override
    public void close() throws Exception {
        SdkNative.freeFundamentalContext(raw);
    }

    /**
     * Get financial reports.
     *
     * @param symbol Security symbol, e.g. "700.HK"
     * @param opts   Options (kind, period); may be null
     * @return JSON string response
     */
    public CompletableFuture<FinancialReports> getFinancialReport(String symbol, FinancialReportOptions opts)
            throws OpenApiException {
        FinancialReportOptions o = opts != null ? opts : new FinancialReportOptions();
        o.symbol = symbol;
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundamentalContextFinancialReport(raw, o, callback);
        });
    }

    /**
     * Get analyst ratings (latest + consensus summary).
     *
     * @param symbol Security symbol
     * @return JSON string response
     */
    public CompletableFuture<InstitutionRating> getInstitutionRating(String symbol)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundamentalContextInstitutionRating(raw, symbol, callback);
        });
    }

    /**
     * Get historical analyst rating details.
     *
     * @param symbol Security symbol
     * @return JSON string response
     */
    public CompletableFuture<InstitutionRatingDetail> getInstitutionRatingDetail(String symbol)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundamentalContextInstitutionRatingDetail(raw, symbol, callback);
        });
    }

    /**
     * Get dividend history.
     *
     * @param symbol Security symbol
     * @return JSON string response
     */
    public CompletableFuture<DividendList> getDividend(String symbol) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundamentalContextDividend(raw, symbol, callback);
        });
    }

    /**
     * Get detailed dividend information.
     *
     * @param symbol Security symbol
     * @return JSON string response
     */
    public CompletableFuture<DividendList> getDividendDetail(String symbol) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundamentalContextDividendDetail(raw, symbol, callback);
        });
    }

    /**
     * Get EPS forecasts.
     *
     * @param symbol Security symbol
     * @return JSON string response
     */
    public CompletableFuture<ForecastEps> getForecastEps(String symbol) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundamentalContextForecastEps(raw, symbol, callback);
        });
    }

    /**
     * Get financial consensus estimates.
     *
     * @param symbol Security symbol
     * @return JSON string response
     */
    public CompletableFuture<FinancialConsensus> getConsensus(String symbol) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundamentalContextConsensus(raw, symbol, callback);
        });
    }

    /**
     * Get valuation metrics (PE / PB / PS / dividend yield).
     *
     * @param symbol Security symbol
     * @return JSON string response
     */
    public CompletableFuture<ValuationData> getValuation(String symbol) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundamentalContextValuation(raw, symbol, callback);
        });
    }

    /**
     * Get historical valuation data.
     *
     * @param symbol Security symbol
     * @return JSON string response
     */
    public CompletableFuture<ValuationHistoryResponse> getValuationHistory(String symbol) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundamentalContextValuationHistory(raw, symbol, callback);
        });
    }

    /**
     * Get industry peer valuation comparison.
     *
     * @param symbol Security symbol
     * @return JSON string response
     */
    public CompletableFuture<IndustryValuationList> getIndustryValuation(String symbol)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundamentalContextIndustryValuation(raw, symbol, callback);
        });
    }

    /**
     * Get industry valuation distribution.
     *
     * @param symbol Security symbol
     * @return JSON string response
     */
    public CompletableFuture<IndustryValuationDist> getIndustryValuationDist(String symbol)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundamentalContextIndustryValuationDist(raw, symbol, callback);
        });
    }

    /**
     * Get company overview.
     *
     * @param symbol Security symbol
     * @return JSON string response
     */
    public CompletableFuture<CompanyOverview> getCompany(String symbol) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundamentalContextCompany(raw, symbol, callback);
        });
    }

    /**
     * Get executive and board member information.
     *
     * @param symbol Security symbol
     * @return JSON string response
     */
    public CompletableFuture<ExecutiveList> getExecutive(String symbol) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundamentalContextExecutive(raw, symbol, callback);
        });
    }

    /**
     * Get major shareholders.
     *
     * @param symbol Security symbol
     * @return JSON string response
     */
    public CompletableFuture<ShareholderList> getShareholder(String symbol) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundamentalContextShareholder(raw, symbol, callback);
        });
    }

    /**
     * Get fund and ETF holders.
     *
     * @param symbol Security symbol
     * @return JSON string response
     */
    public CompletableFuture<FundHolders> getFundHolder(String symbol) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundamentalContextFundHolder(raw, symbol, callback);
        });
    }

    /**
     * Get corporate actions.
     *
     * @param symbol Security symbol
     * @return JSON string response
     */
    public CompletableFuture<CorpActions> getCorpAction(String symbol) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundamentalContextCorpAction(raw, symbol, callback);
        });
    }

    /**
     * Get investor relations data.
     *
     * @param symbol Security symbol
     * @return JSON string response
     */
    public CompletableFuture<InvestRelations> getInvestRelation(String symbol) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundamentalContextInvestRelation(raw, symbol, callback);
        });
    }

    /**
     * Get operating metrics and financial report summaries.
     *
     * @param symbol Security symbol
     * @return JSON string response
     */
    public CompletableFuture<OperatingList> getOperating(String symbol) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundamentalContextOperating(raw, symbol, callback);
        });
    }

    /** Get buyback data. */
    public CompletableFuture<BuybackData> getBuyback(String symbol) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundamentalContextGetBuyback(raw, symbol, callback);
        });
    }

    /** Get stock ratings. */
    public CompletableFuture<StockRatings> getRatings(String symbol) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundamentalContextGetRatings(raw, symbol, callback);
        });
    }
}
