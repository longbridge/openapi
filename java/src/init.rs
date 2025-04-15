use jni::{
    JNIEnv,
    descriptors::Desc,
    objects::{GlobalRef, JClass, JValue},
};
use once_cell::sync::OnceCell;

use crate::types::ClassLoader;

pub(crate) static LONG_CLASS: OnceCell<GlobalRef> = OnceCell::new();
pub(crate) static STRING_CLASS: OnceCell<GlobalRef> = OnceCell::new();
pub(crate) static DECIMAL_CLASS: OnceCell<GlobalRef> = OnceCell::new();
pub(crate) static TIME_INSTANT_CLASS: OnceCell<GlobalRef> = OnceCell::new();
pub(crate) static TIME_OFFSETDATETIME_CLASS: OnceCell<GlobalRef> = OnceCell::new();
pub(crate) static TIME_LOCALDATE_CLASS: OnceCell<GlobalRef> = OnceCell::new();
pub(crate) static TIME_LOCALTIME_CLASS: OnceCell<GlobalRef> = OnceCell::new();
pub(crate) static TIME_LOCALDATETIME_CLASS: OnceCell<GlobalRef> = OnceCell::new();
pub(crate) static TIME_ZONE_ID: OnceCell<GlobalRef> = OnceCell::new();
pub(crate) static QUOTE_CONTEXT_CLASS: OnceCell<GlobalRef> = OnceCell::new();
pub(crate) static TRADE_CONTEXT_CLASS: OnceCell<GlobalRef> = OnceCell::new();
pub(crate) static DERIVATIVE_TYPE_CLASS: OnceCell<GlobalRef> = OnceCell::new();
pub(crate) static OPENAPI_EXCEPTION_CLASS: OnceCell<GlobalRef> = OnceCell::new();

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
pub extern "system" fn Java_com_longport_SdkNative_init<'a>(
    mut env: JNIEnv<'a>,
    _class: JClass<'a>,
) {
    init_class!(
        env,
        (LONG_CLASS, "java/lang/Long"),
        (STRING_CLASS, "java/lang/String"),
        (DECIMAL_CLASS, "java/math/BigDecimal"),
        (TIME_INSTANT_CLASS, "java/time/Instant"),
        (TIME_OFFSETDATETIME_CLASS, "java/time/OffsetDateTime"),
        (TIME_LOCALDATE_CLASS, "java/time/LocalDate"),
        (TIME_LOCALTIME_CLASS, "java/time/LocalTime"),
        (TIME_LOCALDATETIME_CLASS, "java/time/LocalDateTime"),
        (DERIVATIVE_TYPE_CLASS, "com/longport/quote/DerivativeType"),
        (OPENAPI_EXCEPTION_CLASS, "com/longport/OpenApiException"),
        (QUOTE_CONTEXT_CLASS, "com/longport/quote/QuoteContext"),
        (TRADE_CONTEXT_CLASS, "com/longport/trade/TradeContext")
    );

    init_timezone_id(&mut env);

    // enum types
    init_class_by_classloader!(
        env,
        longport::Language,
        longport::PushCandlestickMode,
        longport::Market,
        longport::quote::TradeStatus,
        longport::quote::TradeSession,
        longport::quote::TradeDirection,
        longport::quote::OptionType,
        longport::quote::OptionDirection,
        longport::quote::WarrantType,
        longport::quote::WarrantStatus,
        longport::quote::SortOrderType,
        longport::quote::WarrantSortBy,
        longport::quote::FilterWarrantExpiryDate,
        longport::quote::FilterWarrantInOutBoundsType,
        longport::quote::Period,
        longport::quote::AdjustType,
        longport::quote::SecurityBoard,
        longport::quote::SecuritiesUpdateMode,
        longport::quote::CalcIndex,
        longport::quote::SecurityListCategory,
        longport::quote::TradeSessions,
        longport::trade::OrderSide,
        longport::trade::OrderType,
        longport::trade::OrderStatus,
        longport::trade::OrderTag,
        longport::trade::TriggerStatus,
        longport::trade::TopicType,
        longport::trade::TimeInForceType,
        longport::trade::OutsideRTH,
        longport::trade::BalanceType,
        longport::trade::CashFlowDirection,
        longport::trade::CommissionFreeStatus,
        longport::trade::DeductionStatus,
        longport::trade::ChargeCategoryCode
    );

    // classes
    init_class_by_classloader!(
        env,
        longport::quote::Trade,
        longport::quote::Brokers,
        longport::quote::Depth,
        longport::quote::Subscription,
        longport::quote::PushQuote,
        longport::quote::PushDepth,
        longport::quote::PushBrokers,
        longport::quote::PushTrades,
        longport::quote::PushCandlestick,
        longport::quote::SecurityStaticInfo,
        longport::quote::PrePostQuote,
        longport::quote::SecurityQuote,
        longport::quote::OptionQuote,
        longport::quote::WarrantQuote,
        longport::quote::SecurityDepth,
        longport::quote::SecurityBrokers,
        longport::quote::ParticipantInfo,
        longport::quote::IntradayLine,
        longport::quote::Candlestick,
        longport::quote::StrikePriceInfo,
        longport::quote::IssuerInfo,
        longport::quote::WarrantInfo,
        longport::quote::MarketTradingSession,
        longport::quote::TradingSessionInfo,
        longport::quote::MarketTradingDays,
        longport::quote::CapitalFlowLine,
        longport::quote::CapitalDistribution,
        longport::quote::CapitalDistributionResponse,
        crate::types::SecurityCalcIndex,
        longport::quote::WatchlistGroup,
        longport::quote::WatchlistSecurity,
        crate::types::CreateWatchlistGroupResponse,
        longport::quote::RealtimeQuote,
        longport::quote::Security,
        longport::quote::QuotePackageDetail,
        longport::trade::PushOrderChanged,
        longport::trade::Execution,
        longport::trade::Order,
        longport::trade::SubmitOrderResponse,
        longport::trade::CashInfo,
        longport::trade::AccountBalance,
        longport::trade::CashFlow,
        longport::trade::FundPositionsResponse,
        longport::trade::FundPositionChannel,
        longport::trade::FundPosition,
        crate::types::StockPositionsResponse,
        crate::types::StockPositionChannel,
        crate::types::StockPosition,
        longport::trade::MarginRatio,
        longport::trade::OrderHistoryDetail,
        longport::trade::OrderChargeFee,
        longport::trade::OrderChargeItem,
        longport::trade::OrderChargeDetail,
        longport::trade::OrderDetail,
        longport::trade::EstimateMaxPurchaseQuantityResponse
    );
}
