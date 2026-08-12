package com.longbridge.quote;

import java.time.LocalDate;
import java.time.LocalDateTime;
import java.util.concurrent.CompletableFuture;

import com.longbridge.*;

/**
 * Quote context
 */
public class QuoteContext implements AutoCloseable {
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
     * Create a QuoteContext object
     *
     * @param config Config object
     * @return A QuoteContext object
     */
    public static QuoteContext create(Config config) {
        QuoteContext ctx = new QuoteContext();
        synchronized (config) { ctx.raw = SdkNative.newQuoteContext(config.getRaw()); }
        return ctx;
    }

    @Override
    public synchronized void close() throws Exception {
        long h = this.raw;
        if (h != 0) {
            this.raw = 0;
            SdkNative.freeQuoteContext(h);
        }
    }

    /**
     * Returns the member ID
     *
     * @return A Future representing the member ID
     */
    public synchronized CompletableFuture<Long> getMemberId() {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextGetMemberId(raw(), callback);
        });
    }

    /**
     * Returns the quote level
     *
     * @return A Future representing the quote level
     */
    public synchronized CompletableFuture<String> getQuoteLevel() {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextGetQuoteLevel(raw(), callback);
        });
    }

    /**
     * Returns the quote package details
     *
     * @return A Future representing the quote package details
     */
    public synchronized CompletableFuture<QuotePackageDetail[]> getQuotePackageDetails() {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextGetQuotePackageDetails(raw(), callback);
        });
    }

    /**
     * Set quote callback, after receiving the quote data push, it will call back to
     * this handler.
     * 
     * @param handler A quote handler
     */
    public synchronized void setOnQuote(QuoteHandler handler) {
        SdkNative.quoteContextSetOnQuote(raw(), handler);
    }

    /**
     * Set depth callback, after receiving the depth data push, it will call back to
     * this handler.
     * 
     * @param handler A depth handler
     */
    public synchronized void setOnDepth(DepthHandler handler) {
        SdkNative.quoteContextSetOnDepth(raw(), handler);
    }

    /**
     * Set brokers callback, after receiving the brokers data push, it will call
     * back
     * to this handler.
     * 
     * @param handler A brokers handler
     */
    public synchronized void setOnBrokers(BrokersHandler handler) {
        SdkNative.quoteContextSetOnBrokers(raw(), handler);
    }

    /**
     * Set trades callback, after receiving the trades data push, it will call
     * backto
     * this handler.
     * 
     * @param handler A trades handler
     */
    public synchronized void setOnTrades(TradesHandler handler) {
        SdkNative.quoteContextSetOnTrades(raw(), handler);
    }

    /**
     * Set candlestick callback, after receiving the trades data push, it will call
     * back to this function.
     * 
     * @param handler A candlestick handler
     */
    public synchronized void setOnCandlestick(CandlestickHandler handler) {
        SdkNative.quoteContextSetOnCandlestick(raw(), handler);
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
     *         OAuth oauth = new OAuthBuilder("your-client-id")
     *             .build(url -> System.out.println("Visit: " + url)).get();
     *         try (Config config = Config.fromOAuth(oauth); QuoteContext ctx = QuoteContext.create(config)) {
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
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public synchronized CompletableFuture<Void> subscribe(String[] symbols, int flags) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextSubscribe(raw(), symbols, flags, callback);
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
     *         OAuth oauth = new OAuthBuilder("your-client-id")
     *             .build(url -> System.out.println("Visit: " + url)).get();
     *         try (Config config = Config.fromOAuth(oauth); QuoteContext ctx = QuoteContext.create(config)) {
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
    public synchronized CompletableFuture<Void> unsubscribe(String[] symbols, int flags) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextUnsubscribe(raw(), symbols, flags, callback);
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
     *         OAuth oauth = new OAuthBuilder("your-client-id")
     *             .build(url -> System.out.println("Visit: " + url)).get();
     *         try (Config config = Config.fromOAuth(oauth); QuoteContext ctx = QuoteContext.create(config)) {
     *             ctx.setOnCandlestick((symbol, event) -> {
     *                 System.out.printf("%s\t%s\n", symbol, event);
     *             });
     *             ctx.subscribeCandlesticks("700.HK", Period.Min_1, TradeSessions.Intraday).get();
     *             Thread.sleep(30000);
     *         }
     *     }
     * }
     * }
     * </pre>
     * 
     * @param symbol        Security symbol
     * @param period        Period type
     * @param tradeSessions Trade sessions
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public synchronized CompletableFuture<Candlestick[]> subscribeCandlesticks(String symbol, Period period,
            TradeSessions tradeSessions)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextSubscribeCandlesticks(raw(), symbol, period, tradeSessions, callback);
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
    public synchronized CompletableFuture<Void> unsubscribeCandlesticks(String symbol, Period period) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextUnsubscribeCandlesticks(raw(), symbol, period, callback);
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
     *         OAuth oauth = new OAuthBuilder("your-client-id")
     *             .build(url -> System.out.println("Visit: " + url)).get();
     *         try (Config config = Config.fromOAuth(oauth); QuoteContext ctx = QuoteContext.create(config)) {
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
    public synchronized CompletableFuture<Subscription[]> getSubscrptions() throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextSubscriptions(raw(), callback);
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
     *         OAuth oauth = new OAuthBuilder("your-client-id")
     *             .build(url -> System.out.println("Visit: " + url)).get();
     *         try (Config config = Config.fromOAuth(oauth); QuoteContext ctx = QuoteContext.create(config)) {
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
    public synchronized CompletableFuture<SecurityStaticInfo[]> getStaticInfo(String[] symbols) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextStaticInfo(raw(), symbols, callback);
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
     *         OAuth oauth = new OAuthBuilder("your-client-id")
     *             .build(url -> System.out.println("Visit: " + url)).get();
     *         try (Config config = Config.fromOAuth(oauth); QuoteContext ctx = QuoteContext.create(config)) {
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
    public synchronized CompletableFuture<SecurityQuote[]> getQuote(String[] symbols) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextQuote(raw(), symbols, callback);
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
     *         OAuth oauth = new OAuthBuilder("your-client-id")
     *             .build(url -> System.out.println("Visit: " + url)).get();
     *         try (Config config = Config.fromOAuth(oauth); QuoteContext ctx = QuoteContext.create(config)) {
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
    public synchronized CompletableFuture<OptionQuote[]> getOptionQuote(String[] symbols) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextOptionQuote(raw(), symbols, callback);
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
     *         OAuth oauth = new OAuthBuilder("your-client-id")
     *             .build(url -> System.out.println("Visit: " + url)).get();
     *         try (Config config = Config.fromOAuth(oauth); QuoteContext ctx = QuoteContext.create(config)) {
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
    public synchronized CompletableFuture<WarrantQuote[]> getWarrantQuote(String[] symbols) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextWarrantQuote(raw(), symbols, callback);
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
     *         OAuth oauth = new OAuthBuilder("your-client-id")
     *             .build(url -> System.out.println("Visit: " + url)).get();
     *         try (Config config = Config.fromOAuth(oauth); QuoteContext ctx = QuoteContext.create(config)) {
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
    public synchronized CompletableFuture<SecurityDepth> getDepth(String symbol) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextDepth(raw(), symbol, callback);
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
     *         OAuth oauth = new OAuthBuilder("your-client-id")
     *             .build(url -> System.out.println("Visit: " + url)).get();
     *         try (Config config = Config.fromOAuth(oauth); QuoteContext ctx = QuoteContext.create(config)) {
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
    public synchronized CompletableFuture<SecurityBrokers> getBrokers(String symbol) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextBrokers(raw(), symbol, callback);
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
     *         OAuth oauth = new OAuthBuilder("your-client-id")
     *             .build(url -> System.out.println("Visit: " + url)).get();
     *         try (Config config = Config.fromOAuth(oauth); QuoteContext ctx = QuoteContext.create(config)) {
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
    public synchronized CompletableFuture<ParticipantInfo[]> getParticipants() throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextParticipants(raw(), callback);
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
     *         OAuth oauth = new OAuthBuilder("your-client-id")
     *             .build(url -> System.out.println("Visit: " + url)).get();
     *         try (Config config = Config.fromOAuth(oauth); QuoteContext ctx = QuoteContext.create(config)) {
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
    public synchronized CompletableFuture<Trade[]> getTrades(String symbol, int count) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextTrades(raw(), symbol, count, callback);
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
     *         OAuth oauth = new OAuthBuilder("your-client-id")
     *             .build(url -> System.out.println("Visit: " + url)).get();
     *         try (Config config = Config.fromOAuth(oauth); QuoteContext ctx = QuoteContext.create(config)) {
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
     * @param symbol        Security symbol
     * @param tradeSessions Trade sessions
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public synchronized CompletableFuture<IntradayLine[]> getIntraday(String symbol, TradeSessions tradeSessions)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextIntraday(raw(), symbol, tradeSessions, callback);
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
     *         OAuth oauth = new OAuthBuilder("your-client-id")
     *             .build(url -> System.out.println("Visit: " + url)).get();
     *         try (Config config = Config.fromOAuth(oauth); QuoteContext ctx = QuoteContext.create(config)) {
     *             Candlestick[] resp = ctx
     *                     .getCandlesticks("700.HK", Period.Day, 10, AdjustType.NoAdjust, TradeSessions.Intraday)
     *                     .get();
     *             for (Candlestick obj : resp) {
     *                 System.out.println(obj);
     *             }
     *         }
     *     }
     * }
     * }
     * </pre>
     * 
     * @param symbol        Security symbol
     * @param period        Candlestick period
     * @param count         Count of candlesticks
     * @param adjustType    Adjustment type
     * @param tradeSessions Trade sessions
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public synchronized CompletableFuture<Candlestick[]> getCandlesticks(String symbol, Period period, int count,
            AdjustType adjustType, TradeSessions tradeSessions) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextCandlesticks(raw(), symbol, period, count, adjustType, tradeSessions, callback);
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
     *         OAuth oauth = new OAuthBuilder("your-client-id")
     *             .build(url -> System.out.println("Visit: " + url)).get();
     *         try (Config config = Config.fromOAuth(oauth); QuoteContext ctx = QuoteContext.create(config)) {
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
    public synchronized CompletableFuture<LocalDate[]> getOptionChainExpiryDateList(String symbol) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextOptionChainExpiryDateList(raw(), symbol, callback);
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
     *         OAuth oauth = new OAuthBuilder("your-client-id")
     *             .build(url -> System.out.println("Visit: " + url)).get();
     *         try (Config config = Config.fromOAuth(oauth); QuoteContext ctx = QuoteContext.create(config)) {
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
    public synchronized CompletableFuture<StrikePriceInfo[]> getOptionChainInfoByDate(String symbol, LocalDate expiryDate)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextOptionChainInfoByDate(raw(), symbol, expiryDate, callback);
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
     *         OAuth oauth = new OAuthBuilder("your-client-id")
     *             .build(url -> System.out.println("Visit: " + url)).get();
     *         try (Config config = Config.fromOAuth(oauth); QuoteContext ctx = QuoteContext.create(config)) {
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
    public synchronized CompletableFuture<IssuerInfo[]> getWarrantIssuers()
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextWarrantIssuers(raw(), callback);
        });
    }

    /**
     * Query warrant list
     * 
     * <pre>
     * {@code
     * import com.longbridge.*;
     * import com.longbridge.quote.*;
     * 
     * class Main {
     *     public static void main(String[] args) throws Exception {
     *         OAuth oauth = new OAuthBuilder("your-client-id")
     *             .build(url -> System.out.println("Visit: " + url)).get();
     *         try (Config config = Config.fromOAuth(oauth); QuoteContext ctx = QuoteContext.create(config)) {
     *             QueryWarrantOptions opts = new QueryWarrantOptions("700.HK", WarrantSortBy.LastDone,
     *                     SortOrderType.Ascending);
     *             IssuerInfo[] resp = ctx.queryWarrantList(opts).get();
     *             for (IssuerInfo obj : resp) {
     *                 System.out.println(obj);
     *             }
     *         }
     *     }
     * }
     * }
     * </pre>
     * 
     * @param opts Query options
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public synchronized CompletableFuture<WarrantInfo[]> queryWarrantList(QueryWarrantOptions opts)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextWarrantList(raw(), opts, callback);
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
     *         OAuth oauth = new OAuthBuilder("your-client-id")
     *             .build(url -> System.out.println("Visit: " + url)).get();
     *         try (Config config = Config.fromOAuth(oauth); QuoteContext ctx = QuoteContext.create(config)) {
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
    public synchronized CompletableFuture<MarketTradingSession[]> getTradingSession()
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextTradingSession(raw(), callback);
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
     *         OAuth oauth = new OAuthBuilder("your-client-id")
     *             .build(url -> System.out.println("Visit: " + url)).get();
     *         try (Config config = Config.fromOAuth(oauth); QuoteContext ctx = QuoteContext.create(config)) {
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
    public synchronized CompletableFuture<MarketTradingDays> getTradingDays(Market market, LocalDate begin, LocalDate end)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextTradingDays(raw(), market, begin, end, callback);
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
     *         OAuth oauth = new OAuthBuilder("your-client-id")
     *             .build(url -> System.out.println("Visit: " + url)).get();
     *         try (Config config = Config.fromOAuth(oauth); QuoteContext ctx = QuoteContext.create(config)) {
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
    public synchronized CompletableFuture<CapitalFlowLine[]> getCapitalFlow(String symbol) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextCapitalFlow(raw(), symbol, callback);
        });
    }

    /**
     * Get capital distribution
     * 
     * <pre>
     * {@code
     * import com.longbridge.*;
     * import com.longbridge.quote.*;
     * 
     * class Main {
     *     public static void main(String[] args) throws Exception {
     *         OAuth oauth = new OAuthBuilder("your-client-id")
     *             .build(url -> System.out.println("Visit: " + url)).get();
     *         try (Config config = Config.fromOAuth(oauth); QuoteContext ctx = QuoteContext.create(config)) {
     *             CapitalDistributionResponse resp = ctx.getCapitalDistribution("700.HK").get();
     *             System.out.println(resp);
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
    public synchronized CompletableFuture<CapitalDistributionResponse> getCapitalDistribution(String symbol)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextCapitalDistribution(raw(), symbol, callback);
        });
    }

    /**
     * Get history candlesticks by offset
     * 
     * @param symbol        Security symbol
     * @param period        Candlestick period
     * @param adjustType    Adjustment type
     * @param forward       Forward or backward
     * @param datetime      From datetime
     * @param count         Count of candlesticks
     * @param tradeSessions Trade sessions
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public synchronized CompletableFuture<Candlestick[]> getHistoryCandlesticksByOffset(String symbol, Period period,
            AdjustType adjustType, boolean forward, LocalDateTime datetime, int count, TradeSessions tradeSessions)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextHistoryCandlesticksByOffset(raw(), symbol, period, adjustType, forward, datetime,
                    count, tradeSessions, callback);
        });
    }

    /**
     * Get history candlesticks by date
     * 
     * @param symbol        Security symbol
     * @param period        Candlestick period
     * @param adjustType    Adjustment type
     * @param start         Start date
     * @param end           End date
     * @param tradeSessions Trade sessions
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public synchronized CompletableFuture<Candlestick[]> getHistoryCandlesticksByDate(String symbol, Period period,
            AdjustType adjustType, LocalDate start, LocalDate end, TradeSessions tradeSessions)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextHistoryCandlesticksByDate(raw(), symbol, period, adjustType, start, end,
                    tradeSessions, callback);
        });
    }

    /**
     * Get security calc indexes
     * 
     * @param symbols Security symbols
     * @param indexes Calc indexes
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public synchronized CompletableFuture<SecurityCalcIndex[]> getCalcIndexes(String[] symbols, CalcIndex[] indexes)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextCalcIndexes(raw(), symbols, indexes, callback);
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
     *         OAuth oauth = new OAuthBuilder("your-client-id")
     *             .build(url -> System.out.println("Visit: " + url)).get();
     *         try (Config config = Config.fromOAuth(oauth); QuoteContext ctx = QuoteContext.create(config)) {
     *             WatchlistGroup[] resp = ctx.getWatchlist().get();
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

    public synchronized CompletableFuture<WatchlistGroup[]> getWatchlist()
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextWatchlist(raw(), callback);
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
     *         OAuth oauth = new OAuthBuilder("your-client-id")
     *             .build(url -> System.out.println("Visit: " + url)).get();
     *         try (Config config = Config.fromOAuth(oauth); QuoteContext ctx = QuoteContext.create(config)) {
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
    public synchronized CompletableFuture<Long> createWatchlistGroup(CreateWatchlistGroup req) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextCreateWatchlistGroup(raw(), req, callback);
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
     *         OAuth oauth = new OAuthBuilder("your-client-id")
     *             .build(url -> System.out.println("Visit: " + url)).get();
     *         try (Config config = Config.fromOAuth(oauth); QuoteContext ctx = QuoteContext.create(config)) {
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
    public synchronized CompletableFuture<Void> deleteWatchlistGroup(DeleteWatchlistGroup req) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextDeleteWatchlistGroup(raw(), req, callback);
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
     *         OAuth oauth = new OAuthBuilder("your-client-id")
     *             .build(url -> System.out.println("Visit: " + url)).get();
     *         try (Config config = Config.fromOAuth(oauth); QuoteContext ctx = QuoteContext.create(config)) {
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
    public synchronized CompletableFuture<Long> updateWatchlistGroup(UpdateWatchlistGroup req) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextUpdateWatchlistGroup(raw(), req, callback);
        });
    }

    /**
     * Get filings list
     *
     * @param symbol Security symbol
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public synchronized CompletableFuture<FilingItem[]> getFilings(String symbol)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextFilings(raw(), symbol, callback);
        });
    }

    /**
     * Security list
     * 
     * <pre>
     * {@code
     * import com.longbridge.*;
     * import com.longbridge.quote.*;
     * 
     * class Main {
     *     public static void main(String[] args) throws Exception {
     *         OAuth oauth = new OAuthBuilder("your-client-id")
     *                 .build(url -> System.out.println("Visit: " + url)).get();
     *         try (Config config = Config.fromOAuth(oauth);
     *                 QuoteContext ctx = QuoteContext.create(config)) {
     *             Security[] resp = ctx.securityList(Market.US, SecurityListCategory.Overnight).get();
     *             for (Security obj : resp) {
     *                 System.out.println(obj);
     *             }
     *         }
     *     }
     * }
     * }
     * </pre>
     * 
     * @param market   Market
     * @param category Security list category
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public synchronized CompletableFuture<Security[]> getSecurityList(Market market, SecurityListCategory category)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextSecurityList(raw(), market, category, callback);
        });
    }

    /**
     * Security list without category
     * 
     * <pre>
     * {@code
     * import com.longbridge.*;
     * import com.longbridge.quote.*;
     * 
     * class Main {
     *     public static void main(String[] args) throws Exception {
     *         OAuth oauth = new OAuthBuilder("your-client-id")
     *                 .build(url -> System.out.println("Visit: " + url)).get();
     *         try (Config config = Config.fromOAuth(oauth);
     *                 QuoteContext ctx = QuoteContext.create(config)) {
     *             Security[] resp = ctx.securityList(Market.Crypto).get();
     *             for (Security obj : resp) {
     *                 System.out.println(obj);
     *             }
     *         }
     *     }
     * }
     * }
     * </pre>
     * 
     * @param market Market
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public synchronized CompletableFuture<Security[]> getSecurityList(Market market)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextSecurityList(raw(), market, null, callback);
        });
    }

    /**
     * Get current market temperature
     * 
     * <pre>
     * {@code
     * import com.longbridge.*;
     * import com.longbridge.quote.*;
     * 
     * class Main {
     *     public static void main(String[] args) throws Exception {
     *         OAuth oauth = new OAuthBuilder("your-client-id")
     *             .build(url -> System.out.println("Visit: " + url)).get();
     *         try (Config config = Config.fromOAuth(oauth); QuoteContext ctx = QuoteContext.create(config)) {
     *             MarketTemperature resp = ctx.getMarketTemperature(Market.HK).get();
     *             System.out.println(resp);
     *         }
     *     }
     * }
     * }
     * </pre>
     * 
     * @param market Market
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public synchronized CompletableFuture<MarketTemperature> getMarketTemperature(Market market)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextMarketTemperature(raw(), market, callback);
        });
    }

    /**
     * Get historical market temperature
     * 
     * <pre>
     * {@code
     * import com.longbridge.*;
     * import com.longbridge.quote.*;
     * 
     * class Main {
     *     public static void main(String[] args) throws Exception {
     *         OAuth oauth = new OAuthBuilder("your-client-id")
     *             .build(url -> System.out.println("Visit: " + url)).get();
     *         try (Config config = Config.fromOAuth(oauth); QuoteContext ctx = QuoteContext.create(config)) {
     *             HistoryMarketTemperatureResponse resp = ctx
     *                     .getHistoryMarketTemperature(Market.HK, LocalDate.of(2025, 1, 20), LocalDate.of(2025, 2, 20))
     *                     .get();
     *             System.out.println(resp);
     *         }
     *     }
     * }
     * }
     * </pre>
     * 
     * @param market Market
     * @param start  Start date
     * @param end    End date
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public synchronized CompletableFuture<HistoryMarketTemperatureResponse> getHistoryMarketTemperature(Market market,
            LocalDate start,
            LocalDate end)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextHistoryMarketTemperature(raw(), market, start, end, callback);
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
     *         OAuth oauth = new OAuthBuilder("your-client-id")
     *             .build(url -> System.out.println("Visit: " + url)).get();
     *         try (Config config = Config.fromOAuth(oauth); QuoteContext ctx = QuoteContext.create(config)) {
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
    public synchronized CompletableFuture<RealtimeQuote[]> getRealtimeQuote(String[] symbols)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextRealtimeQuote(raw(), symbols, callback);
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
     *         OAuth oauth = new OAuthBuilder("your-client-id")
     *             .build(url -> System.out.println("Visit: " + url)).get();
     *         try (Config config = Config.fromOAuth(oauth); QuoteContext ctx = QuoteContext.create(config)) {
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
    public synchronized CompletableFuture<SecurityDepth> getRealtimeDepth(String symbol)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextRealtimeDepth(raw(), symbol, callback);
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
     *         OAuth oauth = new OAuthBuilder("your-client-id")
     *             .build(url -> System.out.println("Visit: " + url)).get();
     *         try (Config config = Config.fromOAuth(oauth); QuoteContext ctx = QuoteContext.create(config)) {
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
    public synchronized CompletableFuture<SecurityBrokers> getRealtimeBrokers(String symbol)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextRealtimeBrokers(raw(), symbol, callback);
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
     *         OAuth oauth = new OAuthBuilder("your-client-id")
     *             .build(url -> System.out.println("Visit: " + url)).get();
     *         try (Config config = Config.fromOAuth(oauth); QuoteContext ctx = QuoteContext.create(config)) {
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
    public synchronized CompletableFuture<Trade[]> getRealtimeTrades(String symbol, int count)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextRealtimeTrades(raw(), symbol, count, callback);
        });
    }

    /**
     * Get short positions for a symbol
     *
     * @param symbol Security symbol
     * @return A Future representing the short positions response
     * @throws OpenApiException If an error occurs
     */
    public synchronized CompletableFuture<ShortPositionsResponse> getShortPositions(String symbol) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextShortPositions(raw(), symbol, callback);
        });
    }

    /** Get daily short sale volume for US or HK stocks (market auto-detected from symbol suffix). */
    public synchronized CompletableFuture<ShortTradesResponse> getShortTrades(ShortTradesOptions opts) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextShortTrades(raw(), opts, callback);
        });
    }

    /**
     * Get option volume statistics for a symbol
     *
     * @param symbol Security symbol
     * @return A Future representing the option volume statistics
     * @throws OpenApiException If an error occurs
     */
    public synchronized CompletableFuture<OptionVolumeStats> getOptionVolume(String symbol) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextOptionVolume(raw(), symbol, callback);
        });
    }

    /**
     * Get daily option volume for a symbol
     *
     * @param opts Options including symbol, timestamp, and count
     * @return A Future representing the daily option volume
     * @throws OpenApiException If an error occurs
     */
    public synchronized CompletableFuture<OptionVolumeDaily> getOptionVolumeDaily(OptionVolumeDailyOptions opts)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextOptionVolumeDaily(raw(), opts, callback);
        });
    }

    /**
     * Update pinned securities (add or remove).
     *
     * @param req Request containing mode (Add/Remove) and security symbols
     * @return A Future that completes when the operation is done
     * @throws OpenApiException If an error occurs
     */
    public synchronized CompletableFuture<Void> updatePinned(UpdatePinnedRequest req) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextUpdatePinned(raw(), req, callback);
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
     *         OAuth oauth = new OAuthBuilder("your-client-id")
     *             .build(url -> System.out.println("Visit: " + url)).get();
     *         try (Config config = Config.fromOAuth(oauth); QuoteContext ctx = QuoteContext.create(config)) {
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
    public synchronized CompletableFuture<Candlestick[]> getRealtimeCandlesticks(String symbol, Period period, int count)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextRealtimeCandlesticks(raw(), symbol, period, count, callback);
        });
    }
}
