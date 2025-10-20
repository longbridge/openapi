use std::borrow::Cow;

use jni::{
    JNIEnv,
    errors::Result,
    objects::{JObject, JValueOwned},
};
use longport_java_macros::impl_java_enum;

use crate::{
    init::DERIVATIVE_TYPE_CLASS,
    types::{IntoJValue, JSignature},
};

impl_java_enum!(
    "com/longport/ErrorKind",
    longport::SimpleErrorKind,
    [Http, OpenApi, Other]
);

impl_java_enum!(
    "com/longport/Language",
    longport::Language,
    [ZH_CN, ZH_HK, EN]
);

impl_java_enum!(
    "com/longport/PushCandlestickMode",
    longport::PushCandlestickMode,
    [Realtime, Confirmed]
);

impl_java_enum!(
    "com/longport/Market",
    longport::Market,
    [Unknown, US, HK, CN, SG, Crypto]
);

impl_java_enum!(
    "com/longport/quote/TradeStatus",
    longport::quote::TradeStatus,
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
    "com/longport/quote/TradeSession",
    longport::quote::TradeSession,
    [Intraday, Pre, Post, Overnight]
);

impl_java_enum!(
    "com/longport/quote/TradeDirection",
    longport::quote::TradeDirection,
    [Neutral, Down, Up]
);

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum DerivativeType {
    Option,
    Warrant,
}

pub(crate) struct DerivativeTypes(Vec<DerivativeType>);

impl From<longport::quote::DerivativeType> for DerivativeTypes {
    fn from(ty: longport::quote::DerivativeType) -> Self {
        let mut res = Vec::new();
        if ty.contains(longport::quote::DerivativeType::OPTION) {
            res.push(DerivativeType::Option);
        }
        if ty.contains(longport::quote::DerivativeType::WARRANT) {
            res.push(DerivativeType::Warrant);
        }
        DerivativeTypes(res)
    }
}

impl JSignature for DerivativeTypes {
    fn signature() -> Cow<'static, str> {
        concat!("[L", "com/longport/quote/DerivativeType", ";").into()
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
                    concat!("L", "com/longport/quote/DerivativeType", ";"),
                )?,
                DerivativeType::Warrant => env.get_static_field(
                    cls,
                    "Warrant",
                    concat!("L", "com/longport/quote/DerivativeType", ";"),
                )?,
            };
            env.set_object_array_element(&array, i as i32, value.l()?)?;
        }
        Ok(array.into())
    }
}

impl_java_enum!(
    "com/longport/quote/OptionType",
    longport::quote::OptionType,
    [Unknown, American, Europe]
);

impl_java_enum!(
    "com/longport/quote/OptionDirection",
    longport::quote::OptionDirection,
    [Unknown, Put, Call]
);

impl_java_enum!(
    "com/longport/quote/WarrantType",
    longport::quote::WarrantType,
    [Unknown, Call, Put, Bull, Bear, Inline]
);

impl_java_enum!(
    "com/longport/quote/Period",
    longport::quote::Period,
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
    "com/longport/quote/AdjustType",
    longport::quote::AdjustType,
    [NoAdjust, ForwardAdjust]
);

impl_java_enum!(
    "com/longport/quote/SecurityBoard",
    longport::quote::SecurityBoard,
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
    "com/longport/quote/SecuritiesUpdateMode",
    longport::quote::SecuritiesUpdateMode,
    [Add, Remove, Replace]
);

impl_java_enum!(
    "com/longport/quote/CalcIndex",
    longport::quote::CalcIndex,
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
    "com/longport/quote/WarrantStatus",
    longport::quote::WarrantStatus,
    [Suspend, PrepareList, Normal]
);

impl_java_enum!(
    "com/longport/quote/SortOrderType",
    longport::quote::SortOrderType,
    [Ascending, Descending]
);

impl_java_enum!(
    "com/longport/quote/WarrantSortBy",
    longport::quote::WarrantSortBy,
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
    "com/longport/quote/FilterWarrantExpiryDate",
    longport::quote::FilterWarrantExpiryDate,
    [LT_3, Between_3_6, Between_6_12, GT_12]
);

impl_java_enum!(
    "com/longport/quote/FilterWarrantInOutBoundsType",
    longport::quote::FilterWarrantInOutBoundsType,
    [In, Out]
);

impl_java_enum!(
    "com/longport/quote/SecurityListCategory",
    longport::quote::SecurityListCategory,
    [Overnight]
);

impl_java_enum!(
    "com/longport/quote/TradeSessions",
    longport::quote::TradeSessions,
    [Intraday, All]
);

impl_java_enum!(
    "com/longport/quote/Granularity",
    longport::quote::Granularity,
    [Unknown, Daily, Weekly, Monthly]
);

impl_java_enum!(
    "com/longport/trade/OrderSide",
    longport::trade::OrderSide,
    [Unknown, Buy, Sell]
);

impl_java_enum!(
    "com/longport/trade/OrderType",
    longport::trade::OrderType,
    [
        Unknown, LO, ELO, MO, AO, ALO, ODD, LIT, MIT, TSLPAMT, TSLPPCT, TSMAMT, TSMPCT, SLO
    ]
);

impl_java_enum!(
    "com/longport/trade/OrderStatus",
    longport::trade::OrderStatus,
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
    "com/longport/trade/OrderTag",
    longport::trade::OrderTag,
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
    "com/longport/trade/TriggerStatus",
    longport::trade::TriggerStatus,
    [Unknown, Deactive, Active, Released]
);

impl_java_enum!(
    "com/longport/trade/TopicType",
    longport::trade::TopicType,
    [Private]
);

impl_java_enum!(
    "com/longport/trade/TimeInForceType",
    longport::trade::TimeInForceType,
    [Unknown, Day, GoodTilCanceled, GoodTilDate]
);

impl_java_enum!(
    "com/longport/trade/OutsideRTH",
    longport::trade::OutsideRTH,
    [Unknown, RTHOnly, AnyTime, Overnight]
);

impl_java_enum!(
    "com/longport/trade/BalanceType",
    longport::trade::BalanceType,
    [Unknown, Cash, Stock, Fund]
);

impl_java_enum!(
    "com/longport/trade/CashFlowDirection",
    longport::trade::CashFlowDirection,
    [Unknown, Out, In]
);

impl_java_enum!(
    "com/longport/trade/CommissionFreeStatus",
    longport::trade::CommissionFreeStatus,
    [Unknown, None, Calculated, Pending, Ready]
);

impl_java_enum!(
    "com/longport/trade/DeductionStatus",
    longport::trade::DeductionStatus,
    [Unknown, None, NoData, Pending, Done]
);

impl_java_enum!(
    "com/longport/trade/ChargeCategoryCode",
    longport::trade::ChargeCategoryCode,
    [Unknown, Broker, Third]
);
