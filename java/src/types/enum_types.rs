use longbridge_java_macros::impl_java_enum;

impl_java_enum!(
    "com/longbridge/Market",
    longbridge::Market,
    [Unknown, US, HK, CN, SG]
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
    [NormalTrade, PreTrade, PostTrade]
);

impl_java_enum!(
    "com/longbridge/quote/TradeDirection",
    longbridge::quote::TradeDirection,
    [Neutral, Down, Up]
);

impl_java_enum!(
    "com/longbridge/quote/DerivativeType",
    longbridge::quote::DerivativeType,
    [Option, Warrant]
);

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
        #[java(remote = "FiveMinute")]
        Min_5,
        #[java(remote = "FifteenMinute")]
        Min_15,
        #[java(remote = "ThirtyMinute")]
        Min_30,
        #[java(remote = "SixtyMinute")]
        Min_60,
        Day,
        Week,
        Month,
        Year
    ]
);

impl_java_enum!(
    "com/longbridge/quote/AdjustType",
    longbridge::quote::AdjustType,
    [NoAdjust, ForwardAdjust]
);

impl_java_enum!(
    "com/longbridge/trade/OrderSide",
    longbridge::trade::OrderSide,
    [Unknown, Buy, Sell]
);

impl_java_enum!(
    "com/longbridge/trade/OrderType",
    longbridge::trade::OrderType,
    [Unknown, LO, ELO, MO, AO, ALO, ODD, LIT, MIT, TSLPAMT, TSLPPCT, TSMAMT, TSMPCT]
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
    [Unknown, Normal, LongTerm, Grey]
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
    [Unknown, RTHOnly, AnyTime]
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
