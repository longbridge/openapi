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
pub(crate) static QUOTE_CONTEXT_CLASS: OnceLock<GlobalRef> = OnceLock::new();
pub(crate) static TRADE_CONTEXT_CLASS: OnceLock<GlobalRef> = OnceLock::new();
pub(crate) static CONTENT_CONTEXT_CLASS: OnceLock<GlobalRef> = OnceLock::new();
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
        (OPENAPI_EXCEPTION_CLASS, "com/longbridge/OpenApiException"),
        (QUOTE_CONTEXT_CLASS, "com/longbridge/quote/QuoteContext"),
        (TRADE_CONTEXT_CLASS, "com/longbridge/trade/TradeContext"),
        (
            CONTENT_CONTEXT_CLASS,
            "com/longbridge/content/ContentContext"
        )
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
        longbridge::trade::ChargeCategoryCode
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
        longbridge::trade::PushOrderChanged,
        longbridge::trade::Execution,
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
        longbridge::content::NewsItem
    );
}
