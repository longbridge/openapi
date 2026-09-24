package com.longbridge.fund;

import java.util.concurrent.CompletableFuture;
import com.longbridge.*;

/**
 * Fund (mutual fund) channel context.
 */
public class FundContext implements AutoCloseable {
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
     * Create a FundContext object.
     *
     * @param config Config object
     * @return A new FundContext instance
     */
    public static FundContext create(Config config) {
        FundContext ctx = new FundContext();
        ctx.raw = SdkNative.newFundContext(config.getRaw());
        return ctx;
    }

    @Override
    public synchronized void close() throws Exception {
        long h = this.raw;
        if (h != 0) {
            this.raw = 0;
            SdkNative.freeFundContext(h);
        }
    }

    // ----- fund catalog / market data -----

    /**
     * Get the hot-selling fund list.
     *
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<HotFund[]> hotFunds() throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundContextHotFunds(raw(), callback);
        });
    }

    /**
     * Get the fund list.
     *
     * @param opts Options for this request; may be null
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<FundBrief[]> funds(GetFundsOptions opts) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundContextFunds(raw(), opts, callback);
        });
    }

    /**
     * Get the fund list filter options.
     *
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<FundFilters> filters() throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundContextFilters(raw(), callback);
        });
    }

    /**
     * Get fund detail.
     *
     * @param counterId Fund counter id
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<FundDetail> detail(String counterId) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundContextDetail(raw(), counterId, callback);
        });
    }

    /**
     * Get fund analysis (level 1).
     *
     * @param counterId Fund counter id
     * @param opts   Options for this request; may be null
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<FundAnalysis> analysis(String counterId, GetFundAnalysisOptions opts)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundContextAnalysis(raw(), counterId, opts, callback);
        });
    }

    /**
     * Get fund analysis detail (level 2).
     *
     * @param counterId Fund counter id
     * @param opts   Options for this request; may be null
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<FundAnalysisDetail> analysisDetail(String counterId, GetFundAnalysisOptions opts)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundContextAnalysisDetail(raw(), counterId, opts, callback);
        });
    }

    /**
     * Get fund trend chart.
     *
     * @param counterId Fund counter id
     * @param opts   Options for this request; may be null
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<FundTrend> trend(String counterId, GetFundAnalysisOptions opts)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundContextTrend(raw(), counterId, opts, callback);
        });
    }

    /**
     * Get fund annual returns.
     *
     * @param counterId Fund counter id
     * @param opts   Options for this request; may be null
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<FundAnnualReturn[]> annualReturns(String counterId, FundPageOptions opts)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundContextAnnualReturns(raw(), counterId, opts, callback);
        });
    }

    /**
     * Get fund quarterly returns.
     *
     * @param counterId Fund counter id
     * @param opts   Options for this request; may be null
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<FundQuarterlyReturn[]> quarterlyReturns(String counterId, FundPageOptions opts)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundContextQuarterlyReturns(raw(), counterId, opts, callback);
        });
    }

    /**
     * Get fund performance figures.
     *
     * @param counterId Fund counter id
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<FundPerformance[]> performance(String counterId) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundContextPerformance(raw(), counterId, callback);
        });
    }

    /**
     * Get fund performance comparison.
     *
     * @param counterId Fund counter id
     * @param opts   Options for this request; may be null
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<FundPerformanceComparison> performanceComparison(String counterId,
            GetFundAnalysisOptions opts) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundContextPerformanceComparison(raw(), counterId, opts, callback);
        });
    }

    /**
     * Get fund latest net value.
     *
     * @param counterId Fund counter id
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<FundNavValue[]> nav(String counterId) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundContextNav(raw(), counterId, callback);
        });
    }

    /**
     * Get fund historical net value (paged).
     *
     * @param counterId Fund counter id
     * @param opts   Options for this request; may be null
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<FundNavValue[]> navHistory(String counterId, FundPageOptions opts)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundContextNavHistory(raw(), counterId, opts, callback);
        });
    }

    /**
     * Get fund historical net value by relative time range.
     *
     * @param counterId Fund counter id
     * @param opts   Options for this request; may be null
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<FundNavValue[]> navRange(String counterId, FundNavRangeOptions opts)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundContextNavRange(raw(), counterId, opts, callback);
        });
    }

    /**
     * Get a fund's top-10 holdings.
     *
     * @param counterId Fund counter id
     * @param opts   Options for this request; may be null
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<FundHoldings> holdings(String counterId, GetFundHoldingsOptions opts)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundContextHoldings(raw(), counterId, opts, callback);
        });
    }

    /**
     * Get the stocks held by a fund (reverse lookup).
     *
     * @param counterId Fund counter id
     * @param opts   Options for this request; may be null
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<FundStockHolding[]> stockHoldings(String counterId, GetFundStockHoldingsOptions opts)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundContextStockHoldings(raw(), counterId, opts, callback);
        });
    }

    // ----- user fund positions -----

    /**
     * Get the user's fund positions overview.
     *
     * @param opts Options for this request; may be null
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<FundPositions> positions(GetFundPositionsOptions opts) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundContextPositions(raw(), opts, callback);
        });
    }

    /**
     * Get the user's single fund position detail.
     *
     * @param counterId Fund counter id
     * @param opts   Options for this request; may be null
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<FundPositionDetail> position(String counterId, GetFundPositionOptions opts)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundContextPosition(raw(), counterId, opts, callback);
        });
    }

    /**
     * Get the performance figures of a held fund.
     *
     * @param counterId Fund counter id
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<FundPositionPerformance[]> positionPerformance(String counterId)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundContextPositionPerformance(raw(), counterId, callback);
        });
    }

    /**
     * Get the cumulative-profit series of a held fund.
     *
     * @param counterId Fund counter id
     * @param opts   Options for this request; may be null
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<FundPositionProfits> positionProfits(String counterId, GetFundPositionProfitsOptions opts)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundContextPositionProfits(raw(), counterId, opts, callback);
        });
    }

    /**
     * Get the net-value history of a held fund.
     *
     * @param counterId Fund counter id
     * @param opts   Options for this request; may be null
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<FundPositionNav[]> positionNav(String counterId, FundNavRangeOptions opts)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundContextPositionNav(raw(), counterId, opts, callback);
        });
    }

    /**
     * Get the dividend records of a held fund.
     *
     * @param counterId Fund counter id
     * @param opts   Options for this request; may be null
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<FundDividends> positionDividends(String counterId, GetFundPositionDividendsOptions opts)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundContextPositionDividends(raw(), counterId, opts, callback);
        });
    }

    // ----- fund orders & trading -----

    /**
     * Get the user's fund orders.
     *
     * @param opts Options for this request; may be null
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<FundOrder[]> orders(GetFundOrdersOptions opts) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundContextOrders(raw(), opts, callback);
        });
    }

    /**
     * Get a fund order detail.
     *
     * @param orderId Fund order id
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<FundOrderDetail> order(long orderId) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundContextOrder(raw(), orderId, callback);
        });
    }

    /**
     * Get the user's fund transactions (cash-flow records).
     *
     * @param opts Options for this request; may be null
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<FundTransaction[]> transactions(GetFundTransactionsOptions opts)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundContextTransactions(raw(), opts, callback);
        });
    }

    /**
     * Validate a fund order before submitting.
     *
     * @param opts Options for this request, not null
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<FundOrderValidation> validateOrder(ValidateFundOrderOptions opts)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundContextValidateOrder(raw(), opts, callback);
        });
    }

    /**
     * Submit a fund order (buy / sell).
     *
     * @param opts Options for this request, not null
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<FundOrderSubmitResponse> submitOrder(SubmitFundOrderOptions opts)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundContextSubmitOrder(raw(), opts, callback);
        });
    }

    /**
     * Cancel (withdraw) a fund order.
     *
     * @param orderId Fund order id
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<Void> cancelOrder(long orderId) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.fundContextCancelOrder(raw(), orderId, callback);
        });
    }
}
