use std::sync::OnceLock;

use jni::{
    JNIEnv,
    descriptors::Desc,
    objects::{GlobalRef, JClass, JValue},
};

use crate::types::ClassLoader;

pub(crate) static INTEGER_CLASS: OnceLock<GlobalRef> = OnceLock::new();
pub(crate) static LONG_CLASS: OnceLock<GlobalRef> = OnceLock::new();
pub(crate) static STRING_CLASS: OnceLock<GlobalRef> = OnceLock::new();
pub(crate) static DECIMAL_CLASS: OnceLock<GlobalRef> = OnceLock::new();
pub(crate) static TIME_INSTANT_CLASS: OnceLock<GlobalRef> = OnceLock::new();
pub(crate) static TIME_OFFSETDATETIME_CLASS: OnceLock<GlobalRef> = OnceLock::new();
pub(crate) static TIME_LOCALDATE_CLASS: OnceLock<GlobalRef> = OnceLock::new();
pub(crate) static TIME_LOCALTIME_CLASS: OnceLock<GlobalRef> = OnceLock::new();
pub(crate) static TIME_LOCALDATETIME_CLASS: OnceLock<GlobalRef> = OnceLock::new();
pub(crate) static TIME_ZONE_ID: OnceLock<GlobalRef> = OnceLock::new();
pub(crate) static DERIVATIVE_TYPE_CLASS: OnceLock<GlobalRef> = OnceLock::new();
pub(crate) static OPENAPI_EXCEPTION_CLASS: OnceLock<GlobalRef> = OnceLock::new();

fn init_timezone_id(env: &mut JNIEnv) {
    let utc = env.new_string("UTC").unwrap();
    let zone_id = env
        .call_static_method(
            "java/time/ZoneId",
            "of",
            "(Ljava/lang/String;)Ljava/time/ZoneId;",
            &[JValue::from(&utc)],
        )
        .expect("create zone id");
    let _ = TIME_ZONE_ID.set(env.new_global_ref(zone_id.l().unwrap()).unwrap());
}

macro_rules! init_class {
    ($env:expr, $(($id:ident, $ty:literal)),*) => {
        $(
        let cls = Desc::<JClass>::lookup($ty, &mut $env).expect($ty);
        let _ = $id.set($env.new_global_ref::<&JClass>(&*cls).unwrap());
        )*
    };
}

macro_rules! init_class_by_classloader {
    ($env:expr, $($id:ty),*) => {
        $(
            <$id>::init(&mut $env);
        )*
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_longbridge_SdkNative_init<'a>(
    mut env: JNIEnv<'a>,
    _class: JClass<'a>,
) {
    init_class!(
        env,
        (INTEGER_CLASS, "java/lang/Integer"),
        (LONG_CLASS, "java/lang/Long"),
        (STRING_CLASS, "java/lang/String"),
        (DECIMAL_CLASS, "java/math/BigDecimal"),
        (TIME_INSTANT_CLASS, "java/time/Instant"),
        (TIME_OFFSETDATETIME_CLASS, "java/time/OffsetDateTime"),
        (TIME_LOCALDATE_CLASS, "java/time/LocalDate"),
        (TIME_LOCALTIME_CLASS, "java/time/LocalTime"),
        (TIME_LOCALDATETIME_CLASS, "java/time/LocalDateTime"),
        (DERIVATIVE_TYPE_CLASS, "com/longbridge/quote/DerivativeType"),
        (OPENAPI_EXCEPTION_CLASS, "com/longbridge/OpenApiException")
    );

    init_timezone_id(&mut env);

    // enum types
    init_class_by_classloader!(
        env,
        longbridge::SimpleErrorKind,
        longbridge::Language,
        longbridge::PushCandlestickMode,
        longbridge::Market,
        longbridge::quote::TradeStatus,
        longbridge::quote::TradeSession,
        longbridge::quote::TradeDirection,
        longbridge::quote::OptionType,
        longbridge::quote::OptionDirection,
        longbridge::quote::WarrantType,
        longbridge::quote::WarrantStatus,
        longbridge::quote::SortOrderType,
        longbridge::quote::WarrantSortBy,
        longbridge::quote::FilterWarrantExpiryDate,
        longbridge::quote::FilterWarrantInOutBoundsType,
        longbridge::quote::Period,
        longbridge::quote::AdjustType,
        longbridge::quote::SecurityBoard,
        longbridge::quote::SecuritiesUpdateMode,
        longbridge::quote::CalcIndex,
        longbridge::quote::SecurityListCategory,
        longbridge::quote::TradeSessions,
        longbridge::quote::Granularity,
        longbridge::trade::OrderSide,
        longbridge::trade::OrderType,
        longbridge::trade::OrderStatus,
        longbridge::trade::OrderTag,
        longbridge::trade::TriggerStatus,
        longbridge::trade::TopicType,
        longbridge::trade::TimeInForceType,
        longbridge::trade::OutsideRTH,
        longbridge::trade::BalanceType,
        longbridge::trade::CashFlowDirection,
        longbridge::trade::CommissionFreeStatus,
        longbridge::trade::DeductionStatus,
        longbridge::trade::ChargeCategoryCode,
        longbridge::trade::AttachedOrderType,
        longbridge::quote::PinnedMode,
        longbridge::portfolio::types::FlowDirection,
        longbridge::portfolio::types::AssetType,
        longbridge::fundamental::types::InstitutionRecommend,
        longbridge::fundamental::types::FinancialReportKind,
        longbridge::fundamental::types::FinancialReportPeriod,
        longbridge::market::types::BrokerHoldingPeriod,
        longbridge::market::types::AhPremiumPeriod,
        longbridge::dca::types::DCAFrequency,
        longbridge::dca::types::DCAStatus,
        longbridge::alert::types::AlertCondition,
        longbridge::alert::types::AlertFrequency,
        longbridge::calendar::types::CalendarCategory
    );

    // classes
    init_class_by_classloader!(
        env,
        longbridge::quote::Trade,
        longbridge::quote::Brokers,
        longbridge::quote::Depth,
        longbridge::quote::Subscription,
        longbridge::quote::PushQuote,
        longbridge::quote::PushDepth,
        longbridge::quote::PushBrokers,
        longbridge::quote::PushTrades,
        longbridge::quote::PushCandlestick,
        longbridge::quote::SecurityStaticInfo,
        longbridge::quote::PrePostQuote,
        longbridge::quote::SecurityQuote,
        longbridge::quote::OptionQuote,
        longbridge::quote::WarrantQuote,
        longbridge::quote::SecurityDepth,
        longbridge::quote::SecurityBrokers,
        longbridge::quote::ParticipantInfo,
        longbridge::quote::IntradayLine,
        longbridge::quote::Candlestick,
        longbridge::quote::StrikePriceInfo,
        longbridge::quote::IssuerInfo,
        longbridge::quote::WarrantInfo,
        longbridge::quote::MarketTradingSession,
        longbridge::quote::TradingSessionInfo,
        longbridge::quote::MarketTradingDays,
        longbridge::quote::CapitalFlowLine,
        longbridge::quote::CapitalDistribution,
        longbridge::quote::CapitalDistributionResponse,
        crate::types::SecurityCalcIndex,
        longbridge::quote::WatchlistGroup,
        longbridge::quote::WatchlistSecurity,
        crate::types::CreateWatchlistGroupResponse,
        longbridge::quote::RealtimeQuote,
        longbridge::quote::Security,
        longbridge::quote::QuotePackageDetail,
        longbridge::quote::MarketTemperature,
        longbridge::quote::HistoryMarketTemperatureResponse,
        longbridge::quote::FilingItem,
        longbridge::trade::AttachedOrderDetail,
        longbridge::trade::PushOrderChanged,
        longbridge::trade::Execution,
        longbridge::trade::AllExecutionsResponse,
        longbridge::trade::Order,
        longbridge::trade::SubmitOrderResponse,
        longbridge::trade::CashInfo,
        longbridge::trade::FrozenTransactionFee,
        longbridge::trade::AccountBalance,
        longbridge::trade::CashFlow,
        longbridge::trade::FundPositionsResponse,
        longbridge::trade::FundPositionChannel,
        longbridge::trade::FundPosition,
        crate::types::StockPositionsResponse,
        crate::types::StockPositionChannel,
        crate::types::StockPosition,
        longbridge::trade::MarginRatio,
        longbridge::trade::OrderHistoryDetail,
        longbridge::trade::OrderChargeFee,
        longbridge::trade::OrderChargeItem,
        longbridge::trade::OrderChargeDetail,
        longbridge::trade::OrderDetail,
        longbridge::trade::EstimateMaxPurchaseQuantityResponse,
        longbridge::content::TopicItem,
        longbridge::content::NewsItem,
        longbridge::content::TopicAuthor,
        longbridge::content::TopicImage,
        longbridge::content::OwnedTopic,
        longbridge::dca::DcaPlan,
        longbridge::dca::DcaList,
        longbridge::dca::DcaStats,
        longbridge::sharelist::SharelistStock,
        longbridge::sharelist::SharelistScopes,
        longbridge::sharelist::SharelistInfo,
        longbridge::sharelist::SharelistList,
        longbridge::sharelist::SharelistDetail,
        // DCAContext (partial - types with serde_json::Value use JSON)
        longbridge::dca::DcaHistoryRecord,
        longbridge::dca::DcaHistoryResponse,
        longbridge::dca::DcaSupportInfo,
        longbridge::dca::DcaSupportList,
        longbridge::dca::DcaCalcDateResult,
        // AlertContext - list returns JSON via AlertList IntoJValue (serde_json)
        longbridge::dca::DcaPlan,
        longbridge::dca::DcaList,
        longbridge::dca::DcaStats,
        longbridge::sharelist::SharelistStock,
        longbridge::sharelist::SharelistScopes,
        longbridge::sharelist::SharelistInfo,
        longbridge::sharelist::SharelistList,
        longbridge::sharelist::SharelistDetail,
        // DCAContext (partial - types with serde_json::Value use JSON)
        longbridge::dca::DcaHistoryRecord,
        longbridge::dca::DcaHistoryResponse,
        longbridge::dca::DcaSupportInfo,
        longbridge::dca::DcaSupportList,
        // AlertContext
        longbridge::alert::AlertItem,
        longbridge::alert::AlertSymbolGroup,
        longbridge::alert::AlertList,
        // CalendarContext
        longbridge::calendar::CalendarEventsResponse,
        longbridge::calendar::CalendarDateGroup,
        longbridge::calendar::CalendarEventInfo,
        longbridge::calendar::CalendarDataKv,
        // PortfolioContext
        longbridge::portfolio::ExchangeRates,
        longbridge::portfolio::ExchangeRate,
        longbridge::portfolio::ProfitAnalysis,
        longbridge::portfolio::ProfitAnalysisSummary,
        longbridge::portfolio::ProfitSummaryBreakdown,
        longbridge::portfolio::ProfitSummaryInfo,
        longbridge::portfolio::ProfitAnalysisSublist,
        longbridge::portfolio::ProfitAnalysisItem,
        longbridge::portfolio::ProfitAnalysisDetail,
        longbridge::portfolio::ProfitDetails,
        longbridge::portfolio::ProfitDetailEntry,
        longbridge::portfolio::ProfitAnalysisByMarketItem,
        longbridge::portfolio::ProfitAnalysisByMarket,
        // MarketContext
        longbridge::market::MarketStatusResponse,
        longbridge::market::MarketTimeItem,
        longbridge::market::BrokerHoldingTop,
        longbridge::market::BrokerHoldingEntry,
        longbridge::market::BrokerHoldingDetail,
        longbridge::market::BrokerHoldingDetailItem,
        longbridge::market::BrokerHoldingChanges,
        longbridge::market::BrokerHoldingDailyHistory,
        longbridge::market::BrokerHoldingDailyItem,
        longbridge::market::AhPremiumKlines,
        longbridge::market::AhPremiumIntraday,
        longbridge::market::AhPremiumKline,
        longbridge::market::TradeStatsResponse,
        longbridge::market::TradeStatistics,
        longbridge::market::TradePriceLevel,
        longbridge::market::AnomalyResponse,
        longbridge::market::AnomalyItem,
        longbridge::market::IndexConstituents,
        longbridge::market::ConstituentStock,
        longbridge::dca::DcaPlan,
        longbridge::dca::DcaList,
        longbridge::dca::DcaStats,
        longbridge::sharelist::SharelistStock,
        longbridge::sharelist::SharelistScopes,
        longbridge::sharelist::SharelistInfo,
        longbridge::sharelist::SharelistList,
        longbridge::sharelist::SharelistDetail,
        // DCAContext (partial - types with serde_json::Value use JSON)
        longbridge::dca::DcaHistoryRecord,
        longbridge::dca::DcaHistoryResponse,
        longbridge::dca::DcaSupportInfo,
        longbridge::dca::DcaSupportList,
        longbridge::dca::DcaCalcDateResult,
        // AlertContext - list returns JSON via AlertList IntoJValue (serde_json)
        longbridge::dca::DcaPlan,
        longbridge::dca::DcaList,
        longbridge::dca::DcaStats,
        longbridge::sharelist::SharelistStock,
        longbridge::sharelist::SharelistScopes,
        longbridge::sharelist::SharelistInfo,
        longbridge::sharelist::SharelistList,
        longbridge::sharelist::SharelistDetail,
        // DCAContext (partial - types with serde_json::Value use JSON)
        longbridge::dca::DcaHistoryRecord,
        longbridge::dca::DcaHistoryResponse,
        longbridge::dca::DcaSupportInfo,
        longbridge::dca::DcaSupportList,
        // AlertContext
        longbridge::alert::AlertItem,
        longbridge::alert::AlertSymbolGroup,
        longbridge::alert::AlertList,
        // CalendarContext
        longbridge::calendar::CalendarEventsResponse,
        longbridge::calendar::CalendarDateGroup,
        longbridge::calendar::CalendarEventInfo,
        longbridge::calendar::CalendarDataKv,
        // PortfolioContext
        longbridge::portfolio::ExchangeRates,
        longbridge::portfolio::ExchangeRate,
        longbridge::portfolio::ProfitAnalysis,
        longbridge::portfolio::ProfitAnalysisSummary,
        longbridge::portfolio::ProfitSummaryBreakdown,
        longbridge::portfolio::ProfitSummaryInfo,
        longbridge::portfolio::ProfitAnalysisSublist,
        longbridge::portfolio::ProfitAnalysisItem,
        longbridge::portfolio::ProfitAnalysisDetail,
        longbridge::portfolio::ProfitDetails,
        longbridge::portfolio::ProfitDetailEntry,
        // FundamentalContext
        longbridge::fundamental::FinancialReports,
        longbridge::fundamental::DividendList,
        longbridge::fundamental::DividendItem,
        longbridge::fundamental::InstitutionRating,
        longbridge::fundamental::InstitutionRatingLatest,
        longbridge::fundamental::RatingEvaluate,
        longbridge::fundamental::RatingTarget,
        longbridge::fundamental::InstitutionRatingSummary,
        longbridge::fundamental::RatingSummaryEvaluate,
        longbridge::fundamental::InstitutionRatingDetail,
        longbridge::fundamental::InstitutionRatingDetailEvaluate,
        longbridge::fundamental::InstitutionRatingDetailEvaluateItem,
        longbridge::fundamental::InstitutionRatingDetailTarget,
        longbridge::fundamental::InstitutionRatingDetailTargetItem,
        longbridge::fundamental::ForecastEps,
        longbridge::fundamental::ForecastEpsItem,
        longbridge::fundamental::FinancialConsensus,
        longbridge::fundamental::ConsensusReport,
        longbridge::fundamental::ConsensusDetail,
        longbridge::fundamental::ValuationData,
        longbridge::fundamental::ValuationMetricsData,
        longbridge::fundamental::ValuationMetricData,
        longbridge::fundamental::ValuationPoint,
        longbridge::fundamental::ValuationHistoryResponse,
        longbridge::fundamental::ValuationHistoryData,
        longbridge::fundamental::ValuationHistoryMetrics,
        longbridge::fundamental::ValuationHistoryMetric,
        longbridge::fundamental::IndustryValuationList,
        longbridge::fundamental::IndustryValuationItem,
        longbridge::fundamental::IndustryValuationHistory,
        longbridge::fundamental::IndustryValuationDist,
        longbridge::fundamental::ValuationDist,
        longbridge::fundamental::CompanyOverview,
        longbridge::fundamental::ExecutiveList,
        longbridge::fundamental::ExecutiveGroup,
        longbridge::fundamental::Professional,
        longbridge::fundamental::ShareholderList,
        longbridge::fundamental::Shareholder,
        longbridge::fundamental::ShareholderStock,
        longbridge::fundamental::FundHolders,
        longbridge::fundamental::FundHolder,
        longbridge::fundamental::CorpActions,
        longbridge::fundamental::CorpActionLive,
        longbridge::fundamental::CorpActionItem,
        longbridge::fundamental::InvestRelations,
        longbridge::fundamental::InvestSecurity,
        longbridge::fundamental::OperatingList,
        longbridge::fundamental::OperatingItem,
        longbridge::fundamental::OperatingFinancial,
        longbridge::fundamental::OperatingIndicator,
        // QuoteContext extensions
        longbridge::quote::ShortPositionsItem,
        longbridge::quote::ShortPositionsResponse,
        longbridge::quote::ShortTradesItem,
        longbridge::quote::ShortTradesResponse,
        longbridge::quote::OptionVolumeStats,
        longbridge::quote::OptionVolumeDaily,
        longbridge::quote::OptionVolumeDailyStat,
        // FundamentalContext: BuybackData
        longbridge::fundamental::RecentBuybacks,
        longbridge::fundamental::BuybackHistoryItem,
        longbridge::fundamental::BuybackRatios,
        longbridge::fundamental::BuybackData,
        // FundamentalContext: StockRatings
        longbridge::fundamental::RatingIndicator,
        longbridge::fundamental::RatingLeafIndicator,
        longbridge::fundamental::RatingSubIndicatorGroup,
        longbridge::fundamental::RatingCategory,
        longbridge::fundamental::StockRatings,
        // FundamentalContext: shareholders / valuation comparison
        longbridge::fundamental::ShareholderTopResponse,
        longbridge::fundamental::ShareholderDetailResponse,
        longbridge::fundamental::ValuationHistoryPoint,
        longbridge::fundamental::ValuationComparisonItem,
        longbridge::fundamental::ValuationComparisonResponse,
        // MarketContext: top movers / rank
        longbridge::market::TopMoversStock,
        longbridge::market::TopMoversEvent,
        longbridge::market::TopMoversResponse,
        longbridge::market::RankCategoriesResponse,
        longbridge::market::RankListItem,
        longbridge::market::RankListResponse,
        // ScreenerContext
        longbridge::screener::ScreenerRecommendStrategiesResponse,
        longbridge::screener::ScreenerUserStrategiesResponse,
        longbridge::screener::ScreenerStrategyResponse,
        longbridge::screener::ScreenerSearchResponse,
        longbridge::screener::ScreenerIndicatorsResponse,
        // PortfolioContext: ProfitAnalysisFlows
        longbridge::portfolio::FlowItem,
        longbridge::portfolio::ProfitAnalysisFlows,
        // DCAContext
        longbridge::dca::DcaCreateResult
    );
}
