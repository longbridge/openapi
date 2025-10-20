use longport_c_macros::CEnum;

/// Trade status
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longport::quote::TradeStatus")]
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
#[c(remote = "longport::quote::TradeSession")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CTradeSession {
    /// Trading
    #[c(remote = "Intraday")]
    TradeSessionIntraday,
    /// Pre-Trading
    #[c(remote = "Pre")]
    TradeSessionPre,
    /// Post-Trading
    #[c(remote = "Post")]
    TradeSessionPost,
    /// Post-Trading
    #[c(remote = "Overnight")]
    TradeSessionOvernight,
}

/// Trade direction
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longport::quote::TradeDirection")]
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
#[c(remote = "longport::quote::OptionType")]
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
#[c(remote = "longport::quote::OptionDirection")]
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
#[c(remote = "longport::quote::WarrantType")]
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
#[c(remote = "longport::quote::AdjustType")]
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
#[c(remote = "longport::quote::SecurityBoard")]
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
    /// S&P 500 Index
    #[c(remote = "SPXIndex")]
    SecurityBoardSPXIndex,
    /// CBOE Volatility Index
    #[c(remote = "VIXIndex")]
    SecurityBoardVIXIndex,
}

/// Candlestick period
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longport::quote::Period")]
#[allow(clippy::enum_variant_names, non_camel_case_types)]
#[repr(C)]
pub enum CPeriod {
    /// Unknown
    #[c(remote = "UnknownPeriod")]
    PeriodUnknown,
    /// One Minute
    #[c(remote = "OneMinute")]
    PeriodMin1,
    /// Two Minutes
    #[c(remote = "TwoMinute")]
    PeriodMin2,
    /// Three Minutes
    #[c(remote = "ThreeMinute")]
    PeriodMin3,
    /// Five Minutes
    #[c(remote = "FiveMinute")]
    PeriodMin5,
    /// Ten Minutes
    #[c(remote = "TenMinute")]
    PeriodMin10,
    /// Fifteen Minutes
    #[c(remote = "FifteenMinute")]
    PeriodMin15,
    /// Twenty Minutes
    #[c(remote = "TwentyMinute")]
    PeriodMin20,
    /// Thirty Minutes
    #[c(remote = "ThirtyMinute")]
    PeriodMin30,
    /// Forty-Five Minutes
    #[c(remote = "FortyFiveMinute")]
    PeriodMin45,
    /// One Hour
    #[c(remote = "SixtyMinute")]
    PeriodMin60,
    /// Two Hours
    #[c(remote = "TwoHour")]
    PeriodMin120,
    /// Three Hours
    #[c(remote = "ThreeHour")]
    PeriodMin180,
    /// Four Hours
    #[c(remote = "FourHour")]
    PeriodMin240,
    /// Daily
    #[c(remote = "Day")]
    PeriodDay,
    /// Weekly
    #[c(remote = "Week")]
    PeriodWeek,
    /// Monthly
    #[c(remote = "Month")]
    PeriodMonth,
    /// Quarterly
    #[c(remote = "Quarter")]
    PeriodQuarter,
    /// Yearly
    #[c(remote = "Year")]
    PeriodYear,
}

/// Trade session
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longport::quote::SecuritiesUpdateMode")]
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

/// Calc index
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longport::quote::CalcIndex")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CCalcIndex {
    /// Latest price
    #[c(remote = "LastDone")]
    CalcIndexLastDone,
    /// Change value
    #[c(remote = "ChangeValue")]
    CalcIndexChangeValue,
    /// Change rate
    #[c(remote = "ChangeRate")]
    CalcIndexChangeRate,
    /// Volume
    #[c(remote = "Volume")]
    CalcIndexVolume,
    /// Turnover
    #[c(remote = "Turnover")]
    CalcIndexTurnover,
    /// Year-to-date change ratio
    #[c(remote = "YtdChangeRate")]
    CalcIndexYtdChangeRate,
    /// Turnover rate
    #[c(remote = "TurnoverRate")]
    CalcIndexTurnoverRate,
    /// Total market value
    #[c(remote = "TotalMarketValue")]
    CalcIndexTotalMarketValue,
    /// Capital flow
    #[c(remote = "CapitalFlow")]
    CalcIndexCapitalFlow,
    /// Amplitude
    #[c(remote = "Amplitude")]
    CalcIndexAmplitude,
    /// Volume ratio
    #[c(remote = "VolumeRatio")]
    CalcIndexVolumeRatio,
    /// PE (TTM)
    #[c(remote = "PeTtmRatio")]
    CalcIndexPeTtmRatio,
    /// PB
    #[c(remote = "PbRatio")]
    CalcIndexPbRatio,
    /// Dividend ratio (TTM)
    #[c(remote = "DividendRatioTtm")]
    CalcIndexDividendRatioTtm,
    /// Five days change ratio
    #[c(remote = "FiveDayChangeRate")]
    CalcIndexFiveDayChangeRate,
    /// Ten days change ratio
    #[c(remote = "TenDayChangeRate")]
    CalcIndexTenDayChangeRate,
    /// Half year change ratio
    #[c(remote = "HalfYearChangeRate")]
    CalcIndexHalfYearChangeRate,
    /// Five minutes change ratio
    #[c(remote = "FiveMinutesChangeRate")]
    CalcIndexFiveMinutesChangeRate,
    /// Expiry date
    #[c(remote = "ExpiryDate")]
    CalcIndexExpiryDate,
    /// Strike price
    #[c(remote = "StrikePrice")]
    CalcIndexStrikePrice,
    /// Upper bound price
    #[c(remote = "UpperStrikePrice")]
    CalcIndexUpperStrikePrice,
    /// Lower bound price
    #[c(remote = "LowerStrikePrice")]
    CalcIndexLowerStrikePrice,
    /// Outstanding quantity
    #[c(remote = "OutstandingQty")]
    CalcIndexOutstandingQty,
    /// Outstanding ratio
    #[c(remote = "OutstandingRatio")]
    CalcIndexOutstandingRatio,
    /// Premium
    #[c(remote = "Premium")]
    CalcIndexPremium,
    /// In/out of the bound
    #[c(remote = "ItmOtm")]
    CalcIndexItmOtm,
    /// Implied volatility
    #[c(remote = "ImpliedVolatility")]
    CalcIndexImpliedVolatility,
    /// Warrant delta
    #[c(remote = "WarrantDelta")]
    CalcIndexWarrantDelta,
    /// Call price
    #[c(remote = "CallPrice")]
    CalcIndexCallPrice,
    /// Price interval from the call price
    #[c(remote = "ToCallPrice")]
    CalcIndexToCallPrice,
    /// Effective leverage
    #[c(remote = "EffectiveLeverage")]
    CalcIndexEffectiveLeverage,
    /// Leverage ratio
    #[c(remote = "LeverageRatio")]
    CalcIndexLeverageRatio,
    /// Conversion ratio
    #[c(remote = "ConversionRatio")]
    CalcIndexConversionRatio,
    /// Breakeven point
    #[c(remote = "BalancePoint")]
    CalcIndexBalancePoint,
    /// Open interest
    #[c(remote = "OpenInterest")]
    CalcIndexOpenInterest,
    /// Delta
    #[c(remote = "Delta")]
    CalcIndexDelta,
    /// Gamma
    #[c(remote = "Gamma")]
    CalcIndexGamma,
    /// Theta
    #[c(remote = "Theta")]
    CalcIndexTheta,
    /// Vega
    #[c(remote = "Vega")]
    CalcIndexVega,
    /// Rho
    #[c(remote = "Rho")]
    CalcIndexRho,
}

/// Sort order type
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longport::quote::SortOrderType")]
#[repr(C)]
pub enum CSortOrderType {
    /// Ascending
    #[c(remote = "Ascending")]
    SortOrderAscending,
    /// Descending
    #[c(remote = "Descending")]
    SortOrderDescending,
}

/// Warrant sort by
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longport::quote::WarrantSortBy")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CWarrantSortBy {
    /// Last done
    #[c(remote = "LastDone")]
    WarrantSortByLastDone,
    /// Change rate
    #[c(remote = "ChangeRate")]
    WarrantSortByChangeRate,
    /// Change value
    #[c(remote = "ChangeValue")]
    WarrantSortByChangeValue,
    /// Volume
    #[c(remote = "Volume")]
    WarrantSortByVolume,
    /// Turnover
    #[c(remote = "Turnover")]
    WarrantSortByTurnover,
    /// Expiry date
    #[c(remote = "ExpiryDate")]
    WarrantSortByExpiryDate,
    /// Strike price
    #[c(remote = "StrikePrice")]
    WarrantSortByStrikePrice,
    /// Upper strike price
    #[c(remote = "UpperStrikePrice")]
    WarrantSortByUpperStrikePrice,
    /// Lower strike price
    #[c(remote = "LowerStrikePrice")]
    WarrantSortByLowerStrikePrice,
    /// Outstanding quantity
    #[c(remote = "OutstandingQuantity")]
    WarrantSortByOutstandingQuantity,
    /// Outstanding ratio
    #[c(remote = "OutstandingRatio")]
    WarrantSortByOutstandingRatio,
    /// Premium
    #[c(remote = "Premium")]
    WarrantSortByPremium,
    /// In/out of the bound
    #[c(remote = "ItmOtm")]
    WarrantSortByItmOtm,
    /// Implied volatility
    #[c(remote = "ImpliedVolatility")]
    WarrantSortByImpliedVolatility,
    /// Greek value delta
    #[c(remote = "Delta")]
    WarrantSortByDelta,
    /// Call price
    #[c(remote = "CallPrice")]
    WarrantSortByCallPrice,
    /// Price interval from the call price
    #[c(remote = "ToCallPrice")]
    WarrantSortByToCallPrice,
    /// Effective leverage
    #[c(remote = "EffectiveLeverage")]
    WarrantSortByEffectiveLeverage,
    /// Leverage ratio
    #[c(remote = "LeverageRatio")]
    WarrantSortByLeverageRatio,
    /// Conversion ratio
    #[c(remote = "ConversionRatio")]
    WarrantSortByConversionRatio,
    /// Breakeven point
    #[c(remote = "BalancePoint")]
    WarrantSortByBalancePoint,
    /// Status
    #[c(remote = "Status")]
    WarrantSortByStatus,
}

/// Filter warrant expiry date type
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longport::quote::FilterWarrantExpiryDate")]
#[allow(non_camel_case_types)]
#[repr(C)]
pub enum CFilterWarrantExpiryDate {
    /// Less than 3 months
    #[c(remote = "LT_3")]
    WarrantExpiryDate_LT_3,
    /// 3 - 6 months
    #[c(remote = "Between_3_6")]
    WarrantExpiryDate_Between_3_6,
    /// 6 - 12 months
    #[c(remote = "Between_6_12")]
    WarrantExpiryDate_Between_6_12,
    /// Greater than 12 months
    #[c(remote = "GT_12")]
    WarrantExpiryDate_GT_12,
}

/// Filter warrant in/out of the bounds type
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longport::quote::FilterWarrantInOutBoundsType")]
#[allow(non_camel_case_types)]
#[repr(C)]
pub enum CFilterWarrantInOutBoundsType {
    /// In bounds
    #[c(remote = "In")]
    WarrantInOutBoundsType_In,
    /// Out bounds
    #[c(remote = "Out")]
    WarrantInOutBoundsType_Out,
}

/// Warrant status
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longport::quote::WarrantStatus")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CWarrantStatus {
    /// Suspend
    #[c(remote = "Suspend")]
    WarrantStatusSuspend,
    /// Prepare List
    #[c(remote = "PrepareList")]
    WarrantStatusPrepareList,
    /// Normal
    #[c(remote = "Normal")]
    WarrantStatusNormal,
}

/// Security list category
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longport::quote::SecurityListCategory")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CSecurityListCategory {
    /// Overnight
    #[c(remote = "Overnight")]
    SecurityListCategoryOvernight,
}

/// Trade sessions
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longport::quote::TradeSessions")]
#[repr(C)]
pub enum CTradeSessions {
    /// Intraday
    #[c(remote = "Intraday")]
    TradeSessionsIntraday = 0,
    /// All
    #[c(remote = "All")]
    TradeSessionsAll = 100,
}

/// Data granularity
#[derive(Debug, Copy, Clone, Eq, PartialEq, CEnum)]
#[c(remote = "longport::quote::Granularity")]
#[allow(clippy::enum_variant_names)]
#[repr(C)]
pub enum CGranularity {
    /// Unknown
    #[c(remote = "Unknown")]
    GranularityUnknown,
    /// Daily
    #[c(remote = "Daily")]
    GranularityDaily,
    /// Weekly
    #[c(remote = "Weekly")]
    GranularityWeekly,
    /// Monthly
    #[c(remote = "Monthly")]
    GranularityMonthly,
}
