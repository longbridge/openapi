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

    /**
     * Create a QuoteContext object
     *
     * @param config Config object
     * @return A QuoteContext object
     */
    public static QuoteContext create(Config config) {
        QuoteContext ctx = new QuoteContext();
        ctx.raw = SdkNative.newQuoteContext(config.getRaw());
        return ctx;
    }

    @Override
    public void close() throws Exception {
        SdkNative.freeQuoteContext(raw);
    }

    /**
     * Returns the member ID
     *
     * @return A Future representing the member ID
     */
    public CompletableFuture<Long> getMemberId() {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextGetMemberId(this.raw, callback);
        });
    }

    /**
     * Returns the quote level
     *
     * @return A Future representing the quote level
     */
    public CompletableFuture<String> getQuoteLevel() {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextGetQuoteLevel(this.raw, callback);
        });
    }

    /**
     * Returns the quote package details
     *
     * @return A Future representing the quote package details
     */
    public CompletableFuture<QuotePackageDetail[]> getQuotePackageDetails() {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextGetQuotePackageDetails(this.raw, callback);
        });
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
    public CompletableFuture<Void> subscribe(String[] symbols, int flags) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextSubscribe(this.raw, symbols, flags, callback);
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
    public CompletableFuture<Candlestick[]> subscribeCandlesticks(String symbol, Period period,
            TradeSessions tradeSessions)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextSubscribeCandlesticks(this.raw, symbol, period, tradeSessions, callback);
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
    public CompletableFuture<IntradayLine[]> getIntraday(String symbol, TradeSessions tradeSessions)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextIntraday(this.raw, symbol, tradeSessions, callback);
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
    public CompletableFuture<Candlestick[]> getCandlesticks(String symbol, Period period, int count,
            AdjustType adjustType, TradeSessions tradeSessions) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextCandlesticks(this.raw, symbol, period, count, adjustType, tradeSessions, callback);
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
    public CompletableFuture<IssuerInfo[]> getWarrantIssuers()
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextWarrantIssuers(this.raw, callback);
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
    public CompletableFuture<WarrantInfo[]> queryWarrantList(QueryWarrantOptions opts)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextWarrantList(this.raw, opts, callback);
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
    public CompletableFuture<CapitalFlowLine[]> getCapitalFlow(String symbol) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextCapitalFlow(this.raw, symbol, callback);
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
    public CompletableFuture<CapitalDistributionResponse> getCapitalDistribution(String symbol)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextCapitalDistribution(this.raw, symbol, callback);
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
    public CompletableFuture<Candlestick[]> getHistoryCandlesticksByOffset(String symbol, Period period,
            AdjustType adjustType, boolean forward, LocalDateTime datetime, int count, TradeSessions tradeSessions)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextHistoryCandlesticksByOffset(this.raw, symbol, period, adjustType, forward, datetime,
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
    public CompletableFuture<Candlestick[]> getHistoryCandlesticksByDate(String symbol, Period period,
            AdjustType adjustType, LocalDate start, LocalDate end, TradeSessions tradeSessions)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextHistoryCandlesticksByDate(this.raw, symbol, period, adjustType, start, end,
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
    public CompletableFuture<SecurityCalcIndex[]> getCalcIndexes(String[] symbols, CalcIndex[] indexes)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextCalcIndexes(this.raw, symbols, indexes, callback);
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
    public CompletableFuture<Long> updateWatchlistGroup(UpdateWatchlistGroup req) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextUpdateWatchlistGroup(this.raw, req, callback);
        });
    }

    /**
     * Get filings list
     *
     * @param symbol Security symbol
     * @return A Future representing the result of the operation
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<FilingItem[]> getFilings(String symbol)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextFilings(this.raw, symbol, callback);
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
    public CompletableFuture<Security[]> getSecurityList(Market market, SecurityListCategory category)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextSecurityList(this.raw, market, category, callback);
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
    public CompletableFuture<Security[]> getSecurityList(Market market)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextSecurityList(this.raw, market, null, callback);
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
    public CompletableFuture<MarketTemperature> getMarketTemperature(Market market)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextMarketTemperature(raw, market, callback);
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
    public CompletableFuture<HistoryMarketTemperatureResponse> getHistoryMarketTemperature(Market market,
            LocalDate start,
            LocalDate end)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextHistoryMarketTemperature(raw, market, start, end, callback);
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
    public CompletableFuture<Trade[]> getRealtimeTrades(String symbol, int count)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextRealtimeTrades(this.raw, symbol, count, callback);
        });
    }

    /**
     * Get short positions for a symbol
     *
     * @param symbol Security symbol
     * @return A Future representing the short positions response
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<ShortPositionsResponse> getShortPositions(String symbol) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextShortPositions(this.raw, symbol, callback);
        });
    }

    /** Get daily short sale volume for US or HK stocks (market auto-detected from symbol suffix). */
    public CompletableFuture<ShortTradesResponse> getShortTrades(ShortTradesOptions opts) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextShortTrades(this.raw, opts, callback);
        });
    }

    /**
     * Get option volume statistics for a symbol
     *
     * @param symbol Security symbol
     * @return A Future representing the option volume statistics
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<OptionVolumeStats> getOptionVolume(String symbol) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextOptionVolume(this.raw, symbol, callback);
        });
    }

    /**
     * Get daily option volume for a symbol
     *
     * @param opts Options including symbol, timestamp, and count
     * @return A Future representing the daily option volume
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<OptionVolumeDaily> getOptionVolumeDaily(OptionVolumeDailyOptions opts)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextOptionVolumeDaily(this.raw, opts, callback);
        });
    }

    /**
     * Update pinned securities (add or remove).
     *
     * @param req Request containing mode (Add/Remove) and security symbols
     * @return A Future that completes when the operation is done
     * @throws OpenApiException If an error occurs
     */
    public CompletableFuture<Void> updatePinned(UpdatePinnedRequest req) throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextUpdatePinned(this.raw, req, callback);
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
    public CompletableFuture<Candlestick[]> getRealtimeCandlesticks(String symbol, Period period, int count)
            throws OpenApiException {
        return AsyncCallback.executeTask((callback) -> {
            SdkNative.quoteContextRealtimeCandlesticks(this.raw, symbol, period, count, callback);
        });
    }
}
