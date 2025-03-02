package com.longport;

import java.io.IOException;
import java.time.LocalDate;
import org.scijava.nativelib.NativeLoader;

import com.longport.quote.*;
import com.longport.trade.*;

import java.time.OffsetDateTime;
import java.time.LocalDateTime;

/**
 * @hidden
 */
public class SdkNative {
        static native void init();

        public static native long newHttpClient(String httpUrl, String appKey, String appSecret, String accessToken);

        public static native long newHttpClientFromEnv();

        public static native void freeHttpClient(long httpClient);

        public static native void httpClientRequest(long httpClient, String request, AsyncCallback callback);

        public static native long newConfig(String appKey, String appSecret, String accessToken, String httpUrl,
                        String quoteWsUrl, String tradeWsUrl, Language language, boolean enableOvernight,
                        PushCandlestickMode mode, boolean enablePrintQuotePackages, String logPath);

        public static native long newConfigFromEnv();

        public static native void configRefreshAccessToken(long config, OffsetDateTime expired_at,
                        AsyncCallback callback);

        public static native void freeConfig(long config);

        public static native void newQuoteContext(long config, AsyncCallback callback);

        public static native void freeQuoteContext(long config);

        public static native long quoteContextGetMemberId(long context);

        public static native String quoteContextGetQuoteLevel(long context);

        public static native QuotePackageDetail[] quoteContextGetQuotePackageDetails(long context);

        public static native void quoteContextSetOnQuote(long context, QuoteHandler handler);

        public static native void quoteContextSetOnDepth(long context, DepthHandler handler);

        public static native void quoteContextSetOnBrokers(long context, BrokersHandler handler);

        public static native void quoteContextSetOnTrades(long context, TradesHandler handler);

        public static native void quoteContextSetOnCandlestick(long context, CandlestickHandler handler);

        public static native void quoteContextSubscribe(long context, String[] symbols, int flags, boolean isFirstPush,
                        AsyncCallback callback);

        public static native void quoteContextUnsubscribe(long context, String[] symbols, int flags,
                        AsyncCallback callback);

        public static native void quoteContextSubscribeCandlesticks(long context, String symbol, Period period,
                        TradeSessions tradeSessions, AsyncCallback callback);

        public static native void quoteContextUnsubscribeCandlesticks(long context, String symbol, Period period,
                        AsyncCallback callback);

        public static native void quoteContextSubscriptions(long context, AsyncCallback callback);

        public static native void quoteContextStaticInfo(long context, String[] symbols, AsyncCallback callback);

        public static native void quoteContextQuote(long context, String[] symbols, AsyncCallback callback);

        public static native void quoteContextOptionQuote(long context, String[] symbols, AsyncCallback callback);

        public static native void quoteContextWarrantQuote(long context, String[] symbols, AsyncCallback callback);

        public static native void quoteContextDepth(long context, String symbol, AsyncCallback callback);

        public static native void quoteContextBrokers(long context, String symbol, AsyncCallback callback);

        public static native void quoteContextParticipants(long context, AsyncCallback callback);

        public static native void quoteContextTrades(long context, String symbol, int count, AsyncCallback callback);

        public static native void quoteContextIntraday(long context, String symbol, AsyncCallback callback);

        public static native void quoteContextCandlesticks(long context, String symbol, Period period, int count,
                        AdjustType adjustType, TradeSessions tradeSessions, AsyncCallback callback);

        public static native void quoteContextHistoryCandlesticksByOffset(long context, String symbol, Period period,
                        AdjustType adjustType, boolean forward, LocalDateTime datetime, int count,
                        TradeSessions tradeSessions, AsyncCallback callback);

        public static native void quoteContextHistoryCandlesticksByDate(long context, String symbol, Period period,
                        AdjustType adjustType, LocalDate start, LocalDate end, TradeSessions tradeSessions,
                        AsyncCallback callback);

        public static native void quoteContextOptionChainExpiryDateList(long context, String symbol,
                        AsyncCallback callback);

        public static native void quoteContextOptionChainInfoByDate(long context, String symbol, LocalDate expiryDate,
                        AsyncCallback callback);

        public static native void quoteContextWarrantIssuers(long context, AsyncCallback callback);

        public static native void quoteContextWarrantList(long context, QueryWarrantOptions opts,
                        AsyncCallback callback);

        public static native void quoteContextTradingSession(long context, AsyncCallback callback);

        public static native void quoteContextTradingDays(long context, Market market, LocalDate begin, LocalDate end,
                        AsyncCallback callback);

        public static native void quoteContextCapitalFlow(long context, String symbol, AsyncCallback callback);

        public static native void quoteContextCapitalDistribution(long context, String symbol, AsyncCallback callback);

        public static native void quoteContextCalcIndexes(long context, String[] symbols, CalcIndex[] indexes,
                        AsyncCallback callback);

        public static native void quoteContextWatchlist(long context, AsyncCallback callback);

        public static native void quoteContextCreateWatchlistGroup(long context, CreateWatchlistGroup req,
                        AsyncCallback callback);

        public static native void quoteContextDeleteWatchlistGroup(long context, DeleteWatchlistGroup req,
                        AsyncCallback callback);

        public static native void quoteContextUpdateWatchlistGroup(long context, UpdateWatchlistGroup req,
                        AsyncCallback callback);

        public static native void quoteContextSecurityList(long context, Market market,
                        SecurityListCategory category,
                        AsyncCallback callback);

        public static native void quoteContextRealtimeQuote(long context, String[] symbols, AsyncCallback callback);

        public static native void quoteContextRealtimeDepth(long context, String symbol, AsyncCallback callback);

        public static native void quoteContextRealtimeBrokers(long context, String symbol, AsyncCallback callback);

        public static native void quoteContextRealtimeTrades(long context, String symbol, int count,
                        AsyncCallback callback);

        public static native void quoteContextRealtimeCandlesticks(long context, String symbol, Period period,
                        int count,
                        AsyncCallback callback);

        public static native void newTradeContext(long config, AsyncCallback callback);

        public static native void freeTradeContext(long config);

        public static native void tradeContextSetOnOrderChanged(long context, OrderChangedHandler handler);

        public static native void tradeContextSubscribe(long context, TopicType[] topics, AsyncCallback callback);

        public static native void tradeContextUnsubscribe(long context, TopicType[] topics, AsyncCallback callback);

        public static native void tradeContextHistoryExecutions(long context, GetHistoryExecutionsOptions opts,
                        AsyncCallback callback);

        public static native void tradeContextTodayExecutions(long context, GetTodayExecutionsOptions opts,
                        AsyncCallback callback);

        public static native void tradeContextHistoryOrders(long context, GetHistoryOrdersOptions opts,
                        AsyncCallback callback);

        public static native void tradeContextTodayOrders(long context, GetTodayOrdersOptions opts,
                        AsyncCallback callback);

        public static native void tradeContextReplaceOrder(long context, ReplaceOrderOptions opts,
                        AsyncCallback callback);

        public static native void tradeContextSubmitOrder(long context, SubmitOrderOptions opts,
                        AsyncCallback callback);

        public static native void tradeContextCancelOrder(long context, String orderId, AsyncCallback callback);

        public static native void tradeContextAccountBalance(long context, String currency, AsyncCallback callback);

        public static native void tradeContextCashFlow(long context, GetCashFlowOptions opts, AsyncCallback callback);

        public static native void tradeContextFundPositions(long context, GetFundPositionsOptions opts,
                        AsyncCallback callback);

        public static native void tradeContextStockPositions(long context, GetStockPositionsOptions opts,
                        AsyncCallback callback);

        public static native void tradeContextMarginRatio(long context, String symbol, AsyncCallback callback);

        public static native void tradeContextOrderDetail(long context, String orderId, AsyncCallback callback);

        public static native void tradeContextEstimateMaxPurchaseQuantity(long context,
                        EstimateMaxPurchaseQuantityOptions opts,
                        AsyncCallback callback);

        static {
                try {
                        NativeLoader.loadLibrary("longport_java");
                } catch (IOException e) {
                        System.out.println("======================================");
                        System.out.println("Failed to load longport_java");
                        e.printStackTrace();
                        System.out.println("======================================");
                        System.load("longport_java");
                } finally {
                        SdkNative.init();
                }
        }
}
