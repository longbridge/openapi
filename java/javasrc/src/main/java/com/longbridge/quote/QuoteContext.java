package com.longbridge.quote;

import java.time.LocalDate;
import java.util.concurrent.CompletableFuture;
import com.longbridge.*;

public class QuoteContext implements AutoCloseable {
    private long raw;

    public static CompletableFuture<QuoteContext> create(Config config)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.newQuoteContext(config.getRaw(), callback);
        });
    }

    @Override
    public void close() throws Exception {
        SdkNative.freeQuoteContext(raw);
    }

    public void setOnQuote(QuoteHandler handler) {
        SdkNative.quoteContextSetOnQuote(this.raw, handler);
    }

    public void setOnDepth(DepthHandler handler) {
        SdkNative.quoteContextSetOnDepth(this.raw, handler);
    }

    public void setOnBrokers(BrokersHandler handler) {
        SdkNative.quoteContextSetOnBrokers(this.raw, handler);
    }

    public void setOnTrades(TradesHandler handler) {
        SdkNative.quoteContextSetOnTrades(this.raw, handler);
    }

    public CompletableFuture<Void> subscribe(String[] symbols, int flags, boolean isFirstPush) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextSubscribe(this.raw, symbols, flags, isFirstPush, callback);
        });
    }

    public CompletableFuture<Void> unsubscribe(String[] symbols, int flags) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextUnsubscribe(this.raw, symbols, flags, callback);
        });
    }

    public CompletableFuture<Subscription[]> subscrptions(String[] symbols, int flags) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextSubscriptions(this.raw, callback);
        });
    }

    public CompletableFuture<SecurityStaticInfo[]> staticInfo(String[] symbols) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextStaticInfo(this.raw, symbols, callback);
        });
    }

    public CompletableFuture<SecurityQuote[]> quote(String[] symbols) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextQuote(this.raw, symbols, callback);
        });
    }

    public CompletableFuture<OptionQuote[]> optionQuote(String[] symbols) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextOptionQuote(this.raw, symbols, callback);
        });
    }

    public CompletableFuture<SecurityDepth> depth(String symbol) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextDepth(this.raw, symbol, callback);
        });
    }

    public CompletableFuture<SecurityBrokers> brokers(String symbol) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextBrokers(this.raw, symbol, callback);
        });
    }

    public CompletableFuture<ParticipantInfo[]> participants() throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextParticipants(this.raw, callback);
        });
    }

    public CompletableFuture<Trade[]> trades(String symbol, int count) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextTrades(this.raw, symbol, count, callback);
        });
    }

    public CompletableFuture<IntradayLine[]> intraday(String symbol) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextIntraday(this.raw, symbol, callback);
        });
    }

    public CompletableFuture<Candlestick[]> candlesticks(String symbol, Period period, int count,
            AdjustType adjustType) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextCandlesticks(this.raw, symbol, period, count, adjustType, callback);
        });
    }

    public CompletableFuture<LocalDate[]> optionChainExpiryDateList(String symbol) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextOptionChainExpiryDateList(this.raw, symbol, callback);
        });
    }

    public CompletableFuture<StrikePriceInfo[]> optionChainExpiryDateList(String symbol, LocalDate expiryDate)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextOptionChainInfoByDate(this.raw, symbol, expiryDate, callback);
        });
    }

    public CompletableFuture<IssuerInfo[]> warrantIssuers(String symbol, LocalDate expiryDate)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextWarrantIssuers(this.raw, callback);
        });
    }

    public CompletableFuture<MarketTradingSession[]> tradingSession(String symbol, LocalDate expiryDate)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextTradingSession(this.raw, callback);
        });
    }

    public CompletableFuture<MarketTradingDays> tradingDays(Market market, LocalDate begin, LocalDate end)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextTradingDays(this.raw, market, begin, end, callback);
        });
    }

    public CompletableFuture<RealtimeQuote[]> realtimeQuote(String[] symbols)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextRealtimeQuote(this.raw, symbols, callback);
        });
    }

    public CompletableFuture<SecurityDepth> realtimeDepth(String symbol)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextRealtimeDepth(this.raw, symbol, callback);
        });
    }

    public CompletableFuture<SecurityBrokers> realtimeBrokers(String symbol)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextRealtimeBrokers(this.raw, symbol, callback);
        });
    }

    public CompletableFuture<Trade[]> realtimeBrokers(String symbol, int count)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextRealtimeTrades(this.raw, symbol, count, callback);
        });
    }
}
