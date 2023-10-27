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
     * Set depth callback, after receiving the depth data push, it will call back to
     * this handler.
     * 
     * @param handler A depth handler
     */
    public void setOnDepth(DepthHandler handler) {
        SdkNative.quoteContextSetOnDepth(this.raw, handler);
    }

    /**
     * Set brokers callback, after receiving the brokers data push, it will call
     * back
     * to this handler.
     * 
     * @param handler A brokers handler
     */
    public void setOnBrokers(BrokersHandler handler) {
        SdkNative.quoteContextSetOnBrokers(this.raw, handler);
    }

    /**
     * Set trades callback, after receiving the trades data push, it will call
     * backto
     * this handler.
     * 
     * @param handler A trades handler
     */
    public void setOnTrades(TradesHandler handler) {
        SdkNative.quoteContextSetOnTrades(this.raw, handler);
    }

    /**
     * Set candlestick callback, after receiving the trades data push, it will call
     * back to this function.
     * 
     * @param handler A candlestick handler
     */
    public void setOnCandlestick(CandlestickHandler handler) {
        SdkNative.quoteContextSetOnCandlestick(this.raw, handler);
    }

    /**
     * Subscribe
     * 
     * <pre>
     * {@code
     * import com.longbridge.*;
     * import com.longbridge.quote.*;
     * 
     * class Main {
     *     public static void main(String[] args) throws Exception {
     *         try (Config config = Config.fromEnv(); QuoteContext ctx = QuoteContext.create(config).get()) {
     *             ctx.setOnQuote((symbol, event) -> {
     *                 System.out.printf("%s\t%s\n", symbol, event);
     *             });
     *             ctx.subscribe(new String[] { "700.HK", "AAPL.US" }, SubFlags.Quote, true).get();
     *             Thread.sleep(30000);
     *         }
     *     }
     * }
     * }
     * </pre>
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
     * <pre>
     * {@code
     * import com.longbridge.*;
     * import com.longbridge.quote.*;
     * 
     * class Main {
     *     public static void main(String[] args) throws Exception {
     *         try (Config config = Config.fromEnv(); QuoteContext ctx = QuoteContext.create(config).get()) {
     *             ctx.setOnQuote((symbol, quote) -> {
     *                 System.out.printf("%s\t%s\n", symbol, quote);
     *             });
     *             ctx.subscribe(new String[] { "700.HK", "AAPL.US" }, SubFlags.Quote, true).get();
     *             ctx.unsubscribe(new String[] { "AAPL.US" }, SubFlags.Quote).get();
     *         }
     *     }
     * }
     * }
     * </pre>
     * 
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
     * Subscribe security candlesticks
     * 
     * <pre>
     * {@code
     * import com.longbridge.*;
     * import com.longbridge.quote.*;
     * 
     * class Main {
     *     public static void main(String[] args) throws Exception {
     *         try (Config config = Config.fromEnv(); QuoteContext ctx = QuoteContext.create(config).get()) {
     *             ctx.setOnCandlestick((symbol, event) -> {
     *                 System.out.printf("%s\t%s\n", symbol, event);
     *             });
     *             ctx.subscribeCandlesticks("700.HK", Period.Min_1).get();
     *             Thread.sleep(30000);
     *         }
     *     }
     * }
     * }
     * </pre>
     * 
     * @param symbol Security symbol
     * @param period Period type
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<Void> subscribeCandlesticks(String symbol, Period period) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextSubscribeCandlesticks(this.raw, symbol, period, callback);
        });
    }

    /**
     * Unsubscribe security candlesticks
     * 
     * @param symbol Security symbol
     * @param period Period type
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<Void> unsubscribeCandlesticks(String symbol, Period period) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextUnsubscribeCandlesticks(this.raw, symbol, period, callback);
        });
    }

    /**
     * Get subscription information
     * 
     * <pre>
     * {@code
     * import com.longbridge.*;
     * import com.longbridge.quote.*;
     * 
     * class Main {
     *     public static void main(String[] args) throws Exception {
     *         try (Config config = Config.fromEnv(); QuoteContext ctx = QuoteContext.create(config).get()) {
     *             ctx.subscribe(new String[] { "700.HK", "AAPL.US" }, SubFlags.Quote, true);
     *             Subscription[] subscriptions = ctx.getSubscrptions().get();
     *             for (Subscription obj : subscriptions) {
     *                 System.out.println(obj);
     *             }
     *         }
     *     }
     * }
     * }
     * </pre>
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
     * <pre>
     * {@code
     * import com.longbridge.*;
     * import com.longbridge.quote.*;
     * 
     * class Main {
     *     public static void main(String[] args) throws Exception {
     *         try (Config config = Config.fromEnv(); QuoteContext ctx = QuoteContext.create(config).get()) {
     *             SecurityStaticInfo[] resp = ctx
     *                     .getStaticInfo(new String[] { "700.HK", "AAPL.US", "TSLA.US", "NFLX.US" })
     *                     .get();
     *             for (SecurityStaticInfo obj : resp) {
     *                 System.out.println(obj);
     *             }
     *         }
     *     }
     * }
     * }
     * </pre>
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
     * <pre>
     * {@code
     * import com.longbridge.*;
     * import com.longbridge.quote.*;
     * 
     * class Main {
     *     public static void main(String[] args) throws Exception {
     *         try (Config config = Config.fromEnv(); QuoteContext ctx = QuoteContext.create(config).get()) {
     *             SecurityQuote[] resp = ctx.getQuote(new String[] { "700.HK", "AAPL.US", "TSLA.US", "NFLX.US" })
     *                     .get();
     *             for (SecurityQuote obj : resp) {
     *                 System.out.println(obj);
     *             }
     *         }
     *     }
     * }
     * }
     * </pre>
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
     * <pre>
     * {@code
     * import com.longbridge.*;
     * import com.longbridge.quote.*;
     * 
     * class Main {
     *     public static void main(String[] args) throws Exception {
     *         try (Config config = Config.fromEnv(); QuoteContext ctx = QuoteContext.create(config).get()) {
     *             OptionQuote[] resp = ctx.getOptionQuote(new String[] { "AAPL230317P160000.US" }).get();
     *             for (OptionQuote obj : resp) {
     *                 System.out.println(obj);
     *             }
     *         }
     *     }
     * }
     * }
     * </pre>
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
     * Get quote of warrant securities
     * 
     * <pre>
     * {@code
     * import com.longbridge.*;
     * import com.longbridge.quote.*;
     * 
     * class Main {
     *     public static void main(String[] args) throws Exception {
     *         try (Config config = Config.fromEnv(); QuoteContext ctx = QuoteContext.create(config).get()) {
     *             WarrantQuote[] resp = ctx.getWarrantQuote(new String[] { "21125.HK" }).get();
     *             for (WarrantQuote obj : resp) {
     *                 System.out.println(obj);
     *             }
     *         }
     *     }
     * }
     * }
     * </pre>
     * 
     * @param symbols Security symbols
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<WarrantQuote[]> getWarrantQuote(String[] symbols) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextWarrantQuote(this.raw, symbols, callback);
        });
    }

    /**
     * Get security depth
     * 
     * <pre>
     * {@code
     * import com.longbridge.*;
     * import com.longbridge.quote.*;
     * 
     * class Main {
     *     public static void main(String[] args) throws Exception {
     *         try (Config config = Config.fromEnv(); QuoteContext ctx = QuoteContext.create(config).get()) {
     *             SecurityDepth resp = ctx.getDepth("700.HK").get();
     *             System.out.println(resp);
     *         }
     *     }
     * }
     * }
     * </pre>
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
     * <pre>
     * {@code
     * import com.longbridge.*;
     * import com.longbridge.quote.*;
     * 
     * class Main {
     *     public static void main(String[] args) throws Exception {
     *         try (Config config = Config.fromEnv(); QuoteContext ctx = QuoteContext.create(config).get()) {
     *             SecurityBrokers resp = ctx.getBrokers("700.HK").get();
     *             System.out.println(resp);
     *         }
     *     }
     * }
     * }
     * </pre>
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
     * <pre>
     * {@code
     * import com.longbridge.*;
     * import com.longbridge.quote.*;
     * 
     * class Main {
     *     public static void main(String[] args) throws Exception {
     *         try (Config config = Config.fromEnv(); QuoteContext ctx = QuoteContext.create(config).get()) {
     *             ParticipantInfo[] resp = ctx.getParticipants().get();
     *             for (ParticipantInfo obj : resp) {
     *                 System.out.println(obj);
     *             }
     *         }
     *     }
     * }
     * }
     * </pre>
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
     * <pre>
     * {@code
     * import com.longbridge.*;
     * import com.longbridge.quote.*;
     * 
     * class Main {
     *     public static void main(String[] args) throws Exception {
     *         try (Config config = Config.fromEnv(); QuoteContext ctx = QuoteContext.create(config).get()) {
     *             Trade[] resp = ctx.getTrades("700.HK", 10).get();
     *             for (Trade obj : resp) {
     *                 System.out.println(obj);
     *             }
     *         }
     *     }
     * }
     * }
     * </pre>
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
     * <pre>
     * {@code
     * import com.longbridge.*;
     * import com.longbridge.quote.*;
     * 
     * class Main {
     *     public static void main(String[] args) throws Exception {
     *         try (Config config = Config.fromEnv(); QuoteContext ctx = QuoteContext.create(config).get()) {
     *             IntradayLine[] resp = ctx.getIntraday("700.HK").get();
     *             for (IntradayLine obj : resp) {
     *                 System.out.println(obj);
     *             }
     *         }
     *     }
     * }
     * }
     * </pre>
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
     * <pre>
     * {@code
     * import com.longbridge.*;
     * import com.longbridge.quote.*;
     * 
     * class Main {
     *     public static void main(String[] args) throws Exception {
     *         try (Config config = Config.fromEnv(); QuoteContext ctx = QuoteContext.create(config).get()) {
     *             Candlestick[] resp = ctx.getCandlesticks("700.HK", Period.Day, 10, AdjustType.NoAdjust).get();
     *             for (Candlestick obj : resp) {
     *                 System.out.println(obj);
     *             }
     *         }
     *     }
     * }
     * }
     * </pre>
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
     * <pre>
     * {@code
     * import com.longbridge.*;
     * import com.longbridge.quote.*;
     * import java.time.LocalDate;
     * 
     * class Main {
     *     public static void main(String[] args) throws Exception {
     *         try (Config config = Config.fromEnv(); QuoteContext ctx = QuoteContext.create(config).get()) {
     *             LocalDate[] resp = ctx.getOptionChainExpiryDateList("AAPL.US").get();
     *             for (LocalDate obj : resp) {
     *                 System.out.println(obj);
     *             }
     *         }
     *     }
     * }
     * }
     * </pre>
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
     * <pre>
     * {@code
     * import com.longbridge.*;
     * import com.longbridge.quote.*;
     * import java.time.LocalDate;
     * 
     * class Main {
     *     public static void main(String[] args) throws Exception {
     *         try (Config config = Config.fromEnv(); QuoteContext ctx = QuoteContext.create(config).get()) {
     *             StrikePriceInfo[] resp = ctx.getOptionChainInfoByDate("AAPL.US", LocalDate.of(2023, 1, 20)).get();
     *             for (StrikePriceInfo obj : resp) {
     *                 System.out.println(obj);
     *             }
     *         }
     *     }
     * }
     * }
     * </pre>
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
     * <pre>
     * {@code
     * import com.longbridge.*;
     * import com.longbridge.quote.*;
     * 
     * class Main {
     *     public static void main(String[] args) throws Exception {
     *         try (Config config = Config.fromEnv(); QuoteContext ctx = QuoteContext.create(config).get()) {
     *             IssuerInfo[] resp = ctx.getWarrantIssuers().get();
     *             for (IssuerInfo obj : resp) {
     *                 System.out.println(obj);
     *             }
     *         }
     *     }
     * }
     * }
     * </pre>
     * 
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<IssuerInfo[]> getWarrantIssuers()
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextWarrantIssuers(this.raw, callback);
        });
    }

    /**
     * Get trading session of the day
     * 
     * <pre>
     * {@code
     * import com.longbridge.*;
     * import com.longbridge.quote.*;
     * 
     * class Main {
     *     public static void main(String[] args) throws Exception {
     *         try (Config config = Config.fromEnv(); QuoteContext ctx = QuoteContext.create(config).get()) {
     *             MarketTradingSession[] resp = ctx.getTradingSession().get();
     *             for (MarketTradingSession obj : resp) {
     *                 System.out.println(obj);
     *             }
     *         }
     *     }
     * }
     * }
     * </pre>
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
     * <pre>
     * {@code
     * import com.longbridge.*;
     * import com.longbridge.quote.*;
     * import java.time.LocalDate;
     * 
     * class Main {
     *     public static void main(String[] args) throws Exception {
     *         try (Config config = Config.fromEnv(); QuoteContext ctx = QuoteContext.create(config).get()) {
     *             MarketTradingDays resp = ctx
     *                     .getTradingDays(Market.HK, LocalDate.of(2022, 1, 20), LocalDate.of(2022, 2, 20)).get();
     *             System.out.println(resp);
     *         }
     *     }
     * }
     * }
     * </pre>
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
     * Get capital flow intraday
     * 
     * <pre>
     * {@code
     * import com.longbridge.*;
     * import com.longbridge.quote.*;
     * 
     * class Main {
     *     public static void main(String[] args) throws Exception {
     *         try (Config config = Config.fromEnv(); QuoteContext ctx = QuoteContext.create(config).get()) {
     *             CapitalFlowLine[] resp = ctx.getCapitalFlow("700.HK").get();
     *             for (CapitalFlowLine obj : resp) {
     *                 System.out.println(obj);
     *             }
     *         }
     *     }
     * }
     * }
     * </pre>
     * 
     * @param symbol Security code
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<CapitalFlowLine[]> getCapitalFlow(String symbol) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextCapitalFlow(this.raw, symbol, callback);
        });
    }

    /**
     * Get security calc indexes
     * 
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<SecurityCalcIndex[]> getCalcIndexes(String[] symbols, CalcIndex[] indexes) {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextCalcIndexes(this.raw, symbols, indexes, callback);
        });
    }

    /**
     * Get watchlist
     * 
     * Deprecated: Use `getWatchlist` instead
     */
    @Deprecated
    public CompletableFuture<WatchlistGroup[]> getWatchList()
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextWatchlist(this.raw, callback);
        });
    }

    /**
     * Get watchlist
     * 
     * <pre>
     * {@code
     * import com.longbridge.*;
     * import com.longbridge.quote.*;
     * 
     * class Main {
     *     public static void main(String[] args) throws Exception {
     *         try (Config config = Config.fromEnv(); QuoteContext ctx = QuoteContext.create(config).get()) {
     *             CapitalDistributionResponse resp = ctx.getWatchlist().get();
     *             System.out.println(resp);
     *         }
     *     }
     * }
     * }
     * </pre>
     * 
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */

    public CompletableFuture<WatchlistGroup[]> getWatchlist()
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextWatchlist(this.raw, callback);
        });
    }

    /**
     * Create watchlist group
     * 
     * <pre>
     * {@code
     * import com.longbridge.*;
     * import com.longbridge.quote.*;
     * 
     * class Main {
     *     public static void main(String[] args) throws Exception {
     *         try (Config config = Config.fromEnv(); QuoteContext ctx = QuoteContext.create(config).get()) {
     *             CreateWatchlistGroup req = new CreateWatchlistGroup("Watchlist1")
     *                     .setSecurities(new String[] { "700.HK", "AAPL.US" });
     *             Long groupId = ctx.createWatchlistGroup(req).get();
     *             System.out.println(groupId);
     *         }
     *     }
     * }
     * }
     * </pre>
     * 
     * @param req Create watchlist group request
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<Long> createWatchlistGroup(CreateWatchlistGroup req) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextCreateWatchlistGroup(this.raw, req, callback);
        }).thenApply(resp -> ((CreateWatchlistGroupResponse) resp).id);
    }

    /**
     * Delete watchlist group
     * 
     * <pre>
     * {@code
     * import com.longbridge.*;
     * import com.longbridge.quote.*;
     * 
     * class Main {
     *     public static void main(String[] args) throws Exception {
     *         try (Config config = Config.fromEnv(); QuoteContext ctx = QuoteContext.create(config).get()) {
     *             DeleteWatchlistGroup req = new DeleteWatchlistGroup(10086);
     *             ctx.deleteWatchlistGroup(req).get();
     *         }
     *     }
     * }
     * }
     * </pre>
     * 
     * @param req Delete watchlist group request
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<Void> deleteWatchlistGroup(DeleteWatchlistGroup req) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextDeleteWatchlistGroup(this.raw, req, callback);
        });
    }

    /**
     * Update watchlist group
     * 
     * <pre>
     * {@code
     * import com.longbridge.*;
     * import com.longbridge.quote.*;
     * 
     * class Main {
     *     public static void main(String[] args) throws Exception {
     *         try (Config config = Config.fromEnv(); QuoteContext ctx = QuoteContext.create(config).get()) {
     *             CreateWatchlistGroup req = new UpdateWatchlistGroup(10086)
     *                     .setName("watchlist2")
     *                     .setSecurities(new String[] { "700.HK", "AAPL.US" });
     *             ctx.updateWatchlistGroup(req).get();
     *         }
     *     }
     * }
     * }
     * </pre>
     * 
     * @param req Update watchlist group request
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<Long> updateWatchlistGroup(UpdateWatchlistGroup req) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextUpdateWatchlistGroup(this.raw, req, callback);
        });
    }

    /**
     * Get real-time quotes
     * <p>
     * Get real-time quotes of the subscribed symbols, it always returns the data in
     * the local storage.
     * 
     * <pre>
     * {@code
     * import com.longbridge.*;
     * import com.longbridge.quote.*;
     * 
     * class Main {
     *     public static void main(String[] args) throws Exception {
     *         try (Config config = Config.fromEnv(); QuoteContext ctx = QuoteContext.create(config).get()) {
     *             ctx.subscribe(new String[] { "700.HK", "AAPL.US" }, SubFlags.Quote, true).get();
     *             Thread.sleep(5000);
     *             RealtimeQuote[] resp = ctx.getRealtimeQuote(new String[] { "700.HK", "AAPL.US" }).get();
     *             for (RealtimeQuote obj : resp) {
     *                 System.out.println(obj);
     *             }
     *         }
     *     }
     * }
     * }
     * </pre>
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
     * <pre>
     * {@code
     * import com.longbridge.*;
     * import com.longbridge.quote.*;
     * 
     * class Main {
     *     public static void main(String[] args) throws Exception {
     *         try (Config config = Config.fromEnv(); QuoteContext ctx = QuoteContext.create(config).get()) {
     *             ctx.subscribe(new String[] { "700.HK", "AAPL.US" }, SubFlags.Depth, true).get();
     *             Thread.sleep(5000);
     *             SecurityDepth resp = ctx.getRealtimeDepth("700.HK").get();
     *             System.out.println(resp);
     *         }
     *     }
     * }
     * }
     * </pre>
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
     * <pre>
     * {@code
     * import com.longbridge.*;
     * import com.longbridge.quote.*;
     * 
     * class Main {
     *     public static void main(String[] args) throws Exception {
     *         try (Config config = Config.fromEnv(); QuoteContext ctx = QuoteContext.create(config).get()) {
     *             ctx.subscribe(new String[] { "700.HK", "AAPL.US" }, SubFlags.Brokers, true).get();
     *             Thread.sleep(5000);
     *             SecurityBrokers resp = ctx.getRealtimeBrokers("700.HK").get();
     *             System.out.println(resp);
     *         }
     *     }
     * }
     * }
     * </pre>
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
     * <pre>
     * {@code
     * import com.longbridge.*;
     * import com.longbridge.quote.*;
     * 
     * class Main {
     *     public static void main(String[] args) throws Exception {
     *         try (Config config = Config.fromEnv(); QuoteContext ctx = QuoteContext.create(config).get()) {
     *             ctx.subscribe(new String[] { "700.HK", "AAPL.US" }, SubFlags.Trade, false).get();
     *             Thread.sleep(5000);
     *             Trade[] resp = ctx.getRealtimeTrades("700.HK", 10).get();
     *             for (Trade obj : resp) {
     *                 System.out.println(obj);
     *             }
     *         }
     *     }
     * }
     * }
     * </pre>
     * 
     * @param symbol Security symbol
     * @param count  Count of trades
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<Trade[]> getRealtimeTrades(String symbol, int count)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextRealtimeTrades(this.raw, symbol, count, callback);
        });
    }

    /**
     * Get real-time candlesticks
     * <p>
     * Get real-time candlesticks of the subscribed symbols, it always returns the
     * data in the local storage.
     * 
     * <pre>
     * {@code
     * import com.longbridge.*;
     * import com.longbridge.quote.*;
     * 
     * class Main {
     *     public static void main(String[] args) throws Exception {
     *         try (Config config = Config.fromEnv(); QuoteContext ctx = QuoteContext.create(config).get()) {
     *             ctx.subscribeCandlesticks("AAPL.US", Period.Min_1).get();
     *             Thread.sleep(5000);
     *             Candlestick[] resp = ctx.getRealtimeCandlesticks("AAPL.US", Period.Min_1, 10).get();
     *             for (Candlestick obj : resp) {
     *                 System.out.println(obj);
     *             }
     *         }
     *     }
     * }
     * }
     * </pre>
     * 
     * @param symbol Security symbol
     * @param period Period type
     * @param count  Count of trades
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<Candlestick[]> getRealtimeCandlesticks(String symbol, Period period, int count)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextRealtimeCandlesticks(this.raw, symbol, period, count, callback);
        });
    }
}
