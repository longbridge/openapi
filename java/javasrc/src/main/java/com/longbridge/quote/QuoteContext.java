package com.longbridge.quote;

import java.time.LocalDate;
import java.util.concurrent.CompletableFuture;
import com.longbridge.*;

/**
 * Quote context
 */
public class QuoteContext implements AutoCloseable {
    private long raw;

    /**
     * Create a QuoteContext object
     * 
     * @param config Config object
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
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

    /**
     * Set quote callback, after receiving the quote data push, it will call back to
     * this handler.
     * 
     * @param handler A quote handler
     */
    public void setOnQuote(QuoteHandler handler) {
        SdkNative.quoteContextSetOnQuote(this.raw, handler);
    }

    /**
     * Set quote callback, after receiving the depth data push, it will call back to
     * this handler.
     * 
     * @param handler A depth handler
     */
    public void setOnDepth(DepthHandler handler) {
        SdkNative.quoteContextSetOnDepth(this.raw, handler);
    }

    /**
     * Set quote callback, after receiving the brokers data push, it will call back
     * to this handler.
     * 
     * @param handler A brokers handler
     */
    public void setOnBrokers(BrokersHandler handler) {
        SdkNative.quoteContextSetOnBrokers(this.raw, handler);
    }

    /**
     * Set quote callback, after receiving the trades data push, it will call backto
     * this handler.
     * 
     * @param handler A trades handler
     */
    public void setOnTrades(TradesHandler handler) {
        SdkNative.quoteContextSetOnTrades(this.raw, handler);
    }

    /**
     * Subscribe
     * 
     * @param symbols     Security symbols
     * @param flags       Subscription flags
     * @param isFirstPush Whether to perform a data push immediately after
     *                    subscribing.
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<Void> subscribe(String[] symbols, int flags, boolean isFirstPush) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextSubscribe(this.raw, symbols, flags, isFirstPush, callback);
        });
    }

    /**
     * Unsubscribe
     * 
     * @param symbols Security symbols
     * @param flags   Subscription flags
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<Void> unsubscribe(String[] symbols, int flags) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextUnsubscribe(this.raw, symbols, flags, callback);
        });
    }

    /**
     * Get subscription information
     * 
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<Subscription[]> getSubscrptions() throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextSubscriptions(this.raw, callback);
        });
    }

    /**
     * Get basic information of securities
     * 
     * @param symbols Security symbols
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<SecurityStaticInfo[]> getStaticInfo(String[] symbols) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextStaticInfo(this.raw, symbols, callback);
        });
    }

    /**
     * Get quote of securities
     * 
     * @param symbols Security symbols
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<SecurityQuote[]> getQuote(String[] symbols) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextQuote(this.raw, symbols, callback);
        });
    }

    /**
     * Get quote of option securities
     * 
     * @param symbols Security symbols
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<OptionQuote[]> getOptionQuote(String[] symbols) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextOptionQuote(this.raw, symbols, callback);
        });
    }

    /**
     * Get security depth
     * 
     * @param symbol Security symbol
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<SecurityDepth> getDepth(String symbol) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextDepth(this.raw, symbol, callback);
        });
    }

    /**
     * Get security brokers
     * 
     * @param symbol Security symbol
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<SecurityBrokers> getBrokers(String symbol) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextBrokers(this.raw, symbol, callback);
        });
    }

    /**
     * Get participants
     * 
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<ParticipantInfo[]> getParticipants() throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextParticipants(this.raw, callback);
        });
    }

    /**
     * Get security trades
     * 
     * @param symbol Security symbol
     * @param count  Count of trades
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<Trade[]> getTrades(String symbol, int count) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextTrades(this.raw, symbol, count, callback);
        });
    }

    /**
     * Get security intraday lines
     * 
     * @param symbol Security symbol
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<IntradayLine[]> getIntraday(String symbol) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextIntraday(this.raw, symbol, callback);
        });
    }

    /**
     * Get security candlesticks
     * 
     * @param symbol     Security symbol
     * @param period     Candlestick period
     * @param count      Count of candlesticks
     * @param adjustType Adjustment type
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<Candlestick[]> getCandlesticks(String symbol, Period period, int count,
            AdjustType adjustType) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextCandlesticks(this.raw, symbol, period, count, adjustType, callback);
        });
    }

    /**
     * Get option chain expiry date list
     * 
     * @param symbol Security symbol
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<LocalDate[]> getOptionChainExpiryDateList(String symbol) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextOptionChainExpiryDateList(this.raw, symbol, callback);
        });
    }

    /**
     * Get option chain info by date
     * 
     * @param symbol     Security symbol
     * @param expiryDate Option expiry date
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<StrikePriceInfo[]> getOptionChainInfoByDate(String symbol, LocalDate expiryDate)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextOptionChainInfoByDate(this.raw, symbol, expiryDate, callback);
        });
    }

    /**
     * Get warrant issuers
     * 
     * @param symbol Security symbol
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<IssuerInfo[]> getWarrantIssuers(String symbol)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextWarrantIssuers(this.raw, callback);
        });
    }

    /**
     * Get trading session of the day
     * 
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<MarketTradingSession[]> getTradingSession()
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextTradingSession(this.raw, callback);
        });
    }

    /**
     * Get market trading days
     * <p>
     * The interval must be less than one month, and only the most recent year is
     * supported.
     * 
     * @param market Market
     * @param begin  Begin date
     * @param end    End date
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<MarketTradingDays> getTradingDays(Market market, LocalDate begin, LocalDate end)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextTradingDays(this.raw, market, begin, end, callback);
        });
    }

    /**
     * Get real-time quotes
     * <p>
     * Get real-time quotes of the subscribed symbols, it always returns the data in
     * the local storage.
     * 
     * @param symbols Security symbols
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<RealtimeQuote[]> getRealtimeQuote(String[] symbols)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextRealtimeQuote(this.raw, symbols, callback);
        });
    }

    /**
     * Get real-time depth
     * <p>
     * Get real-time depth of the subscribed symbols, it always returns the data in
     * the local storage.
     * 
     * @param symbol Security symbol
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<SecurityDepth> getRealtimeDepth(String symbol)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextRealtimeDepth(this.raw, symbol, callback);
        });
    }

    /**
     * Get real-time broker queue
     * <p>
     * Get real-time broker queue of the subscribed symbols, it always returns the
     * data in the local storage.
     * 
     * @param symbol Security symbol
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<SecurityBrokers> getRealtimeBrokers(String symbol)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextRealtimeBrokers(this.raw, symbol, callback);
        });
    }

    /**
     * Get real-time trades
     * <p>
     * Get real-time trades of the subscribed symbols, it always returns the data in
     * the local storage.
     * 
     * @param symbol Security symbol
     * @param count  Count of trades
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<Trade[]> getRealtimeBrokers(String symbol, int count)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextRealtimeTrades(this.raw, symbol, count, callback);
        });
    }
}
