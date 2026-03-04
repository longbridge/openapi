use std::borrow::Cow;

use jni::{
    JNIEnv,
    errors::Result,
    objects::{JObject, JValueOwned},
};
use longbridge_java_macros::impl_java_enum;

use crate::{
    init::DERIVATIVE_TYPE_CLASS,
    types::{IntoJValue, JSignature},
};

impl_java_enum!(
    "com/longbridge/ErrorKind",
    longbridge::SimpleErrorKind,
    [Http, OpenApi, Other, OAuth]
);

impl_java_enum!(
    "com/longbridge/Language",
    longbridge::Language,
    [ZH_CN, ZH_HK, EN]
);

impl_java_enum!(
    "com/longbridge/PushCandlestickMode",
    longbridge::PushCandlestickMode,
    [Realtime, Confirmed]
);

impl_java_enum!(
    "com/longbridge/Market",
    longbridge::Market,
    [Unknown, US, HK, CN, SG, Crypto]
);

impl_java_enum!(
    "com/longbridge/quote/TradeStatus",
    longbridge::quote::TradeStatus,
    [
        Normal,
        Halted,
        Delisted,
        Fuse,
        PrepareList,
        CodeMoved,
        ToBeOpened,
        SplitStockHalts,
        Expired,
        WarrantPrepareList,
        SuspendTrade
    ]
);

impl_java_enum!(
    "com/longbridge/quote/TradeSession",
    longbridge::quote::TradeSession,
    [Intraday, Pre, Post, Overnight]
);

impl_java_enum!(
    "com/longbridge/quote/TradeDirection",
    longbridge::quote::TradeDirection,
    [Neutral, Down, Up]
);

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum DerivativeType {
    Option,
    Warrant,
}

pub(crate) struct DerivativeTypes(Vec<DerivativeType>);

impl From<longbridge::quote::DerivativeType> for DerivativeTypes {
    fn from(ty: longbridge::quote::DerivativeType) -> Self {
        let mut res = Vec::new();
        if ty.contains(longbridge::quote::DerivativeType::OPTION) {
            res.push(DerivativeType::Option);
        }
        if ty.contains(longbridge::quote::DerivativeType::WARRANT) {
            res.push(DerivativeType::Warrant);
        }
        DerivativeTypes(res)
    }
}

impl JSignature for DerivativeTypes {
    fn signature() -> Cow<'static, str> {
        concat!("[L", "com/longbridge/quote/DerivativeType", ";").into()
    }
}

impl IntoJValue for DerivativeTypes {
    fn into_jvalue<'a>(self, env: &mut JNIEnv<'a>) -> Result<JValueOwned<'a>> {
        let cls = DERIVATIVE_TYPE_CLASS.get().unwrap();
        let array = env.new_object_array(self.0.len() as i32, cls, JObject::null())?;
        for (i, obj) in self.0.into_iter().enumerate() {
            let value = match obj {
                DerivativeType::Option => env.get_static_field(
                    cls,
                    "Option",
                    concat!("L", "com/longbridge/quote/DerivativeType", ";"),
                )?,
                DerivativeType::Warrant => env.get_static_field(
                    cls,
                    "Warrant",
                    concat!("L", "com/longbridge/quote/DerivativeType", ";"),
                )?,
            };
            env.set_object_array_element(&array, i as i32, value.l()?)?;
        }
        Ok(array.into())
    }
}

impl_java_enum!(
    "com/longbridge/quote/OptionType",
    longbridge::quote::OptionType,
    [Unknown, American, Europe]
);

impl_java_enum!(
    "com/longbridge/quote/OptionDirection",
    longbridge::quote::OptionDirection,
    [Unknown, Put, Call]
);

impl_java_enum!(
    "com/longbridge/quote/WarrantType",
    longbridge::quote::WarrantType,
    [Unknown, Call, Put, Bull, Bear, Inline]
);

impl_java_enum!(
    "com/longbridge/quote/Period",
    longbridge::quote::Period,
    [
        #[java(remote = "UnknownPeriod")]
        Unknown,
        #[java(remote = "OneMinute")]
        Min_1,
        #[java(remote = "TwoMinute")]
        Min_2,
        #[java(remote = "ThreeMinute")]
        Min_3,
        #[java(remote = "FiveMinute")]
        Min_5,
        #[java(remote = "TenMinute")]
        Min_10,
        #[java(remote = "FifteenMinute")]
        Min_15,
        #[java(remote = "TwentyMinute")]
        Min_20,
        #[java(remote = "ThirtyMinute")]
        Min_30,
        #[java(remote = "FortyFiveMinute")]
        Min_45,
        #[java(remote = "SixtyMinute")]
        Min_60,
        #[java(remote = "TwoHour")]
        Min_120,
        #[java(remote = "ThreeHour")]
        Min_180,
        #[java(remote = "FourHour")]
        Min_240,
        Day,
        Week,
        Month,
        #[java(remote = "Quarter")]
        Quarter,
        Year,
    ]
);

impl_java_enum!(
    "com/longbridge/quote/AdjustType",
    longbridge::quote::AdjustType,
    [NoAdjust, ForwardAdjust]
);

impl_java_enum!(
    "com/longbridge/quote/SecurityBoard",
    longbridge::quote::SecurityBoard,
    [
        Unknown,
        USMain,
        USPink,
        USDJI,
        USNSDQ,
        USSector,
        USOption,
        USOptionS,
        HKEquity,
        HKPreIPO,
        HKWarrant,
        HKHS,
        HKSector,
        SHMainConnect,
        SHMainNonConnect,
        SHSTAR,
        CNIX,
        CNSector,
        SZMainConnect,
        SZMainNonConnect,
        SZGEMConnect,
        SZGEMNonConnect,
        SGMain,
        STI,
        SGSector,
        SPXIndex,
        VIXIndex,
    ]
);

impl_java_enum!(
    "com/longbridge/quote/SecuritiesUpdateMode",
    longbridge::quote::SecuritiesUpdateMode,
    [Add, Remove, Replace]
);

impl_java_enum!(
    "com/longbridge/quote/CalcIndex",
    longbridge::quote::CalcIndex,
    [
        LastDone,
        ChangeValue,
        ChangeRate,
        Volume,
        Turnover,
        YtdChangeRate,
        TurnoverRate,
        TotalMarketValue,
        CapitalFlow,
        Amplitude,
        VolumeRatio,
        PeTtmRatio,
        PbRatio,
        DividendRatioTtm,
        FiveDayChangeRate,
        TenDayChangeRate,
        HalfYearChangeRate,
        FiveMinutesChangeRate,
        ExpiryDate,
        StrikePrice,
        UpperStrikePrice,
        LowerStrikePrice,
        OutstandingQty,
        OutstandingRatio,
        Premium,
        ItmOtm,
        ImpliedVolatility,
        WarrantDelta,
        CallPrice,
        ToCallPrice,
        EffectiveLeverage,
        LeverageRatio,
        ConversionRatio,
        BalancePoint,
        OpenInterest,
        Delta,
        Gamma,
        Theta,
        Vega,
        Rho,
    ]
);

impl_java_enum!(
    "com/longbridge/quote/WarrantStatus",
    longbridge::quote::WarrantStatus,
    [Suspend, PrepareList, Normal]
);

impl_java_enum!(
    "com/longbridge/quote/SortOrderType",
    longbridge::quote::SortOrderType,
    [Ascending, Descending]
);

impl_java_enum!(
    "com/longbridge/quote/WarrantSortBy",
    longbridge::quote::WarrantSortBy,
    [
        LastDone,
        ChangeRate,
        ChangeValue,
        Volume,
        Turnover,
        ExpiryDate,
        StrikePrice,
        UpperStrikePrice,
        LowerStrikePrice,
        OutstandingQuantity,
        OutstandingRatio,
        Premium,
        ItmOtm,
        ImpliedVolatility,
        Delta,
        CallPrice,
        ToCallPrice,
        EffectiveLeverage,
        LeverageRatio,
        ConversionRatio,
        BalancePoint,
        Status,
    ]
);

impl_java_enum!(
    "com/longbridge/quote/FilterWarrantExpiryDate",
    longbridge::quote::FilterWarrantExpiryDate,
    [LT_3, Between_3_6, Between_6_12, GT_12]
);

impl_java_enum!(
    "com/longbridge/quote/FilterWarrantInOutBoundsType",
    longbridge::quote::FilterWarrantInOutBoundsType,
    [In, Out]
);

impl_java_enum!(
    "com/longbridge/quote/SecurityListCategory",
    longbridge::quote::SecurityListCategory,
    [Overnight]
);

impl_java_enum!(
    "com/longbridge/quote/TradeSessions",
    longbridge::quote::TradeSessions,
    [Intraday, All]
);

impl_java_enum!(
    "com/longbridge/quote/Granularity",
    longbridge::quote::Granularity,
    [Unknown, Daily, Weekly, Monthly]
);

impl_java_enum!(
    "com/longbridge/trade/OrderSide",
    longbridge::trade::OrderSide,
    [Unknown, Buy, Sell]
);

impl_java_enum!(
    "com/longbridge/trade/OrderType",
    longbridge::trade::OrderType,
    [
        Unknown, LO, ELO, MO, AO, ALO, ODD, LIT, MIT, TSLPAMT, TSLPPCT, TSMAMT, TSMPCT, SLO
    ]
);

impl_java_enum!(
    "com/longbridge/trade/OrderStatus",
    longbridge::trade::OrderStatus,
    [
        Unknown,
        NotReported,
        ReplacedNotReported,
        ProtectedNotReported,
        VarietiesNotReported,
        Filled,
        WaitToNew,
        New,
        WaitToReplace,
        PendingReplace,
        Replaced,
        PartialFilled,
        WaitToCancel,
        PendingCancel,
        Rejected,
        Canceled,
        Expired,
        PartialWithdrawal,
    ]
);

impl_java_enum!(
    "com/longbridge/trade/OrderTag",
    longbridge::trade::OrderTag,
    [
        Unknown,
        Normal,
        LongTerm,
        Grey,
        MarginCall,
        Offline,
        Creditor,
        Debtor,
        NonExercise,
        AllocatedSub,
    ]
);

impl_java_enum!(
    "com/longbridge/trade/TriggerStatus",
    longbridge::trade::TriggerStatus,
    [Unknown, Deactive, Active, Released]
);

impl_java_enum!(
    "com/longbridge/trade/TopicType",
    longbridge::trade::TopicType,
    [Private]
);

impl_java_enum!(
    "com/longbridge/trade/TimeInForceType",
    longbridge::trade::TimeInForceType,
    [Unknown, Day, GoodTilCanceled, GoodTilDate]
);

impl_java_enum!(
    "com/longbridge/trade/OutsideRTH",
    longbridge::trade::OutsideRTH,
    [Unknown, RTHOnly, AnyTime, Overnight]
);

impl_java_enum!(
    "com/longbridge/trade/BalanceType",
    longbridge::trade::BalanceType,
    [Unknown, Cash, Stock, Fund]
);

impl_java_enum!(
    "com/longbridge/trade/CashFlowDirection",
    longbridge::trade::CashFlowDirection,
    [Unknown, Out, In]
);

impl_java_enum!(
    "com/longbridge/trade/CommissionFreeStatus",
    longbridge::trade::CommissionFreeStatus,
    [Unknown, None, Calculated, Pending, Ready]
);

impl_java_enum!(
    "com/longbridge/trade/DeductionStatus",
    longbridge::trade::DeductionStatus,
    [Unknown, None, NoData, Pending, Done]
);

impl_java_enum!(
    "com/longbridge/trade/ChargeCategoryCode",
    longbridge::trade::ChargeCategoryCode,
    [Unknown, Broker, Third]
);
