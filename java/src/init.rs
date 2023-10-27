use jni::{
    descriptors::Desc,
    objects::{GlobalRef, JClass, JValue},
    JNIEnv,
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

#[no_mangle]
pub extern "system" fn Java_com_longbridge_SdkNative_init<'a>(
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
        (DERIVATIVE_TYPE_CLASS, "com/longbridge/quote/DerivativeType"),
        (OPENAPI_EXCEPTION_CLASS, "com/longbridge/OpenApiException"),
        (QUOTE_CONTEXT_CLASS, "com/longbridge/quote/QuoteContext"),
        (TRADE_CONTEXT_CLASS, "com/longbridge/trade/TradeContext")
    );

    init_timezone_id(&mut env);

    // enum types
    init_class_by_classloader!(
        env,
        longbridge::Language,
        longbridge::Market,
        longbridge::quote::TradeStatus,
        longbridge::quote::TradeSession,
        longbridge::quote::TradeDirection,
        longbridge::quote::OptionType,
        longbridge::quote::OptionDirection,
        longbridge::quote::WarrantType,
        longbridge::quote::Period,
        longbridge::quote::AdjustType,
        longbridge::quote::SecurityBoard,
        longbridge::quote::SecuritiesUpdateMode,
        longbridge::quote::CalcIndex,
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
        longbridge::trade::PushOrderChanged,
        longbridge::trade::Execution,
        longbridge::trade::Order,
        longbridge::trade::SubmitOrderResponse,
        longbridge::trade::CashInfo,
        longbridge::trade::AccountBalance,
        longbridge::trade::CashFlow,
        longbridge::trade::FundPositionsResponse,
        longbridge::trade::FundPositionChannel,
        longbridge::trade::FundPosition,
        longbridge::trade::StockPositionsResponse,
        longbridge::trade::StockPositionChannel,
        longbridge::trade::StockPosition,
        longbridge::trade::MarginRatio,
        longbridge::trade::OrderHistoryDetail,
        longbridge::trade::OrderChargeFee,
        longbridge::trade::OrderChargeItem,
        longbridge::trade::OrderChargeDetail,
        longbridge::trade::OrderDetail,
        longbridge::trade::EstimateMaxPurchaseQuantityResponse
    );
}
