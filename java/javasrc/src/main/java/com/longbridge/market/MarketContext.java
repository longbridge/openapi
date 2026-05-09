package com.longbridge.market;

import java.util.concurrent.CompletableFuture;
import com.longbridge.*;

/**
 * Market data context — broker holdings, A/H premium, trade statistics,
 * market anomalies, index constituents and more.
 */
public class MarketContext implements AutoCloseable {
    private long raw;

    public static MarketContext create(Config config) {
        MarketContext ctx = new MarketContext();
        ctx.raw = SdkNative.newMarketContext(config.getRaw());
        return ctx;
    }

    @Override
    public void close() throws Exception {
        SdkNative.freeMarketContext(raw);
    }

    /** Get current trading status for all markets */
    public CompletableFuture<MarketStatusResponse> getMarketStatus() throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> SdkNative.marketContextMarketStatus(raw, callback));
    }

    /** Get top broker holdings. period: 0=rct_1,1=rct_5,2=rct_20,3=rct_60 */
    public CompletableFuture<BrokerHoldingTop> getBrokerHolding(BrokerHoldingOptions opts) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> SdkNative.marketContextBrokerHolding(raw, opts, callback));
    }

    /** Get full broker holding details */
    public CompletableFuture<BrokerHoldingDetail> getBrokerHoldingDetail(String symbol) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> SdkNative.marketContextBrokerHoldingDetail(raw, symbol, callback));
    }

    /** Get daily broker holding history */
    public CompletableFuture<BrokerHoldingDailyHistory> getBrokerHoldingDaily(BrokerHoldingDailyOptions opts) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> SdkNative.marketContextBrokerHoldingDaily(raw, opts, callback));
    }

    /** Get A/H premium K-lines */
    public CompletableFuture<AhPremiumKlines> getAhPremium(AhPremiumOptions opts) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> SdkNative.marketContextAhPremium(raw, opts, callback));
    }

    /** Get A/H premium intraday */
    public CompletableFuture<AhPremiumIntraday> getAhPremiumIntraday(String symbol) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> SdkNative.marketContextAhPremiumIntraday(raw, symbol, callback));
    }

    /** Get trade statistics */
    public CompletableFuture<TradeStatsResponse> getTradeStats(String symbol) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> SdkNative.marketContextTradeStats(raw, symbol, callback));
    }

    /** Get market anomaly alerts */
    public CompletableFuture<AnomalyResponse> getAnomaly(String market) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> SdkNative.marketContextAnomaly(raw, market, callback));
    }

    /** Get index constituent stocks */
    public CompletableFuture<IndexConstituents> getConstituent(String symbol) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> SdkNative.marketContextConstituent(raw, symbol, callback));
    }
}
