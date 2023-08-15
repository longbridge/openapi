use longbridge_c_macros::CEnum;

/// Trade status
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::quote::TradeStatus")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CTradeStatus {
    /// Normal
    #[c(remote = "Normal")]
    TradeStatusNormal,
    /// Suspension
    #[c(remote = "Halted")]
    TradeStatusHalted,
    /// Delisted
    #[c(remote = "Delisted")]
    TradeStatusDelisted,
    /// Fuse
    #[c(remote = "Fuse")]
    TradeStatusFuse,
    /// Papare List
    #[c(remote = "PrepareList")]
    TradeStatusPrepareList,
    /// Code Moved
    #[c(remote = "CodeMoved")]
    TradeStatusCodeMoved,
    /// To Be Opened
    #[c(remote = "ToBeOpened")]
    TradeStatusToBeOpened,
    /// Split Stock Halts
    #[c(remote = "SplitStockHalts")]
    TradeStatusSplitStockHalts,
    /// Expired
    #[c(remote = "Expired")]
    TradeStatusExpired,
    /// Warrant To BeListed
    #[c(remote = "WarrantPrepareList")]
    TradeStatusWarrantPrepareList,
    /// Suspend
    #[c(remote = "SuspendTrade")]
    TradeStatusSuspendTrade,
}

/// Trade session
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::quote::TradeSession")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CTradeSession {
    /// Trading
    #[c(remote = "NormalTrade")]
    TradeSessionNormal,
    /// Pre-Trading
    #[c(remote = "PreTrade")]
    TradeSessionPre,
    /// Post-Trading
    #[c(remote = "PostTrade")]
    TradeSessionPost,
}

/// Trade direction
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::quote::TradeDirection")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CTradeDirection {
    /// Neutral
    #[c(remote = "Neutral")]
    TradeDirectionNeutral,
    /// Down
    #[c(remote = "Down")]
    TradeDirectionDown,
    /// Up
    #[c(remote = "Up")]
    TradeDirectionUp,
}

/// Option type
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::quote::OptionType")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum COptionType {
    /// Unknown
    #[c(remote = "Unknown")]
    OptionTypeUnknown,
    /// American
    #[c(remote = "American")]
    OptionTypeAmerican,
    /// Enrope
    #[c(remote = "Europe")]
    OptionTypeEurope,
}

/// Option direction
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::quote::OptionDirection")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum COptionDirection {
    /// Unknown
    #[c(remote = "Unknown")]
    OptionDirectionUnknown,
    /// Put
    #[c(remote = "Put")]
    OptionDirectionPut,
    /// Call
    #[c(remote = "Call")]
    OptionDirectionCall,
}

/// Warrant type
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::quote::WarrantType")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CWarrantType {
    /// Unknown
    #[c(remote = "Unknown")]
    WarrantTypeUnknown,
    /// Put
    #[c(remote = "Put")]
    WarrantTypePut,
    /// Call
    #[c(remote = "Call")]
    WarrantTypeCall,
    /// Bull
    #[c(remote = "Bull")]
    WarrantTypeBull,
    /// Bear
    #[c(remote = "Bear")]
    WarrantTypeBear,
    /// Inline
    #[c(remote = "Inline")]
    WarrantTypeInline,
}

/// Adjust type
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::quote::AdjustType")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CAdjustType {
    /// Actual
    #[c(remote = "NoAdjust")]
    AdjustTypeNoAdjust,
    /// Adjust forward
    #[c(remote = "ForwardAdjust")]
    AdjustTypeForward,
}

/// Adjust type
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::quote::SecurityBoard")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CSecurityBoard {
    /// Unknown
    #[c(remote = "Unknown")]
    SecurityBoardUnknown,
    /// US Main Board
    #[c(remote = "USMain")]
    SecurityBoardUSMain,
    /// US Pink Board
    #[c(remote = "USPink")]
    SecurityBoardUSPink,
    /// Dow Jones Industrial Average
    #[c(remote = "USDJI")]
    SecurityBoardUSDJI,
    /// Nasdsaq Index
    #[c(remote = "USNSDQ")]
    SecurityBoardUSNSDQ,
    /// US Industry Board
    #[c(remote = "USSector")]
    SecurityBoardUSSector,
    /// US Option
    #[c(remote = "USOption")]
    SecurityBoardUSOption,
    /// US Sepecial Option
    #[c(remote = "USOptionS")]
    SecurityBoardUSOptionS,
    /// Hong Kong Equity Securities
    #[c(remote = "HKEquity")]
    SecurityBoardHKEquity,
    /// HK PreIPO Security
    #[c(remote = "HKPreIPO")]
    SecurityBoardHKPreIPO,
    /// HK Warrant
    #[c(remote = "HKWarrant")]
    SecurityBoardHKWarrant,
    /// Hang Seng Index
    #[c(remote = "HKHS")]
    SecurityBoardHKHS,
    /// HK Industry Board
    #[c(remote = "HKSector")]
    SecurityBoardHKSector,
    /// SH Main Board(Connect)
    #[c(remote = "SHMainConnect")]
    SecurityBoardSHMainConnect,
    /// SH Main Board(Non Connect)
    #[c(remote = "SHMainNonConnect")]
    SecurityBoardSHMainNonConnect,
    /// SH Science and Technology Innovation Board
    #[c(remote = "SHSTAR")]
    SecurityBoardSHSTAR,
    /// CN Index
    #[c(remote = "CNIX")]
    SecurityBoardCNIX,
    /// CN Industry Board
    #[c(remote = "CNSector")]
    SecurityBoardCNSector,
    /// SZ Main Board(Connect)
    #[c(remote = "SZMainConnect")]
    SecurityBoardSZMainConnect,
    /// SZ Main Board(Non Connect)
    #[c(remote = "SZMainNonConnect")]
    SecurityBoardSZMainNonConnect,
    /// SZ Gem Board(Connect)
    #[c(remote = "SZGEMConnect")]
    SecurityBoardSZGEMConnect,
    /// SZ Gem Board(Non Connect)
    #[c(remote = "SZGEMNonConnect")]
    SecurityBoardSZGEMNonConnect,
    /// SG Main Board
    #[c(remote = "SGMain")]
    SecurityBoardSGMain,
    /// Singapore Straits Index
    #[c(remote = "STI")]
    SecurityBoardSTI,
    /// SG Industry Board
    #[c(remote = "SGSector")]
    SecurityBoardSGSector,
}

/// Candlestick period
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::quote::Period")]
#[allow(clippy::enum_variant_names, non_camel_case_types)]
#[repr(C)]
pub enum CPeriod {
    /// Unknown
    #[c(remote = "UnknownPeriod")]
    PeriodUnknown,
    /// One Minute
    #[c(remote = "OneMinute")]
    PeriodMin1,
    /// Five Minutes
    #[c(remote = "FiveMinute")]
    PeriodMin5,
    /// Fifteen Minutes
    #[c(remote = "FifteenMinute")]
    PeriodMin15,
    /// Thirty Minutes
    #[c(remote = "ThirtyMinute")]
    PeriodMin30,
    /// Sixty Minutes
    #[c(remote = "SixtyMinute")]
    PeriodMin60,
    /// One Day
    #[c(remote = "Day")]
    PeriodDay,
    /// One Week
    #[c(remote = "Week")]
    PeriodWeek,
    /// One Month
    #[c(remote = "Month")]
    PeriodMonth,
    /// One Year
    #[c(remote = "Year")]
    PeriodYear,
}

/// Trade session
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longbridge::quote::SecuritiesUpdateMode")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CSecuritiesUpdateMode {
    /// Add securities
    #[c(remote = "Add")]
    SecuritiesUpdateModeAdd,
    /// Remove securities
    #[c(remote = "Remove")]
    SecuritiesUpdateModeRemove,
    /// Replace securities
    #[c(remote = "Replace")]
    SecuritiesUpdateModeReplace,
}
