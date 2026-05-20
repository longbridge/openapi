package com.longbridge;

import java.io.IOException;
import java.time.LocalDate;
import java.util.function.Consumer;
import org.scijava.nativelib.NativeLoader;

import com.longbridge.asset.*;
import com.longbridge.content.*;
import com.longbridge.quote.*;
import com.longbridge.trade.*;

import java.time.LocalDateTime;
import java.time.OffsetDateTime;

/**
 * @hidden
 */
public class SdkNative {
        static native void init();

        public static native long newHttpClientFromApikey(String appKey, String appSecret, String accessToken,
                        String httpUrl);

        public static native long newHttpClientFromApikeyEnv();

        public static native long newHttpClientFromOauth(long oauthToken, String httpUrl);

        public static native void freeHttpClient(long httpClient);

        public static native void httpClientRequest(long httpClient, String request, AsyncCallback callback);

        public static native long newConfigFromApikey(String appKey, String appSecret, String accessToken);

        public static native long newConfigFromApikeyEnv();

        public static native long newConfigFromOauth(long oauth);

        public static native long configSetHttpUrl(long config, String httpUrl);

        public static native long configSetQuoteWsUrl(long config, String quoteWsUrl);

        public static native long configSetTradeWsUrl(long config, String tradeWsUrl);

        public static native long configSetLanguage(long config, Language language);

        public static native long configSetEnableOvernight(long config);

        public static native long configSetPushCandlestickMode(long config, PushCandlestickMode mode);

        public static native long configSetEnablePrintQuotePackages(long config, boolean enable);

        public static native long configSetLogPath(long config, String logPath);

        public static native void configRefreshAccessToken(long config, OffsetDateTime expiredAt,
                        AsyncCallback callback);

        public static native void freeConfig(long config);

        public static native void oauthBuild(String clientId, int callbackPort,
                        Consumer<String> openUrlCallback, AsyncCallback callback);

        public static native void freeOAuth(long oauth);

        public static native long newAssetContext(long config);

        public static native void freeAssetContext(long context);

        public static native void assetContextStatements(long context, Object opts, AsyncCallback callback);

        public static native void assetContextDownloadUrl(long context, String fileKey, AsyncCallback callback);

        public static native long newContentContext(long config);

        public static native void freeContentContext(long context);

        public static native void contentContextMyTopics(long context, Object opts, AsyncCallback callback);

        public static native void contentContextCreateTopic(long context, Object opts, AsyncCallback callback);

        public static native void contentContextTopics(long context, String symbol, AsyncCallback callback);

        public static native void contentContextNews(long context, String symbol, AsyncCallback callback);

        public static native long newQuoteContext(long config);

        public static native void freeQuoteContext(long config);

        public static native void quoteContextGetMemberId(long context, AsyncCallback callback);

        public static native void quoteContextGetQuoteLevel(long context, AsyncCallback callback);

        public static native void quoteContextGetQuotePackageDetails(long context, AsyncCallback callback);

        public static native void quoteContextSetOnQuote(long context, QuoteHandler handler);

        public static native void quoteContextSetOnDepth(long context, DepthHandler handler);

        public static native void quoteContextSetOnBrokers(long context, BrokersHandler handler);

        public static native void quoteContextSetOnTrades(long context, TradesHandler handler);

        public static native void quoteContextSetOnCandlestick(long context, CandlestickHandler handler);

        public static native void quoteContextSubscribe(long context, String[] symbols, int flags, AsyncCallback callback);

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

        public static native void quoteContextIntraday(long context, String symbol, TradeSessions tradeSessions,
                        AsyncCallback callback);

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

        public static native void quoteContextFilings(long context, String symbol, AsyncCallback callback);

        public static native void quoteContextSecurityList(long context, Market market,
                        SecurityListCategory category,
                        AsyncCallback callback);

        public static native void quoteContextMarketTemperature(long context, Market market, AsyncCallback callback);

        public static native void quoteContextHistoryMarketTemperature(long context, Market market, LocalDate start,
                        LocalDate end, AsyncCallback callback);

        public static native void quoteContextRealtimeQuote(long context, String[] symbols, AsyncCallback callback);

        public static native void quoteContextRealtimeDepth(long context, String symbol, AsyncCallback callback);

        public static native void quoteContextRealtimeBrokers(long context, String symbol, AsyncCallback callback);

        public static native void quoteContextRealtimeTrades(long context, String symbol, int count,
                        AsyncCallback callback);

        public static native void quoteContextRealtimeCandlesticks(long context, String symbol, Period period,
                        int count,
                        AsyncCallback callback);

        public static native long newTradeContext(long config);

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

        // ── DCAContext ────────────────────────────────────────────────
        public static native long newDcaContext(long config);
        public static native void freeDcaContext(long context);
        public static native void dcaContextList(long context, Object opts, AsyncCallback callback);
        public static native void dcaContextStats(long context, String symbol, AsyncCallback callback);
        public static native void dcaContextCheckSupport(long context, String[] symbols, AsyncCallback callback);
        public static native void dcaContextHistory(long context, Object opts, AsyncCallback callback);
        public static native void dcaContextPause(long context, String planId, AsyncCallback callback);
        public static native void dcaContextResume(long context, String planId, AsyncCallback callback);
        public static native void dcaContextStop(long context, String planId, AsyncCallback callback);
        public static native void dcaContextCalcDate(long context, Object opts, AsyncCallback callback);
        public static native void dcaContextSetReminder(long context, String hours, AsyncCallback callback);
        public static native void dcaContextCreate(long context, Object opts, AsyncCallback callback);
        public static native void dcaContextUpdate(long context, Object opts, AsyncCallback callback);

        // ── SharelistContext ──────────────────────────────────────────
        public static native long newSharelistContext(long config);
        public static native void freeSharelistContext(long context);
        public static native void sharelistContextList(long context, int count, AsyncCallback callback);
        public static native void sharelistContextDetail(long context, long id, AsyncCallback callback);
        public static native void sharelistContextPopular(long context, int count, AsyncCallback callback);
        public static native void sharelistContextCreate(long context, Object opts, AsyncCallback callback);
        public static native void sharelistContextAddSecurities(long context, long id, String[] symbols, AsyncCallback callback);
        public static native void sharelistContextRemoveSecurities(long context, long id, String[] symbols, AsyncCallback callback);
        public static native void sharelistContextSortSecurities(long context, long id, String[] symbols, AsyncCallback callback);
        public static native void sharelistContextDelete(long context, long id, AsyncCallback callback);

                // ── AlertContext ──────────────────────────────────────────────

        public static native long newAlertContext(long config);
        public static native void freeAlertContext(long context);
        public static native void alertContextList(long context, AsyncCallback callback);
        public static native void alertContextAdd(long context, Object opts, AsyncCallback callback);
        public static native void alertContextEnable(long context, String alertId, AsyncCallback callback);
        public static native void alertContextDisable(long context, String alertId, AsyncCallback callback);
        public static native void alertContextDelete(long context, Object opts, AsyncCallback callback);

                // ── CalendarContext ───────────────────────────────────────────

        public static native long newCalendarContext(long config);
        public static native void freeCalendarContext(long context);
        public static native void calendarContextFinanceCalendar(long context, com.longbridge.calendar.FinanceCalendarOptions opts, AsyncCallback callback);

        // ── PortfolioContext ──────────────────────────────────────────

        public static native long newPortfolioContext(long config);
        public static native void freePortfolioContext(long context);
        public static native void portfolioContextExchangeRate(long context, AsyncCallback callback);
        public static native void portfolioContextProfitAnalysis(long context, Object opts, AsyncCallback callback);
        public static native void portfolioContextProfitAnalysisDetail(long context, Object opts, AsyncCallback callback);
        public static native void portfolioContextProfitAnalysisByMarket(long context, Object opts, AsyncCallback callback);

        // ── QuoteContext extensions (Step 3) ─────────────────────────

        public static native void quoteContextShortPositions(long context, String symbol, AsyncCallback callback);
        public static native void quoteContextOptionVolume(long context, String symbol, AsyncCallback callback);
        public static native void quoteContextOptionVolumeDaily(long context, Object opts, AsyncCallback callback);

        // ── MarketContext ─────────────────────────────────────────────

        public static native long newMarketContext(long config);
        public static native void freeMarketContext(long context);
        public static native void marketContextMarketStatus(long context, AsyncCallback callback);
        public static native void marketContextBrokerHolding(long context, com.longbridge.market.BrokerHoldingOptions opts, AsyncCallback callback);
        public static native void marketContextBrokerHoldingDetail(long context, String symbol, AsyncCallback callback);
        public static native void marketContextBrokerHoldingDaily(long context, com.longbridge.market.BrokerHoldingDailyOptions opts, AsyncCallback callback);
        public static native void marketContextAhPremium(long context, com.longbridge.market.AhPremiumOptions opts, AsyncCallback callback);
        public static native void marketContextAhPremiumIntraday(long context, String symbol, AsyncCallback callback);
        public static native void marketContextTradeStats(long context, String symbol, AsyncCallback callback);
        public static native void marketContextAnomaly(long context, String market, AsyncCallback callback);
        public static native void marketContextConstituent(long context, String symbol, AsyncCallback callback);

        // ── FundamentalContext ────────────────────────────────────────

        public static native long newFundamentalContext(long config);

        public static native void freeFundamentalContext(long context);

        public static native void fundamentalContextFinancialReport(long context,
                        com.longbridge.fundamental.FinancialReportOptions opts,
                        AsyncCallback callback);

        public static native void fundamentalContextInstitutionRating(long context, String symbol,
                        AsyncCallback callback);

        public static native void fundamentalContextInstitutionRatingDetail(long context,
                        String symbol, AsyncCallback callback);

        public static native void fundamentalContextDividend(long context, String symbol,
                        AsyncCallback callback);

        public static native void fundamentalContextDividendDetail(long context, String symbol,
                        AsyncCallback callback);

        public static native void fundamentalContextForecastEps(long context, String symbol,
                        AsyncCallback callback);

        public static native void fundamentalContextConsensus(long context, String symbol,
                        AsyncCallback callback);

        public static native void fundamentalContextValuation(long context, String symbol,
                        AsyncCallback callback);

        public static native void fundamentalContextValuationHistory(long context, String symbol,
                        AsyncCallback callback);

        public static native void fundamentalContextIndustryValuation(long context, String symbol,
                        AsyncCallback callback);

        public static native void fundamentalContextIndustryValuationDist(long context,
                        String symbol, AsyncCallback callback);

        public static native void fundamentalContextCompany(long context, String symbol,
                        AsyncCallback callback);

        public static native void fundamentalContextExecutive(long context, String symbol,
                        AsyncCallback callback);

        public static native void fundamentalContextShareholder(long context, String symbol,
                        AsyncCallback callback);

        public static native void fundamentalContextFundHolder(long context, String symbol,
                        AsyncCallback callback);

        public static native void fundamentalContextCorpAction(long context, String symbol,
                        AsyncCallback callback);

        public static native void fundamentalContextInvestRelation(long context, String symbol,
                        AsyncCallback callback);

        public static native void fundamentalContextOperating(long context, String symbol,
                        AsyncCallback callback);

        public static native void fundamentalContextGetBuyback(long context, String symbol,
                        AsyncCallback callback);

        public static native void fundamentalContextGetRatings(long context, String symbol,
                        AsyncCallback callback);

        public static native void fundamentalContextGetBusinessSegments(long context, String symbol,
                        AsyncCallback callback);

        public static native void fundamentalContextGetBusinessSegmentsHistory(long context,
                        Object opts,
                        AsyncCallback callback);

        public static native void fundamentalContextGetInstitutionRatingViews(long context,
                        String symbol,
                        AsyncCallback callback);

        public static native void fundamentalContextGetIndustryRank(long context, Object opts,
                        AsyncCallback callback);

        public static native void fundamentalContextGetIndustryPeers(long context, Object opts,
                        AsyncCallback callback);

        public static native void fundamentalContextGetFinancialReportSnapshot(long context,
                        Object opts,
                        AsyncCallback callback);

        public static native void portfolioContextProfitAnalysisFlows(long context, Object opts,
                        AsyncCallback callback);

        public static native void quoteContextUpdatePinned(long context, Object req,
                        AsyncCallback callback);

        static {
                try {
                        NativeLoader.loadLibrary("longbridge_java");
                } catch (IOException e) {
                        System.out.println("======================================");
                        System.out.println("Failed to load longbridge_java");
                        e.printStackTrace();
                        System.out.println("======================================");
                        System.load("longbridge_java");
                } finally {
                        SdkNative.init();
                }
        }
}
