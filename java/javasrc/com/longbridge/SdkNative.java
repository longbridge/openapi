package com.longbridge;

import java.time.LocalDate;

import com.longbridge.quote.*;
import com.longbridge.trade.*;

public class SdkNative {
        static native void init();

        public static native long newConfig(String appKey, String appSecret, String accessToken, String httpUrl,
                        String quoteWsUrl, String tradeWsUrl);

        public static native long newConfigFromEnv();

        public static native void freeConfig(long config);

        public static native void newQuoteContext(long config, AsyncCallback callback);

        public static native void freeQuoteContext(long config);

        public static native void quoteContextSetOnQuote(long context, QuoteHandler handler);

        public static native void quoteContextSetOnDepth(long context, DepthHandler handler);

        public static native void quoteContextSetOnBrokers(long context, BrokersHandler handler);

        public static native void quoteContextSetOnTrades(long context, TradesHandler handler);

        public static native void quoteContextSubscribe(long context, String[] symbols, int flags, boolean isFirstPush,
                        AsyncCallback callback);

        public static native void quoteContextUnsubscribe(long context, String[] symbols, int flags,
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
                        AdjustType adjustType, AsyncCallback callback);

        public static native void quoteContextOptionChainExpiryDateList(long context, String symbol,
                        AsyncCallback callback);

        public static native void quoteContextOptionChainInfoByDate(long context, String symbol, LocalDate expiryDate,
                        AsyncCallback callback);

        public static native void quoteContextWarrantIssuers(long context, AsyncCallback callback);

        public static native void quoteContextTradingSession(long context, AsyncCallback callback);

        public static native void quoteContextTradingDays(long context, Market market, LocalDate begin, LocalDate end,
                        AsyncCallback callback);

        public static native void quoteContextRealtimeQuote(long context, String[] symbols, AsyncCallback callback);

        public static native void quoteContextRealtimeDepth(long context, String symbol, AsyncCallback callback);

        public static native void quoteContextRealtimeBrokers(long context, String symbol, AsyncCallback callback);

        public static native void quoteContextRealtimeTrades(long context, String symbol, int count,
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

        public static native void tradeContextAccountBalance(long context, AsyncCallback callback);

        public static native void tradeContextCashFlow(long context, GetCashFlowOptions opts, AsyncCallback callback);

        public static native void tradeContextFundPositions(long context, GetFundPositionsOptions opts,
                        AsyncCallback callback);

        public static native void tradeContextStockPositions(long context, GetStockPositionsOptions opts,
                        AsyncCallback callback);

        static {
                System.loadLibrary("longbridge_java");
                SdkNative.init();
        }
}
