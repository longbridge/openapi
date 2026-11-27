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

    /**
     * Get the linked bank cards, with their verification, binding and
     * authorization (eDDA, bank securities account, FPS) status.
     * <p>
     * AP accounts only: US data-center accounts get a region error.
     *
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public synchronized CompletableFuture<BankCard[]> getBankCards() throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.assetContextBankCards(raw(), callback);
        });
    }

    /**
     * Get the eDDA authorization applications of a bank card in one currency.
     * The card id must be all digits; any other value fails with an error
     * before anything is sent.
     * <p>
     * AP accounts only: US data-center accounts get a region error.
     *
     * @param cardId   Bank card ID ({@link BankCard#getId()})
     * @param currency Currency, e.g. {@code HKD} or {@code USD}
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public synchronized CompletableFuture<EddaApplication[]> getEddaApplications(String cardId, String currency)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.assetContextEddaApplications(raw(), cardId, currency, callback);
        });
    }

    /**
     * Get stock transfer-in applications (another broker to this account).
     * <p>
     * AP accounts only: US data-center accounts get a region error.
     *
     * @param opts Query options; may be null
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public synchronized CompletableFuture<StockTransferInResponse> getStockTransfersIn(GetStockTransfersInOptions opts)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.assetContextStockTransfersIn(raw(), opts, callback);
        });
    }

    /**
     * Get stock transfer-out applications (this account to another broker).
     * <p>
     * AP accounts only: US data-center accounts get a region error.
     *
     * @param opts Query options; may be null
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public synchronized CompletableFuture<StockTransferOut[]> getStockTransfersOut(GetStockTransfersOutOptions opts)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.assetContextStockTransfersOut(raw(), opts, callback);
        });
    }

    /**
     * Get reward records from the reward center (cash, gifts, coupons, task
     * rewards).
     * <p>
     * The route has no date filter: to find rewards in a time range, page
     * through the records and filter on their times.
     * <p>
     * AP accounts only: US data-center accounts get a region error.
     *
     * @param opts Query options; may be null
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public synchronized CompletableFuture<RewardRecordsResponse> getRewardRecords(GetRewardRecordsOptions opts)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.assetContextRewardRecords(raw(), opts, callback);
        });
    }

    /**
     * Get coupons (reward cards). By default lists the coupons that have not
     * expired. The pinned top / bottom lists are repeated on every page.
     * <p>
     * AP accounts only: US data-center accounts get a region error.
     *
     * @param opts Query options; may be null
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public synchronized CompletableFuture<CouponsResponse> getCoupons(GetCouponsOptions opts)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.assetContextCoupons(raw(), opts, callback);
        });
    }

    /**
     * Get the orders a coupon was applied to. The gateway rate-limits this
     * route tightly per member, so avoid calling it concurrently. The card id
     * must be 32 hex characters; any other value fails with an error before
     * anything is sent.
     * <p>
     * AP accounts only: US data-center accounts get a region error.
     *
     * @param cardId    Coupon ID ({@link Coupon#getCardId()}, 32 hex
     *                  characters; not a bank card ID)
     * @param orderType Which orders to list
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public synchronized CompletableFuture<CouponOrder[]> getCouponOrders(String cardId, CouponOrderType orderType)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.assetContextCouponOrders(raw(), cardId, orderType, callback);
        });
    }

    /**
     * Get one page of deposit records. The server's total follows the page
     * size rather than the record count, so it is not returned;
     * {@link DepositsResponse#getHasMore()} reports a full page instead. The
     * page size is always sent: at most 20, default 20.
     * <p>
     * AP accounts only: US data-center accounts get a region error.
     *
     * @param opts Query options; may be null
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public synchronized CompletableFuture<DepositsResponse> getDeposits(GetDepositsOptions opts)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.assetContextDeposits(raw(), opts, callback);
        });
    }

    /**
     * Get withdrawal records. {@link WithdrawalsResponse#getHasMore()} reports
     * a full page of the size sent (default 20, at most 50).
     * <p>
     * AP accounts only: US data-center accounts get a region error.
     *
     * @param opts Query options; may be null
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public synchronized CompletableFuture<WithdrawalsResponse> getWithdrawals(GetWithdrawalsOptions opts)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.assetContextWithdrawals(raw(), opts, callback);
        });
    }
}
