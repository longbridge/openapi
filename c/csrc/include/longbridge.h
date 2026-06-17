#ifndef _LONGBRIDGE_H_
#define _LONGBRIDGE_H_

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Quote
 */
#define LB_SUBFLAGS_QUOTE 1

/**
 * Depth
 */
#define LB_SUBFLAGS_DEPTH 2

/**
 * Broker
 */
#define LB_SUBFLAGS_BROKER 4

/**
 * Trade
 */
#define LB_SUBFLAGS_TRADE 8

/**
 * US stock options
 */
#define LB_DERIVATIVE_TYPE_OPTION 1

/**
 * HK warrants
 */
#define LB_DERIVATIVE_TYPE_WARRANT 2

/**
 * Update name of watchlist group
 */
#define LB_WATCHLIST_GROUP_NAME 1

/**
 * Update securities of watchlist group
 */
#define LB_WATCHLIST_GROUP_SECURITIES 2

/**
 * Alert trigger condition
 */
typedef enum lb_alert_condition_t {
  /**
   * Price rises above threshold
   */
  AlertConditionPriceRise,
  /**
   * Price falls below threshold
   */
  AlertConditionPriceFall,
  /**
   * Percentage rises above threshold
   */
  AlertConditionPercentRise,
  /**
   * Percentage falls below threshold
   */
  AlertConditionPercentFall,
} lb_alert_condition_t;

/**
 * Alert notification frequency
 */
typedef enum lb_alert_frequency_t {
  /**
   * Trigger at most once per day
   */
  AlertFrequencyDaily,
  /**
   * Trigger every time the condition is met
   */
  AlertFrequencyEveryTime,
  /**
   * Trigger only the first time
   */
  AlertFrequencyOnce,
} lb_alert_frequency_t;

/**
 * Financial calendar event category
 */
typedef enum lb_calendar_category_t {
  /**
   * Earnings reports
   */
  CalendarCategoryReport,
  /**
   * Dividend announcements
   */
  CalendarCategoryDividend,
  /**
   * Stock splits
   */
  CalendarCategorySplit,
  /**
   * IPOs
   */
  CalendarCategoryIpo,
  /**
   * Macro-economic data
   */
  CalendarCategoryMacroData,
  /**
   * Market closure days
   */
  CalendarCategoryClosed,
  /**
   * Shareholder / analyst meetings
   */
  CalendarCategoryMeeting,
  /**
   * Stock consolidations / mergers
   */
  CalendarCategoryMerge,
} lb_calendar_category_t;

/**
 * Language identifer
 */
typedef enum lb_language_t {
  /**
   * zh-CN
   */
  Language_ZH_CN,
  /**
   * zh-HK
   */
  Language_ZH_HK,
  /**
   * en
   */
  Language_EN,
} lb_language_t;

/**
 * Language identifer
 */
typedef enum lb_push_candlestick_mode_t {
  /**
   * Real-time
   */
  PushCandlestickMode_Realtime,
  /**
   * Confirmed
   */
  PushCandlestickMode_Confirmed,
} lb_push_candlestick_mode_t;

/**
 * DCA investment frequency
 */
typedef enum lb_dca_frequency_t {
  /**
   * Invest every trading day
   */
  DcaFrequencyDaily,
  /**
   * Invest once per week
   */
  DcaFrequencyWeekly,
  /**
   * Invest every two weeks
   */
  DcaFrequencyFortnightly,
  /**
   * Invest once per month
   */
  DcaFrequencyMonthly,
} lb_dca_frequency_t;

/**
 * Error kind
 */
typedef enum lb_error_kind_t {
  /**
   * HTTP error
   */
  ErrorKindHttp,
  /**
   * OpenAPI error
   */
  ErrorKindOpenApi,
  /**
   * Other error
   */
  ErrorKindOther,
  /**
   * OAuth error
   */
  ErrorKindOAuth,
} lb_error_kind_t;

/**
 * Financial report kind
 */
typedef enum lb_financial_report_kind_t {
  /**
   * Income statement
   */
  FinancialReportKindIncomeStatement,
  /**
   * Balance sheet
   */
  FinancialReportKindBalanceSheet,
  /**
   * Cash flow statement
   */
  FinancialReportKindCashFlow,
  /**
   * All statements (default)
   */
  FinancialReportKindAll,
} lb_financial_report_kind_t;

/**
 * Broker holding lookback period
 */
typedef enum lb_broker_holding_period_t {
  /**
   * 1 recent trading day
   */
  BrokerHoldingPeriodRct1,
  /**
   * 5 recent trading days
   */
  BrokerHoldingPeriodRct5,
  /**
   * 20 recent trading days
   */
  BrokerHoldingPeriodRct20,
  /**
   * 60 recent trading days
   */
  BrokerHoldingPeriodRct60,
} lb_broker_holding_period_t;

/**
 * A/H premium K-line period
 */
typedef enum lb_ah_premium_period_t {
  /**
   * 1-minute
   */
  AhPremiumPeriodMin1,
  /**
   * 5-minute
   */
  AhPremiumPeriodMin5,
  /**
   * 15-minute
   */
  AhPremiumPeriodMin15,
  /**
   * 30-minute
   */
  AhPremiumPeriodMin30,
  /**
   * 60-minute
   */
  AhPremiumPeriodMin60,
  /**
   * Daily
   */
  AhPremiumPeriodDay,
  /**
   * Weekly
   */
  AhPremiumPeriodWeek,
  /**
   * Monthly
   */
  AhPremiumPeriodMonth,
  /**
   * Yearly
   */
  AhPremiumPeriodYear,
} lb_ah_premium_period_t;

/**
 * Trade status
 */
typedef enum lb_trade_status_t {
  /**
   * Normal
   */
  TradeStatusNormal,
  /**
   * Suspension
   */
  TradeStatusHalted,
  /**
   * Delisted
   */
  TradeStatusDelisted,
  /**
   * Fuse
   */
  TradeStatusFuse,
  /**
   * Papare List
   */
  TradeStatusPrepareList,
  /**
   * Code Moved
   */
  TradeStatusCodeMoved,
  /**
   * To Be Opened
   */
  TradeStatusToBeOpened,
  /**
   * Split Stock Halts
   */
  TradeStatusSplitStockHalts,
  /**
   * Expired
   */
  TradeStatusExpired,
  /**
   * Warrant To BeListed
   */
  TradeStatusWarrantPrepareList,
  /**
   * Suspend
   */
  TradeStatusSuspendTrade,
} lb_trade_status_t;

/**
 * Trade session
 */
typedef enum lb_trade_session_t {
  /**
   * Trading
   */
  TradeSessionIntraday,
  /**
   * Pre-Trading
   */
  TradeSessionPre,
  /**
   * Post-Trading
   */
  TradeSessionPost,
  /**
   * Post-Trading
   */
  TradeSessionOvernight,
} lb_trade_session_t;

/**
 * Trade direction
 */
typedef enum lb_trade_direction_t {
  /**
   * Neutral
   */
  TradeDirectionNeutral,
  /**
   * Down
   */
  TradeDirectionDown,
  /**
   * Up
   */
  TradeDirectionUp,
} lb_trade_direction_t;

/**
 * Candlestick period
 */
typedef enum lb_period_t {
  /**
   * Unknown
   */
  PeriodUnknown,
  /**
   * One Minute
   */
  PeriodMin1,
  /**
   * Two Minutes
   */
  PeriodMin2,
  /**
   * Three Minutes
   */
  PeriodMin3,
  /**
   * Five Minutes
   */
  PeriodMin5,
  /**
   * Ten Minutes
   */
  PeriodMin10,
  /**
   * Fifteen Minutes
   */
  PeriodMin15,
  /**
   * Twenty Minutes
   */
  PeriodMin20,
  /**
   * Thirty Minutes
   */
  PeriodMin30,
  /**
   * Forty-Five Minutes
   */
  PeriodMin45,
  /**
   * One Hour
   */
  PeriodMin60,
  /**
   * Two Hours
   */
  PeriodMin120,
  /**
   * Three Hours
   */
  PeriodMin180,
  /**
   * Four Hours
   */
  PeriodMin240,
  /**
   * Daily
   */
  PeriodDay,
  /**
   * Weekly
   */
  PeriodWeek,
  /**
   * Monthly
   */
  PeriodMonth,
  /**
   * Quarterly
   */
  PeriodQuarter,
  /**
   * Yearly
   */
  PeriodYear,
} lb_period_t;

/**
 * Trade sessions
 */
typedef enum lb_trade_sessions_t {
  /**
   * Intraday
   */
  TradeSessionsIntraday = 0,
  /**
   * All
   */
  TradeSessionsAll = 100,
} lb_trade_sessions_t;

/**
 * Adjust type
 */
typedef enum lb_adjust_type_t {
  /**
   * Actual
   */
  AdjustTypeNoAdjust,
  /**
   * Adjust forward
   */
  AdjustTypeForward,
} lb_adjust_type_t;

/**
 * Warrant sort by
 */
typedef enum lb_warrant_sort_by_t {
  /**
   * Last done
   */
  WarrantSortByLastDone,
  /**
   * Change rate
   */
  WarrantSortByChangeRate,
  /**
   * Change value
   */
  WarrantSortByChangeValue,
  /**
   * Volume
   */
  WarrantSortByVolume,
  /**
   * Turnover
   */
  WarrantSortByTurnover,
  /**
   * Expiry date
   */
  WarrantSortByExpiryDate,
  /**
   * Strike price
   */
  WarrantSortByStrikePrice,
  /**
   * Upper strike price
   */
  WarrantSortByUpperStrikePrice,
  /**
   * Lower strike price
   */
  WarrantSortByLowerStrikePrice,
  /**
   * Outstanding quantity
   */
  WarrantSortByOutstandingQuantity,
  /**
   * Outstanding ratio
   */
  WarrantSortByOutstandingRatio,
  /**
   * Premium
   */
  WarrantSortByPremium,
  /**
   * In/out of the bound
   */
  WarrantSortByItmOtm,
  /**
   * Implied volatility
   */
  WarrantSortByImpliedVolatility,
  /**
   * Greek value delta
   */
  WarrantSortByDelta,
  /**
   * Call price
   */
  WarrantSortByCallPrice,
  /**
   * Price interval from the call price
   */
  WarrantSortByToCallPrice,
  /**
   * Effective leverage
   */
  WarrantSortByEffectiveLeverage,
  /**
   * Leverage ratio
   */
  WarrantSortByLeverageRatio,
  /**
   * Conversion ratio
   */
  WarrantSortByConversionRatio,
  /**
   * Breakeven point
   */
  WarrantSortByBalancePoint,
  /**
   * Status
   */
  WarrantSortByStatus,
} lb_warrant_sort_by_t;

/**
 * Sort order type
 */
typedef enum lb_sort_order_type_t {
  /**
   * Ascending
   */
  SortOrderAscending,
  /**
   * Descending
   */
  SortOrderDescending,
} lb_sort_order_type_t;

/**
 * Warrant type
 */
typedef enum lb_warrant_type_t {
  /**
   * Unknown
   */
  WarrantTypeUnknown,
  /**
   * Put
   */
  WarrantTypePut,
  /**
   * Call
   */
  WarrantTypeCall,
  /**
   * Bull
   */
  WarrantTypeBull,
  /**
   * Bear
   */
  WarrantTypeBear,
  /**
   * Inline
   */
  WarrantTypeInline,
} lb_warrant_type_t;

/**
 * Filter warrant expiry date type
 */
typedef enum lb_filter_warrant_expiry_date_t {
  /**
   * Less than 3 months
   */
  WarrantExpiryDate_LT_3,
  /**
   * 3 - 6 months
   */
  WarrantExpiryDate_Between_3_6,
  /**
   * 6 - 12 months
   */
  WarrantExpiryDate_Between_6_12,
  /**
   * Greater than 12 months
   */
  WarrantExpiryDate_GT_12,
} lb_filter_warrant_expiry_date_t;

/**
 * Filter warrant in/out of the bounds type
 */
typedef enum lb_filter_warrant_in_out_bounds_type_t {
  /**
   * In bounds
   */
  WarrantInOutBoundsType_In,
  /**
   * Out bounds
   */
  WarrantInOutBoundsType_Out,
} lb_filter_warrant_in_out_bounds_type_t;

/**
 * Warrant status
 */
typedef enum lb_warrant_status_t {
  /**
   * Suspend
   */
  WarrantStatusSuspend,
  /**
   * Prepare List
   */
  WarrantStatusPrepareList,
  /**
   * Normal
   */
  WarrantStatusNormal,
} lb_warrant_status_t;

/**
 * Market type
 */
typedef enum lb_market_t {
  /**
   * Unknown
   */
  MarketUnknown,
  /**
   * US market
   */
  MarketUS,
  /**
   * HK market
   */
  MarketHK,
  /**
   * CN market
   */
  MarketCN,
  /**
   * SG market
   */
  MarketSG,
  /**
   * Crypto market
   */
  MarketCrypto,
} lb_market_t;

/**
 * Calc index
 */
typedef enum lb_calc_index_t {
  /**
   * Latest price
   */
  CalcIndexLastDone,
  /**
   * Change value
   */
  CalcIndexChangeValue,
  /**
   * Change rate
   */
  CalcIndexChangeRate,
  /**
   * Volume
   */
  CalcIndexVolume,
  /**
   * Turnover
   */
  CalcIndexTurnover,
  /**
   * Year-to-date change ratio
   */
  CalcIndexYtdChangeRate,
  /**
   * Turnover rate
   */
  CalcIndexTurnoverRate,
  /**
   * Total market value
   */
  CalcIndexTotalMarketValue,
  /**
   * Capital flow
   */
  CalcIndexCapitalFlow,
  /**
   * Amplitude
   */
  CalcIndexAmplitude,
  /**
   * Volume ratio
   */
  CalcIndexVolumeRatio,
  /**
   * PE (TTM)
   */
  CalcIndexPeTtmRatio,
  /**
   * PB
   */
  CalcIndexPbRatio,
  /**
   * Dividend ratio (TTM)
   */
  CalcIndexDividendRatioTtm,
  /**
   * Five days change ratio
   */
  CalcIndexFiveDayChangeRate,
  /**
   * Ten days change ratio
   */
  CalcIndexTenDayChangeRate,
  /**
   * Half year change ratio
   */
  CalcIndexHalfYearChangeRate,
  /**
   * Five minutes change ratio
   */
  CalcIndexFiveMinutesChangeRate,
  /**
   * Expiry date
   */
  CalcIndexExpiryDate,
  /**
   * Strike price
   */
  CalcIndexStrikePrice,
  /**
   * Upper bound price
   */
  CalcIndexUpperStrikePrice,
  /**
   * Lower bound price
   */
  CalcIndexLowerStrikePrice,
  /**
   * Outstanding quantity
   */
  CalcIndexOutstandingQty,
  /**
   * Outstanding ratio
   */
  CalcIndexOutstandingRatio,
  /**
   * Premium
   */
  CalcIndexPremium,
  /**
   * In/out of the bound
   */
  CalcIndexItmOtm,
  /**
   * Implied volatility
   */
  CalcIndexImpliedVolatility,
  /**
   * Warrant delta
   */
  CalcIndexWarrantDelta,
  /**
   * Call price
   */
  CalcIndexCallPrice,
  /**
   * Price interval from the call price
   */
  CalcIndexToCallPrice,
  /**
   * Effective leverage
   */
  CalcIndexEffectiveLeverage,
  /**
   * Leverage ratio
   */
  CalcIndexLeverageRatio,
  /**
   * Conversion ratio
   */
  CalcIndexConversionRatio,
  /**
   * Breakeven point
   */
  CalcIndexBalancePoint,
  /**
   * Open interest
   */
  CalcIndexOpenInterest,
  /**
   * Delta
   */
  CalcIndexDelta,
  /**
   * Gamma
   */
  CalcIndexGamma,
  /**
   * Theta
   */
  CalcIndexTheta,
  /**
   * Vega
   */
  CalcIndexVega,
  /**
   * Rho
   */
  CalcIndexRho,
} lb_calc_index_t;

/**
 * Trade session
 */
typedef enum lb_securities_update_mode_t {
  /**
   * Add securities
   */
  SecuritiesUpdateModeAdd,
  /**
   * Remove securities
   */
  SecuritiesUpdateModeRemove,
  /**
   * Replace securities
   */
  SecuritiesUpdateModeReplace,
} lb_securities_update_mode_t;

/**
 * Security list category
 */
typedef enum lb_security_list_category_t {
  /**
   * Overnight
   */
  SecurityListCategoryOvernight,
} lb_security_list_category_t;

/**
 * Order side
 */
typedef enum lb_order_side_t {
  /**
   * Unknown
   */
  OrderSideUnknown,
  /**
   * Buy
   */
  OrderSideBuy,
  /**
   * Sell
   */
  OrderSideSell,
} lb_order_side_t;

/**
 * Order type
 */
typedef enum lb_order_type_t {
  /**
   * Unknown
   */
  OrderTypeUnknown,
  /**
   * Limit Order
   */
  OrderTypeLO,
  /**
   * Enhanced Limit Order
   */
  OrderTypeELO,
  /**
   * Market Order
   */
  OrderTypeMO,
  /**
   * At-auction Order
   */
  OrderTypeAO,
  /**
   * At-auction Limit Order
   */
  OrderTypeALO,
  /**
   * Odd Lots
   */
  OrderTypeODD,
  /**
   * Limit If Touched
   */
  OrderTypeLIT,
  /**
   * Market If Touched
   */
  OrderTypeMIT,
  /**
   * Trailing Limit If Touched (Trailing Amount)
   */
  OrderTypeTSLPAMT,
  /**
   * Trailing Limit If Touched (Trailing Percent)
   */
  OrderTypeTSLPPCT,
  /**
   * Trailing Market If Touched (Trailing Amount)
   */
  OrderTypeTSMAMT,
  /**
   * Trailing Market If Touched (Trailing Percent)
   */
  OrderTypeTSMPCT,
  /**
   * Special Limit Order
   */
  OrderTypeSLO,
} lb_order_type_t;

/**
 * Order status
 */
typedef enum lb_order_status_t {
  /**
   * Unknown
   */
  OrderStatusUnknown,
  /**
   * Not reported
   */
  OrderStatusNotReported,
  /**
   * Not reported (Replaced Order)
   */
  OrderStatusReplacedNotReported,
  /**
   * Not reported (Protected Order)
   */
  OrderStatusProtectedNotReported,
  /**
   * Not reported (Conditional Order)
   */
  OrderStatusVarietiesNotReported,
  /**
   * Filled
   */
  OrderStatusFilled,
  /**
   * Wait To New
   */
  OrderStatusWaitToNew,
  /**
   * New
   */
  OrderStatusNew,
  /**
   * Wait To Replace
   */
  OrderStatusWaitToReplace,
  /**
   * Pending Replace
   */
  OrderStatusPendingReplace,
  /**
   * Replaced
   */
  OrderStatusReplaced,
  /**
   * Partial Filled
   */
  OrderStatusPartialFilled,
  /**
   * Wait To Cancel
   */
  OrderStatusWaitToCancel,
  /**
   * Pending Cancel
   */
  OrderStatusPendingCancel,
  /**
   * Rejected
   */
  OrderStatusRejected,
  /**
   * Canceled
   */
  OrderStatusCanceled,
  /**
   * Expired
   */
  OrderStatusExpired,
  /**
   * Partial Withdrawal
   */
  OrderStatusPartialWithdrawal,
} lb_order_status_t;

/**
 * Order tag
 */
typedef enum lb_order_tag_t {
  /**
   * Unknown
   */
  OrderTagUnknown,
  /**
   * Normal Order
   */
  OrderTagNormal,
  /**
   * Long term Order
   */
  OrderTagLongTerm,
  /**
   * Grey Order
   */
  OrderTagGrey,
} lb_order_tag_t;

/**
 * Order tag
 */
typedef enum lb_trigger_status_t {
  /**
   * Unknown
   */
  TriggerStatusUnknown,
  /**
   * Deactive
   */
  TriggerStatusDeactive,
  /**
   * Active
   */
  TriggerStatusActive,
  /**
   * Released
   */
  TriggerStatusReleased,
} lb_trigger_status_t;

/**
 * Topic type
 */
typedef enum lb_topic_type_t {
  /**
   * Trading
   */
  TopicPrivate,
} lb_topic_type_t;

/**
 * Attached order type
 */
typedef enum CAttachedOrderType {
  /**
   * Unknown
   */
  AttachedOrderTypeUnknown = 0,
  /**
   * Take profit
   */
  AttachedOrderTypeProfitTaker = 1,
  /**
   * Stop loss
   */
  AttachedOrderTypeStopLoss = 2,
  /**
   * Bracket order
   */
  AttachedOrderTypeBracket = 3,
} CAttachedOrderType;

/**
 * Time in force Type
 */
typedef enum lb_time_in_force_type_t {
  /**
   * Unknown
   */
  TimeInForceUnknown,
  /**
   * Day Order
   */
  TimeInForceDay,
  /**
   * Good Til Canceled Order
   */
  TimeInForceGoodTilCanceled,
  /**
   * Good Til Date Order
   */
  TimeInForceGoodTilDate,
} lb_time_in_force_type_t;

/**
 * Enable or disable outside regular trading hours
 */
typedef enum lb_outside_rth_t {
  /**
   * Unknown
   */
  OutsideRTHUnknown,
  /**
   * Regular trading hour only
   */
  OutsideRTHOnly,
  /**
   * Any time
   */
  OutsideRTHAnyTime,
  /**
   * Overnight
   */
  OutsideRTHOvernight,
  /**
   * Overnight option
   */
  OutsideRTHOptionPreMarket,
} lb_outside_rth_t;

/**
 * Balance type
 */
typedef enum lb_balance_type_t {
  /**
   * Unknown
   */
  BalanceTypeUnknown,
  /**
   * Cash
   */
  BalanceTypeCash,
  /**
   * Stock
   */
  BalanceTypeStock,
  /**
   * Fund
   */
  BalanceTypeFund,
} lb_balance_type_t;

/**
 * Adjust type
 */
typedef enum lb_security_board_t {
  /**
   * Unknown
   */
  SecurityBoardUnknown,
  /**
   * US Main Board
   */
  SecurityBoardUSMain,
  /**
   * US Pink Board
   */
  SecurityBoardUSPink,
  /**
   * Dow Jones Industrial Average
   */
  SecurityBoardUSDJI,
  /**
   * Nasdsaq Index
   */
  SecurityBoardUSNSDQ,
  /**
   * US Industry Board
   */
  SecurityBoardUSSector,
  /**
   * US Option
   */
  SecurityBoardUSOption,
  /**
   * US Sepecial Option
   */
  SecurityBoardUSOptionS,
  /**
   * Hong Kong Equity Securities
   */
  SecurityBoardHKEquity,
  /**
   * HK PreIPO Security
   */
  SecurityBoardHKPreIPO,
  /**
   * HK Warrant
   */
  SecurityBoardHKWarrant,
  /**
   * Hang Seng Index
   */
  SecurityBoardHKHS,
  /**
   * HK Industry Board
   */
  SecurityBoardHKSector,
  /**
   * SH Main Board(Connect)
   */
  SecurityBoardSHMainConnect,
  /**
   * SH Main Board(Non Connect)
   */
  SecurityBoardSHMainNonConnect,
  /**
   * SH Science and Technology Innovation Board
   */
  SecurityBoardSHSTAR,
  /**
   * CN Index
   */
  SecurityBoardCNIX,
  /**
   * CN Industry Board
   */
  SecurityBoardCNSector,
  /**
   * SZ Main Board(Connect)
   */
  SecurityBoardSZMainConnect,
  /**
   * SZ Main Board(Non Connect)
   */
  SecurityBoardSZMainNonConnect,
  /**
   * SZ Gem Board(Connect)
   */
  SecurityBoardSZGEMConnect,
  /**
   * SZ Gem Board(Non Connect)
   */
  SecurityBoardSZGEMNonConnect,
  /**
   * SG Main Board
   */
  SecurityBoardSGMain,
  /**
   * Singapore Straits Index
   */
  SecurityBoardSTI,
  /**
   * SG Industry Board
   */
  SecurityBoardSGSector,
  /**
   * S&P 500 Index
   */
  SecurityBoardSPXIndex,
  /**
   * CBOE Volatility Index
   */
  SecurityBoardVIXIndex,
} lb_security_board_t;

/**
 * Option type
 */
typedef enum lb_option_type_t {
  /**
   * Unknown
   */
  OptionTypeUnknown,
  /**
   * American
   */
  OptionTypeAmerican,
  /**
   * Enrope
   */
  OptionTypeEurope,
} lb_option_type_t;

/**
 * Option direction
 */
typedef enum lb_option_direction_t {
  /**
   * Unknown
   */
  OptionDirectionUnknown,
  /**
   * Put
   */
  OptionDirectionPut,
  /**
   * Call
   */
  OptionDirectionCall,
} lb_option_direction_t;

/**
 * Cash flow direction
 */
typedef enum lb_cash_flow_direction_t {
  /**
   * Unknown
   */
  CashFlowDirectionUnknown,
  /**
   * Out
   */
  CashFlowDirectionOut,
  /**
   * In
   */
  CashFlowDirectionIn,
} lb_cash_flow_direction_t;

/**
 * Commission-free Status
 */
typedef enum lb_commission_free_status_t {
  /**
   * Unknown
   */
  CommissionFreeStatusUnknown,
  /**
   * None
   */
  CommissionFreeStatusNone,
  /**
   * Commission-free amount to be calculated
   */
  CommissionFreeStatusCalculated,
  /**
   * Pending commission-free
   */
  CommissionFreeStatusPending,
  /**
   * Commission-free applied
   */
  CommissionFreeStatusReady,
} lb_commission_free_status_t;

/**
 * Deduction status
 */
typedef enum lb_deduction_status_t {
  /**
   * Unknown
   */
  DeductionStatusUnknown,
  /**
   * Pending Settlement
   */
  DeductionStatusNone,
  /**
   * Commission-free amount to be calculated
   */
  DeductionStatusNoData,
  /**
   * Pending commission-free
   */
  DeductionStatusPending,
  /**
   * Commission-free applied
   */
  DeductionStatusDone,
} lb_deduction_status_t;

/**
 * Charge category code
 */
typedef enum lb_charge_category_code_t {
  /**
   * Unknown
   */
  ChargeCategoryCodeUnknown,
  /**
   * Broker
   */
  ChargeCategoryCodeBroker,
  /**
   * Third
   */
  ChargeCategoryCodeThird,
} lb_charge_category_code_t;

/**
 * Data granularity
 */
typedef enum lb_granularity_t {
  /**
   * Unknown
   */
  GranularityUnknown,
  /**
   * Daily
   */
  GranularityDaily,
  /**
   * Weekly
   */
  GranularityWeekly,
  /**
   * Monthly
   */
  GranularityMonthly,
} lb_granularity_t;

/**
 * Institutional analyst recommendation
 */
typedef enum lb_institution_recommend_t {
  /**
   * Unknown
   */
  InstitutionRecommendUnknown,
  /**
   * Strong buy
   */
  InstitutionRecommendStrongBuy,
  /**
   * Buy
   */
  InstitutionRecommendBuy,
  /**
   * Hold
   */
  InstitutionRecommendHold,
  /**
   * Sell
   */
  InstitutionRecommendSell,
  /**
   * Strong sell
   */
  InstitutionRecommendStrongSell,
  /**
   * Underperform
   */
  InstitutionRecommendUnderperform,
  /**
   * No opinion
   */
  InstitutionRecommendNoOpinion,
} lb_institution_recommend_t;

/**
 * DCA plan status
 */
typedef enum lb_dca_status_t {
  /**
   * Plan is active
   */
  DcaStatusActive,
  /**
   * Plan has been paused
   */
  DcaStatusSuspended,
  /**
   * Plan has finished
   */
  DcaStatusFinished,
} lb_dca_status_t;

/**
 * Financial report period
 */
typedef enum lb_financial_report_period_t {
  /**
   * Annual report
   */
  FinancialReportPeriodAnnual,
  /**
   * Semi-annual report
   */
  FinancialReportPeriodSemiAnnual,
  /**
   * Q1 report
   */
  FinancialReportPeriodQ1,
  /**
   * Q2 report
   */
  FinancialReportPeriodQ2,
  /**
   * Q3 report
   */
  FinancialReportPeriodQ3,
  /**
   * Full quarterly report
   */
  FinancialReportPeriodQuarterlyFull,
  /**
   * Three-quarter report (first three quarters)
   */
  FinancialReportPeriodThreeQ,
} lb_financial_report_period_t;

/**
 * Flow direction
 */
typedef enum lb_flow_direction_t {
  /**
   * Unknown direction
   */
  FlowDirectionUnknown,
  /**
   * Buy
   */
  FlowDirectionBuy,
  /**
   * Sell
   */
  FlowDirectionSell,
} lb_flow_direction_t;

/**
 * Asset type
 */
typedef enum lb_asset_type_t {
  /**
   * Unknown type
   */
  AssetTypeUnknown,
  /**
   * Stock
   */
  AssetTypeStock,
  /**
   * Fund
   */
  AssetTypeFund,
  /**
   * Crypto
   */
  AssetTypeCrypto,
} lb_asset_type_t;

/**
 * ETF asset allocation element type
 */
typedef enum lb_element_type_t {
  /**
   * Unknown
   */
  ElementTypeUnknown,
  /**
   * Holdings
   */
  ElementTypeHoldings,
  /**
   * Regional
   */
  ElementTypeRegional,
  /**
   * Asset class
   */
  ElementTypeAssetClass,
  /**
   * Industry
   */
  ElementTypeIndustry,
} lb_element_type_t;

typedef struct lb_alert_context_t lb_alert_context_t;

/**
 * Asset context
 */
typedef struct CAssetContext CAssetContext;

typedef struct lb_calendar_context_t lb_calendar_context_t;

/**
 * Configuration options for Longbridge SDK
 */
typedef struct lb_config_t lb_config_t;

/**
 * Content context
 */
typedef struct lb_content_context_t lb_content_context_t;

typedef struct lb_dca_context_t lb_dca_context_t;

typedef struct lb_decimal_t lb_decimal_t;

typedef struct lb_error_t lb_error_t;

typedef struct lb_fundamental_context_t lb_fundamental_context_t;

/**
 * A HTTP client for Longbridge OpenAPI
 */
typedef struct lb_http_client_t lb_http_client_t;

typedef struct lb_http_result_t lb_http_result_t;

/**
 * Market data context
 */
typedef struct lb_market_context_t lb_market_context_t;

/**
 * OAuth 2.0 client — owns the Rust `OAuth` instance (opaque handle)
 *
 * Callers must never dereference or inspect the struct layout.
 * Always free with `lb_oauth_free`.
 */
typedef struct lb_oauth_t lb_oauth_t;

typedef struct lb_portfolio_context_t lb_portfolio_context_t;

/**
 * Quote context
 */
typedef struct lb_quote_context_t lb_quote_context_t;

typedef struct lb_screener_context_t lb_screener_context_t;

typedef struct lb_sharelist_context_t lb_sharelist_context_t;

/**
 * Trade context
 */
typedef struct lb_trade_context_t lb_trade_context_t;

typedef struct lb_async_result_t {
  const void *ctx;
  const struct lb_error_t *error;
  void *data;
  uintptr_t length;
  void *userdata;
} lb_async_result_t;

typedef void (*lb_async_callback_t)(const struct lb_async_result_t*);

/**
 * A single alert indicator configuration for a symbol.
 */
typedef struct lb_alert_item_t {
  /**
   * Unique alert identifier.
   */
  const char *id;
  /**
   * Identifier of the indicator that triggers this alert.
   */
  const char *indicator_id;
  /**
   * Whether this alert is currently enabled.
   */
  bool enabled;
  /**
   * Alert notification frequency code.
   */
  int32_t frequency;
  /**
   * Scope of the alert (e.g. per-symbol or global).
   */
  int32_t scope;
  /**
   * Human-readable description text for the alert.
   */
  const char *text;
  /**
   * Pointer to an array of state codes associated with this alert.
   */
  const int32_t *state;
  /**
   * Number of elements in the `state` array.
   */
  uintptr_t num_state;
  /**
   * JSON-serialized map of additional indicator parameter values.
   */
  const char *value_map;
} lb_alert_item_t;

/**
 * HTTP Header
 */
typedef struct lb_http_header_t {
  const char *name;
  const char *value;
} lb_http_header_t;

typedef void (*lb_free_userdata_func_t)(void*);

/**
 * Quote message
 */
typedef struct lb_push_quote_t {
  /**
   * Security code
   */
  const char *symbol;
  /**
   * Latest price
   */
  const struct lb_decimal_t *last_done;
  /**
   * Open
   */
  const struct lb_decimal_t *open;
  /**
   * High
   */
  const struct lb_decimal_t *high;
  /**
   * Low
   */
  const struct lb_decimal_t *low;
  /**
   * Time of latest price
   */
  int64_t timestamp;
  /**
   * Volume
   */
  int64_t volume;
  /**
   * Turnover
   */
  const struct lb_decimal_t *turnover;
  /**
   * Security trading status
   */
  enum lb_trade_status_t trade_status;
  /**
   * Trade session
   */
  enum lb_trade_session_t trade_session;
  /**
   * Increase volume between pushes
   */
  int64_t current_volume;
  /**
   * Increase turnover between pushes
   */
  const struct lb_decimal_t *current_turnover;
} lb_push_quote_t;

typedef void (*lb_quote_callback_t)(const struct lb_quote_context_t*,
                                    const struct lb_push_quote_t*,
                                    void*);

/**
 * Depth
 */
typedef struct lb_depth_t {
  /**
   * Position
   */
  int32_t position;
  /**
   * Price
   */
  const struct lb_decimal_t *price;
  /**
   * Volume
   */
  int64_t volume;
  /**
   * Number of orders
   */
  int64_t order_num;
} lb_depth_t;

/**
 * Quote message
 */
typedef struct lb_push_depth_t {
  /**
   * Security code
   */
  const char *symbol;
  /**
   * Ask depth
   */
  const struct lb_depth_t *asks;
  /**
   * Number of asks
   */
  uintptr_t num_asks;
  /**
   * Bid depth
   */
  const struct lb_depth_t *bids;
  /**
   * Number of bids
   */
  uintptr_t num_bids;
} lb_push_depth_t;

typedef void (*lb_depth_callback_t)(const struct lb_quote_context_t*,
                                    const struct lb_push_depth_t*,
                                    void*);

/**
 * Brokers
 */
typedef struct lb_brokers_t {
  /**
   * Position
   */
  int32_t position;
  /**
   * Broker IDs
   */
  const int32_t *broker_ids;
  /**
   * Number of broker IDs
   */
  uintptr_t num_broker_ids;
} lb_brokers_t;

/**
 * Brokers message
 */
typedef struct lb_push_brokers_t {
  /**
   * Security code
   */
  const char *symbol;
  /**
   * Ask depth
   */
  const struct lb_brokers_t *ask_brokers;
  /**
   * Number of ask brokers
   */
  uintptr_t num_ask_brokers;
  /**
   * Bid depth
   */
  const struct lb_brokers_t *bid_brokers;
  /**
   * Number of bid brokers
   */
  uintptr_t num_bid_brokers;
} lb_push_brokers_t;

typedef void (*lb_brokers_callback_t)(const struct lb_quote_context_t*,
                                      const struct lb_push_brokers_t*,
                                      void*);

/**
 * Trade
 */
typedef struct lb_trade_t {
  /**
   * Price
   */
  const struct lb_decimal_t *price;
  /**
   * Volume
   */
  int64_t volume;
  /**
   * Time of trading
   */
  int64_t timestamp;
  /**
   * Trade type
   *
   * HK
   *
   * - `*` - Overseas trade
   * - `D` - Odd-lot trade
   * - `M` - Non-direct off-exchange trade
   * - `P` - Late trade (Off-exchange previous day)
   * - `U` - Auction trade
   * - `X` - Direct off-exchange trade
   * - `Y` - Automatch internalized
   * - `<empty string>` -  Automatch normal
   *
   * US
   *
   * - `<empty string>` - Regular sale
   * - `A` - Acquisition
   * - `B` - Bunched trade
   * - `D` - Distribution
   * - `F` - Intermarket sweep
   * - `G` - Bunched sold trades
   * - `H` - Price variation trade
   * - `I` - Odd lot trade
   * - `K` - Rule 155 trde(NYSE MKT)
   * - `M` - Market center close price
   * - `P` - Prior reference price
   * - `Q` - Market center open price
   * - `S` - Split trade
   * - `V` - Contingent trade
   * - `W` - Average price trade
   * - `X` - Cross trade
   * - `1` - Stopped stock(Regular trade)
   */
  const char *trade_type;
  /**
   * Trade direction
   */
  enum lb_trade_direction_t direction;
  /**
   * Trade session
   */
  enum lb_trade_session_t trade_session;
} lb_trade_t;

/**
 * Trades message
 */
typedef struct lb_push_trades_t {
  /**
   * Security code
   */
  const char *symbol;
  /**
   * Trades data
   */
  const struct lb_trade_t *trades;
  /**
   * Number of trades
   */
  uintptr_t num_trades;
} lb_push_trades_t;

typedef void (*lb_trades_callback_t)(const struct lb_quote_context_t*,
                                     const struct lb_push_trades_t*,
                                     void*);

/**
 * Candlestick
 */
typedef struct lb_candlestick_t {
  /**
   * Close price
   */
  const struct lb_decimal_t *close;
  /**
   * Open price
   */
  const struct lb_decimal_t *open;
  /**
   * Low price
   */
  const struct lb_decimal_t *low;
  /**
   * High price
   */
  const struct lb_decimal_t *high;
  /**
   * Volume
   */
  int64_t volume;
  /**
   * Turnover
   */
  const struct lb_decimal_t *turnover;
  /**
   * Timestamp
   */
  int64_t timestamp;
  /**
   * Trade session
   */
  enum lb_trade_session_t trade_session;
} lb_candlestick_t;

/**
 * Candlestick updated message
 */
typedef struct lb_push_candlestick_t {
  /**
   * Security code
   */
  const char *symbol;
  /**
   * Period type
   */
  enum lb_period_t period;
  /**
   * Candlestick
   */
  struct lb_candlestick_t candlestick;
  /**
   * Is confirmed
   */
  bool is_confirmed;
} lb_push_candlestick_t;

typedef void (*lb_candlestick_callback_t)(const struct lb_quote_context_t*,
                                          const struct lb_push_candlestick_t*,
                                          void*);

typedef struct lb_date_t {
  int32_t year;
  uint8_t month;
  uint8_t day;
} lb_date_t;

typedef struct lb_time_t {
  uint8_t hour;
  uint8_t minute;
  uint8_t second;
} lb_time_t;

typedef struct lb_datetime_t {
  struct lb_date_t date;
  struct lb_time_t time;
} lb_datetime_t;

/**
 * An request to create a watchlist group
 */
typedef struct lb_create_watchlist_group_t {
  /**
   * Group name
   */
  const char *name;
  /**
   * Securities
   */
  const char *const *securities;
  /**
   * Number of securities
   */
  uintptr_t num_securities;
} lb_create_watchlist_group_t;

/**
 * An request to update a watchlist group
 */
typedef struct lb_update_watchlist_group_t {
  /**
   * Flags
   */
  uint32_t flags;
  /**
   * Group id
   */
  int64_t id;
  /**
   * Group name
   */
  const char *name;
  /**
   * Securities
   */
  const char *const *securities;
  /**
   * Number of securities
   */
  uintptr_t num_securities;
  /**
   * Securities update mode
   */
  enum lb_securities_update_mode_t mode;
} lb_update_watchlist_group_t;

/**
 * Order changed message
 */
typedef struct lb_push_order_changed_t {
  /**
   * Order side
   */
  enum lb_order_side_t side;
  /**
   * Stock name
   */
  const char *stock_name;
  /**
   * Submitted quantity
   */
  const struct lb_decimal_t *submitted_quantity;
  /**
   * Order symbol
   */
  const char *symbol;
  /**
   * Order type
   */
  enum lb_order_type_t order_type;
  /**
   * Submitted price
   */
  const struct lb_decimal_t *submitted_price;
  /**
   * Executed quantity
   */
  const struct lb_decimal_t *executed_quantity;
  /**
   * Executed price (maybe null)
   */
  const struct lb_decimal_t *executed_price;
  /**
   * Order ID
   */
  const char *order_id;
  /**
   * Currency
   */
  const char *currency;
  /**
   * Order status
   */
  enum lb_order_status_t status;
  /**
   * Submitted time
   */
  int64_t submitted_at;
  /**
   * Last updated time
   */
  int64_t updated_at;
  /**
   * Order trigger price (maybe null)
   */
  const struct lb_decimal_t *trigger_price;
  /**
   * Rejected message or remark
   */
  const char *msg;
  /**
   * Order tag
   */
  enum lb_order_tag_t tag;
  /**
   * Conditional order trigger status (maybe null)
   */
  const enum lb_trigger_status_t *trigger_status;
  /**
   * Conditional order trigger time (maybe null)
   */
  const int64_t *trigger_at;
  /**
   * Trailing amount (maybe null)
   */
  const struct lb_decimal_t *trailing_amount;
  /**
   * Trailing percent (maybe null)
   */
  const struct lb_decimal_t *trailing_percent;
  /**
   * Limit offset amount (maybe null)
   */
  const struct lb_decimal_t *limit_offset;
  /**
   * Account no
   */
  const char *account_no;
  /**
   * Last share (maybe null)
   */
  const struct lb_decimal_t *last_share;
  /**
   * Last price (maybe null)
   */
  const struct lb_decimal_t *last_price;
  /**
   * Remark message
   */
  const char *remark;
} lb_push_order_changed_t;

typedef void (*lb_order_changed_callback_t)(const struct lb_trade_context_t*,
                                            const struct lb_push_order_changed_t*,
                                            void*);

/**
 * Options for get history executions request
 */
typedef struct lb_get_history_executions_options_t {
  /**
   * Start time (can be null)
   */
  const int64_t *start_at;
  /**
   * End time (can be null)
   */
  const int64_t *end_at;
  /**
   * Security code (can be null)
   */
  const char *symbol;
} lb_get_history_executions_options_t;

/**
 * Options for get today executions request
 */
typedef struct lb_get_today_executions_options_t {
  /**
   * Security code (can be null)
   */
  const char *symbol;
  /**
   * Order id (can be null)
   */
  const char *order_id;
} lb_get_today_executions_options_t;

/**
 * Options for get history orders request
 */
typedef struct lb_get_history_orders_options_t {
  /**
   * Security symbol (can be null)
   */
  const char *symbol;
  /**
   * Order status (can be null)
   */
  const enum lb_order_status_t *status;
  /**
   * Number of order status
   */
  uintptr_t num_status;
  /**
   * Order side (can be null)
   */
  const enum lb_order_side_t *side;
  /**
   * Market (can be null)
   */
  const enum lb_market_t *market;
  /**
   * Start time (can be null)
   */
  const int64_t *start_at;
  /**
   * End time (can be null)
   */
  const int64_t *end_at;
} lb_get_history_orders_options_t;

/**
 * Options for get today orders request
 */
typedef struct lb_get_today_orders_options_t {
  /**
   * Security symbol (can be null)
   */
  const char *symbol;
  /**
   * Order status (can be null)
   */
  const enum lb_order_status_t *status;
  /**
   * Number of order status
   */
  uintptr_t num_status;
  /**
   * Order side (can be null)
   */
  const enum lb_order_side_t *side;
  /**
   * Market (can be null)
   */
  const enum lb_market_t *market;
  /**
   * Order id (can be null)
   */
  const char *order_id;
  /**
   * Filter by attached order (can be null)
   */
  const bool *is_attached;
} lb_get_today_orders_options_t;

/**
 * Options for replace attached order params
 */
typedef struct CReplaceAttachedParams {
  /**
   * Attached order type
   */
  enum CAttachedOrderType attached_order_type;
  /**
   * Take-profit trigger price (can be null)
   */
  const struct lb_decimal_t *profit_taker_price;
  /**
   * Stop-loss trigger price (can be null)
   */
  const struct lb_decimal_t *stop_loss_price;
  /**
   * Time in force type (can be null)
   */
  const enum lb_time_in_force_type_t *time_in_force;
  /**
   * Expiry time unix timestamp (can be null)
   */
  const int64_t *expire_time;
  /**
   * Order type to submit after trigger (can be null)
   */
  const enum lb_order_type_t *activate_order_type;
  /**
   * Take-profit limit price (can be null)
   */
  const struct lb_decimal_t *profit_taker_submit_price;
  /**
   * Stop-loss limit price (can be null)
   */
  const struct lb_decimal_t *stop_loss_submit_price;
  /**
   * RTH setting for activated order (can be null)
   */
  const enum lb_outside_rth_t *activate_rth;
  /**
   * Take-profit order ID (can be null)
   */
  const int64_t *profit_taker_id;
  /**
   * Stop-loss order ID (can be null)
   */
  const int64_t *stop_loss_id;
  /**
   * Cancel all attached orders flag (can be null)
   */
  const bool *cancel_all_attached;
  /**
   * Main order ID (can be null)
   */
  const int64_t *main_id;
  /**
   * Quantity (can be null)
   */
  const struct lb_decimal_t *quantity;
  /**
   * Market price (can be null)
   */
  const struct lb_decimal_t *market_price;
} CReplaceAttachedParams;

/**
 * Options for replace order request
 */
typedef struct lb_replace_order_options_t {
  /**
   * Order ID
   */
  const char *order_id;
  /**
   * Quantity
   */
  const struct lb_decimal_t *quantity;
  /**
   * Price (can be null)
   */
  const struct lb_decimal_t *price;
  /**
   * Trigger price (can be null)
   */
  const struct lb_decimal_t *trigger_price;
  /**
   * Limit offset (can be null)
   */
  const struct lb_decimal_t *limit_offset;
  /**
   * Trailing amount (can be null)
   */
  const struct lb_decimal_t *trailing_amount;
  /**
   * Trailing percent (can be null)
   */
  const struct lb_decimal_t *trailing_percent;
  /**
   * Limit depth level (can be null)
   */
  const int32_t *limit_depth_level;
  /**
   * Trigger count (can be null)
   */
  const int32_t *trigger_count;
  /**
   * Monitor price (can be null)
   */
  const struct lb_decimal_t *monitor_price;
  /**
   * Remark (can be null)
   */
  const char *remark;
  /**
   * Attached order parameters (can be null)
   */
  const struct CReplaceAttachedParams *attached_params;
} lb_replace_order_options_t;

/**
 * Options for submit attached order params
 */
typedef struct CSubmitAttachedParams {
  /**
   * Attached order type
   */
  enum CAttachedOrderType attached_order_type;
  /**
   * Take-profit trigger price (can be null)
   */
  const struct lb_decimal_t *profit_taker_price;
  /**
   * Stop-loss trigger price (can be null)
   */
  const struct lb_decimal_t *stop_loss_price;
  /**
   * Time in force type (can be null)
   */
  const enum lb_time_in_force_type_t *time_in_force;
  /**
   * Expiry time unix timestamp (can be null)
   */
  const int64_t *expire_time;
  /**
   * Order type to submit after trigger (can be null)
   */
  const enum lb_order_type_t *activate_order_type;
  /**
   * Take-profit limit price (can be null)
   */
  const struct lb_decimal_t *profit_taker_submit_price;
  /**
   * Stop-loss limit price (can be null)
   */
  const struct lb_decimal_t *stop_loss_submit_price;
  /**
   * RTH setting for activated order (can be null)
   */
  const enum lb_outside_rth_t *activate_rth;
} CSubmitAttachedParams;

/**
 * Options for submit order request
 */
typedef struct lb_submit_order_options_t {
  /**
   * Security symbol
   */
  const char *symbol;
  /**
   * Order type
   */
  enum lb_order_type_t order_type;
  /**
   * Order side
   */
  enum lb_order_side_t side;
  /**
   * Submitted price
   */
  const struct lb_decimal_t *submitted_quantity;
  /**
   * Time in force type
   */
  enum lb_time_in_force_type_t time_in_force;
  /**
   * Submitted price (can be null)
   */
  const struct lb_decimal_t *submitted_price;
  /**
   * Trigger price (`LIT` / `MIT` Required) (can be null)
   */
  const struct lb_decimal_t *trigger_price;
  /**
   * Limit offset amount (`TSLPAMT` / `TSLPPCT` Required) (can be null)
   */
  const struct lb_decimal_t *limit_offset;
  /**
   * Trailing amount (`TSLPAMT` / `TSMAMT` Required) (can be null)
   */
  const struct lb_decimal_t *trailing_amount;
  /**
   * Trailing percent (`TSLPPCT` / `TSMAPCT` Required) (can be null)
   */
  const struct lb_decimal_t *trailing_percent;
  /**
   * Long term order expire date (Required when `time_in_force` is
   * `GoodTilDate`) (can be null)
   */
  const struct lb_date_t *expire_date;
  /**
   * Enable or disable outside regular trading hours (can be null)
   */
  const enum lb_outside_rth_t *outside_rth;
  /**
   * Limit depth level (can be null)
   */
  const int32_t *limit_depth_level;
  /**
   * Trigger count (can be null)
   */
  const int32_t *trigger_count;
  /**
   * Monitor price (can be null)
   */
  const struct lb_decimal_t *monitor_price;
  /**
   * Remark (Maximum 64 characters) (can be null)
   */
  const char *remark;
  /**
   * Idempotent request ID for preventing duplicate orders (can be null).
   * If not specified, idempotency control is skipped.
   * The server caches this ID for 10 minutes.
   */
  const char *client_request_id;
  /**
   * Attached order parameters (can be null)
   */
  const struct CSubmitAttachedParams *attached_params;
} lb_submit_order_options_t;

/**
 * Options for get cash flow request
 */
typedef struct lb_get_cash_flow_options_t {
  /**
   * Start time
   */
  int64_t start_at;
  /**
   * End time
   */
  int64_t end_at;
  /**
   * Business type (can be null)
   */
  const enum lb_balance_type_t *business_type;
  /**
   * Security symbol (can be null)
   */
  const char *symbol;
  /**
   * Page number (can be null)
   */
  const uintptr_t *page;
  /**
   * Page size (can be null)
   */
  const uintptr_t *size;
} lb_get_cash_flow_options_t;

/**
 * Options for get fund positions request
 */
typedef struct lb_get_fund_positions_options_t {
  /**
   * Fund symbols (can be null)
   */
  const char *const *symbols;
  /**
   * Number of fund symbols
   */
  uintptr_t num_symbols;
} lb_get_fund_positions_options_t;

/**
 * Options for get stock positions request
 */
typedef struct lb_get_stock_positions_options_t {
  /**
   * Fund symbols (can be null)
   */
  const char *const *symbols;
  /**
   * Number of stock symbols
   */
  uintptr_t num_symbols;
} lb_get_stock_positions_options_t;

/**
 * Options for estimate maximum purchase quantity
 */
typedef struct lb_estimate_max_purchase_quantity_options_t {
  /**
   * Security symbol to estimate for
   */
  const char *symbol;
  /**
   * Order type
   */
  enum lb_order_type_t order_type;
  /**
   * Order price; may be null for market orders
   */
  const struct lb_decimal_t *price;
  /**
   * Order side (buy or sell)
   */
  enum lb_order_side_t side;
  /**
   * Settlement currency to use for the estimate (can be null)
   */
  const char *currency;
  /**
   * Existing order ID to exclude from available funds calculation (can be
   * null)
   */
  const char *order_id;
  /**
   * Whether to allow fractional share quantities in the result
   */
  bool fractional_shares;
} lb_estimate_max_purchase_quantity_options_t;

/**
 * Active subscription for a security
 */
typedef struct lb_subscription_t {
  /**
   * Security code
   */
  const char *symbol;
  /**
   * Bitmask of subscribed sub-types
   */
  uint8_t sub_types;
  /**
   * Pointer to array of subscribed candlestick periods
   */
  const enum lb_period_t *candlesticks;
  /**
   * Number of elements in the array.
   */
  uintptr_t num_candlesticks;
} lb_subscription_t;

/**
 * Security
 */
typedef struct lb_security_t {
  /**
   * Security code
   */
  const char *symbol;
  /**
   * Security name (zh-CN)
   */
  const char *name_cn;
  /**
   * Security name (en)
   */
  const char *name_en;
  /**
   * Security name (zh-HK)
   */
  const char *name_hk;
} lb_security_t;

/**
 * The basic information of securities
 */
typedef struct lb_security_static_info_t {
  /**
   * Security code
   */
  const char *symbol;
  /**
   * Security name (zh-CN)
   */
  const char *name_cn;
  /**
   * Security name (en)
   */
  const char *name_en;
  /**
   * Security name (zh-HK)
   */
  const char *name_hk;
  /**
   * Exchange which the security belongs to
   */
  const char *exchange;
  /**
   * Trading currency
   */
  const char *currency;
  /**
   * Lot size
   */
  int32_t lot_size;
  /**
   * Total shares
   */
  int64_t total_shares;
  /**
   * Circulating shares
   */
  int64_t circulating_shares;
  /**
   * HK shares (only HK stocks)
   */
  int64_t hk_shares;
  /**
   * Earnings per share
   */
  const struct lb_decimal_t *eps;
  /**
   * Earnings per share (TTM)
   */
  const struct lb_decimal_t *eps_ttm;
  /**
   * Net assets per share
   */
  const struct lb_decimal_t *bps;
  /**
   * Dividend (per share), **not** the dividend yield (ratio).
   */
  const struct lb_decimal_t *dividend_yield;
  /**
   * Types of supported derivatives
   */
  uint8_t stock_derivatives;
  /**
   * Board
   */
  enum lb_security_board_t board;
} lb_security_static_info_t;

/**
 * Quote of US pre/post market
 */
typedef struct lb_prepost_quote_t {
  /**
   * Latest price
   */
  const struct lb_decimal_t *last_done;
  /**
   * Time of latest price
   */
  int64_t timestamp;
  /**
   * Volume
   */
  int64_t volume;
  /**
   * Turnover
   */
  const struct lb_decimal_t *turnover;
  /**
   * High
   */
  const struct lb_decimal_t *high;
  /**
   * Low
   */
  const struct lb_decimal_t *low;
  /**
   * Close of the last trade session
   */
  const struct lb_decimal_t *prev_close;
} lb_prepost_quote_t;

/**
 * Quote of securitity
 */
typedef struct lb_security_quote_t {
  /**
   * Security code
   */
  const char *symbol;
  /**
   * Latest price
   */
  const struct lb_decimal_t *last_done;
  /**
   * Yesterday's close
   */
  const struct lb_decimal_t *prev_close;
  /**
   * Open
   */
  const struct lb_decimal_t *open;
  /**
   * High
   */
  const struct lb_decimal_t *high;
  /**
   * Low
   */
  const struct lb_decimal_t *low;
  /**
   * Time of latest price
   */
  int64_t timestamp;
  /**
   * Volume
   */
  int64_t volume;
  /**
   * Turnover
   */
  const struct lb_decimal_t *turnover;
  /**
   * Security trading status
   */
  enum lb_trade_status_t trade_status;
  /**
   * Quote of US pre market
   */
  const struct lb_prepost_quote_t *pre_market_quote;
  /**
   * Quote of US post market
   */
  const struct lb_prepost_quote_t *post_market_quote;
  /**
   * Quote of US overnight market
   */
  const struct lb_prepost_quote_t *overnight_quote;
} lb_security_quote_t;

/**
 * Quote of option
 */
typedef struct lb_option_quote_t {
  /**
   * Security code
   */
  const char *symbol;
  /**
   * Latest price
   */
  const struct lb_decimal_t *last_done;
  /**
   * Yesterday's close
   */
  const struct lb_decimal_t *prev_close;
  /**
   * Open
   */
  const struct lb_decimal_t *open;
  /**
   * High
   */
  const struct lb_decimal_t *high;
  /**
   * Low
   */
  const struct lb_decimal_t *low;
  /**
   * Time of latest price
   */
  int64_t timestamp;
  /**
   * Volume
   */
  int64_t volume;
  /**
   * Turnover
   */
  const struct lb_decimal_t *turnover;
  /**
   * Security trading status
   */
  enum lb_trade_status_t trade_status;
  /**
   * Implied volatility
   */
  const struct lb_decimal_t *implied_volatility;
  /**
   * Number of open positions
   */
  int64_t open_interest;
  /**
   * Exprity date
   */
  struct lb_date_t expiry_date;
  /**
   * Strike price
   */
  const struct lb_decimal_t *strike_price;
  /**
   * Contract multiplier
   */
  const struct lb_decimal_t *contract_multiplier;
  /**
   * Option type
   */
  enum lb_option_type_t contract_type;
  /**
   * Contract size
   */
  const struct lb_decimal_t *contract_size;
  /**
   * Option direction
   */
  enum lb_option_direction_t direction;
  /**
   * Underlying security historical volatility of the option
   */
  const struct lb_decimal_t *historical_volatility;
  /**
   * Underlying security symbol of the option
   */
  const char *underlying_symbol;
} lb_option_quote_t;

/**
 * Quote of warrant
 */
typedef struct lb_warrant_quote_t {
  /**
   * Security code
   */
  const char *symbol;
  /**
   * Latest price
   */
  const struct lb_decimal_t *last_done;
  /**
   * Yesterday's close
   */
  const struct lb_decimal_t *prev_close;
  /**
   * Open
   */
  const struct lb_decimal_t *open;
  /**
   * High
   */
  const struct lb_decimal_t *high;
  /**
   * Low
   */
  const struct lb_decimal_t *low;
  /**
   * Time of latest price
   */
  int64_t timestamp;
  /**
   * Volume
   */
  int64_t volume;
  /**
   * Turnover
   */
  const struct lb_decimal_t *turnover;
  /**
   * Security trading status
   */
  enum lb_trade_status_t trade_status;
  /**
   * Implied volatility
   */
  const struct lb_decimal_t *implied_volatility;
  /**
   * Exprity date
   */
  struct lb_date_t expiry_date;
  /**
   * Last tradalbe date
   */
  struct lb_date_t last_trade_date;
  /**
   * Outstanding ratio
   */
  const struct lb_decimal_t *outstanding_ratio;
  /**
   * Outstanding quantity
   */
  int64_t outstanding_quantity;
  /**
   * Conversion ratio
   */
  const struct lb_decimal_t *conversion_ratio;
  /**
   * Warrant type
   */
  enum lb_warrant_type_t category;
  /**
   * Strike price
   */
  const struct lb_decimal_t *strike_price;
  /**
   * Upper bound price
   */
  const struct lb_decimal_t *upper_strike_price;
  /**
   * Lower bound price
   */
  const struct lb_decimal_t *lower_strike_price;
  /**
   * Call price
   */
  const struct lb_decimal_t *call_price;
  /**
   * Underlying security symbol of the warrant
   */
  const char *underlying_symbol;
} lb_warrant_quote_t;

/**
 * Quote message
 */
typedef struct lb_security_depth_t {
  /**
   * Ask depth
   */
  const struct lb_depth_t *asks;
  /**
   * Number of asks
   */
  uintptr_t num_asks;
  /**
   * Bid depth
   */
  const struct lb_depth_t *bids;
  /**
   * Number of bids
   */
  uintptr_t num_bids;
} lb_security_depth_t;

/**
 * Security brokers
 */
typedef struct lb_security_brokers_t {
  /**
   * Ask brokers
   */
  const struct lb_brokers_t *ask_brokers;
  /**
   * Number of ask brokers
   */
  uintptr_t num_ask_brokers;
  /**
   * Bid brokers
   */
  const struct lb_brokers_t *bid_brokers;
  /**
   * Number of bid brokers
   */
  uintptr_t num_bid_brokers;
} lb_security_brokers_t;

/**
 * Participant info
 */
typedef struct lb_participant_info_t {
  /**
   * Broker IDs
   */
  const int32_t *broker_ids;
  /**
   * Number of broker IDs
   */
  uintptr_t num_broker_ids;
  /**
   * Participant name (zh-CN)
   */
  const char *name_cn;
  /**
   * Participant name (en)
   */
  const char *name_en;
  /**
   * Participant name (zh-HK)
   */
  const char *name_hk;
} lb_participant_info_t;

/**
 * Intraday line
 */
typedef struct lb_intraday_line_t {
  /**
   * Close price of the minute
   */
  const struct lb_decimal_t *price;
  /**
   * Start time of the minute
   */
  int64_t timestamp;
  /**
   * Volume
   */
  int64_t volume;
  /**
   * Turnover
   */
  const struct lb_decimal_t *turnover;
  /**
   * Average price
   */
  const struct lb_decimal_t *avg_price;
} lb_intraday_line_t;

/**
 * Strike price info
 */
typedef struct lb_strike_price_info_t {
  /**
   * Strike price
   */
  const struct lb_decimal_t *price;
  /**
   * Security code of call option
   */
  const char *call_symbol;
  /**
   * Security code of put option
   */
  const char *put_symbol;
  /**
   * Is standard
   */
  bool standard;
} lb_strike_price_info_t;

/**
 * Issuer info
 */
typedef struct lb_issuer_info_t {
  /**
   * Issuer ID
   */
  int32_t issuer_id;
  /**
   * Issuer name (zh-CN)
   */
  const char *name_cn;
  /**
   * Issuer name (en)
   */
  const char *name_en;
  /**
   * Issuer name (zh-HK)
   */
  const char *name_hk;
} lb_issuer_info_t;

/**
 * The information of trading session
 */
typedef struct lb_trading_session_info_t {
  /**
   * Being trading time
   */
  struct lb_time_t begin_time;
  /**
   * End trading time
   */
  struct lb_time_t end_time;
  /**
   * Trading session
   */
  enum lb_trade_session_t trade_session;
} lb_trading_session_info_t;

/**
 * Market trading session
 */
typedef struct lb_market_trading_session_t {
  /**
   * Market
   */
  enum lb_market_t market;
  /**
   * Trading sessions
   */
  const struct lb_trading_session_info_t *trade_sessions;
  /**
   * Number trading sessions
   */
  uintptr_t num_trade_sessions;
} lb_market_trading_session_t;

/**
 * Market trading days
 */
typedef struct lb_market_trading_days_t {
  /**
   * Trading days
   */
  const struct lb_date_t *trading_days;
  /**
   * Number of trading days
   */
  uintptr_t num_trading_days;
  /**
   * Half trading days
   */
  const struct lb_date_t *half_trading_days;
  /**
   * Number of half trading days
   */
  uintptr_t num_half_trading_days;
} lb_market_trading_days_t;

/**
 * Capital flow line data point
 */
typedef struct lb_capital_flow_line_t {
  /**
   * Inflow capital data
   */
  const struct lb_decimal_t *inflow;
  /**
   * Time
   */
  int64_t timestamp;
} lb_capital_flow_line_t;

/**
 * Capital distribution
 */
typedef struct lb_capital_distribution_t {
  /**
   * Large order
   */
  const struct lb_decimal_t *large;
  /**
   * Medium order
   */
  const struct lb_decimal_t *medium;
  /**
   * Small order
   */
  const struct lb_decimal_t *small;
} lb_capital_distribution_t;

/**
 * Capital distribution response
 */
typedef struct lb_capital_distribution_response_t {
  /**
   * Time
   */
  int64_t timestamp;
  /**
   * Inflow capital data
   */
  struct lb_capital_distribution_t capital_in;
  /**
   * Outflow capital data
   */
  struct lb_capital_distribution_t capital_out;
} lb_capital_distribution_response_t;

/**
 * Real-time quote
 */
typedef struct lb_realtime_quote_t {
  /**
   * Security code
   */
  const char *symbol;
  /**
   * Latest price
   */
  const struct lb_decimal_t *last_done;
  /**
   * Open
   */
  const struct lb_decimal_t *open;
  /**
   * High
   */
  const struct lb_decimal_t *high;
  /**
   * Low
   */
  const struct lb_decimal_t *low;
  /**
   * Time of latest price
   */
  int64_t timestamp;
  /**
   * Volume
   */
  int64_t volume;
  /**
   * Turnover
   */
  const struct lb_decimal_t *turnover;
  /**
   * Security trading status
   */
  enum lb_trade_status_t trade_status;
} lb_realtime_quote_t;

/**
 * Execution
 */
typedef struct lb_execution_t {
  /**
   * Order ID
   */
  const char *order_id;
  /**
   * Execution ID
   */
  const char *trade_id;
  /**
   * Security code
   */
  const char *symbol;
  /**
   * Trade done time
   */
  int64_t trade_done_at;
  /**
   * Executed quantity
   */
  const struct lb_decimal_t *quantity;
  /**
   * Executed price
   */
  const struct lb_decimal_t *price;
} lb_execution_t;

/**
 * All executions response
 */
typedef struct lb_all_executions_response_t {
  /**
   * Has more records
   */
  bool has_more;
  /**
   * Executions
   */
  const struct lb_execution_t *trades;
  /**
   * Number of executions
   */
  uintptr_t num_trades;
} lb_all_executions_response_t;

/**
 * Attached order detail
 */
typedef struct CAttachedOrderDetail {
  /**
   * Attached order ID
   */
  const char *order_id;
  /**
   * Attached order type
   */
  enum CAttachedOrderType attached_type_display;
  /**
   * Trigger price (maybe null)
   */
  const struct lb_decimal_t *trigger_price;
  /**
   * Quantity
   */
  const struct lb_decimal_t *quantity;
  /**
   * Executed quantity
   */
  const struct lb_decimal_t *executed_qty;
  /**
   * Order status
   */
  enum lb_order_status_t status;
  /**
   * Last updated time (unix timestamp)
   */
  int64_t updated_at;
  /**
   * Whether withdrawn
   */
  bool withdrawn;
  /**
   * GTD date (maybe null)
   */
  const struct lb_date_t *gtd;
  /**
   * Time in force type
   */
  enum lb_time_in_force_type_t time_in_force;
  /**
   * Counter order ID
   */
  const char *counter_id;
  /**
   * Trigger status (maybe null)
   */
  const enum lb_trigger_status_t *trigger_status;
  /**
   * Executed amount
   */
  const struct lb_decimal_t *executed_amount;
  /**
   * Tag
   */
  enum lb_order_tag_t tag;
  /**
   * Submitted time (unix timestamp)
   */
  int64_t submitted_at;
  /**
   * Executed price
   */
  const struct lb_decimal_t *executed_price;
  /**
   * Force RTH only (maybe null)
   */
  const enum lb_outside_rth_t *force_only_rth;
  /**
   * Whether reviewed
   */
  bool reviewed;
  /**
   * Order type to submit after trigger
   */
  enum lb_order_type_t activate_order_type;
  /**
   * RTH setting for activated order (maybe null)
   */
  const enum lb_outside_rth_t *activate_rth;
  /**
   * Submit price (maybe null)
   */
  const struct lb_decimal_t *submit_price;
} CAttachedOrderDetail;

/**
 * Order
 */
typedef struct lb_order_t {
  /**
   * Order ID
   */
  const char *order_id;
  /**
   * Order status
   */
  enum lb_order_status_t status;
  /**
   * Stock name
   */
  const char *stock_name;
  /**
   * Submitted quantity
   */
  const struct lb_decimal_t *quantity;
  /**
   * Executed quantity
   */
  const struct lb_decimal_t *executed_quantity;
  /**
   * Submitted price (maybe null)
   */
  const struct lb_decimal_t *price;
  /**
   * Executed price (maybe null)
   */
  const struct lb_decimal_t *executed_price;
  /**
   * Submitted time
   */
  int64_t submitted_at;
  /**
   * Order side
   */
  enum lb_order_side_t side;
  /**
   * Security code
   */
  const char *symbol;
  /**
   * Order type
   */
  enum lb_order_type_t order_type;
  /**
   * Last done (maybe null)
   */
  const struct lb_decimal_t *last_done;
  /**
   * `LIT` / `MIT` Order Trigger Price (maybe null)
   */
  const struct lb_decimal_t *trigger_price;
  /**
   * Rejected Message or remark
   */
  const char *msg;
  /**
   * Order tag
   */
  enum lb_order_tag_t tag;
  /**
   * Time in force type
   */
  enum lb_time_in_force_type_t time_in_force;
  /**
   * Long term order expire date (maybe null)
   */
  const struct lb_date_t *expire_date;
  /**
   * Last updated time (maybe null)
   */
  const int64_t *updated_at;
  /**
   * Conditional order trigger time (maybe null)
   */
  const int64_t *trigger_at;
  /**
   * `TSMAMT` / `TSLPAMT` order trailing amount (maybe null)
   */
  const struct lb_decimal_t *trailing_amount;
  /**
   * `TSMPCT` / `TSLPPCT` order trailing percent (maybe null)
   */
  const struct lb_decimal_t *trailing_percent;
  /**
   * `TSLPAMT` / `TSLPPCT` order limit offset amount (maybe null)
   */
  const struct lb_decimal_t *limit_offset;
  /**
   * Conditional order trigger status (maybe null)
   */
  const enum lb_trigger_status_t *trigger_status;
  /**
   * Currency
   */
  const char *currency;
  /**
   * Enable or disable outside regular trading hours (maybe null)
   */
  const enum lb_outside_rth_t *outside_rth;
  /**
   * Limit depth level (maybe null)
   */
  const int32_t *limit_depth_level;
  /**
   * Trigger count (maybe null)
   */
  const int32_t *trigger_count;
  /**
   * Monitor price (maybe null)
   */
  const struct lb_decimal_t *monitor_price;
  /**
   * Remark
   */
  const char *remark;
  /**
   * Attached orders
   */
  const struct CAttachedOrderDetail *attached_orders;
  /**
   * Number of attached orders
   */
  uintptr_t num_attached_orders;
} lb_order_t;

/**
 * Frozen transaction fee entry for a given currency
 */
typedef struct lb_frozen_transaction_fee_t {
  /**
   * Currency of the frozen fee
   */
  const char *currency;
  /**
   * Amount of transaction fee frozen for pending orders
   */
  const struct lb_decimal_t *frozen_transaction_fee;
} lb_frozen_transaction_fee_t;

/**
 * Account balance
 */
typedef struct lb_cash_info_t {
  /**
   * Withdraw cash
   */
  const struct lb_decimal_t *withdraw_cash;
  /**
   * Available cash
   */
  const struct lb_decimal_t *available_cash;
  /**
   * Frozen cash
   */
  const struct lb_decimal_t *frozen_cash;
  /**
   * Cash to be settled
   */
  const struct lb_decimal_t *settling_cash;
  /**
   * Currency
   */
  const char *currency;
} lb_cash_info_t;

/**
 * Account balance
 */
typedef struct lb_account_balance_t {
  /**
   * Total cash
   */
  const struct lb_decimal_t *total_cash;
  /**
   * Maximum financing amount
   */
  const struct lb_decimal_t *max_finance_amount;
  /**
   * Remaining financing amount
   */
  const struct lb_decimal_t *remaining_finance_amount;
  /**
   * Risk control level
   */
  int32_t risk_level;
  /**
   * Margin call
   */
  const struct lb_decimal_t *margin_call;
  /**
   * Currency
   */
  const char *currency;
  /**
   * Cash details
   */
  const struct lb_cash_info_t *cash_infos;
  /**
   * Number of cash details
   */
  uintptr_t num_cash_infos;
  /**
   * Net assets
   */
  const struct lb_decimal_t *net_assets;
  /**
   * Initial margin
   */
  const struct lb_decimal_t *init_margin;
  /**
   * Maintenance margin
   */
  const struct lb_decimal_t *maintenance_margin;
  /**
   * Buy power
   */
  const struct lb_decimal_t *buy_power;
  /**
   * Frozen transaction fees
   */
  const struct lb_frozen_transaction_fee_t *frozen_transaction_fees;
  /**
   * Number of frozen transaction fees
   */
  uintptr_t num_frozen_transaction_fees;
} lb_account_balance_t;

/**
 * Cash flow
 */
typedef struct lb_cash_flow_t {
  /**
   * Cash flow name
   */
  const char *transaction_flow_name;
  /**
   * Outflow direction
   */
  enum lb_cash_flow_direction_t direction;
  /**
   * Balance type
   */
  enum lb_balance_type_t business_type;
  /**
   * Cash amount
   */
  const struct lb_decimal_t *balance;
  /**
   * Cash currency
   */
  const char *currency;
  /**
   * Business time
   */
  int64_t business_time;
  /**
   * Associated Stock code information (maybe null)
   */
  const char *symbol;
  /**
   * Cash flow description
   */
  const char *description;
} lb_cash_flow_t;

/**
 * Fund position
 */
typedef struct lb_fund_position_t {
  /**
   * Fund ISIN code
   */
  const char *symbol;
  /**
   * Current equity
   */
  const struct lb_decimal_t *current_net_asset_value;
  /**
   * Current equity time
   */
  int64_t net_asset_value_day;
  /**
   * Fund name
   */
  const char *symbol_name;
  /**
   * Currency
   */
  const char *currency;
  /**
   * Net cost
   */
  const struct lb_decimal_t *cost_net_asset_value;
  /**
   * Holding units
   */
  const struct lb_decimal_t *holding_units;
} lb_fund_position_t;

/**
 * Fund position channel
 */
typedef struct lb_fund_position_channel_t {
  /**
   * Account type
   */
  const char *account_channel;
  /**
   * Fund positions
   */
  const struct lb_fund_position_t *positions;
  /**
   * Number of fund positions
   */
  uintptr_t num_positions;
} lb_fund_position_channel_t;

/**
 * Fund positions response
 */
typedef struct lb_fund_position_response_t {
  /**
   * Channels
   */
  const struct lb_fund_position_channel_t *channels;
  /**
   * Number of channels
   */
  uintptr_t num_channels;
} lb_fund_position_response_t;

/**
 * Stock position
 */
typedef struct lb_stock_position_t {
  /**
   * Stock code
   */
  const char *symbol;
  /**
   * Stock name
   */
  const char *symbol_name;
  /**
   * The number of holdings
   */
  const struct lb_decimal_t *quantity;
  /**
   * Available quantity
   */
  const struct lb_decimal_t *available_quantity;
  /**
   * Currency
   */
  const char *currency;
  /**
   * Cost Price(According to the client's choice of average purchase or
   * diluted cost)
   */
  const struct lb_decimal_t *cost_price;
  /**
   * Market
   */
  enum lb_market_t market;
  /**
   * Initial position before market opening
   */
  const struct lb_decimal_t *init_quantity;
} lb_stock_position_t;

/**
 * Stock position channel
 */
typedef struct lb_stock_position_channel_t {
  /**
   * Account type
   */
  const char *account_channel;
  /**
   * Stock positions
   */
  const struct lb_stock_position_t *positions;
  /**
   * Number of stock positions
   */
  uintptr_t num_positions;
} lb_stock_position_channel_t;

/**
 * Stock positions response
 */
typedef struct lb_stock_position_response_t {
  /**
   * Channels
   */
  const struct lb_stock_position_channel_t *channels;
  /**
   * Number of channels
   */
  uintptr_t num_channels;
} lb_stock_position_response_t;

/**
 * Response for submit order request
 */
typedef struct lb_submit_order_response_t {
  /**
   * Order id
   */
  const char *order_id;
} lb_submit_order_response_t;

/**
 * Watchlist security
 */
typedef struct lb_watchlist_security_t {
  /**
   * Security symbol
   */
  const char *symbol;
  /**
   * Market
   */
  enum lb_market_t market;
  /**
   * Security name
   */
  const char *name;
  /**
   * Watched price (maybe null)
   */
  const struct lb_decimal_t *watched_price;
  /**
   * Watched time
   */
  int64_t watched_at;
} lb_watchlist_security_t;

/**
 * Watchlist group
 */
typedef struct lb_watchlist_group_t {
  /**
   * Group id
   */
  int64_t id;
  /**
   * Group name
   */
  const char *name;
  /**
   * Securities
   */
  const struct lb_watchlist_security_t *securities;
  /**
   * Number of securities
   */
  uintptr_t num_securities;
} lb_watchlist_group_t;

/**
 * Margin ratio
 */
typedef struct lb_margin_ratio_t {
  /**
   * Initial margin ratio
   */
  const struct lb_decimal_t *im_factor;
  /**
   * Maintain the initial margin ratio
   */
  const struct lb_decimal_t *mm_factor;
  /**
   * Forced close-out margin ratio
   */
  const struct lb_decimal_t *fm_factor;
} lb_margin_ratio_t;

/**
 * Historical status record for a single order transition
 */
typedef struct lb_order_history_detail_t {
  /**
   * Order price at the time of this status transition
   */
  const struct lb_decimal_t *price;
  /**
   * Order quantity at the time of this status transition
   */
  const struct lb_decimal_t *quantity;
  /**
   * Order status for this history entry
   */
  enum lb_order_status_t status;
  /**
   * Rejection or remark message associated with this transition
   */
  const char *msg;
  /**
   * Unix timestamp of this status transition
   */
  int64_t time;
} lb_order_history_detail_t;

/**
 * Order charge fee
 */
typedef struct lb_order_charge_fee_t {
  /**
   * Charge code
   */
  const char *code;
  /**
   * Charge name
   */
  const char *name;
  /**
   * Charge amount
   */
  const struct lb_decimal_t *amount;
  /**
   * Charge currency
   */
  const char *currency;
} lb_order_charge_fee_t;

/**
 * Order charge item
 */
typedef struct lb_order_charge_item_t {
  /**
   * Charge category code
   */
  enum lb_charge_category_code_t code;
  /**
   * Charge category name
   */
  const char *name;
  /**
   * Charge details
   */
  const struct lb_order_charge_fee_t *fees;
  /**
   * Number of charge details
   */
  uintptr_t num_fees;
} lb_order_charge_item_t;

/**
 * Order charge detail
 */
typedef struct lb_order_charge_detail_t {
  /**
   * Total charges amount
   */
  const struct lb_decimal_t *total_amount;
  /**
   * Settlement currency
   */
  const char *currency;
  /**
   * Order charge items
   */
  const struct lb_order_charge_item_t *items;
  /**
   * Number of items
   */
  uintptr_t num_items;
} lb_order_charge_detail_t;

/**
 * Order detail
 */
typedef struct lb_order_detail_t {
  /**
   * Order ID
   */
  const char *order_id;
  /**
   * Order status
   */
  enum lb_order_status_t status;
  /**
   * Stock name
   */
  const char *stock_name;
  /**
   * Submitted quantity
   */
  const struct lb_decimal_t *quantity;
  /**
   * Executed quantity
   */
  const struct lb_decimal_t *executed_quantity;
  /**
   * Submitted price (maybe null)
   */
  const struct lb_decimal_t *price;
  /**
   * Executed price (maybe null)
   */
  const struct lb_decimal_t *executed_price;
  /**
   * Submitted time
   */
  int64_t submitted_at;
  /**
   * Order side
   */
  enum lb_order_side_t side;
  /**
   * Security code
   */
  const char *symbol;
  /**
   * Order type
   */
  enum lb_order_type_t order_type;
  /**
   * Last done (maybe null)
   */
  const struct lb_decimal_t *last_done;
  /**
   * `LIT` / `MIT` Order Trigger Price (maybe null)
   */
  const struct lb_decimal_t *trigger_price;
  /**
   * Rejected Message or remark
   */
  const char *msg;
  /**
   * Order tag
   */
  enum lb_order_tag_t tag;
  /**
   * Time in force type
   */
  enum lb_time_in_force_type_t time_in_force;
  /**
   * Long term order expire date (maybe null)
   */
  const struct lb_date_t *expire_date;
  /**
   * Last updated time (maybe null)
   */
  const int64_t *updated_at;
  /**
   * Conditional order trigger time (maybe null)
   */
  const int64_t *trigger_at;
  /**
   * `TSMAMT` / `TSLPAMT` order trailing amount (maybe null)
   */
  const struct lb_decimal_t *trailing_amount;
  /**
   * `TSMPCT` / `TSLPPCT` order trailing percent (maybe null)
   */
  const struct lb_decimal_t *trailing_percent;
  /**
   * `TSLPAMT` / `TSLPPCT` order limit offset amount (maybe null)
   */
  const struct lb_decimal_t *limit_offset;
  /**
   * Conditional order trigger status (maybe null)
   */
  const enum lb_trigger_status_t *trigger_status;
  /**
   * Currency
   */
  const char *currency;
  /**
   * Enable or disable outside regular trading hours (maybe null)
   */
  const enum lb_outside_rth_t *outside_rth;
  /**
   * Limit depth level (maybe null)
   */
  const int32_t *limit_depth_level;
  /**
   * Trigger count (maybe null)
   */
  const int32_t *trigger_count;
  /**
   * Monitor price (maybe null)
   */
  const struct lb_decimal_t *monitor_price;
  /**
   * Remark
   */
  const char *remark;
  /**
   * Commission-free Status
   */
  enum lb_commission_free_status_t free_status;
  /**
   * Commission-free amount
   */
  const struct lb_decimal_t *free_amount;
  /**
   * Commission-free currency
   */
  const char *free_currency;
  /**
   * Deduction status
   */
  enum lb_deduction_status_t deductions_status;
  /**
   * Deduction amount
   */
  const struct lb_decimal_t *deductions_amount;
  /**
   * Deduction currency
   */
  const char *deductions_currency;
  /**
   * Platform fee deduction status
   */
  enum lb_deduction_status_t platform_deducted_status;
  /**
   * Platform deduction amount
   */
  const struct lb_decimal_t *platform_deducted_amount;
  /**
   * Platform deduction currency
   */
  const char *platform_deducted_currency;
  /**
   * Order history details
   */
  const struct lb_order_history_detail_t *history;
  /**
   * Number of history
   */
  uintptr_t num_history;
  /**
   * Whether charge_detail is valid (false when the order has no charge info)
   */
  bool has_charge_detail;
  /**
   * Order charges (only valid when has_charge_detail is true)
   */
  struct lb_order_charge_detail_t charge_detail;
  /**
   * Attached orders
   */
  const struct CAttachedOrderDetail *attached_orders;
  /**
   * Number of attached orders
   */
  uintptr_t num_attached_orders;
} lb_order_detail_t;

/**
 * Options for estimate maximum purchase quantity
 */
typedef struct lb_estimate_max_purchase_quantity_response_t {
  /**
   * Cash available quantity
   */
  const struct lb_decimal_t *cash_max_qty;
  /**
   * Margin available quantity
   */
  const struct lb_decimal_t *margin_max_qty;
} lb_estimate_max_purchase_quantity_response_t;

/**
 * Security calc index response
 */
typedef struct lb_security_calc_index_t {
  /**
   * Security code
   */
  const char *symbol;
  /**
   * Latest price
   */
  const struct lb_decimal_t *last_done;
  /**
   * Change value
   */
  const struct lb_decimal_t *change_value;
  /**
   * Change ratio
   */
  const struct lb_decimal_t *change_rate;
  /**
   * Volume
   */
  const int64_t *volume;
  /**
   * Turnover
   */
  const struct lb_decimal_t *turnover;
  /**
   * Year-to-date change ratio
   */
  const struct lb_decimal_t *ytd_change_rate;
  /**
   * Turnover rate
   */
  const struct lb_decimal_t *turnover_rate;
  /**
   * Total market value
   */
  const struct lb_decimal_t *total_market_value;
  /**
   * Capital flow
   */
  const struct lb_decimal_t *capital_flow;
  /**
   * Amplitude
   */
  const struct lb_decimal_t *amplitude;
  /**
   * Volume ratio
   */
  const struct lb_decimal_t *volume_ratio;
  /**
   * PE (TTM)
   */
  const struct lb_decimal_t *pe_ttm_ratio;
  /**
   * PB
   */
  const struct lb_decimal_t *pb_ratio;
  /**
   * Dividend ratio (TTM)
   */
  const struct lb_decimal_t *dividend_ratio_ttm;
  /**
   * Five days change ratio
   */
  const struct lb_decimal_t *five_day_change_rate;
  /**
   * Ten days change ratio
   */
  const struct lb_decimal_t *ten_day_change_rate;
  /**
   * Half year change ratio
   */
  const struct lb_decimal_t *half_year_change_rate;
  /**
   * Five minutes change ratio
   */
  const struct lb_decimal_t *five_minutes_change_rate;
  /**
   * Expiry date
   */
  const struct lb_date_t *expiry_date;
  /**
   * Strike price
   */
  const struct lb_decimal_t *strike_price;
  /**
   * Upper bound price
   */
  const struct lb_decimal_t *upper_strike_price;
  /**
   * Lower bound price
   */
  const struct lb_decimal_t *lower_strike_price;
  /**
   * Outstanding quantity
   */
  const int64_t *outstanding_qty;
  /**
   * Outstanding ratio
   */
  const struct lb_decimal_t *outstanding_ratio;
  /**
   * Premium
   */
  const struct lb_decimal_t *premium;
  /**
   * In/out of the bound
   */
  const struct lb_decimal_t *itm_otm;
  /**
   * Implied volatility
   */
  const struct lb_decimal_t *implied_volatility;
  /**
   * Warrant delta
   */
  const struct lb_decimal_t *warrant_delta;
  /**
   * Call price
   */
  const struct lb_decimal_t *call_price;
  /**
   * Price interval from the call price
   */
  const struct lb_decimal_t *to_call_price;
  /**
   * Effective leverage
   */
  const struct lb_decimal_t *effective_leverage;
  /**
   * Leverage ratio
   */
  const struct lb_decimal_t *leverage_ratio;
  /**
   * Conversion ratio
   */
  const struct lb_decimal_t *conversion_ratio;
  /**
   * Breakeven point
   */
  const struct lb_decimal_t *balance_point;
  /**
   * Open interest
   */
  const int64_t *open_interest;
  /**
   * Delta
   */
  const struct lb_decimal_t *delta;
  /**
   * Gamma
   */
  const struct lb_decimal_t *gamma;
  /**
   * Theta
   */
  const struct lb_decimal_t *theta;
  /**
   * Vega
   */
  const struct lb_decimal_t *vega;
  /**
   * Rho
   */
  const struct lb_decimal_t *rho;
} lb_security_calc_index_t;

/**
 * Warrant info
 */
typedef struct lb_warrant_info_t {
  /**
   * Security code
   */
  const char *symbol;
  /**
   * Warrant type
   */
  enum lb_warrant_type_t warrant_type;
  /**
   * Security name
   */
  const char *name;
  /**
   * Latest price
   */
  const struct lb_decimal_t *last_done;
  /**
   * Quote change rate
   */
  const struct lb_decimal_t *change_rate;
  /**
   * Quote change
   */
  const struct lb_decimal_t *change_value;
  /**
   * Volume
   */
  int64_t volume;
  /**
   * Turnover
   */
  const struct lb_decimal_t *turnover;
  /**
   * Expiry date
   */
  struct lb_date_t expiry_date;
  /**
   * Strike price
   */
  const struct lb_decimal_t *strike_price;
  /**
   * Upper strike price
   */
  const struct lb_decimal_t *upper_strike_price;
  /**
   * Lower strike price
   */
  const struct lb_decimal_t *lower_strike_price;
  /**
   * Outstanding quantity
   */
  int64_t outstanding_qty;
  /**
   * Outstanding ratio
   */
  const struct lb_decimal_t *outstanding_ratio;
  /**
   * Premium
   */
  const struct lb_decimal_t *premium;
  /**
   * In/out of the bound
   */
  const struct lb_decimal_t *itm_otm;
  /**
   * Implied volatility
   */
  const struct lb_decimal_t *implied_volatility;
  /**
   * Delta
   */
  const struct lb_decimal_t *delta;
  /**
   * Call price
   */
  const struct lb_decimal_t *call_price;
  /**
   * Price interval from the call price
   */
  const struct lb_decimal_t *to_call_price;
  /**
   * Effective leverage
   */
  const struct lb_decimal_t *effective_leverage;
  /**
   * Leverage ratio
   */
  const struct lb_decimal_t *leverage_ratio;
  /**
   * Conversion ratio
   */
  const struct lb_decimal_t *conversion_ratio;
  /**
   * Breakeven point
   */
  const struct lb_decimal_t *balance_point;
  /**
   * Status
   */
  enum lb_warrant_status_t status;
} lb_warrant_info_t;

/**
 * Quote package detail
 */
typedef struct lb_quote_package_detail_t {
  /**
   * Key
   */
  const char *key;
  /**
   * Name
   */
  const char *name;
  /**
   * Description
   */
  const char *description;
  /**
   * Start at
   */
  int64_t start_at;
  /**
   * End at
   */
  int64_t end_at;
} lb_quote_package_detail_t;

/**
 * Market temperature
 */
typedef struct lb_market_temperature_t {
  /**
   * Temperature value
   */
  int32_t temperature;
  /**
   * Temperature description
   */
  const char *description;
  /**
   * Market valuation
   */
  int32_t valuation;
  /**
   * Market sentiment
   */
  int32_t sentiment;
  /**
   * Time
   */
  int64_t timestamp;
} lb_market_temperature_t;

/**
 * Historical market temperature response
 */
typedef struct lb_history_market_temperature_response_t {
  /**
   * Granularity
   */
  enum lb_granularity_t granularity;
  /**
   * Records
   */
  const struct lb_market_temperature_t *records;
  /**
   * Number of records
   */
  uintptr_t num_records;
} lb_history_market_temperature_response_t;

/**
 * Filing item
 */
typedef struct lb_filing_item_t {
  /**
   * Filing ID
   */
  const char *id;
  /**
   * Title
   */
  const char *title;
  /**
   * Description
   */
  const char *description;
  /**
   * File name
   */
  const char *file_name;
  /**
   * File URLs
   */
  const char *const *file_urls;
  /**
   * Number of file URLs
   */
  uintptr_t num_file_urls;
  /**
   * Published time (Unix timestamp)
   */
  int64_t published_at;
} lb_filing_item_t;

/**
 * Topic author
 */
typedef struct lb_topic_author_t {
  /**
   * Member ID
   */
  const char *member_id;
  /**
   * Display name
   */
  const char *name;
  /**
   * Avatar URL
   */
  const char *avatar;
} lb_topic_author_t;

/**
 * Topic image
 */
typedef struct lb_topic_image_t {
  /**
   * Original image URL
   */
  const char *url;
  /**
   * Small thumbnail URL
   */
  const char *sm;
  /**
   * Large image URL
   */
  const char *lg;
} lb_topic_image_t;

/**
 * My topic item (topic created by the current authenticated user)
 */
typedef struct lb_owned_topic_t {
  /**
   * Topic ID
   */
  const char *id;
  /**
   * Title
   */
  const char *title;
  /**
   * Plain text excerpt
   */
  const char *description;
  /**
   * Markdown body
   */
  const char *body;
  /**
   * Author
   */
  struct lb_topic_author_t author;
  /**
   * Related stock tickers
   */
  const char *const *tickers;
  /**
   * Number of tickers
   */
  uintptr_t num_tickers;
  /**
   * Hashtag names
   */
  const char *const *hashtags;
  /**
   * Number of hashtags
   */
  uintptr_t num_hashtags;
  /**
   * Images
   */
  const struct lb_topic_image_t *images;
  /**
   * Number of images
   */
  uintptr_t num_images;
  /**
   * Likes count
   */
  int32_t likes_count;
  /**
   * Comments count
   */
  int32_t comments_count;
  /**
   * Views count
   */
  int32_t views_count;
  /**
   * Shares count
   */
  int32_t shares_count;
  /**
   * Content type: "article" or "post"
   */
  const char *topic_type;
  /**
   * URL to the full topic page
   */
  const char *detail_url;
  /**
   * Created time (Unix timestamp)
   */
  int64_t created_at;
  /**
   * Updated time (Unix timestamp)
   */
  int64_t updated_at;
} lb_owned_topic_t;

/**
 * Topic item
 */
typedef struct lb_topic_item_t {
  /**
   * Topic ID
   */
  const char *id;
  /**
   * Title
   */
  const char *title;
  /**
   * Description
   */
  const char *description;
  /**
   * URL
   */
  const char *url;
  /**
   * Published time (Unix timestamp)
   */
  int64_t published_at;
  /**
   * Comments count
   */
  int32_t comments_count;
  /**
   * Likes count
   */
  int32_t likes_count;
  /**
   * Shares count
   */
  int32_t shares_count;
} lb_topic_item_t;

/**
 * News item
 */
typedef struct lb_news_item_t {
  /**
   * News ID
   */
  const char *id;
  /**
   * Title
   */
  const char *title;
  /**
   * Description
   */
  const char *description;
  /**
   * URL
   */
  const char *url;
  /**
   * Published time (Unix timestamp)
   */
  int64_t published_at;
  /**
   * Comments count
   */
  int32_t comments_count;
  /**
   * Likes count
   */
  int32_t likes_count;
  /**
   * Shares count
   */
  int32_t shares_count;
} lb_news_item_t;

/**
 * Market trading time item describing the current status of a single market.
 */
typedef struct lb_market_time_item_t {
  /**
   * Market identifier.
   */
  enum lb_market_t market;
  /**
   * Current market trade status code. See the market status definition for
   * the complete code table.
   */
  int32_t trade_status;
  /**
   * Timestamp of the current trade status as an ISO-8601 string.
   */
  const char *timestamp;
  /**
   * Delayed market trade status code.
   */
  int32_t delay_trade_status;
  /**
   * Timestamp of the delayed trade status as an ISO-8601 string.
   */
  const char *delay_timestamp;
  /**
   * Sub-status code for the current trade status.
   */
  int32_t sub_status;
  /**
   * Sub-status code for the delayed trade status.
   */
  int32_t delay_sub_status;
} lb_market_time_item_t;

/**
 * Response containing the trading status for all markets.
 */
typedef struct lb_market_status_response_t {
  /**
   * Pointer to array of market time items.
   */
  const struct lb_market_time_item_t *market_time;
  /**
   * Number of elements in the array.
   */
  uintptr_t num_market_time;
} lb_market_status_response_t;

/**
 * A single broker entry in a broker holding top list.
 */
typedef struct lb_broker_holding_entry_t {
  /**
   * Name of the broker.
   */
  const char *name;
  /**
   * Participant number identifying the broker.
   */
  const char *parti_number;
  /**
   * Change value as a decimal string.
   */
  const char *chg;
  /**
   * Whether this broker is marked as a strong holder.
   */
  bool strong;
} lb_broker_holding_entry_t;

/**
 * Top broker holdings for a security, split into top buyers and top sellers.
 */
typedef struct lb_broker_holding_top_t {
  /**
   * Pointer to array of top-buying broker entries.
   */
  const struct lb_broker_holding_entry_t *buy;
  /**
   * Number of elements in the buy array.
   */
  uintptr_t num_buy;
  /**
   * Pointer to array of top-selling broker entries.
   */
  const struct lb_broker_holding_entry_t *sell;
  /**
   * Number of elements in the sell array.
   */
  uintptr_t num_sell;
  /**
   * Timestamp of the last update as an ISO-8601 string.
   */
  const char *updated_at;
} lb_broker_holding_top_t;

/**
 * A set of holding change values over multiple time windows.
 */
typedef struct lb_broker_holding_changes_t {
  /**
   * Current value as a decimal string.
   */
  const char *value;
  /**
   * Change over 1 day as a decimal string.
   */
  const char *chg_1;
  /**
   * Change over 5 days as a decimal string.
   */
  const char *chg_5;
  /**
   * Change over 20 days as a decimal string.
   */
  const char *chg_20;
  /**
   * Change over 60 days as a decimal string.
   */
  const char *chg_60;
} lb_broker_holding_changes_t;

/**
 * Detailed holding information for a single broker in the broker holding
 * detail list.
 */
typedef struct lb_broker_holding_detail_item_t {
  /**
   * Name of the broker.
   */
  const char *name;
  /**
   * Participant number identifying the broker.
   */
  const char *parti_number;
  /**
   * Holding ratio and its changes over multiple time windows.
   */
  struct lb_broker_holding_changes_t ratio;
  /**
   * Absolute share count and its changes over multiple time windows.
   */
  struct lb_broker_holding_changes_t shares;
  /**
   * Whether this broker is marked as a strong holder.
   */
  bool strong;
} lb_broker_holding_detail_item_t;

/**
 * Full broker holding detail response for a security.
 */
typedef struct lb_broker_holding_detail_t {
  /**
   * Pointer to array of broker holding detail items.
   */
  const struct lb_broker_holding_detail_item_t *list;
  /**
   * Number of elements in the array.
   */
  uintptr_t num_list;
  /**
   * Timestamp of the last update as an ISO-8601 string.
   */
  const char *updated_at;
} lb_broker_holding_detail_t;

/**
 * A single day's broker holding record.
 */
typedef struct lb_broker_holding_daily_item_t {
  /**
   * Date of the record as a string (e.g. `"2024-01-15"`).
   */
  const char *date;
  /**
   * Total shares held by the broker on this date as a decimal string.
   */
  const char *holding;
  /**
   * Holding ratio as a decimal string.
   */
  const char *ratio;
  /**
   * Day-over-day change in holdings as a decimal string.
   */
  const char *chg;
} lb_broker_holding_daily_item_t;

/**
 * Historical daily broker holding records for a security.
 */
typedef struct lb_broker_holding_daily_history_t {
  /**
   * Pointer to array of daily broker holding items.
   */
  const struct lb_broker_holding_daily_item_t *list;
  /**
   * Number of elements in the array.
   */
  uintptr_t num_list;
} lb_broker_holding_daily_history_t;

/**
 * A single candlestick data point for the A/H share premium.
 */
typedef struct lb_ah_premium_kline_t {
  /**
   * A-share price as a decimal string.
   */
  const char *aprice;
  /**
   * A-share previous close price as a decimal string.
   */
  const char *apreclose;
  /**
   * H-share price as a decimal string.
   */
  const char *hprice;
  /**
   * H-share previous close price as a decimal string.
   */
  const char *hpreclose;
  /**
   * CNY/HKD currency exchange rate as a decimal string.
   */
  const char *currency_rate;
  /**
   * A/H premium rate as a decimal string.
   */
  const char *ahpremium_rate;
  /**
   * Price spread between A-share and H-share as a decimal string.
   */
  const char *price_spread;
  /**
   * Unix timestamp (seconds) of this data point.
   */
  int64_t timestamp;
} lb_ah_premium_kline_t;

/**
 * Historical A/H premium kline data.
 */
typedef struct lb_ah_premium_klines_t {
  /**
   * Pointer to array of A/H premium kline data points.
   */
  const struct lb_ah_premium_kline_t *klines;
  /**
   * Number of elements in the array.
   */
  uintptr_t num_klines;
} lb_ah_premium_klines_t;

/**
 * Intraday A/H premium data for the current trading session.
 */
typedef struct lb_ah_premium_intraday_t {
  /**
   * Pointer to array of intraday A/H premium kline data points.
   */
  const struct lb_ah_premium_kline_t *klines;
  /**
   * Number of elements in the array.
   */
  uintptr_t num_klines;
} lb_ah_premium_intraday_t;

/**
 * Trade volume breakdown at a single price level.
 */
typedef struct lb_trade_price_level_t {
  /**
   * Total buy-side trade amount at this price level as a decimal string.
   */
  const char *buy_amount;
  /**
   * Total neutral (unknown direction) trade amount at this price level as a
   * decimal string.
   */
  const char *neutral_amount;
  /**
   * Price of this level as a decimal string.
   */
  const char *price;
  /**
   * Total sell-side trade amount at this price level as a decimal string.
   */
  const char *sell_amount;
} lb_trade_price_level_t;

/**
 * Aggregated trade statistics for a security over a period.
 */
typedef struct lb_trade_statistics_t {
  /**
   * Volume-weighted average price as a decimal string.
   */
  const char *avgprice;
  /**
   * Total buy-side trade amount as a decimal string.
   */
  const char *buy;
  /**
   * Total neutral (unknown direction) trade amount as a decimal string.
   */
  const char *neutral;
  /**
   * Previous close price as a decimal string.
   */
  const char *preclose;
  /**
   * Total sell-side trade amount as a decimal string.
   */
  const char *sell;
  /**
   * Timestamp of the statistics snapshot as an ISO-8601 string.
   */
  const char *timestamp;
  /**
   * Total traded amount (buy + sell + neutral) as a decimal string.
   */
  const char *total_amount;
  /**
   * Pointer to array of trade date strings (e.g. `"2024-01-15"`).
   */
  const char *const *trade_date;
  /**
   * Number of elements in the trade_date array.
   */
  uintptr_t num_trade_date;
  /**
   * Total number of individual trades as a decimal string.
   */
  const char *trades_count;
} lb_trade_statistics_t;

/**
 * Full trade statistics response combining aggregate stats and per-price-level
 * breakdown.
 */
typedef struct lb_trade_stats_response_t {
  /**
   * Aggregated trade statistics for the security.
   */
  struct lb_trade_statistics_t statistics;
  /**
   * Pointer to array of per-price-level trade breakdowns.
   */
  const struct lb_trade_price_level_t *trades;
  /**
   * Number of elements in the trades array.
   */
  uintptr_t num_trades;
} lb_trade_stats_response_t;

/**
 * A single market anomaly alert item.
 */
typedef struct lb_anomaly_item_t {
  /**
   * Security symbol (e.g. `"700.HK"`).
   */
  const char *symbol;
  /**
   * Security name string.
   */
  const char *name;
  /**
   * Name of the anomaly alert type.
   */
  const char *alert_name;
  /**
   * Unix timestamp (seconds) when the alert was triggered.
   */
  int64_t alert_time;
  /**
   * Pointer to array of change value strings describing the anomaly.
   */
  const char *const *change_values;
  /**
   * Number of elements in the change_values array.
   */
  uintptr_t num_change_values;
  /**
   * Sentiment/emotion indicator for the anomaly (positive/negative
   * direction).
   */
  int32_t emotion;
} lb_anomaly_item_t;

/**
 * Response containing a list of market anomaly alerts.
 */
typedef struct lb_anomaly_response_t {
  /**
   * Whether all anomaly alerts are turned off.
   */
  bool all_off;
  /**
   * Pointer to array of anomaly alert items.
   */
  const struct lb_anomaly_item_t *changes;
  /**
   * Number of elements in the changes array.
   */
  uintptr_t num_changes;
} lb_anomaly_response_t;

/**
 * A constituent stock within an index.
 */
typedef struct lb_constituent_stock_t {
  /**
   * Security symbol (e.g. `"700.HK"`).
   */
  const char *symbol;
  /**
   * Security name string.
   */
  const char *name;
  /**
   * Latest traded price as a decimal string.
   */
  const char *last_done;
  /**
   * Previous close price as a decimal string.
   */
  const char *prev_close;
  /**
   * Net capital inflow for the stock as a decimal string.
   */
  const char *inflow;
  /**
   * Outstanding balance (remaining sell-side liquidity) as a decimal string.
   */
  const char *balance;
  /**
   * Total traded amount for the session as a decimal string.
   */
  const char *amount;
  /**
   * Total issued shares as a decimal string.
   */
  const char *total_shares;
  /**
   * Pointer to array of tag strings associated with the stock.
   */
  const char *const *tags;
  /**
   * Number of elements in the tags array.
   */
  uintptr_t num_tags;
  /**
   * Brief introductory description of the stock.
   */
  const char *intro;
  /**
   * Market identifier string (e.g. `"HK"`, `"US"`).
   */
  const char *market;
  /**
   * Number of circulating (publicly tradeable) shares as a decimal string.
   */
  const char *circulating_shares;
  /**
   * Whether the quote data for this stock is delayed.
   */
  bool delay;
  /**
   * Price change (from previous close) as a decimal string.
   */
  const char *chg;
  /**
   * Current trade status code for the stock.
   */
  int32_t trade_status;
} lb_constituent_stock_t;

/**
 * Index constituent data including breadth statistics and the list of member
 * stocks.
 */
typedef struct lb_index_constituents_t {
  /**
   * Number of constituent stocks that declined in this session.
   */
  int32_t fall_num;
  /**
   * Number of constituent stocks that were unchanged in this session.
   */
  int32_t flat_num;
  /**
   * Number of constituent stocks that advanced in this session.
   */
  int32_t rise_num;
  /**
   * Pointer to array of constituent stock data.
   */
  const struct lb_constituent_stock_t *stocks;
  /**
   * Number of elements in the stocks array.
   */
  uintptr_t num_stocks;
} lb_index_constituents_t;

/**
 * A single dividend event for a security (C-facing FFI type).
 */
typedef struct lb_dividend_item_t {
  /**
   * Security symbol (e.g. `"700.HK"`).
   */
  const char *symbol;
  /**
   * Unique identifier for the dividend event.
   */
  const char *id;
  /**
   * Human-readable description of the dividend.
   */
  const char *desc;
  /**
   * Record date ("YYYY-MM-DD").
   */
  const char *record_date;
  /**
   * Ex-dividend date ("YYYY-MM-DD").
   */
  const char *ex_date;
  /**
   * Payment date ("YYYY-MM-DD").
   */
  const char *payment_date;
} lb_dividend_item_t;

/**
 * List of dividend items for a security (C-facing FFI type).
 */
typedef struct lb_dividend_list_t {
  /**
   * Pointer to the array of dividend items.
   */
  const struct lb_dividend_item_t *list;
  /**
   * Number of items in the array.
   */
  uintptr_t num_list;
} lb_dividend_list_t;

/**
 * Aggregated institutional rating opinion counts over a date range (C-facing
 * FFI type).
 */
typedef struct lb_rating_evaluate_t {
  /**
   * Number of "buy" ratings.
   */
  int32_t buy;
  /**
   * Number of "outperform" ratings.
   */
  int32_t over;
  /**
   * Number of "hold" ratings.
   */
  int32_t hold;
  /**
   * Number of "underperform" ratings.
   */
  int32_t under;
  /**
   * Number of "sell" ratings.
   */
  int32_t sell;
  /**
   * Number of "no opinion" ratings.
   */
  int32_t no_opinion;
  /**
   * Total number of ratings.
   */
  int32_t total;
  /**
   * Start date of the evaluation period ("YYYY-MM-DD").
   */
  const char *start_date;
  /**
   * End date of the evaluation period ("YYYY-MM-DD").
   */
  const char *end_date;
} lb_rating_evaluate_t;

/**
 * Institutional price-target range over a date period (C-facing FFI type).
 */
typedef struct lb_rating_target_t {
  /**
   * Highest analyst price target in the period.
   */
  const char *highest_price;
  /**
   * Lowest analyst price target in the period.
   */
  const char *lowest_price;
  /**
   * Previous closing price at the start of the period.
   */
  const char *prev_close;
  /**
   * Start date of the target period ("YYYY-MM-DD").
   */
  const char *start_date;
  /**
   * End date of the target period ("YYYY-MM-DD").
   */
  const char *end_date;
} lb_rating_target_t;

/**
 * Summary of rating opinion counts on a specific date (C-facing FFI type).
 */
typedef struct lb_rating_summary_evaluate_t {
  /**
   * Number of "buy" ratings.
   */
  int32_t buy;
  /**
   * Date of the rating summary ("YYYY-MM-DD").
   */
  const char *date;
  /**
   * Number of "hold" ratings.
   */
  int32_t hold;
  /**
   * Number of "sell" ratings.
   */
  int32_t sell;
  /**
   * Number of "strong buy" ratings.
   */
  int32_t strong_buy;
  /**
   * Number of "underperform" ratings.
   */
  int32_t under;
} lb_rating_summary_evaluate_t;

/**
 * Latest institutional rating data including evaluate counts, price targets,
 * and industry context (C-facing FFI type).
 */
typedef struct lb_institution_rating_latest_t {
  /**
   * Aggregated opinion counts for the current period.
   */
  struct lb_rating_evaluate_t evaluate;
  /**
   * Consensus price target range for the current period.
   */
  struct lb_rating_target_t target;
  /**
   * Industry identifier.
   */
  int64_t industry_id;
  /**
   * Industry name.
   */
  const char *industry_name;
  /**
   * Rank of the security within its industry by rating.
   */
  int32_t industry_rank;
  /**
   * Total number of securities in the industry.
   */
  int32_t industry_total;
  /**
   * Mean rating score for the industry.
   */
  int32_t industry_mean;
  /**
   * Median rating score for the industry.
   */
  int32_t industry_median;
} lb_institution_rating_latest_t;

/**
 * Summary of the latest institutional rating for a security (C-facing FFI
 * type).
 */
typedef struct lb_institution_rating_summary_t {
  /**
   * Currency symbol used for price targets (e.g. `"HKD"`).
   */
  const char *ccy_symbol;
  /**
   * Price change since the previous rating cycle.
   */
  const char *change;
  /**
   * Aggregated opinion counts on the summary date.
   */
  struct lb_rating_summary_evaluate_t evaluate;
  /**
   * Consensus recommendation.
   */
  enum lb_institution_recommend_t recommend;
  /**
   * Consensus price target value.
   */
  const char *target;
  /**
   * Timestamp of the last update.
   */
  const char *updated_at;
} lb_institution_rating_summary_t;

/**
 * Full institutional rating for a security, combining latest details and a
 * summary (C-facing FFI type).
 */
typedef struct lb_institution_rating_t {
  /**
   * Most recent detailed rating data.
   */
  struct lb_institution_rating_latest_t latest;
  /**
   * High-level summary of the rating.
   */
  struct lb_institution_rating_summary_t summary;
} lb_institution_rating_t;

/**
 * A single data point in the historical evaluate series for institution rating
 * detail (C-facing FFI type).
 */
typedef struct lb_institution_rating_detail_evaluate_item_t {
  /**
   * Number of "buy" ratings on this date.
   */
  int32_t buy;
  /**
   * Date of this evaluate snapshot ("YYYY-MM-DD").
   */
  const char *date;
  /**
   * Number of "hold" ratings on this date.
   */
  int32_t hold;
  /**
   * Number of "sell" ratings on this date.
   */
  int32_t sell;
  /**
   * Number of "strong buy" / "outperform" ratings on this date.
   */
  int32_t strong_buy;
  /**
   * Number of "no opinion" ratings on this date.
   */
  int32_t no_opinion;
  /**
   * Number of "underperform" ratings on this date.
   */
  int32_t under;
} lb_institution_rating_detail_evaluate_item_t;

/**
 * A single data point in the historical price-target series for institution
 * rating detail (C-facing FFI type).
 */
typedef struct lb_institution_rating_detail_target_item_t {
  /**
   * Average analyst price target on this date.
   */
  const char *avg_target;
  /**
   * Date of this target snapshot ("YYYY-MM-DD").
   */
  const char *date;
  /**
   * Maximum analyst price target on this date.
   */
  const char *max_target;
  /**
   * Minimum analyst price target on this date.
   */
  const char *min_target;
  /**
   * Whether the price target was met.
   */
  bool meet;
  /**
   * Actual price on this date.
   */
  const char *price;
  /**
   * Unix timestamp of this data point.
   */
  const char *timestamp;
} lb_institution_rating_detail_target_item_t;

/**
 * Detailed historical institution rating data including evaluate and
 * price-target series (C-facing FFI type).
 */
typedef struct lb_institution_rating_detail_t {
  /**
   * Currency symbol used for price targets (e.g. `"HKD"`).
   */
  const char *ccy_symbol;
  /**
   * Pointer to the array of historical evaluate snapshots.
   */
  const struct lb_institution_rating_detail_evaluate_item_t *evaluate_list;
  /**
   * Number of items in `evaluate_list`.
   */
  uintptr_t num_evaluate_list;
  /**
   * Percentage of price targets that were met (as a string).
   */
  const char *data_percent;
  /**
   * Prediction accuracy rate for price targets (as a string).
   */
  const char *prediction_accuracy;
  /**
   * Timestamp of the last update.
   */
  const char *updated_at;
  /**
   * Pointer to the array of historical price-target snapshots.
   */
  const struct lb_institution_rating_detail_target_item_t *target_list;
  /**
   * Number of items in `target_list`.
   */
  uintptr_t num_target_list;
} lb_institution_rating_detail_t;

/**
 * A single EPS forecast item covering a fiscal period (C-facing FFI type).
 */
typedef struct lb_forecast_eps_item_t {
  /**
   * Median EPS forecast across all contributing institutions.
   */
  const char *forecast_eps_median;
  /**
   * Mean EPS forecast across all contributing institutions.
   */
  const char *forecast_eps_mean;
  /**
   * Lowest individual EPS forecast.
   */
  const char *forecast_eps_lowest;
  /**
   * Highest individual EPS forecast.
   */
  const char *forecast_eps_highest;
  /**
   * Total number of institutions providing an EPS forecast.
   */
  int32_t institution_total;
  /**
   * Number of institutions that revised their forecast upward.
   */
  int32_t institution_up;
  /**
   * Number of institutions that revised their forecast downward.
   */
  int32_t institution_down;
  /**
   * Unix timestamp of the forecast period start date.
   */
  int64_t forecast_start_date;
  /**
   * Unix timestamp of the forecast period end date.
   */
  int64_t forecast_end_date;
} lb_forecast_eps_item_t;

/**
 * Collection of EPS forecast items (C-facing FFI type).
 */
typedef struct lb_forecast_eps_t {
  /**
   * Pointer to the array of EPS forecast items.
   */
  const struct lb_forecast_eps_item_t *items;
  /**
   * Number of items in the array.
   */
  uintptr_t num_items;
} lb_forecast_eps_t;

/**
 * A single (timestamp, value) data point in a valuation time series (C-facing
 * FFI type).
 */
typedef struct lb_valuation_point_t {
  /**
   * Unix timestamp of the data point.
   */
  int64_t timestamp;
  /**
   * Valuation metric value at this timestamp (as a decimal string).
   */
  const char *value;
} lb_valuation_point_t;

/**
 * Historical data for a single valuation metric (e.g. PE, PB) including
 * summary statistics (C-facing FFI type).
 */
typedef struct lb_valuation_metric_data_t {
  /**
   * Description or label of the valuation metric.
   */
  const char *desc;
  /**
   * Highest value of the metric over the series.
   */
  const char *high;
  /**
   * Lowest value of the metric over the series.
   */
  const char *low;
  /**
   * Median value of the metric over the series.
   */
  const char *median;
  /**
   * Pointer to the array of time-series data points.
   */
  const struct lb_valuation_point_t *list;
  /**
   * Number of data points in `list`.
   */
  uintptr_t num_list;
} lb_valuation_metric_data_t;

/**
 * Set of valuation metric data series for a security (C-facing FFI type).
 */
typedef struct lb_valuation_metrics_data_t {
  /**
   * Price-to-earnings ratio series, or null if unavailable.
   */
  const struct lb_valuation_metric_data_t *pe;
  /**
   * Price-to-book ratio series, or null if unavailable.
   */
  const struct lb_valuation_metric_data_t *pb;
  /**
   * Price-to-sales ratio series, or null if unavailable.
   */
  const struct lb_valuation_metric_data_t *ps;
  /**
   * Dividend yield series, or null if unavailable.
   */
  const struct lb_valuation_metric_data_t *dvd_yld;
} lb_valuation_metrics_data_t;

/**
 * Valuation data container holding all metric series for a security (C-facing
 * FFI type).
 */
typedef struct lb_valuation_data_t {
  /**
   * The set of valuation metric data series (PE, PB, PS, dividend yield).
   */
  struct lb_valuation_metrics_data_t metrics;
} lb_valuation_data_t;

typedef struct lb_valuation_metric_data_t CValuationHistoryMetric;

/**
 * Set of historical valuation metric series (PE, PB, PS) for a security
 * (C-facing FFI type).
 */
typedef struct lb_valuation_history_metrics_t {
  /**
   * Historical price-to-earnings ratio series, or null if unavailable.
   */
  const CValuationHistoryMetric *pe;
  /**
   * Historical price-to-book ratio series, or null if unavailable.
   */
  const CValuationHistoryMetric *pb;
  /**
   * Historical price-to-sales ratio series, or null if unavailable.
   */
  const CValuationHistoryMetric *ps;
} lb_valuation_history_metrics_t;

/**
 * Response containing historical valuation metric series (C-facing FFI type).
 */
typedef struct lb_valuation_history_response_t {
  /**
   * Historical price-to-earnings ratio series, or null if unavailable.
   */
  const CValuationHistoryMetric *pe;
  /**
   * Historical price-to-book ratio series, or null if unavailable.
   */
  const CValuationHistoryMetric *pb;
  /**
   * Historical price-to-sales ratio series, or null if unavailable.
   */
  const CValuationHistoryMetric *ps;
} lb_valuation_history_response_t;

/**
 * High-level company profile and metadata (C-facing FFI type).
 */
typedef struct lb_company_overview_t {
  /**
   * Short display name of the company.
   */
  const char *name;
  /**
   * Full legal company name.
   */
  const char *company_name;
  /**
   * Year the company was founded.
   */
  const char *founded;
  /**
   * Stock listing date ("YYYY-MM-DD").
   */
  const char *listing_date;
  /**
   * Exchange or market where the stock is listed.
   */
  const char *market;
  /**
   * Geographic region of the company's primary operations.
   */
  const char *region;
  /**
   * Registered address of the company.
   */
  const char *address;
  /**
   * Principal office address.
   */
  const char *office_address;
  /**
   * Company website URL.
   */
  const char *website;
  /**
   * IPO issue price.
   */
  const char *issue_price;
  /**
   * Number of shares offered at IPO.
   */
  const char *shares_offered;
  /**
   * Name of the board chairman.
   */
  const char *chairman;
  /**
   * Name of the company secretary.
   */
  const char *secretary;
  /**
   * Name of the auditing institution.
   */
  const char *audit_inst;
  /**
   * Business category or industry classification label.
   */
  const char *category;
  /**
   * Fiscal year-end date (e.g. `"12/31"`).
   */
  const char *year_end;
  /**
   * Number of employees (as a string).
   */
  const char *employees;
  /**
   * Corporate phone number.
   */
  const char *phone;
  /**
   * Corporate fax number.
   */
  const char *fax;
  /**
   * Corporate email address.
   */
  const char *email;
  /**
   * Legal representative of the company.
   */
  const char *legal_repr;
  /**
   * General manager or CEO name.
   */
  const char *manager;
  /**
   * Stock ticker symbol.
   */
  const char *ticker;
  /**
   * Business description / company profile text.
   */
  const char *profile;
  /**
   * Numeric sector code.
   */
  int32_t sector;
} lb_company_overview_t;

/**
 * A stock position held by a shareholder (C-facing FFI type).
 */
typedef struct lb_shareholder_stock_t {
  /**
   * Security symbol (e.g. `"700.HK"`).
   */
  const char *symbol;
  /**
   * Stock code.
   */
  const char *code;
  /**
   * Exchange or market of the stock.
   */
  const char *market;
  /**
   * Change in the holding since the previous report.
   */
  const char *chg;
} lb_shareholder_stock_t;

/**
 * A single institutional or major shareholder entry (C-facing FFI type).
 */
typedef struct lb_shareholder_t {
  /**
   * Unique identifier for the shareholder.
   */
  const char *shareholder_id;
  /**
   * Display name of the shareholder.
   */
  const char *shareholder_name;
  /**
   * Type of institution (e.g. fund, insurance company).
   */
  const char *institution_type;
  /**
   * Percentage of total shares held.
   */
  const char *percent_of_shares;
  /**
   * Change in shares held since the previous report.
   */
  const char *shares_changed;
  /**
   * Date of the holdings report ("YYYY-MM-DD").
   */
  const char *report_date;
  /**
   * Pointer to the array of stock positions held by this shareholder.
   */
  const struct lb_shareholder_stock_t *stocks;
  /**
   * Number of stock positions in `stocks`.
   */
  uintptr_t num_stocks;
} lb_shareholder_t;

/**
 * Paginated list of shareholders for a security (C-facing FFI type).
 */
typedef struct lb_shareholder_list_t {
  /**
   * Pointer to the array of shareholder entries.
   */
  const struct lb_shareholder_t *shareholder_list;
  /**
   * Number of entries in `shareholder_list`.
   */
  uintptr_t num_shareholder_list;
  /**
   * URL to fetch the next page of results, or empty if no next page.
   */
  const char *forward_url;
  /**
   * Total number of shareholders across all pages.
   */
  int32_t total;
} lb_shareholder_list_t;

/**
 * A single fund that holds a position in a security (C-facing FFI type).
 */
typedef struct lb_fund_holder_t {
  /**
   * Fund code.
   */
  const char *code;
  /**
   * Security symbol held by the fund.
   */
  const char *symbol;
  /**
   * Currency of the fund's reported holding value.
   */
  const char *currency;
  /**
   * Fund name.
   */
  const char *name;
  /**
   * Proportion of the fund's portfolio allocated to this position.
   */
  const char *position_ratio;
  /**
   * Date of the holdings report ("YYYY-MM-DD").
   */
  const char *report_date;
} lb_fund_holder_t;

/**
 * Collection of fund holders for a security (C-facing FFI type).
 */
typedef struct lb_fund_holders_t {
  /**
   * Pointer to the array of fund holder entries.
   */
  const struct lb_fund_holder_t *lists;
  /**
   * Number of entries in `lists`.
   */
  uintptr_t num_lists;
} lb_fund_holders_t;

/**
 * A single corporate action event for a security (C-facing FFI type).
 */
typedef struct lb_corp_action_item_t {
  /**
   * Unique identifier for the corporate action.
   */
  const char *id;
  /**
   * Action date as a Unix timestamp string.
   */
  const char *date;
  /**
   * Human-readable action date string.
   */
  const char *date_str;
  /**
   * Type classification of the date (e.g. record date, ex-date).
   */
  const char *date_type;
  /**
   * Time zone associated with the action date.
   */
  const char *date_zone;
  /**
   * Type of corporate action (e.g. dividend, split).
   */
  const char *act_type;
  /**
   * Human-readable description of the action type.
   */
  const char *act_desc;
  /**
   * Action details or ratio string.
   */
  const char *action;
  /**
   * Whether this action occurred recently.
   */
  bool recent;
  /**
   * Whether announcement of this action was delayed.
   */
  bool is_delay;
  /**
   * Additional content explaining any delay.
   */
  const char *delay_content;
} lb_corp_action_item_t;

/**
 * Collection of corporate action events for a security (C-facing FFI type).
 */
typedef struct lb_corp_actions_t {
  /**
   * Pointer to the array of corporate action items.
   */
  const struct lb_corp_action_item_t *items;
  /**
   * Number of items in the array.
   */
  uintptr_t num_items;
} lb_corp_actions_t;

/**
 * A security held by an institutional investor (C-facing FFI type).
 */
typedef struct lb_invest_security_t {
  /**
   * Unique identifier for the investing company.
   */
  const char *company_id;
  /**
   * Display name of the investing company.
   */
  const char *company_name;
  /**
   * English name of the investing company.
   */
  const char *company_name_en;
  /**
   * Simplified Chinese name of the investing company.
   */
  const char *company_name_zhcn;
  /**
   * Security symbol held (e.g. `"700.HK"`).
   */
  const char *symbol;
  /**
   * Currency of the holding value.
   */
  const char *currency;
  /**
   * Percentage of total shares held.
   */
  const char *percent_of_shares;
  /**
   * Ranking of the holding within the investor's portfolio.
   */
  const char *shares_rank;
  /**
   * Market value of the holding.
   */
  const char *shares_value;
} lb_invest_security_t;

/**
 * Paginated list of investment relations for a security (C-facing FFI type).
 */
typedef struct lb_invest_relations_t {
  /**
   * URL to fetch the next page of results, or empty if no next page.
   */
  const char *forward_url;
  /**
   * Pointer to the array of invested securities.
   */
  const struct lb_invest_security_t *invest_securities;
  /**
   * Number of entries in `invest_securities`.
   */
  uintptr_t num_invest_securities;
} lb_invest_relations_t;

/**
 * A single operating/financial indicator within an operating report item
 * (C-facing FFI type).
 */
typedef struct lb_operating_indicator_t {
  /**
   * Machine-readable field name for the indicator.
   */
  const char *field_name;
  /**
   * Human-readable display name for the indicator.
   */
  const char *indicator_name;
  /**
   * Value of the indicator (as a decimal string).
   */
  const char *indicator_value;
  /**
   * Year-over-year change for the indicator.
   */
  const char *yoy;
} lb_operating_indicator_t;

/**
 * A single operating report entry including associated financial indicators
 * (C-facing FFI type).
 */
typedef struct lb_operating_item_t {
  /**
   * Unique identifier for the operating report item.
   */
  const char *id;
  /**
   * Report period identifier (e.g. `"2024Q1"`).
   */
  const char *report;
  /**
   * Title of the operating report.
   */
  const char *title;
  /**
   * Plain-text content of the operating report.
   */
  const char *txt;
  /**
   * Whether this is the most recent operating report.
   */
  bool latest;
  /**
   * URL to the original web page for this report.
   */
  const char *web_url;
  /**
   * Currency used in the financial data.
   */
  const char *financial_currency;
  /**
   * Name of the financial reporting entity.
   */
  const char *financial_name;
  /**
   * Region associated with the financial report.
   */
  const char *financial_region;
  /**
   * Financial report period label.
   */
  const char *financial_report;
  /**
   * Pointer to the array of operating indicators for this item.
   */
  const struct lb_operating_indicator_t *indicators;
  /**
   * Number of indicators in the `indicators` array.
   */
  uintptr_t num_indicators;
} lb_operating_item_t;

/**
 * Collection of operating report items for a security (C-facing FFI type).
 */
typedef struct lb_operating_list_t {
  /**
   * Pointer to the array of operating report items.
   */
  const struct lb_operating_item_t *list;
  /**
   * Number of items in the array.
   */
  uintptr_t num_list;
} lb_operating_list_t;

/**
 * Financial reports serialised as a JSON string (C-facing FFI type).
 */
typedef struct lb_financial_reports_t {
  /**
   * JSON-encoded array of financial report entries.
   */
  const char *list_json;
} lb_financial_reports_t;

/**
 * One consensus estimate detail for a financial metric.
 */
typedef struct lb_consensus_detail_t {
  /**
   * Metric key, e.g. "revenue", "eps".
   */
  const char *key;
  /**
   * Display name.
   */
  const char *name;
  /**
   * Metric description.
   */
  const char *description;
  /**
   * Actual reported value (empty string if not yet released).
   */
  const char *actual;
  /**
   * Consensus estimate value.
   */
  const char *estimate;
  /**
   * Actual minus estimate.
   */
  const char *comp_value;
  /**
   * Beat/miss description.
   */
  const char *comp_desc;
  /**
   * Comparison result code.
   */
  const char *comp;
  /**
   * Whether actual results have been published.
   */
  bool is_released;
} lb_consensus_detail_t;

/**
 * Consensus report for one fiscal period.
 */
typedef struct lb_consensus_report_t {
  /**
   * Fiscal year, e.g. 2025.
   */
  int32_t fiscal_year;
  /**
   * Fiscal period code, e.g. "Q4".
   */
  const char *fiscal_period;
  /**
   * Human-readable period label, e.g. "Q4 FY2025".
   */
  const char *period_text;
  /**
   * Pointer to the array of consensus detail items.
   */
  const struct lb_consensus_detail_t *details;
  /**
   * Number of items in `details`.
   */
  uintptr_t num_details;
} lb_consensus_report_t;

/**
 * Financial consensus response.
 */
typedef struct lb_financial_consensus_t {
  /**
   * Pointer to the array of consensus reports.
   */
  const struct lb_consensus_report_t *list;
  /**
   * Number of reports in `list`.
   */
  uintptr_t num_list;
  /**
   * Index of the most recently released period.
   */
  int32_t current_index;
  /**
   * Reporting currency, e.g. "HKD".
   */
  const char *currency;
  /**
   * Pointer to the array of available period type strings.
   */
  const char *const *opt_periods;
  /**
   * Number of items in `opt_periods`.
   */
  uintptr_t num_opt_periods;
  /**
   * Currently returned period type.
   */
  const char *current_period;
} lb_financial_consensus_t;

/**
 * Historical valuation snapshot for an industry peer.
 */
typedef struct lb_industry_valuation_history_t {
  /**
   * Unix timestamp string.
   */
  const char *date;
  /**
   * Price-to-Earnings ratio.
   */
  const char *pe;
  /**
   * Price-to-Book ratio.
   */
  const char *pb;
  /**
   * Price-to-Sales ratio.
   */
  const char *ps;
} lb_industry_valuation_history_t;

/**
 * Valuation data for one industry peer security.
 */
typedef struct lb_industry_valuation_item_t {
  /**
   * Security symbol.
   */
  const char *symbol;
  /**
   * Company name.
   */
  const char *name;
  /**
   * Reporting currency.
   */
  const char *currency;
  /**
   * Total assets.
   */
  const char *assets;
  /**
   * Book value per share.
   */
  const char *bps;
  /**
   * Earnings per share.
   */
  const char *eps;
  /**
   * Dividends per share.
   */
  const char *dps;
  /**
   * Dividend yield.
   */
  const char *div_yld;
  /**
   * Dividend payout ratio.
   */
  const char *div_payout_ratio;
  /**
   * 5-year average dividends per share.
   */
  const char *five_y_avg_dps;
  /**
   * Current PE ratio.
   */
  const char *pe;
  /**
   * Pointer to the array of historical snapshots.
   */
  const struct lb_industry_valuation_history_t *history;
  /**
   * Number of items in `history`.
   */
  uintptr_t num_history;
} lb_industry_valuation_item_t;

/**
 * List of industry valuation items.
 */
typedef struct lb_industry_valuation_list_t {
  /**
   * Pointer to the array of industry valuation items.
   */
  const struct lb_industry_valuation_item_t *list;
  /**
   * Number of items in `list`.
   */
  uintptr_t num_list;
} lb_industry_valuation_list_t;

/**
 * Distribution statistics for one valuation metric within an industry.
 */
typedef struct lb_valuation_dist_t {
  /**
   * Minimum value in the industry.
   */
  const char *low;
  /**
   * Maximum value in the industry.
   */
  const char *high;
  /**
   * Median value in the industry.
   */
  const char *median;
  /**
   * Current value of the queried security.
   */
  const char *value;
  /**
   * Percentile ranking (0-1 range as string).
   */
  const char *ranking;
  /**
   * Ordinal rank index (1-based).
   */
  const char *rank_index;
  /**
   * Total number of securities in the industry.
   */
  const char *rank_total;
} lb_valuation_dist_t;

/**
 * Industry valuation distribution for PE, PB, PS ratios.
 */
typedef struct lb_industry_valuation_dist_t {
  /**
   * PE ratio distribution, or null if unavailable.
   */
  const struct lb_valuation_dist_t *pe;
  /**
   * PB ratio distribution, or null if unavailable.
   */
  const struct lb_valuation_dist_t *pb;
  /**
   * PS ratio distribution, or null if unavailable.
   */
  const struct lb_valuation_dist_t *ps;
} lb_industry_valuation_dist_t;

/**
 * One executive or board member.
 */
typedef struct lb_professional_t {
  /**
   * Internal wiki person ID.
   */
  const char *id;
  /**
   * Full name.
   */
  const char *name;
  /**
   * Full name in Simplified Chinese.
   */
  const char *name_zhcn;
  /**
   * Full name in English.
   */
  const char *name_en;
  /**
   * Job title.
   */
  const char *title;
  /**
   * Biography text.
   */
  const char *biography;
  /**
   * URL to the person's photo.
   */
  const char *photo;
  /**
   * URL to the wiki profile page.
   */
  const char *wiki_url;
} lb_professional_t;

/**
 * Executives for one security.
 */
typedef struct lb_executive_group_t {
  /**
   * Security symbol.
   */
  const char *symbol;
  /**
   * Link to the company wiki page.
   */
  const char *forward_url;
  /**
   * Total number of executives.
   */
  int32_t total;
  /**
   * Pointer to the array of professionals.
   */
  const struct lb_professional_t *professionals;
  /**
   * Number of items in `professionals`.
   */
  uintptr_t num_professionals;
} lb_executive_group_t;

/**
 * List of executive groups per security.
 */
typedef struct lb_executive_list_t {
  /**
   * Pointer to the array of executive groups.
   */
  const struct lb_executive_group_t *professional_list;
  /**
   * Number of groups in `professional_list`.
   */
  uintptr_t num_professional_list;
} lb_executive_list_t;

/**
 * TTM (trailing twelve months) buyback summary.
 */
typedef struct lb_recent_buybacks_t {
  /**
   * Reporting currency.
   */
  const char *currency;
  /**
   * Net buyback amount TTM.
   */
  const char *net_buyback_ttm;
  /**
   * Net buyback yield TTM.
   */
  const char *net_buyback_yield_ttm;
} lb_recent_buybacks_t;

/**
 * Historical annual buyback data point.
 */
typedef struct lb_buyback_history_item_t {
  /**
   * Fiscal year label, e.g. "FY2024".
   */
  const char *fiscal_year;
  /**
   * Fiscal year date range string.
   */
  const char *fiscal_year_range;
  /**
   * Net buyback amount.
   */
  const char *net_buyback;
  /**
   * Net buyback yield.
   */
  const char *net_buyback_yield;
  /**
   * Year-over-year net buyback growth rate.
   */
  const char *net_buyback_growth_rate;
  /**
   * Reporting currency.
   */
  const char *currency;
} lb_buyback_history_item_t;

/**
 * Buyback payout and cash-flow ratios.
 */
typedef struct lb_buyback_ratios_t {
  /**
   * Net buyback payout ratio.
   */
  const char *net_buyback_payout_ratio;
  /**
   * Net buyback to free cash-flow ratio.
   */
  const char *net_buyback_to_cashflow_ratio;
} lb_buyback_ratios_t;

/**
 * Buyback data response.
 */
typedef struct lb_buyback_data_t {
  /**
   * TTM buyback summary, or null if unavailable.
   */
  const struct lb_recent_buybacks_t *recent_buybacks;
  /**
   * Pointer to the array of historical buyback items.
   */
  const struct lb_buyback_history_item_t *buyback_history;
  /**
   * Number of items in `buyback_history`.
   */
  uintptr_t num_buyback_history;
  /**
   * Pointer to the array of buyback ratios.
   */
  const struct lb_buyback_ratios_t *buyback_ratios;
  /**
   * Number of items in `buyback_ratios`.
   */
  uintptr_t num_buyback_ratios;
} lb_buyback_data_t;

/**
 * A leaf rating indicator with a raw value.
 */
typedef struct lb_rating_leaf_indicator_t {
  /**
   * Indicator display name.
   */
  const char *name;
  /**
   * Formatted value string.
   */
  const char *value;
  /**
   * Value type hint, e.g. "percent".
   */
  const char *value_type;
  /**
   * Score (serialised as JSON string).
   */
  const char *score;
  /**
   * Letter grade.
   */
  const char *letter;
} lb_rating_leaf_indicator_t;

/**
 * A rating indicator node (parent or leaf).
 */
typedef struct lb_rating_indicator_t {
  /**
   * Indicator display name.
   */
  const char *name;
  /**
   * Score (serialised as JSON string).
   */
  const char *score;
  /**
   * Letter grade.
   */
  const char *letter;
} lb_rating_indicator_t;

/**
 * A group of sub-indicators under one category indicator.
 */
typedef struct lb_rating_sub_indicator_group_t {
  /**
   * Parent indicator for this group.
   */
  struct lb_rating_indicator_t indicator;
  /**
   * Pointer to the array of leaf sub-indicators.
   */
  const struct lb_rating_leaf_indicator_t *sub_indicators;
  /**
   * Number of items in `sub_indicators`.
   */
  uintptr_t num_sub_indicators;
} lb_rating_sub_indicator_group_t;

/**
 * One rating category (e.g. growth, profitability).
 */
typedef struct lb_rating_category_t {
  /**
   * Category type code.
   */
  int32_t kind;
  /**
   * Pointer to the array of sub-indicator groups.
   */
  const struct lb_rating_sub_indicator_group_t *sub_indicators;
  /**
   * Number of items in `sub_indicators`.
   */
  uintptr_t num_sub_indicators;
} lb_rating_category_t;

/**
 * Stock ratings response.
 */
typedef struct lb_stock_ratings_t {
  /**
   * Style display name.
   */
  const char *style_txt_name;
  /**
   * Scale display name.
   */
  const char *scale_txt_name;
  /**
   * Report period display text.
   */
  const char *report_period_txt;
  /**
   * Composite score (JSON string).
   */
  const char *multi_score;
  /**
   * Composite score letter grade.
   */
  const char *multi_letter;
  /**
   * Score change vs previous period.
   */
  int32_t multi_score_change;
  /**
   * Industry name.
   */
  const char *industry_name;
  /**
   * Industry rank (JSON string).
   */
  const char *industry_rank;
  /**
   * Total securities in the industry (JSON string).
   */
  const char *industry_total;
  /**
   * Industry mean score (JSON string).
   */
  const char *industry_mean_score;
  /**
   * Industry median score (JSON string).
   */
  const char *industry_median_score;
  /**
   * Pointer to the array of rating categories.
   */
  const struct lb_rating_category_t *ratings;
  /**
   * Number of items in `ratings`.
   */
  uintptr_t num_ratings;
} lb_stock_ratings_t;

/**
 * One business segment item (latest snapshot).
 */
typedef struct lb_business_segment_item_t {
  /**
   * Segment name.
   */
  const char *name;
  /**
   * Percentage of total revenue.
   */
  const char *percent;
} lb_business_segment_item_t;

/**
 * Current business segment breakdown for a security.
 */
typedef struct lb_business_segments_t {
  /**
   * Report date.
   */
  const char *date;
  /**
   * Total revenue.
   */
  const char *total;
  /**
   * Reporting currency.
   */
  const char *currency;
  /**
   * Pointer to business segment items.
   */
  const struct lb_business_segment_item_t *business;
  /**
   * Number of items in `business`.
   */
  uintptr_t num_business;
} lb_business_segments_t;

/**
 * One business/regional segment item in a historical snapshot.
 */
typedef struct lb_business_segment_history_item_t {
  /**
   * Segment name.
   */
  const char *name;
  /**
   * Percentage of total.
   */
  const char *percent;
  /**
   * Absolute value.
   */
  const char *value;
} lb_business_segment_history_item_t;

/**
 * One historical business segments snapshot.
 */
typedef struct lb_business_segments_historical_item_t {
  /**
   * Report date.
   */
  const char *date;
  /**
   * Total revenue.
   */
  const char *total;
  /**
   * Reporting currency.
   */
  const char *currency;
  /**
   * Pointer to business segment breakdown items.
   */
  const struct lb_business_segment_history_item_t *business;
  /**
   * Number of items in `business`.
   */
  uintptr_t num_business;
  /**
   * Pointer to regional breakdown items.
   */
  const struct lb_business_segment_history_item_t *regionals;
  /**
   * Number of items in `regionals`.
   */
  uintptr_t num_regionals;
} lb_business_segments_historical_item_t;

/**
 * Historical business segment breakdowns for a security.
 */
typedef struct lb_business_segments_history_t {
  /**
   * Pointer to historical snapshot items.
   */
  const struct lb_business_segments_historical_item_t *historical;
  /**
   * Number of items in `historical`.
   */
  uintptr_t num_historical;
} lb_business_segments_history_t;

/**
 * One historical institutional rating distribution snapshot.
 */
typedef struct lb_institution_rating_view_item_t {
  /**
   * Date (unix timestamp string).
   */
  const char *date;
  /**
   * Number of "Buy" ratings.
   */
  const char *buy;
  /**
   * Number of "Outperform" ratings.
   */
  const char *over;
  /**
   * Number of "Hold" ratings.
   */
  const char *hold;
  /**
   * Number of "Underperform" ratings.
   */
  const char *under;
  /**
   * Number of "Sell" ratings.
   */
  const char *sell;
  /**
   * Total analyst count.
   */
  const char *total;
} lb_institution_rating_view_item_t;

/**
 * Historical institutional rating views time-series for a security.
 */
typedef struct lb_institution_rating_views_t {
  /**
   * Pointer to rating view items.
   */
  const struct lb_institution_rating_view_item_t *elist;
  /**
   * Number of items in `elist`.
   */
  uintptr_t num_elist;
} lb_institution_rating_views_t;

/**
 * One ranked industry item.
 */
typedef struct lb_industry_rank_item_t {
  /**
   * Industry / sector name.
   */
  const char *name;
  /**
   * Counter ID of the industry.
   */
  const char *counter_id;
  /**
   * Change percentage.
   */
  const char *chg;
  /**
   * Name of the leading stock.
   */
  const char *leading_name;
  /**
   * Ticker of the leading stock.
   */
  const char *leading_ticker;
  /**
   * Change percentage of the leading stock.
   */
  const char *leading_chg;
  /**
   * Value label name.
   */
  const char *value_name;
  /**
   * Value data.
   */
  const char *value_data;
} lb_industry_rank_item_t;

/**
 * A group of ranked industry items.
 */
typedef struct lb_industry_rank_group_t {
  /**
   * Pointer to ranked items.
   */
  const struct lb_industry_rank_item_t *lists;
  /**
   * Number of items in `lists`.
   */
  uintptr_t num_lists;
} lb_industry_rank_group_t;

/**
 * Industry rank response.
 */
typedef struct lb_industry_rank_response_t {
  /**
   * Pointer to grouped rank items.
   */
  const struct lb_industry_rank_group_t *items;
  /**
   * Number of groups in `items`.
   */
  uintptr_t num_items;
} lb_industry_rank_response_t;

/**
 * Top-level industry info in the peers response.
 */
typedef struct lb_industry_peers_top_t {
  /**
   * Industry name.
   */
  const char *name;
  /**
   * Market code.
   */
  const char *market;
} lb_industry_peers_top_t;

/**
 * A node in the industry peer chain (recursive children serialised as JSON).
 */
typedef struct lb_industry_peer_node_t {
  /**
   * Node name.
   */
  const char *name;
  /**
   * Counter ID.
   */
  const char *counter_id;
  /**
   * Number of stocks in this node.
   */
  int32_t stock_num;
  /**
   * Change percentage.
   */
  const char *chg;
  /**
   * Year-to-date change.
   */
  const char *ytd_chg;
  /**
   * Child nodes serialised as a JSON string (may be NULL if empty).
   */
  const char *next_json;
} lb_industry_peer_node_t;

/**
 * Industry peer chain response.
 */
typedef struct lb_industry_peers_response_t {
  /**
   * Top-level industry node info.
   */
  struct lb_industry_peers_top_t top;
  /**
   * Root peer chain node (NULL if absent).
   */
  const struct lb_industry_peer_node_t *chain;
} lb_industry_peers_response_t;

/**
 * A forecast metric in the financial report snapshot.
 */
typedef struct lb_snapshot_forecast_metric_t {
  /**
   * Actual value.
   */
  const char *value;
  /**
   * Year-over-year change.
   */
  const char *yoy;
  /**
   * Beat/miss description.
   */
  const char *cmp_desc;
  /**
   * Consensus estimate value.
   */
  const char *est_value;
} lb_snapshot_forecast_metric_t;

/**
 * A reported metric in the financial report snapshot.
 */
typedef struct lb_snapshot_reported_metric_t {
  /**
   * Actual value.
   */
  const char *value;
  /**
   * Year-over-year change.
   */
  const char *yoy;
} lb_snapshot_reported_metric_t;

/**
 * Financial report snapshot (earnings snapshot) for a security.
 */
typedef struct lb_financial_report_snapshot_t {
  /**
   * Company name.
   */
  const char *name;
  /**
   * Ticker code.
   */
  const char *ticker;
  /**
   * Fiscal period start date.
   */
  const char *fp_start;
  /**
   * Fiscal period end date.
   */
  const char *fp_end;
  /**
   * Reporting currency.
   */
  const char *currency;
  /**
   * Report description.
   */
  const char *report_desc;
  /**
   * Forecast revenue (NULL if absent).
   */
  const struct lb_snapshot_forecast_metric_t *fo_revenue;
  /**
   * Forecast EBIT (NULL if absent).
   */
  const struct lb_snapshot_forecast_metric_t *fo_ebit;
  /**
   * Forecast EPS (NULL if absent).
   */
  const struct lb_snapshot_forecast_metric_t *fo_eps;
  /**
   * Reported revenue (NULL if absent).
   */
  const struct lb_snapshot_reported_metric_t *fr_revenue;
  /**
   * Reported net profit (NULL if absent).
   */
  const struct lb_snapshot_reported_metric_t *fr_profit;
  /**
   * Reported operating cash flow (NULL if absent).
   */
  const struct lb_snapshot_reported_metric_t *fr_operate_cash;
  /**
   * Reported investing cash flow (NULL if absent).
   */
  const struct lb_snapshot_reported_metric_t *fr_invest_cash;
  /**
   * Reported financing cash flow (NULL if absent).
   */
  const struct lb_snapshot_reported_metric_t *fr_finance_cash;
  /**
   * Reported total assets (NULL if absent).
   */
  const struct lb_snapshot_reported_metric_t *fr_total_assets;
  /**
   * Reported total liabilities (NULL if absent).
   */
  const struct lb_snapshot_reported_metric_t *fr_total_liability;
  /**
   * ROE TTM.
   */
  const char *fr_roe_ttm;
  /**
   * Profit margin.
   */
  const char *fr_profit_margin;
  /**
   * Profit margin TTM.
   */
  const char *fr_profit_margin_ttm;
  /**
   * Asset turnover TTM.
   */
  const char *fr_asset_turn_ttm;
  /**
   * Leverage TTM.
   */
  const char *fr_leverage_ttm;
  /**
   * Debt-to-assets ratio.
   */
  const char *fr_debt_assets_ratio;
} lb_financial_report_snapshot_t;

/**
 * A key-value pair carrying calendar data fields.
 */
typedef struct lb_calendar_data_kv_t {
  /**
   * Field key name.
   */
  const char *key;
  /**
   * Display value.
   */
  const char *value;
  /**
   * Type of the value (e.g. "string", "number").
   */
  const char *value_type;
  /**
   * Raw, unformatted value.
   */
  const char *value_raw;
} lb_calendar_data_kv_t;

/**
 * Detailed information for a single calendar event.
 */
typedef struct lb_calendar_event_info_t {
  /**
   * Associated ticker symbol (may be empty).
   */
  const char *symbol;
  /**
   * Market the symbol belongs to (e.g. "US", "HK").
   */
  const char *market;
  /**
   * Human-readable event description / content.
   */
  const char *content;
  /**
   * Display name of the issuer or counter party.
   */
  const char *counter_name;
  /**
   * Classification of the date field (e.g. "announce", "ex-dividend").
   */
  const char *date_type;
  /**
   * Event date string (e.g. "2025-03-15").
   */
  const char *date;
  /**
   * Unique identifier used to retrieve the associated chart.
   */
  const char *chart_uid;
  /**
   * Pointer to an array of extra key-value data pairs for this event.
   */
  const struct lb_calendar_data_kv_t *data_kv;
  /**
   * Number of elements in the `data_kv` array.
   */
  uintptr_t num_data_kv;
  /**
   * Event type identifier string.
   */
  const char *event_type;
  /**
   * Full datetime string for events with a specific time component.
   */
  const char *datetime;
  /**
   * URL of the icon image representing this event.
   */
  const char *icon;
  /**
   * Star / importance rating for the event (higher is more important).
   */
  int32_t star;
  /**
   * Unique event ID.
   */
  const char *id;
  /**
   * Financial-market local time string for this event.
   */
  const char *financial_market_time;
  /**
   * Currency code relevant to the event (e.g. "USD").
   */
  const char *currency;
  /**
   * Activity type classification string.
   */
  const char *activity_type;
} lb_calendar_event_info_t;

/**
 * A group of calendar events that share the same date.
 */
typedef struct lb_calendar_date_group_t {
  /**
   * Date string for this group (e.g. "2025-03-15").
   */
  const char *date;
  /**
   * Total number of events on this date.
   */
  int32_t count;
  /**
   * Pointer to an array of event info items.
   */
  const struct lb_calendar_event_info_t *infos;
  /**
   * Number of elements in the `infos` array.
   */
  uintptr_t num_infos;
} lb_calendar_date_group_t;

/**
 * Response containing calendar events grouped by date.
 */
typedef struct lb_calendar_events_response_t {
  /**
   * Reference date string used for the query (e.g. "2025-03-15").
   */
  const char *date;
  /**
   * Pointer to an array of date-grouped event lists.
   */
  const struct lb_calendar_date_group_t *list;
  /**
   * Number of elements in the `list` array.
   */
  uintptr_t num_list;
  /**
   * Pagination cursor; pass as start to fetch the next page, empty when
   * there are no more pages.
   */
  const char *next_date;
} lb_calendar_events_response_t;

/**
 * A single currency exchange rate entry.
 */
typedef struct lb_exchange_rate_t {
  /**
   * Mid (average) exchange rate between the two currencies.
   */
  double average_rate;
  /**
   * Base currency code (e.g. "USD").
   */
  const char *base_currency;
  /**
   * Bid rate (buy price) for the base currency.
   */
  double bid_rate;
  /**
   * Offer rate (sell price) for the base currency.
   */
  double offer_rate;
  /**
   * Counter currency code (e.g. "HKD").
   */
  const char *other_currency;
} lb_exchange_rate_t;

/**
 * Collection of exchange rate entries.
 */
typedef struct lb_exchange_rates_t {
  /**
   * Pointer to an array of exchange rate items.
   */
  const struct lb_exchange_rate_t *exchanges;
  /**
   * Number of elements in the `exchanges` array.
   */
  uintptr_t num_exchanges;
} lb_exchange_rates_t;

/**
 * A symbol together with all of its associated alert indicators.
 */
typedef struct lb_alert_symbol_group_t {
  /**
   * Full symbol string (e.g. "700.HK").
   */
  const char *symbol;
  /**
   * Short ticker code without market suffix.
   */
  const char *code;
  /**
   * Market the symbol belongs to (e.g. "HK", "US").
   */
  const char *market;
  /**
   * Display name of the security.
   */
  const char *name;
  /**
   * Latest price as a string.
   */
  const char *price;
  /**
   * Absolute price change as a string.
   */
  const char *chg;
  /**
   * Percentage price change as a string.
   */
  const char *p_chg;
  /**
   * Product type string (e.g. "stock", "fund").
   */
  const char *product;
  /**
   * Pointer to an array of alert indicator items for this symbol.
   */
  const struct lb_alert_item_t *indicators;
  /**
   * Number of elements in the `indicators` array.
   */
  uintptr_t num_indicators;
} lb_alert_symbol_group_t;

/**
 * Top-level response containing alert symbol groups.
 */
typedef struct lb_alert_list_t {
  /**
   * Pointer to an array of symbol group items.
   */
  const struct lb_alert_symbol_group_t *lists;
  /**
   * Number of elements in the `lists` array.
   */
  uintptr_t num_lists;
} lb_alert_list_t;

/**
 * DCA (dollar-cost averaging) plan details.
 */
typedef struct lb_dca_plan_t {
  /**
   * Unique plan identifier.
   */
  const char *plan_id;
  /**
   * Current status of the plan.
   */
  enum lb_dca_status_t status;
  /**
   * Stock symbol (e.g. "AAPL.US").
   */
  const char *symbol;
  /**
   * Member ID that owns this plan.
   */
  const char *member_id;
  /**
   * Account identifier (AAID).
   */
  const char *aaid;
  /**
   * Account channel identifier.
   */
  const char *account_channel;
  /**
   * Display-friendly account name.
   */
  const char *display_account;
  /**
   * Market code.
   */
  enum lb_market_t market;
  /**
   * Investment amount per period (decimal string).
   */
  const char *per_invest_amount;
  /**
   * Investment frequency.
   */
  enum lb_dca_frequency_t invest_frequency;
  /**
   * Day of the week on which investment is executed (if weekly frequency).
   */
  const char *invest_day_of_week;
  /**
   * Day of the month on which investment is executed (if monthly frequency).
   */
  const char *invest_day_of_month;
  /**
   * Whether margin financing is allowed for this plan.
   */
  bool allow_margin_finance;
  /**
   * After-hours trading setting.
   */
  const char *alter_hours;
  /**
   * Plan creation timestamp (ISO 8601 string).
   */
  const char *created_at;
  /**
   * Plan last-updated timestamp (ISO 8601 string).
   */
  const char *updated_at;
  /**
   * Next scheduled trading date (ISO 8601 date string).
   */
  const char *next_trd_date;
  /**
   * Stock display name.
   */
  const char *stock_name;
  /**
   * Cumulative invested amount (decimal string).
   */
  const char *cum_amount;
  /**
   * Total number of investment executions to date.
   */
  int64_t issue_number;
  /**
   * Average cost per share across all executions (decimal string).
   */
  const char *average_cost;
  /**
   * Cumulative profit/loss (decimal string).
   */
  const char *cum_profit;
} lb_dca_plan_t;

/**
 * List of DCA plans.
 */
typedef struct lb_dca_list_t {
  /**
   * Pointer to the array of DCA plans.
   */
  const struct lb_dca_plan_t *plans;
  /**
   * Number of plans in the array.
   */
  uintptr_t num_plans;
} lb_dca_list_t;

/**
 * Aggregate statistics across all DCA plans for a user.
 */
typedef struct lb_dca_stats_t {
  /**
   * Number of currently active plans (decimal string).
   */
  const char *active_count;
  /**
   * Number of finished plans (decimal string).
   */
  const char *finished_count;
  /**
   * Number of suspended plans (decimal string).
   */
  const char *suspended_count;
  /**
   * Pointer to the array of nearest upcoming plans.
   */
  const struct lb_dca_plan_t *nearest_plans;
  /**
   * Number of plans in the nearest_plans array.
   */
  uintptr_t num_nearest_plans;
  /**
   * Days remaining until the next scheduled investment (decimal string).
   */
  const char *rest_days;
  /**
   * Total invested amount across all plans (decimal string).
   */
  const char *total_amount;
  /**
   * Total profit/loss across all plans (decimal string).
   */
  const char *total_profit;
} lb_dca_stats_t;

/**
 * DCA support information for a single security.
 */
typedef struct lb_dca_support_info_t {
  /**
   * Stock symbol (e.g. "AAPL.US").
   */
  const char *symbol;
  /**
   * Whether regular (recurring) saving/investment is supported for this
   * symbol.
   */
  bool support_regular_saving;
} lb_dca_support_info_t;

/**
 * List of DCA support information entries.
 */
typedef struct lb_dca_support_list_t {
  /**
   * Pointer to the array of support info entries.
   */
  const struct lb_dca_support_info_t *infos;
  /**
   * Number of entries in the array.
   */
  uintptr_t num_infos;
} lb_dca_support_list_t;

/**
 * Result returned by DCA create and update operations.
 */
typedef struct lb_dca_create_result_t {
  /**
   * The plan ID of the created or updated DCA plan.
   */
  const char *plan_id;
} lb_dca_create_result_t;

/**
 * Result returned by DCA calc_date operation.
 */
typedef struct lb_dca_calc_date_result_t {
  /**
   * Next projected trade date (unix timestamp string).
   */
  const char *trade_date;
} lb_dca_calc_date_result_t;

/**
 * One DCA execution history record.
 */
typedef struct lb_dca_history_record_t {
  /**
   * Execution timestamp (ISO 8601 string).
   */
  const char *created_at;
  /**
   * Associated order ID.
   */
  const char *order_id;
  /**
   * Execution status string.
   */
  const char *status;
  /**
   * Action type string.
   */
  const char *action;
  /**
   * Order type string.
   */
  const char *order_type;
  /**
   * Executed quantity (decimal string, may be empty).
   */
  const char *executed_qty;
  /**
   * Executed price (decimal string, may be empty).
   */
  const char *executed_price;
  /**
   * Executed amount (decimal string, may be empty).
   */
  const char *executed_amount;
  /**
   * Rejection reason (empty string if not rejected).
   */
  const char *rejected_reason;
  /**
   * Security symbol.
   */
  const char *symbol;
} lb_dca_history_record_t;

/**
 * Paginated DCA execution history response.
 */
typedef struct lb_dca_history_response_t {
  /**
   * Pointer to the array of history records.
   */
  const struct lb_dca_history_record_t *records;
  /**
   * Number of records in the array.
   */
  uintptr_t num_records;
  /**
   * Whether more records exist.
   */
  bool has_more;
} lb_dca_history_response_t;

/**
 * P&L summary for one asset category.
 */
typedef struct lb_profit_summary_info_t {
  /**
   * Asset type.
   */
  enum lb_asset_type_t asset_type;
  /**
   * Security with the maximum profit.
   */
  const char *profit_max;
  /**
   * Name of the max-profit security.
   */
  const char *profit_max_name;
  /**
   * Security with the maximum loss.
   */
  const char *loss_max;
  /**
   * Name of the max-loss security.
   */
  const char *loss_max_name;
} lb_profit_summary_info_t;

/**
 * P&L breakdown by asset type.
 */
typedef struct lb_profit_summary_breakdown_t {
  /**
   * Stock P&L.
   */
  const char *stock;
  /**
   * Fund P&L.
   */
  const char *fund;
  /**
   * Crypto P&L.
   */
  const char *crypto;
  /**
   * Money market fund P&L.
   */
  const char *mmf;
  /**
   * Other P&L.
   */
  const char *other;
  /**
   * Cumulative transaction amount.
   */
  const char *cumulative_transaction_amount;
  /**
   * Total number of orders.
   */
  const char *trade_order_num;
  /**
   * Total number of traded securities.
   */
  const char *trade_stock_num;
  /**
   * IPO P&L.
   */
  const char *ipo;
  /**
   * IPO hits.
   */
  int32_t ipo_hit;
  /**
   * IPO subscriptions.
   */
  int32_t ipo_subscription;
  /**
   * Pointer to array of per-category summary info.
   */
  const struct lb_profit_summary_info_t *summary_info;
  /**
   * Number of items in `summary_info`.
   */
  uintptr_t num_summary_info;
} lb_profit_summary_breakdown_t;

/**
 * Account-level P&L summary.
 */
typedef struct lb_profit_analysis_summary_t {
  /**
   * Account currency.
   */
  const char *currency;
  /**
   * Current total asset value.
   */
  const char *current_total_asset;
  /**
   * Query start date string.
   */
  const char *start_date;
  /**
   * Query end date string.
   */
  const char *end_date;
  /**
   * Start time (unix timestamp string).
   */
  const char *start_time;
  /**
   * End time (unix timestamp string).
   */
  const char *end_time;
  /**
   * Ending asset value.
   */
  const char *ending_asset_value;
  /**
   * Initial asset value.
   */
  const char *initial_asset_value;
  /**
   * Total invested amount.
   */
  const char *invest_amount;
  /**
   * Whether any trades occurred.
   */
  bool is_traded;
  /**
   * Total profit/loss.
   */
  const char *sum_profit;
  /**
   * Total profit/loss rate.
   */
  const char *sum_profit_rate;
  /**
   * Per-asset-type breakdown (inline).
   */
  struct lb_profit_summary_breakdown_t profits;
} lb_profit_analysis_summary_t;

/**
 * P&L for one security.
 */
typedef struct lb_profit_analysis_item_t {
  /**
   * Security name.
   */
  const char *name;
  /**
   * Market.
   */
  const char *market;
  /**
   * Whether still holding.
   */
  bool is_holding;
  /**
   * Profit/loss amount.
   */
  const char *profit;
  /**
   * Profit/loss rate.
   */
  const char *profit_rate;
  /**
   * Number of completed trades.
   */
  int64_t clearance_times;
  /**
   * Asset type.
   */
  enum lb_asset_type_t item_type;
  /**
   * Currency.
   */
  const char *currency;
  /**
   * Security symbol.
   */
  const char *symbol;
  /**
   * Holding period display string.
   */
  const char *holding_period;
  /**
   * Ticker code.
   */
  const char *security_code;
  /**
   * ISIN (for funds).
   */
  const char *isin;
  /**
   * Underlying stock P&L.
   */
  const char *underlying_profit;
  /**
   * Derivatives P&L.
   */
  const char *derivatives_profit;
  /**
   * P&L in order currency.
   */
  const char *order_profit;
} lb_profit_analysis_item_t;

/**
 * Per-security P&L breakdown.
 */
typedef struct lb_profit_analysis_sublist_t {
  /**
   * Start time (unix timestamp string).
   */
  const char *start;
  /**
   * End time (unix timestamp string).
   */
  const char *end;
  /**
   * Start date string.
   */
  const char *start_date;
  /**
   * End date string.
   */
  const char *end_date;
  /**
   * Last updated time (unix timestamp string).
   */
  const char *updated_at;
  /**
   * Last updated date string.
   */
  const char *updated_date;
  /**
   * Pointer to array of per-security items.
   */
  const struct lb_profit_analysis_item_t *items;
  /**
   * Number of items.
   */
  uintptr_t num_items;
} lb_profit_analysis_sublist_t;

/**
 * Combined portfolio P&L analysis response.
 */
typedef struct lb_profit_analysis_t {
  /**
   * Account-level summary (inline).
   */
  struct lb_profit_analysis_summary_t summary;
  /**
   * Per-security breakdown (inline).
   */
  struct lb_profit_analysis_sublist_t sublist;
} lb_profit_analysis_t;

/**
 * One security entry in a by-market P&L response.
 */
typedef struct lb_profit_analysis_by_market_item_t {
  /**
   * Security symbol (ticker code).
   */
  const char *code;
  /**
   * Security name.
   */
  const char *name;
  /**
   * Market, e.g. "HK", "US".
   */
  const char *market;
  /**
   * Profit/loss amount.
   */
  const char *profit;
} lb_profit_analysis_by_market_item_t;

/**
 * P&L grouped by market response.
 */
typedef struct lb_profit_analysis_by_market_t {
  /**
   * Total P&L across all returned items.
   */
  const char *profit;
  /**
   * Whether more pages are available.
   */
  bool has_more;
  /**
   * Pointer to array of per-security items.
   */
  const struct lb_profit_analysis_by_market_item_t *stock_items;
  /**
   * Number of items in `stock_items`.
   */
  uintptr_t num_stock_items;
} lb_profit_analysis_by_market_t;

/**
 * One profit-analysis flow record.
 */
typedef struct lb_flow_item_t {
  /**
   * Execution date string, e.g. "2024-01-15".
   */
  const char *executed_date;
  /**
   * Execution timestamp (serialised as JSON string).
   */
  const char *executed_timestamp;
  /**
   * Security code / ticker.
   */
  const char *code;
  /**
   * Direction of the flow.
   */
  enum lb_flow_direction_t direction;
  /**
   * Executed quantity.
   */
  const char *executed_quantity;
  /**
   * Executed price.
   */
  const char *executed_price;
  /**
   * Executed cost.
   */
  const char *executed_cost;
  /**
   * Human-readable description.
   */
  const char *describe;
} lb_flow_item_t;

/**
 * Paginated list of profit-analysis flow records.
 */
typedef struct lb_profit_analysis_flows_t {
  /**
   * Pointer to array of flow items.
   */
  const struct lb_flow_item_t *flows_list;
  /**
   * Number of items in `flows_list`.
   */
  uintptr_t num_flows_list;
  /**
   * Whether there are more pages.
   */
  bool has_more;
} lb_profit_analysis_flows_t;

/**
 * One P&L detail line item (credit, debit, or fee).
 */
typedef struct lb_profit_detail_entry_t {
  /**
   * Description.
   */
  const char *describe;
  /**
   * Amount.
   */
  const char *amount;
} lb_profit_detail_entry_t;

/**
 * Detailed P&L breakdown for one asset class.
 */
typedef struct lb_profit_details_t {
  /**
   * Current holding market value.
   */
  const char *holding_value;
  /**
   * Total profit/loss.
   */
  const char *profit;
  /**
   * Cumulative credited amount.
   */
  const char *cumulative_credited_amount;
  /**
   * Pointer to array of credit detail entries.
   */
  const struct lb_profit_detail_entry_t *credited_details;
  /**
   * Number of items in `credited_details`.
   */
  uintptr_t num_credited_details;
  /**
   * Cumulative debited amount.
   */
  const char *cumulative_debited_amount;
  /**
   * Pointer to array of debit detail entries.
   */
  const struct lb_profit_detail_entry_t *debited_details;
  /**
   * Number of items in `debited_details`.
   */
  uintptr_t num_debited_details;
  /**
   * Cumulative fee amount.
   */
  const char *cumulative_fee_amount;
  /**
   * Pointer to array of fee detail entries.
   */
  const struct lb_profit_detail_entry_t *fee_details;
  /**
   * Number of items in `fee_details`.
   */
  uintptr_t num_fee_details;
  /**
   * Short position holding value.
   */
  const char *short_holding_value;
  /**
   * Long position holding value.
   */
  const char *long_holding_value;
  /**
   * Opening position market value at period start.
   */
  const char *holding_value_at_beginning;
  /**
   * Closing position market value at period end.
   */
  const char *holding_value_at_ending;
} lb_profit_details_t;

/**
 * Detailed P&L for one security.
 */
typedef struct lb_profit_analysis_detail_t {
  /**
   * Total profit/loss.
   */
  const char *profit;
  /**
   * Underlying stock P&L details (inline).
   */
  struct lb_profit_details_t underlying_details;
  /**
   * Derivative P&L details (inline).
   */
  struct lb_profit_details_t derivative_pnl_details;
  /**
   * Security name.
   */
  const char *name;
  /**
   * Last updated time (unix timestamp string).
   */
  const char *updated_at;
  /**
   * Last updated date string.
   */
  const char *updated_date;
  /**
   * Currency.
   */
  const char *currency;
  /**
   * Default detail tab: 0=underlying, 1=derivative.
   */
  int32_t default_tag;
  /**
   * Query start time (unix timestamp string).
   */
  const char *start;
  /**
   * Query end time (unix timestamp string).
   */
  const char *end;
  /**
   * Query start date string.
   */
  const char *start_date;
  /**
   * Query end date string.
   */
  const char *end_date;
} lb_profit_analysis_detail_t;

/**
 * A stock entry within a sharelist.
 */
typedef struct lb_sharelist_stock_t {
  /**
   * Stock symbol (e.g. "AAPL.US").
   */
  const char *symbol;
  /**
   * Display name of the stock.
   */
  const char *name;
  /**
   * Market code (e.g. "US", "HK").
   */
  const char *market;
  /**
   * Stock code (ticker without market suffix).
   */
  const char *code;
  /**
   * Short introduction or description of the stock.
   */
  const char *intro;
  /**
   * Category of unread change log entries for this stock.
   */
  const char *unread_change_log_category;
  /**
   * Price change amount (decimal string); null if not available.
   */
  const char *change;
  /**
   * Last traded price (decimal string); null if not available.
   */
  const char *last_done;
  /**
   * Trade status code; valid only when `has_trade_status` is true.
   */
  int32_t trade_status;
  /**
   * Whether `trade_status` contains a valid value.
   */
  bool has_trade_status;
} lb_sharelist_stock_t;

/**
 * Access/permission scopes associated with a sharelist.
 */
typedef struct lb_sharelist_scopes_t {
  /**
   * Whether the current user is subscribed to this sharelist.
   */
  bool subscription;
  /**
   * Whether this sharelist was created by the current authenticated user.
   */
  bool is_self;
} lb_sharelist_scopes_t;

/**
 * Summary information about a sharelist.
 */
typedef struct lb_sharelist_info_t {
  /**
   * Unique sharelist identifier.
   */
  int64_t id;
  /**
   * Display name of the sharelist.
   */
  const char *name;
  /**
   * Human-readable description of the sharelist.
   */
  const char *description;
  /**
   * URL of the cover image for the sharelist.
   */
  const char *cover;
  /**
   * Total number of subscribers.
   */
  int64_t subscribers_count;
  /**
   * Creation timestamp (Unix seconds).
   */
  int64_t created_at;
  /**
   * Last-edited timestamp (Unix seconds).
   */
  int64_t edited_at;
  /**
   * Year-to-date price change percentage (decimal string).
   */
  const char *this_year_chg;
  /**
   * Creator information serialised as a JSON string.
   */
  const char *creator;
  /**
   * Pointer to the array of stocks in this sharelist.
   */
  const struct lb_sharelist_stock_t *stocks;
  /**
   * Number of stocks in the array.
   */
  uintptr_t num_stocks;
  /**
   * Whether the current user has subscribed to this sharelist.
   */
  bool subscribed;
  /**
   * Overall price change percentage of the sharelist (decimal string).
   */
  const char *chg;
  /**
   * Type code of the sharelist (e.g. 0 = normal, 1 = industry, …).
   */
  int32_t sharelist_type;
  /**
   * Industry code associated with the sharelist (if applicable).
   */
  const char *industry_code;
} lb_sharelist_info_t;

/**
 * Paginated list of sharelists with subscription information.
 */
typedef struct lb_sharelist_list_t {
  /**
   * Pointer to the array of all sharelists.
   */
  const struct lb_sharelist_info_t *sharelists;
  /**
   * Number of sharelists in the array.
   */
  uintptr_t num_sharelists;
  /**
   * Pointer to the array of sharelists the current user has subscribed to.
   */
  const struct lb_sharelist_info_t *subscribed_sharelists;
  /**
   * Number of subscribed sharelists in the array.
   */
  uintptr_t num_subscribed_sharelists;
  /**
   * Pagination cursor for fetching the next page of results.
   */
  const char *tail_mark;
} lb_sharelist_list_t;

/**
 * Full detail of a sharelist including access scopes.
 */
typedef struct lb_sharelist_detail_t {
  /**
   * Sharelist summary information.
   */
  struct lb_sharelist_info_t sharelist;
  /**
   * Access/permission scopes for the current user relative to this
   * sharelist.
   */
  struct lb_sharelist_scopes_t scopes;
} lb_sharelist_detail_t;

/**
 * One short-position record, unified for US and HK markets.
 */
typedef struct lb_short_positions_item_t {
  /**
   * Trading date in RFC 3339 format
   */
  const char *timestamp;
  /**
   * Short ratio
   */
  const char *rate;
  /**
   * Closing price
   */
  const char *close;
  /**
   * [US] Number of short shares outstanding
   */
  const char *current_shares_short;
  /**
   * [US] Average daily share volume
   */
  const char *avg_daily_share_volume;
  /**
   * [US] Days-to-cover ratio
   */
  const char *days_to_cover;
  /**
   * [HK] Short sale amount (HKD)
   */
  const char *amount;
  /**
   * [HK] Short position balance
   */
  const char *balance;
  /**
   * [HK] Closing price (HK naming)
   */
  const char *cost;
} lb_short_positions_item_t;

/**
 * Short positions / interest response (HK or US).
 */
typedef struct lb_short_positions_response_t {
  /**
   * Pointer to the array of short position items
   */
  const struct lb_short_positions_item_t *data;
  /**
   * Number of items in `data`
   */
  uintptr_t num_data;
} lb_short_positions_response_t;

/**
 * One short-trade record, unified for US and HK markets.
 */
typedef struct lb_short_trades_item_t {
  /**
   * Trading date in RFC 3339 format
   */
  const char *timestamp;
  /**
   * Short ratio
   */
  const char *rate;
  /**
   * Closing price
   */
  const char *close;
  /**
   * [US] NASDAQ short sale volume
   */
  const char *nus_amount;
  /**
   * [US] NYSE short sale volume
   */
  const char *ny_amount;
  /**
   * [US] Total short amount
   */
  const char *total_amount;
  /**
   * [HK] Short sale turnover amount (HKD)
   */
  const char *amount;
  /**
   * [HK] Short position balance
   */
  const char *balance;
} lb_short_trades_item_t;

/**
 * Short trade records response (HK or US).
 */
typedef struct lb_short_trades_response_t {
  /**
   * Pointer to the array of short trade items
   */
  const struct lb_short_trades_item_t *data;
  /**
   * Number of items in `data`
   */
  uintptr_t num_data;
} lb_short_trades_response_t;

/**
 * Option volume statistics (call and put totals)
 */
typedef struct lb_option_volume_stats_t {
  /**
   * Call option volume (formatted string)
   */
  const char *c;
  /**
   * Put option volume (formatted string)
   */
  const char *p;
} lb_option_volume_stats_t;

/**
 * Daily option volume statistics for a single security
 */
typedef struct lb_option_volume_daily_stat_t {
  /**
   * Security code
   */
  const char *symbol;
  /**
   * Date of the record (formatted string)
   */
  const char *timestamp;
  /**
   * Total option volume (calls + puts, formatted string)
   */
  const char *total_volume;
  /**
   * Total put option volume (formatted string)
   */
  const char *total_put_volume;
  /**
   * Total call option volume (formatted string)
   */
  const char *total_call_volume;
  /**
   * Put-to-call volume ratio (formatted string)
   */
  const char *put_call_volume_ratio;
  /**
   * Total open interest across all options (formatted string)
   */
  const char *total_open_interest;
  /**
   * Total put open interest (formatted string)
   */
  const char *total_put_open_interest;
  /**
   * Total call open interest (formatted string)
   */
  const char *total_call_open_interest;
  /**
   * Put-to-call open interest ratio (formatted string)
   */
  const char *put_call_open_interest_ratio;
} lb_option_volume_daily_stat_t;

/**
 * Collection of daily option volume statistics
 */
typedef struct lb_option_volume_daily_t {
  /**
   * Pointer to array of daily option volume stat records
   */
  const struct lb_option_volume_daily_stat_t *stats;
  /**
   * Number of elements in the array.
   */
  uintptr_t num_stats;
} lb_option_volume_daily_t;

/**
 * Localized name entry (locale → name)
 */
typedef struct lb_locale_name_t {
  /**
   * Locale (e.g. `zh-CN`)
   */
  const char *locale;
  /**
   * Localized name
   */
  const char *name;
} lb_locale_name_t;

/**
 * Holding detail of an ETF asset allocation element (holdings only)
 */
typedef struct lb_holding_detail_t {
  /**
   * Industry ID
   */
  const char *industry_id;
  /**
   * Industry name
   */
  const char *industry_name;
  /**
   * Index counter ID (e.g. `BK/US/CP99000`)
   */
  const char *index;
  /**
   * Index name
   */
  const char *index_name;
  /**
   * Holding type (e.g. `E` for stock)
   */
  const char *holding_type;
  /**
   * Holding type name
   */
  const char *holding_type_name;
} lb_holding_detail_t;

/**
 * One element of an ETF asset allocation group
 */
typedef struct lb_asset_allocation_item_t {
  /**
   * Element name
   */
  const char *name;
  /**
   * Security code (holdings only, e.g. `NVDA`)
   */
  const char *code;
  /**
   * Position ratio (e.g. `0.0861114`)
   */
  const char *position_ratio;
  /**
   * Security symbol (holdings only, e.g. `NVDA.US`)
   */
  const char *symbol;
  /**
   * Pointer to array of localized name entries
   */
  const struct lb_locale_name_t *name_locales;
  /**
   * Number of elements in the localized name array
   */
  uintptr_t num_name_locales;
  /**
   * Holding detail (holdings only, maybe null)
   */
  const struct lb_holding_detail_t *holding_detail;
} lb_asset_allocation_item_t;

/**
 * One ETF asset allocation group (grouped by element type)
 */
typedef struct lb_asset_allocation_group_t {
  /**
   * Report date (e.g. `20260601`)
   */
  const char *report_date;
  /**
   * Element type of this group
   */
  enum lb_element_type_t asset_type;
  /**
   * Pointer to array of elements
   */
  const struct lb_asset_allocation_item_t *lists;
  /**
   * Number of elements in the array
   */
  uintptr_t num_lists;
} lb_asset_allocation_group_t;

/**
 * ETF asset allocation response
 */
typedef struct lb_asset_allocation_response_t {
  /**
   * Pointer to array of asset allocation groups
   */
  const struct lb_asset_allocation_group_t *info;
  /**
   * Number of elements in the array
   */
  uintptr_t num_info;
} lb_asset_allocation_response_t;

/**
 * Top-shareholder list response. `data` is a NUL-terminated JSON string.
 */
typedef struct lb_shareholder_top_response_t {
  /**
   * Raw top-shareholder data as a JSON string
   */
  const char *data;
} lb_shareholder_top_response_t;

/**
 * Shareholder detail response. `data` is a NUL-terminated JSON string.
 */
typedef struct lb_shareholder_detail_response_t {
  /**
   * Raw shareholder detail data as a JSON string
   */
  const char *data;
} lb_shareholder_detail_response_t;

/**
 * One historical valuation data point.
 */
typedef struct lb_valuation_history_point_t {
  /**
   * Date in RFC 3339 format
   */
  const char *date;
  /**
   * P/E ratio
   */
  const char *pe;
  /**
   * P/B ratio
   */
  const char *pb;
  /**
   * P/S ratio
   */
  const char *ps;
} lb_valuation_history_point_t;

/**
 * One security's valuation comparison item.
 */
typedef struct lb_valuation_comparison_item_t {
  /**
   * Symbol, e.g. "AAPL.US"
   */
  const char *symbol;
  /**
   * Security name
   */
  const char *name;
  /**
   * Currency
   */
  const char *currency;
  /**
   * Market capitalisation
   */
  const char *market_value;
  /**
   * Latest closing price
   */
  const char *price_close;
  /**
   * P/E ratio
   */
  const char *pe;
  /**
   * P/B ratio
   */
  const char *pb;
  /**
   * P/S ratio
   */
  const char *ps;
  /**
   * Return on equity
   */
  const char *roe;
  /**
   * Earnings per share
   */
  const char *eps;
  /**
   * Book value per share
   */
  const char *bps;
  /**
   * Dividends per share
   */
  const char *dps;
  /**
   * Dividend yield
   */
  const char *div_yld;
  /**
   * Total assets
   */
  const char *assets;
  /**
   * Pointer to the array of historical valuation points
   */
  const struct lb_valuation_history_point_t *history;
  /**
   * Number of items in `history`
   */
  uintptr_t num_history;
} lb_valuation_comparison_item_t;

/**
 * Valuation comparison response.
 */
typedef struct lb_valuation_comparison_response_t {
  /**
   * Pointer to the array of valuation comparison items
   */
  const struct lb_valuation_comparison_item_t *list;
  /**
   * Number of items in `list`
   */
  uintptr_t num_list;
} lb_valuation_comparison_response_t;

/**
 * Stock information within a top-movers event.
 */
typedef struct lb_top_movers_stock_t {
  /**
   * Symbol, e.g. "TSLA.US"
   */
  const char *symbol;
  /**
   * Ticker code
   */
  const char *code;
  /**
   * Security name
   */
  const char *name;
  /**
   * Full name
   */
  const char *full_name;
  /**
   * Price change (decimal ratio)
   */
  const char *change;
  /**
   * Latest price
   */
  const char *last_done;
  /**
   * Market code
   */
  const char *market;
  /**
   * Logo URL
   */
  const char *logo;
  /**
   * Labels / tags
   */
  const char *const *labels;
  /**
   * Number of items in `labels`
   */
  uintptr_t num_labels;
} lb_top_movers_stock_t;

/**
 * One top-movers event entry.
 */
typedef struct lb_top_movers_event_t {
  /**
   * Event time (RFC 3339)
   */
  const char *timestamp;
  /**
   * Alert reason description
   */
  const char *alert_reason;
  /**
   * Alert type code
   */
  int64_t alert_type;
  /**
   * Stock information
   */
  struct lb_top_movers_stock_t stock;
  /**
   * Associated news post as a JSON string (may be null)
   */
  const char *post;
} lb_top_movers_event_t;

/**
 * Top movers response.
 */
typedef struct lb_top_movers_response_t {
  /**
   * Pointer to the array of top-mover events
   */
  const struct lb_top_movers_event_t *events;
  /**
   * Number of items in `events`
   */
  uintptr_t num_events;
  /**
   * Pagination cursor as a JSON string
   */
  const char *next_params;
} lb_top_movers_response_t;

/**
 * Rank categories response. `data` is a NUL-terminated JSON string.
 */
typedef struct lb_rank_categories_response_t {
  /**
   * Raw rank categories data as a JSON string
   */
  const char *data;
} lb_rank_categories_response_t;

/**
 * One ranked security item.
 */
typedef struct lb_rank_list_item_t {
  /**
   * Symbol, e.g. "MU.US"
   */
  const char *symbol;
  /**
   * Ticker code
   */
  const char *code;
  /**
   * Security name
   */
  const char *name;
  /**
   * Latest price
   */
  const char *last_done;
  /**
   * Price change ratio (decimal)
   */
  const char *chg;
  /**
   * Absolute price change
   */
  const char *change;
  /**
   * Net inflow
   */
  const char *inflow;
  /**
   * Market cap
   */
  const char *market_cap;
  /**
   * Industry name
   */
  const char *industry;
  /**
   * Pre/post market price
   */
  const char *pre_post_price;
  /**
   * Pre/post market change
   */
  const char *pre_post_chg;
  /**
   * Amplitude
   */
  const char *amplitude;
  /**
   * 5-day change
   */
  const char *five_day_chg;
  /**
   * Turnover rate
   */
  const char *turnover_rate;
  /**
   * Volume ratio
   */
  const char *volume_rate;
  /**
   * P/B ratio (TTM)
   */
  const char *pb_ttm;
} lb_rank_list_item_t;

/**
 * Rank list response.
 */
typedef struct lb_rank_list_response_t {
  /**
   * Whether the response is delayed / BMP data
   */
  bool bmp;
  /**
   * Pointer to the array of ranked security items
   */
  const struct lb_rank_list_item_t *lists;
  /**
   * Number of items in `lists`
   */
  uintptr_t num_lists;
} lb_rank_list_response_t;

/**
 * Recommended screener strategies response. `data` is a JSON string.
 */
typedef struct lb_screener_recommend_strategies_response_t {
  const char *data;
} lb_screener_recommend_strategies_response_t;

/**
 * User screener strategies response. `data` is a JSON string.
 */
typedef struct lb_screener_user_strategies_response_t {
  const char *data;
} lb_screener_user_strategies_response_t;

/**
 * Single screener strategy response. `data` is a JSON string.
 */
typedef struct lb_screener_strategy_response_t {
  const char *data;
} lb_screener_strategy_response_t;

/**
 * Screener search results response. `data` is a JSON string.
 */
typedef struct lb_screener_search_response_t {
  const char *data;
} lb_screener_search_response_t;

/**
 * Screener indicator definitions response. `data` is a JSON string.
 */
typedef struct lb_screener_indicators_response_t {
  const char *data;
} lb_screener_indicators_response_t;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

const struct lb_alert_context_t *lb_alert_context_new(const struct lb_config_t *config);

void lb_alert_context_retain(const struct lb_alert_context_t *ctx);

void lb_alert_context_release(const struct lb_alert_context_t *ctx);

/**
 * List all price alerts. Returns `CAlertList`.
 */
void lb_alert_context_list(const struct lb_alert_context_t *ctx,
                           lb_async_callback_t callback,
                           void *userdata);

/**
 * Add a price alert.
 */
void lb_alert_context_add(const struct lb_alert_context_t *ctx,
                          const char *symbol,
                          enum lb_alert_condition_t condition,
                          const char *trigger_value,
                          enum lb_alert_frequency_t frequency,
                          lb_async_callback_t callback,
                          void *userdata);

/**
 * Update (enable or disable) a price alert.
 *
 * `item` must point to a valid [`CAlertItem`] obtained from
 * [`lb_alert_context_list`]. Set `enabled` to `true` to re-enable or
 * `false` to disable. All fields of `item` are read before the function
 * returns, so the pointer only needs to be valid for the duration of
 * the call.
 */
void lb_alert_context_update(const struct lb_alert_context_t *ctx,
                             const struct lb_alert_item_t *item,
                             lb_async_callback_t callback,
                             void *userdata);

/**
 * Delete price alerts. alert_ids: array of alert ID strings, num_ids: count.
 */
void lb_alert_context_delete(const struct lb_alert_context_t *ctx,
                             const char *const *alert_ids,
                             uintptr_t num_ids,
                             lb_async_callback_t callback,
                             void *userdata);

/**
 * Create a new `AssetContext`
 *
 * @param config  Config object
 * @return A new asset context
 */
const struct CAssetContext *lb_asset_context_new(const struct lb_config_t *config);

/**
 * Retain the asset context (increment reference count)
 */
void lb_asset_context_retain(const struct CAssetContext *ctx);

/**
 * Release the asset context (decrement reference count)
 */
void lb_asset_context_release(const struct CAssetContext *ctx);

/**
 * Get statement data list
 *
 * @param ctx             Asset context
 * @param statement_type  1 = daily, 2 = monthly
 * @param start_date      Start date for pagination (0 = default)
 * @param limit           Number of results (0 = default 20)
 * @param callback        Async callback
 * @param userdata        User data passed to the callback
 */
void lb_asset_context_statements(const struct CAssetContext *ctx,
                                 int32_t statement_type,
                                 int32_t start_date,
                                 int32_t limit,
                                 lb_async_callback_t callback,
                                 void *userdata);

/**
 * Get statement data download URL
 *
 * @param ctx       Asset context
 * @param file_key  File key from the list response
 * @param callback  Async callback
 * @param userdata  User data passed to the callback
 */
void lb_asset_context_download_url(const struct CAssetContext *ctx,
                                   const char *file_key,
                                   lb_async_callback_t callback,
                                   void *userdata);

const struct lb_calendar_context_t *lb_calendar_context_new(const struct lb_config_t *config);

void lb_calendar_context_retain(const struct lb_calendar_context_t *ctx);

void lb_calendar_context_release(const struct lb_calendar_context_t *ctx);

/**
 * Get financial calendar events.
 */
void lb_calendar_context_finance_calendar(const struct lb_calendar_context_t *ctx,
                                          enum lb_calendar_category_t category,
                                          const char *start,
                                          const char *end,
                                          const char *market,
                                          lb_async_callback_t callback,
                                          void *userdata);

/**
 * Create a new `Config` using API Key authentication
 *
 * Optional environment variables are read automatically:
 * `LONGBRIDGE_HTTP_URL`, `LONGBRIDGE_LANGUAGE`, `LONGBRIDGE_QUOTE_WS_URL`,
 * `LONGBRIDGE_TRADE_WS_URL`, `LONGBRIDGE_ENABLE_OVERNIGHT`,
 * `LONGBRIDGE_PUSH_CANDLESTICK_MODE`, `LONGBRIDGE_PRINT_QUOTE_PACKAGES`,
 * `LONGBRIDGE_LOG_PATH`.  Use the corresponding `lb_config_set_*` functions
 * to override any of these values after construction.
 *
 * @param app_key       App key
 * @param app_secret    App secret
 * @param access_token  Access token
 */
struct lb_config_t *lb_config_from_apikey(const char *app_key,
                                          const char *app_secret,
                                          const char *access_token);

/**
 * Create a new `Config` from environment variables (API Key mode)
 *
 * It first reads the `.env` file in the current directory.
 *
 * Variables: `LONGBRIDGE_APP_KEY`, `LONGBRIDGE_APP_SECRET`,
 * `LONGBRIDGE_ACCESS_TOKEN`, `LONGBRIDGE_HTTP_URL`, `LONGBRIDGE_QUOTE_WS_URL`,
 * `LONGBRIDGE_TRADE_WS_URL`, `LONGBRIDGE_LANGUAGE`,
 * `LONGBRIDGE_ENABLE_OVERNIGHT`, `LONGBRIDGE_PUSH_CANDLESTICK_MODE`,
 * `LONGBRIDGE_PRINT_QUOTE_PACKAGES`, `LONGBRIDGE_LOG_PATH`
 */
struct lb_config_t *lb_config_from_apikey_env(struct lb_error_t **error);

/**
 * Create a new `Config` for OAuth 2.0 authentication
 *
 * Optional environment variables are read automatically:
 * `LONGBRIDGE_HTTP_URL`, `LONGBRIDGE_LANGUAGE`, `LONGBRIDGE_QUOTE_WS_URL`,
 * `LONGBRIDGE_TRADE_WS_URL`, `LONGBRIDGE_ENABLE_OVERNIGHT`,
 * `LONGBRIDGE_PUSH_CANDLESTICK_MODE`, `LONGBRIDGE_PRINT_QUOTE_PACKAGES`,
 * `LONGBRIDGE_LOG_PATH`.  Use the corresponding `lb_config_set_*` functions
 * to override any of these values after construction.
 *
 * Does **not** take ownership of `oauth`. The caller must free `oauth` with
 * `lb_oauth_free` after this call returns.
 *
 * @param oauth  OAuth 2.0 client obtained from `lb_oauth_new`
 */
struct lb_config_t *lb_config_from_oauth(const struct lb_oauth_t *oauth);

/**
 * Set the HTTP endpoint URL
 *
 * @param config   Config object
 * @param http_url HTTP endpoint URL (e.g. `https://openapi.longbridge.com`)
 */
void lb_config_set_http_url(struct lb_config_t *config, const char *http_url);

/**
 * Set the Quote WebSocket endpoint URL
 *
 * @param config        Config object
 * @param quote_ws_url  Quote WebSocket URL
 */
void lb_config_set_quote_ws_url(struct lb_config_t *config, const char *quote_ws_url);

/**
 * Set the Trade WebSocket endpoint URL
 *
 * @param config        Config object
 * @param trade_ws_url  Trade WebSocket URL
 */
void lb_config_set_trade_ws_url(struct lb_config_t *config, const char *trade_ws_url);

/**
 * Set the language identifier
 *
 * @param config    Config object
 * @param language  Language identifier
 */
void lb_config_set_language(struct lb_config_t *config, enum lb_language_t language);

/**
 * Enable overnight quote
 *
 * @param config  Config object
 */
void lb_config_enable_overnight(struct lb_config_t *config);

/**
 * Set the push candlestick mode
 *
 * @param config  Config object
 * @param mode    Push candlestick mode
 */
void lb_config_set_push_candlestick_mode(struct lb_config_t *config,
                                         enum lb_push_candlestick_mode_t mode);

/**
 * Disable printing of quote packages on connection
 *
 * @param config  Config object
 */
void lb_config_disable_print_quote_packages(struct lb_config_t *config);

/**
 * Enable paper trading mode.
 *
 * When enabled, all API calls target the paper trading (simulation)
 * environment.  The server validates the token: if it belongs to a real-money
 * account the server returns an error.
 *
 * When disabled (the default) the server imposes no restrictions — both
 * paper trading and real-money accounts are accepted.
 *
 * Paper trading users should call this function as a safety guard.
 *
 * @param config  Config object
 */
void lb_config_enable_papertrading(struct lb_config_t *config);

/**
 * Set the log file path
 *
 * @param config    Config object
 * @param log_path  Path for log files
 */
void lb_config_set_log_path(struct lb_config_t *config, const char *log_path);

/**
 * Gets a new `access_token`
 *
 * This function is only available when using **Legacy API Key**
 * authentication (i.e. `lb_config_from_apikey`). It is not supported for
 * OAuth 2.0 mode.
 *
 * @param config      Config object
 * @param expired_at  Unix timestamp for token expiry. Pass `0` to use the
 *                    default (90 days from now).
 * @param callback    Callback function; on success `res->data` is a
 *                    `const char*` access token (valid only within the
 *                    callback body).
 * @param userdata    Opaque pointer forwarded to the callback
 */
void lb_config_refresh_access_token(struct lb_config_t *config,
                                    int64_t expired_at,
                                    lb_async_callback_t callback,
                                    void *userdata);

/**
 * Free the config object
 */
void lb_config_free(struct lb_config_t *config);

/**
 * Create a new `ContentContext`
 *
 * @param config  Config object
 * @return A new content context
 */
const struct lb_content_context_t *lb_content_context_new(const struct lb_config_t *config);

/**
 * Retain the content context (increment reference count)
 */
void lb_content_context_retain(const struct lb_content_context_t *ctx);

/**
 * Release the content context (decrement reference count)
 */
void lb_content_context_release(const struct lb_content_context_t *ctx);

/**
 * Get topics created by the current authenticated user
 *
 * @param ctx         Content context
 * @param page        Page number (0 = default 1)
 * @param size        Records per page, range 1~500 (0 = default 50)
 * @param topic_type  Filter by content type: "article" or "post" (NULL = all)
 * @param callback    Async callback
 * @param userdata    User data passed to the callback
 */
void lb_content_context_my_topics(const struct lb_content_context_t *ctx,
                                  int32_t page,
                                  int32_t size,
                                  const char *topic_type,
                                  lb_async_callback_t callback,
                                  void *userdata);

/**
 * Create a new topic
 *
 * @param ctx          Content context
 * @param title        Topic title (required)
 * @param body         Topic body in Markdown format (required)
 * @param topic_type   Type: "article" or "post" (NULL = "post")
 * @param tickers      Related stock tickers array (NULL = none)
 * @param num_tickers  Number of tickers
 * @param hashtags     Hashtag names array (NULL = none)
 * @param num_hashtags Number of hashtags
 * @param callback     Async callback
 * @param userdata     User data passed to the callback
 */
void lb_content_context_create_topic(const struct lb_content_context_t *ctx,
                                     const char *title,
                                     const char *body,
                                     const char *topic_type,
                                     const char *const *tickers,
                                     uintptr_t num_tickers,
                                     const char *const *hashtags,
                                     uintptr_t num_hashtags,
                                     lb_async_callback_t callback,
                                     void *userdata);

/**
 * Get discussion topics list for a symbol
 *
 * @param ctx       Content context
 * @param symbol    Security symbol (e.g. "700.HK")
 * @param callback  Async callback
 * @param userdata  User data passed to the callback
 */
void lb_content_context_topics(const struct lb_content_context_t *ctx,
                               const char *symbol,
                               lb_async_callback_t callback,
                               void *userdata);

/**
 * Get news list for a symbol
 *
 * @param ctx       Content context
 * @param symbol    Security symbol (e.g. "700.HK")
 * @param callback  Async callback
 * @param userdata  User data passed to the callback
 */
void lb_content_context_news(const struct lb_content_context_t *ctx,
                             const char *symbol,
                             lb_async_callback_t callback,
                             void *userdata);

const struct lb_dca_context_t *lb_dca_context_new(const struct lb_config_t *config);

void lb_dca_context_retain(const struct lb_dca_context_t *ctx);

void lb_dca_context_release(const struct lb_dca_context_t *ctx);

/**
 * List DCA plans (status: 0=Active,1=Suspended,2=Finished,-1=all).
 * Returns `CDcaList`.
 */
void lb_dca_context_list(const struct lb_dca_context_t *ctx,
                         int32_t status,
                         lb_async_callback_t callback,
                         void *userdata);

/**
 * Get DCA stats. Returns `CDcaStats`.
 */
void lb_dca_context_stats(const struct lb_dca_context_t *ctx,
                          lb_async_callback_t callback,
                          void *userdata);

/**
 * Check which symbols support DCA. Returns `CDcaSupportList`.
 */
void lb_dca_context_check_support(const struct lb_dca_context_t *ctx,
                                  const char *const *symbols,
                                  uintptr_t num_symbols,
                                  lb_async_callback_t callback,
                                  void *userdata);

/**
 * Pause a DCA plan. Returns no data (empty response).
 */
void lb_dca_context_pause(const struct lb_dca_context_t *ctx,
                          const char *plan_id,
                          lb_async_callback_t callback,
                          void *userdata);

/**
 * Resume a DCA plan. Returns no data (empty response).
 */
void lb_dca_context_resume(const struct lb_dca_context_t *ctx,
                           const char *plan_id,
                           lb_async_callback_t callback,
                           void *userdata);

/**
 * Stop a DCA plan. Returns no data (empty response).
 */
void lb_dca_context_stop(const struct lb_dca_context_t *ctx,
                         const char *plan_id,
                         lb_async_callback_t callback,
                         void *userdata);

/**
 * Calculate next projected trade date. Returns `CDcaCalcDateResult`.
 * day_of_month: 0 = not set; 1–28 = day of month for monthly plans.
 */
void lb_dca_context_calc_date(const struct lb_dca_context_t *ctx,
                              const char *symbol,
                              enum lb_dca_frequency_t frequency,
                              const char *day_of_week,
                              uint32_t day_of_month,
                              lb_async_callback_t callback,
                              void *userdata);

/**
 * Get DCA execution history for a plan. Returns `CDcaHistoryResponse`.
 */
void lb_dca_context_history(const struct lb_dca_context_t *ctx,
                            const char *plan_id,
                            int32_t page,
                            int32_t limit,
                            lb_async_callback_t callback,
                            void *userdata);

/**
 * Update advance reminder hours. `hours` must be `"1"`, `"6"`, or `"12"`.
 */
void lb_dca_context_set_reminder(const struct lb_dca_context_t *ctx,
                                 const char *hours,
                                 lb_async_callback_t callback,
                                 void *userdata);

/**
 * Create a new DCA plan. Returns `CDcaCreateResult`.
 * day_of_week: optional (e.g. "Mon"), pass NULL if not applicable
 * day_of_month: 0 = not set
 */
void lb_dca_context_create(const struct lb_dca_context_t *ctx,
                           const char *symbol,
                           const char *amount,
                           enum lb_dca_frequency_t frequency,
                           const char *day_of_week,
                           uint32_t day_of_month,
                           bool allow_margin,
                           lb_async_callback_t callback,
                           void *userdata);

/**
 * Update an existing DCA plan. Returns `CDcaCreateResult`.
 * Pass -1 for frequency to leave unchanged; pass NULL for optional string
 * fields.
 */
void lb_dca_context_update(const struct lb_dca_context_t *ctx,
                           const char *plan_id,
                           const char *amount,
                           int32_t frequency,
                           const char *day_of_week,
                           const char *day_of_month,
                           int32_t allow_margin,
                           lb_async_callback_t callback,
                           void *userdata);

/**
 * Free the error object
 */
void lb_error_free(struct lb_error_t *error);

const char *lb_error_message(const struct lb_error_t *error);

int64_t lb_error_code(const struct lb_error_t *error);

enum lb_error_kind_t lb_error_kind(const struct lb_error_t *error);

const struct lb_fundamental_context_t *lb_fundamental_context_new(const struct lb_config_t *config);

void lb_fundamental_context_retain(const struct lb_fundamental_context_t *ctx);

void lb_fundamental_context_release(const struct lb_fundamental_context_t *ctx);

/**
 * Get financial reports — returns `CFinancialReports` (list_json is JSON
 * string)
 *
 * @param kind   report kind enum value
 * @param period 0=af, 1=saf, 2=q1, 3=q2, 4=q3, 5=qf, -1=none
 */
void lb_fundamental_context_financial_report(const struct lb_fundamental_context_t *ctx,
                                             const char *symbol,
                                             enum lb_financial_report_kind_t kind,
                                             int32_t period,
                                             lb_async_callback_t callback,
                                             void *userdata);

/**
 * Get analyst ratings. Returns `CInstitutionRating`.
 */
void lb_fundamental_context_institution_rating(const struct lb_fundamental_context_t *ctx,
                                               const char *symbol,
                                               lb_async_callback_t callback,
                                               void *userdata);

/**
 * Get analyst rating detail. Returns `CInstitutionRatingDetail`.
 */
void lb_fundamental_context_institution_rating_detail(const struct lb_fundamental_context_t *ctx,
                                                      const char *symbol,
                                                      lb_async_callback_t callback,
                                                      void *userdata);

/**
 * Get dividend history. Returns `CDividendList`.
 */
void lb_fundamental_context_dividend(const struct lb_fundamental_context_t *ctx,
                                     const char *symbol,
                                     lb_async_callback_t callback,
                                     void *userdata);

/**
 * Get detailed dividend information. Returns `CDividendList`.
 */
void lb_fundamental_context_dividend_detail(const struct lb_fundamental_context_t *ctx,
                                            const char *symbol,
                                            lb_async_callback_t callback,
                                            void *userdata);

/**
 * Get EPS forecasts. Returns `CForecastEps`.
 */
void lb_fundamental_context_forecast_eps(const struct lb_fundamental_context_t *ctx,
                                         const char *symbol,
                                         lb_async_callback_t callback,
                                         void *userdata);

/**
 * Get valuation metrics. Returns `CValuationData`.
 */
void lb_fundamental_context_valuation(const struct lb_fundamental_context_t *ctx,
                                      const char *symbol,
                                      lb_async_callback_t callback,
                                      void *userdata);

/**
 * Get historical valuation data. Returns `CValuationHistoryResponse`.
 */
void lb_fundamental_context_valuation_history(const struct lb_fundamental_context_t *ctx,
                                              const char *symbol,
                                              lb_async_callback_t callback,
                                              void *userdata);

/**
 * Get company overview. Returns `CCompanyOverview`.
 */
void lb_fundamental_context_company(const struct lb_fundamental_context_t *ctx,
                                    const char *symbol,
                                    lb_async_callback_t callback,
                                    void *userdata);

/**
 * Get major shareholders. Returns `CShareholderList`.
 */
void lb_fundamental_context_shareholder(const struct lb_fundamental_context_t *ctx,
                                        const char *symbol,
                                        lb_async_callback_t callback,
                                        void *userdata);

/**
 * Get fund and ETF holders. Returns `CFundHolders`.
 */
void lb_fundamental_context_fund_holder(const struct lb_fundamental_context_t *ctx,
                                        const char *symbol,
                                        lb_async_callback_t callback,
                                        void *userdata);

/**
 * Get corporate actions. Returns `CCorpActions`.
 */
void lb_fundamental_context_corp_action(const struct lb_fundamental_context_t *ctx,
                                        const char *symbol,
                                        lb_async_callback_t callback,
                                        void *userdata);

/**
 * Get investor relations data. Returns `CInvestRelations`.
 */
void lb_fundamental_context_invest_relation(const struct lb_fundamental_context_t *ctx,
                                            const char *symbol,
                                            lb_async_callback_t callback,
                                            void *userdata);

/**
 * Get operating metrics. Returns `COperatingList`.
 */
void lb_fundamental_context_operating(const struct lb_fundamental_context_t *ctx,
                                      const char *symbol,
                                      lb_async_callback_t callback,
                                      void *userdata);

/**
 * Get consensus estimates. Returns `CFinancialConsensus`.
 */
void lb_fundamental_context_consensus(const struct lb_fundamental_context_t *ctx,
                                      const char *symbol,
                                      lb_async_callback_t callback,
                                      void *userdata);

/**
 * Get industry valuation. Returns `CIndustryValuationList`.
 */
void lb_fundamental_context_industry_valuation(const struct lb_fundamental_context_t *ctx,
                                               const char *symbol,
                                               lb_async_callback_t callback,
                                               void *userdata);

/**
 * Get industry valuation distribution. Returns `CIndustryValuationDist`.
 */
void lb_fundamental_context_industry_valuation_dist(const struct lb_fundamental_context_t *ctx,
                                                    const char *symbol,
                                                    lb_async_callback_t callback,
                                                    void *userdata);

/**
 * Get executive info. Returns `CExecutiveList`.
 */
void lb_fundamental_context_executive(const struct lb_fundamental_context_t *ctx,
                                      const char *symbol,
                                      lb_async_callback_t callback,
                                      void *userdata);

/**
 * Get buyback data. Returns `CBuybackData`.
 */
void lb_fundamental_context_buyback(const struct lb_fundamental_context_t *ctx,
                                    const char *symbol,
                                    lb_async_callback_t callback,
                                    void *userdata);

/**
 * Get stock ratings. Returns `CStockRatings`.
 */
void lb_fundamental_context_ratings(const struct lb_fundamental_context_t *ctx,
                                    const char *symbol,
                                    lb_async_callback_t callback,
                                    void *userdata);

/**
 * Get ranked list of top shareholders. Returns `CShareholderTopResponse`.
 */
void lb_fundamental_context_shareholder_top(const struct lb_fundamental_context_t *ctx,
                                            const char *symbol,
                                            lb_async_callback_t callback,
                                            void *userdata);

/**
 * Get holding history and detail for one shareholder. Returns
 * `CShareholderDetailResponse`.
 */
void lb_fundamental_context_shareholder_detail(const struct lb_fundamental_context_t *ctx,
                                               const char *symbol,
                                               int64_t object_id,
                                               lb_async_callback_t callback,
                                               void *userdata);

/**
 * Get current business segment breakdown for a security.
 * Returns `CBusinessSegments`.
 */
void lb_fundamental_context_business_segments(const struct lb_fundamental_context_t *ctx,
                                              const char *symbol,
                                              lb_async_callback_t callback,
                                              void *userdata);

/**
 * Get historical business segment breakdowns for a security.
 * Returns `CBusinessSegmentsHistory`.
 * Pass NULL for `report` and/or `cate` to omit those filters.
 */
void lb_fundamental_context_business_segments_history(const struct lb_fundamental_context_t *ctx,
                                                      const char *symbol,
                                                      const char *report,
                                                      const char *cate,
                                                      lb_async_callback_t callback,
                                                      void *userdata);

/**
 * Get historical institutional rating views for a security.
 * Returns `CInstitutionRatingViews`.
 */
void lb_fundamental_context_institution_rating_views(const struct lb_fundamental_context_t *ctx,
                                                     const char *symbol,
                                                     lb_async_callback_t callback,
                                                     void *userdata);

/**
 * Get industry rank for a market.
 * Returns `CIndustryRankResponse`.
 */
void lb_fundamental_context_industry_rank(const struct lb_fundamental_context_t *ctx,
                                          const char *market,
                                          const char *indicator,
                                          const char *sort_type,
                                          uint32_t limit,
                                          lb_async_callback_t callback,
                                          void *userdata);

/**
 * Get the industry peer chain for a security or industry.
 * Returns `CIndustryPeersResponse`.
 * Pass NULL for `industry_id` to omit it.
 */
void lb_fundamental_context_industry_peers(const struct lb_fundamental_context_t *ctx,
                                           const char *counter_id,
                                           const char *market,
                                           const char *industry_id,
                                           lb_async_callback_t callback,
                                           void *userdata);

/**
 * Get a financial report snapshot for a security.
 * Returns `CFinancialReportSnapshot`.
 * Pass NULL for `report`, `fiscal_year_str`, and/or `fiscal_period` to omit
 * them. `fiscal_year_str` should be a decimal integer string (e.g. `"2024"`).
 */
void lb_fundamental_context_financial_report_snapshot(const struct lb_fundamental_context_t *ctx,
                                                      const char *symbol,
                                                      const char *report,
                                                      const char *fiscal_year_str,
                                                      const char *fiscal_period,
                                                      lb_async_callback_t callback,
                                                      void *userdata);

/**
 * Get valuation comparison between a security and optional peers.
 * Returns `CValuationComparisonResponse`.
 * Pass NULL for `comparison_symbols` to skip peer comparison.
 */
void lb_fundamental_context_valuation_comparison(const struct lb_fundamental_context_t *ctx,
                                                 const char *symbol,
                                                 const char *currency,
                                                 const char *const *comparison_symbols,
                                                 uintptr_t num_comparison_symbols,
                                                 lb_async_callback_t callback,
                                                 void *userdata);

/**
 * Get ETF asset allocation (holdings / regional / asset class / industry).
 * Returns `CAssetAllocationResponse`.
 */
void lb_fundamental_context_etf_asset_allocation(const struct lb_fundamental_context_t *ctx,
                                                 const char *symbol,
                                                 lb_async_callback_t callback,
                                                 void *userdata);

/**
 * Create a HTTP client using API Key authentication
 *
 * @param http_url     HTTP endpoint URL, or NULL to use the default
 * @param app_key      App key
 * @param app_secret   App secret
 * @param access_token Access token
 */
struct lb_http_client_t *lb_http_client_from_apikey(const char *http_url,
                                                    const char *app_key,
                                                    const char *app_secret,
                                                    const char *access_token);

/**
 * Free the http client
 */
void lb_http_client_free(struct lb_http_client_t *http_client);

/**
 * Create a new `HttpClient` from environment variables (API Key mode)
 *
 * It first reads the `.env` file in the current directory.
 *
 * # Variables
 *
 * - `LONGBRIDGE_HTTP_URL` - HTTP endpoint url
 * - `LONGBRIDGE_APP_KEY` - App key
 * - `LONGBRIDGE_APP_SECRET` - App secret
 * - `LONGBRIDGE_ACCESS_TOKEN` - Access token
 */
struct lb_http_client_t *lb_http_client_from_apikey_env(struct lb_error_t **error);

/**
 * Create a new `HttpClient` from an OAuth 2.0 client
 *
 * @param oauth     OAuth 2.0 client obtained from `lb_oauth_new`
 * @param http_url  HTTP endpoint URL, or NULL to use the default
 */
struct lb_http_client_t *lb_http_client_from_oauth(const struct lb_oauth_t *oauth,
                                                   const char *http_url);

/**
 * Performs a HTTP request
 */
void lb_http_client_request(struct lb_http_client_t *http_client,
                            const char *method,
                            const char *path,
                            const struct lb_http_header_t *headers,
                            const char *request_body,
                            lb_async_callback_t callback,
                            void *userdata);

/**
 * Free the HTTP result
 */
void lb_http_result_free(struct lb_http_result_t *http_result);

const char *lb_http_result_response_body(const struct lb_http_result_t *http_result);

/**
 * Create a new `MarketContext`
 */
const struct lb_market_context_t *lb_market_context_new(const struct lb_config_t *config);

/**
 * Retain the market context
 */
void lb_market_context_retain(const struct lb_market_context_t *ctx);

/**
 * Release the market context
 */
void lb_market_context_release(const struct lb_market_context_t *ctx);

/**
 * Get market trading status
 *
 * Returns `CMarketStatusResponse`
 */
void lb_market_context_market_status(const struct lb_market_context_t *ctx,
                                     lb_async_callback_t callback,
                                     void *userdata);

/**
 * Get top broker holdings
 *
 * Returns `CBrokerHoldingTop`
 */
void lb_market_context_broker_holding(const struct lb_market_context_t *ctx,
                                      const char *symbol,
                                      enum lb_broker_holding_period_t period,
                                      lb_async_callback_t callback,
                                      void *userdata);

/**
 * Get full broker holding details
 * Returns `CBrokerHoldingDetail`
 */
void lb_market_context_broker_holding_detail(const struct lb_market_context_t *ctx,
                                             const char *symbol,
                                             lb_async_callback_t callback,
                                             void *userdata);

/**
 * Get daily broker holding history
 * Returns `CBrokerHoldingDailyHistory`
 */
void lb_market_context_broker_holding_daily(const struct lb_market_context_t *ctx,
                                            const char *symbol,
                                            const char *broker_id,
                                            lb_async_callback_t callback,
                                            void *userdata);

/**
 * Get A/H premium K-lines
 *
 * @param count   Number of K-lines
 * Returns `CAhPremiumKlines`
 */
void lb_market_context_ah_premium(const struct lb_market_context_t *ctx,
                                  const char *symbol,
                                  enum lb_ah_premium_period_t period,
                                  uint32_t count,
                                  lb_async_callback_t callback,
                                  void *userdata);

/**
 * Get A/H premium intraday data
 * Returns `CAhPremiumIntraday`
 */
void lb_market_context_ah_premium_intraday(const struct lb_market_context_t *ctx,
                                           const char *symbol,
                                           lb_async_callback_t callback,
                                           void *userdata);

/**
 * Get trade statistics
 * Returns `CTradeStatsResponse`
 */
void lb_market_context_trade_stats(const struct lb_market_context_t *ctx,
                                   const char *symbol,
                                   lb_async_callback_t callback,
                                   void *userdata);

/**
 * Get market anomaly alerts
 * Returns `CAnomalyResponse`
 */
void lb_market_context_anomaly(const struct lb_market_context_t *ctx,
                               const char *market,
                               lb_async_callback_t callback,
                               void *userdata);

/**
 * Get index constituent stocks
 * Returns `CIndexConstituents`
 */
void lb_market_context_constituent(const struct lb_market_context_t *ctx,
                                   const char *symbol,
                                   lb_async_callback_t callback,
                                   void *userdata);

/**
 * Get top movers (stocks with unusual price movements) across one or more
 * markets. Pass markets as a NULL-terminated array of C strings.
 * Returns `CTopMoversResponse`.
 */
void lb_market_context_top_movers(const struct lb_market_context_t *ctx,
                                  const char *const *markets,
                                  uintptr_t num_markets,
                                  uint32_t sort,
                                  const char *date,
                                  uint32_t limit,
                                  lb_async_callback_t callback,
                                  void *userdata);

/**
 * Get all available rank category keys and labels.
 * Returns `CRankCategoriesResponse`.
 */
void lb_market_context_rank_categories(const struct lb_market_context_t *ctx,
                                       lb_async_callback_t callback,
                                       void *userdata);

/**
 * Get a ranked list of securities for the given category key.
 * Returns `CRankListResponse`.
 */
void lb_market_context_rank_list(const struct lb_market_context_t *ctx,
                                 const char *key,
                                 bool need_article,
                                 lb_async_callback_t callback,
                                 void *userdata);

/**
 * Asynchronously build an OAuth 2.0 client.
 *
 * Tries to load an existing token from
 * `~/.longbridge/openapi/tokens/<client_id>`. If the token is missing or
 * expired, starts a local callback server and calls `open_url_callback` so
 * the caller can open the authorization URL in a browser.
 *
 * @param client_id          NUL-terminated OAuth 2.0 client ID
 * @param callback_port      Local callback server port; pass 0 to use the
 *                           default (60355)
 * @param open_url_callback  Called with the authorization URL and
 *                           `open_url_userdata` during the auth flow
 * @param open_url_userdata  Opaque pointer forwarded to `open_url_callback`
 * @param callback           Async completion callback; `data` is
 *                           `*mut LbOAuth` on success (free with
 *                           `lb_oauth_free`)
 * @param userdata           Opaque pointer forwarded to `callback`
 */
void lb_oauth_new(const char *client_id,
                  uint16_t callback_port,
                  void (*open_url_callback)(const char*, void*),
                  void *open_url_userdata,
                  lb_async_callback_t callback,
                  void *userdata);

/**
 * Clone an OAuth 2.0 client object
 *
 * Increments the internal Arc reference count; the returned pointer must be
 * freed independently with `lb_oauth_free`.
 */
struct lb_oauth_t *lb_oauth_clone(const struct lb_oauth_t *oauth);

/**
 * Free an OAuth 2.0 client object
 */
void lb_oauth_free(struct lb_oauth_t *oauth);

const struct lb_portfolio_context_t *lb_portfolio_context_new(const struct lb_config_t *config);

void lb_portfolio_context_retain(const struct lb_portfolio_context_t *ctx);

void lb_portfolio_context_release(const struct lb_portfolio_context_t *ctx);

/**
 * Get exchange rates. Returns CExchangeRates.
 */
void lb_portfolio_context_exchange_rate(const struct lb_portfolio_context_t *ctx,
                                        lb_async_callback_t callback,
                                        void *userdata);

/**
 * Get portfolio P&L analysis. Returns `CProfitAnalysis`.
 */
void lb_portfolio_context_profit_analysis(const struct lb_portfolio_context_t *ctx,
                                          const char *start,
                                          const char *end,
                                          lb_async_callback_t callback,
                                          void *userdata);

/**
 * Get P&L by market. Returns `CProfitAnalysisByMarket`.
 */
void lb_portfolio_context_profit_analysis_by_market(const struct lb_portfolio_context_t *ctx,
                                                    const char *market,
                                                    const char *start,
                                                    const char *end,
                                                    const char *currency,
                                                    int32_t page,
                                                    int32_t size,
                                                    lb_async_callback_t callback,
                                                    void *userdata);

/**
 * Get P&L flow records for a security. Returns `CProfitAnalysisFlows`.
 */
void lb_portfolio_context_profit_analysis_flows(const struct lb_portfolio_context_t *ctx,
                                                const char *symbol,
                                                int32_t page,
                                                int32_t size,
                                                bool derivative,
                                                const char *start,
                                                const char *end,
                                                lb_async_callback_t callback,
                                                void *userdata);

/**
 * Get P&L detail for a security. Returns `CProfitAnalysisDetail`.
 */
void lb_portfolio_context_profit_analysis_detail(const struct lb_portfolio_context_t *ctx,
                                                 const char *symbol,
                                                 const char *start,
                                                 const char *end,
                                                 lb_async_callback_t callback,
                                                 void *userdata);

const struct lb_quote_context_t *lb_quote_context_new(const struct lb_config_t *config);

void lb_quote_context_retain(const struct lb_quote_context_t *ctx);

void lb_quote_context_release(const struct lb_quote_context_t *ctx);

uintptr_t lb_quote_context_ref_count(const struct lb_quote_context_t *ctx);

void lb_quote_context_set_userdata(const struct lb_quote_context_t *ctx, void *userdata);

void *lb_quote_context_userdata(const struct lb_quote_context_t *ctx);

void lb_quote_context_set_free_userdata_func(const struct lb_quote_context_t *ctx,
                                             lb_free_userdata_func_t f);

void lb_quote_context_member_id(const struct lb_quote_context_t *ctx,
                                lb_async_callback_t callback,
                                void *userdata);

void lb_quote_context_quote_level(const struct lb_quote_context_t *ctx,
                                  lb_async_callback_t callback,
                                  void *userdata);

void lb_quote_context_quote_package_details(const struct lb_quote_context_t *ctx,
                                            lb_async_callback_t callback,
                                            void *userdata);

/**
 * Set quote callback, after receiving the quote data push, it will call back
 * to this function.
 */
void lb_quote_context_set_on_quote(const struct lb_quote_context_t *ctx,
                                   lb_quote_callback_t callback,
                                   void *userdata,
                                   lb_free_userdata_func_t free_userdata);

/**
 * Set depth callback, after receiving the depth data push, it will call
 * back to this function.
 */
void lb_quote_context_set_on_depth(const struct lb_quote_context_t *ctx,
                                   lb_depth_callback_t callback,
                                   void *userdata,
                                   lb_free_userdata_func_t free_userdata);

/**
 * Set brokers callback, after receiving the brokers data push, it will
 * call back to this function.
 */
void lb_quote_context_set_on_brokers(const struct lb_quote_context_t *ctx,
                                     lb_brokers_callback_t callback,
                                     void *userdata,
                                     lb_free_userdata_func_t free_userdata);

/**
 * Set trades callback, after receiving the trades data push, it will call
 * back to this function.
 */
void lb_quote_context_set_on_trades(const struct lb_quote_context_t *ctx,
                                    lb_trades_callback_t callback,
                                    void *userdata,
                                    lb_free_userdata_func_t free_userdata);

/**
 * Set candlestick callback, after receiving the trades data push, it will
 * call back to this function.
 */
void lb_quote_context_set_on_candlestick(const struct lb_quote_context_t *ctx,
                                         lb_candlestick_callback_t callback,
                                         void *userdata,
                                         lb_free_userdata_func_t free_userdata);

void lb_quote_context_subscribe(const struct lb_quote_context_t *ctx,
                                const char *const *symbols,
                                uintptr_t num_symbols,
                                uint8_t sub_types,
                                lb_async_callback_t callback,
                                void *userdata);

/**
 * Unsubscribe
 */
void lb_quote_context_unsubscribe(const struct lb_quote_context_t *ctx,
                                  const char *const *symbols,
                                  uintptr_t num_symbols,
                                  uint8_t sub_types,
                                  lb_async_callback_t callback,
                                  void *userdata);

/**
 * Subscribe security candlesticks
 */
void lb_quote_context_subscribe_candlesticks(const struct lb_quote_context_t *ctx,
                                             const char *symbol,
                                             enum lb_period_t period,
                                             enum lb_trade_sessions_t trade_sessions,
                                             lb_async_callback_t callback,
                                             void *userdata);

/**
 * Unsubscribe security candlesticks
 */
void lb_quote_context_unsubscribe_candlesticks(const struct lb_quote_context_t *ctx,
                                               const char *symbol,
                                               enum lb_period_t period,
                                               lb_async_callback_t callback,
                                               void *userdata);

/**
 * Get subscription information
 */
void lb_quote_context_subscriptions(const struct lb_quote_context_t *ctx,
                                    lb_async_callback_t callback,
                                    void *userdata);

/**
 * Get basic information of securities
 */
void lb_quote_context_static_info(const struct lb_quote_context_t *ctx,
                                  const char *const *symbols,
                                  uintptr_t num_symbols,
                                  lb_async_callback_t callback,
                                  void *userdata);

/**
 * Get quote of securities
 */
void lb_quote_context_quote(const struct lb_quote_context_t *ctx,
                            const char *const *symbols,
                            uintptr_t num_symbols,
                            lb_async_callback_t callback,
                            void *userdata);

/**
 * Get quote of option securities
 */
void lb_quote_context_option_quote(const struct lb_quote_context_t *ctx,
                                   const char *const *symbols,
                                   uintptr_t num_symbols,
                                   lb_async_callback_t callback,
                                   void *userdata);

/**
 * Get quote of warrant securities
 */
void lb_quote_context_warrant_quote(const struct lb_quote_context_t *ctx,
                                    const char *const *symbols,
                                    uintptr_t num_symbols,
                                    lb_async_callback_t callback,
                                    void *userdata);

/**
 * Get security depth
 */
void lb_quote_context_depth(const struct lb_quote_context_t *ctx,
                            const char *symbol,
                            lb_async_callback_t callback,
                            void *userdata);

/**
 * Get security brokers
 */
void lb_quote_context_brokers(const struct lb_quote_context_t *ctx,
                              const char *symbol,
                              lb_async_callback_t callback,
                              void *userdata);

/**
 * Get participants
 */
void lb_quote_context_participants(const struct lb_quote_context_t *ctx,
                                   lb_async_callback_t callback,
                                   void *userdata);

/**
 * Get security trades
 */
void lb_quote_context_trades(const struct lb_quote_context_t *ctx,
                             const char *symbol,
                             uintptr_t count,
                             lb_async_callback_t callback,
                             void *userdata);

/**
 * Get security intraday lines
 */
void lb_quote_context_intraday(const struct lb_quote_context_t *ctx,
                               const char *symbol,
                               enum lb_trade_sessions_t trade_sessions,
                               lb_async_callback_t callback,
                               void *userdata);

/**
 * Get security candlesticks
 */
void lb_quote_context_candlesticks(const struct lb_quote_context_t *ctx,
                                   const char *symbol,
                                   enum lb_period_t period,
                                   uintptr_t count,
                                   enum lb_adjust_type_t adjust_type,
                                   enum lb_trade_sessions_t trade_sessions,
                                   lb_async_callback_t callback,
                                   void *userdata);

/**
 * Get security history candlesticks by offset
 */
void lb_quote_context_history_candlesticks_by_offset(const struct lb_quote_context_t *ctx,
                                                     const char *symbol,
                                                     enum lb_period_t period,
                                                     enum lb_adjust_type_t adjust_type,
                                                     bool forward,
                                                     const struct lb_datetime_t *time,
                                                     uintptr_t count,
                                                     enum lb_trade_sessions_t trade_sessions,
                                                     lb_async_callback_t callback,
                                                     void *userdata);

/**
 * Get security history candlesticks by date
 */
void lb_quote_context_history_candlesticks_by_date(const struct lb_quote_context_t *ctx,
                                                   const char *symbol,
                                                   enum lb_period_t period,
                                                   enum lb_adjust_type_t adjust_type,
                                                   const struct lb_date_t *start,
                                                   const struct lb_date_t *end,
                                                   enum lb_trade_sessions_t trade_sessions,
                                                   lb_async_callback_t callback,
                                                   void *userdata);

/**
 * Get option chain expiry date list
 */
void lb_quote_context_option_chain_expiry_date_list(const struct lb_quote_context_t *ctx,
                                                    const char *symbol,
                                                    lb_async_callback_t callback,
                                                    void *userdata);

/**
 * Get option chain info by date
 */
void lb_quote_context_option_chain_info_by_date(const struct lb_quote_context_t *ctx,
                                                const char *symbol,
                                                const struct lb_date_t *expiry_date,
                                                lb_async_callback_t callback,
                                                void *userdata);

/**
 * Get warrant issuers
 */
void lb_quote_context_warrant_issuers(const struct lb_quote_context_t *ctx,
                                      lb_async_callback_t callback,
                                      void *userdata);

/**
 * Query warrant list
 */
void lb_quote_context_warrant_list(const struct lb_quote_context_t *ctx,
                                   const char *symbol,
                                   enum lb_warrant_sort_by_t sort_by,
                                   enum lb_sort_order_type_t sort_order,
                                   const enum lb_warrant_type_t *warrant_type,
                                   uintptr_t num_warrant_type,
                                   const int32_t *issuer,
                                   uintptr_t num_issuer,
                                   const enum lb_filter_warrant_expiry_date_t *expiry_date,
                                   uintptr_t num_expiry_date,
                                   const enum lb_filter_warrant_in_out_bounds_type_t *price_type,
                                   uintptr_t num_price_type,
                                   const enum lb_warrant_status_t *status,
                                   uintptr_t num_status,
                                   lb_async_callback_t callback,
                                   void *userdata);

/**
 * Get trading session of the day
 */
void lb_quote_context_trading_session(const struct lb_quote_context_t *ctx,
                                      lb_async_callback_t callback,
                                      void *userdata);

/**
 * Get market trading days
 */
void lb_quote_context_trading_days(const struct lb_quote_context_t *ctx,
                                   enum lb_market_t market,
                                   const struct lb_date_t *begin,
                                   const struct lb_date_t *end,
                                   lb_async_callback_t callback,
                                   void *userdata);

/**
 * Get capital flow intraday
 */
void lb_quote_context_capital_flow(const struct lb_quote_context_t *ctx,
                                   const char *symbol,
                                   lb_async_callback_t callback,
                                   void *userdata);

/**
 * Get capital distribution
 */
void lb_quote_context_capital_distribution(const struct lb_quote_context_t *ctx,
                                           const char *symbol,
                                           lb_async_callback_t callback,
                                           void *userdata);

/**
 * Get calc indexes
 */
void lb_quote_context_calc_indexes(const struct lb_quote_context_t *ctx,
                                   const char *const *symbols,
                                   uintptr_t num_symbols,
                                   const enum lb_calc_index_t *indexes,
                                   uintptr_t num_indexes,
                                   lb_async_callback_t callback,
                                   void *userdata);

/**
 * Get watchlist
 */
void lb_quote_context_watchlist(const struct lb_quote_context_t *ctx,
                                lb_async_callback_t callback,
                                void *userdata);

/**
 * Create watchlist group
 */
void lb_quote_context_create_watchlist_group(const struct lb_quote_context_t *ctx,
                                             const struct lb_create_watchlist_group_t *req,
                                             lb_async_callback_t callback,
                                             void *userdata);

/**
 * Delete watchlist group
 */
void lb_quote_context_delete_watchlist_group(const struct lb_quote_context_t *ctx,
                                             int64_t id,
                                             bool purge,
                                             lb_async_callback_t callback,
                                             void *userdata);

/**
 * Update pinned watchlist securities (mode: 0=add, 1=remove)
 */
void lb_quote_context_update_pinned(const struct lb_quote_context_t *ctx,
                                    int32_t mode,
                                    const char *const *securities,
                                    uintptr_t num_securities,
                                    lb_async_callback_t callback,
                                    void *userdata);

/**
 * Create watchlist group
 */
void lb_quote_context_update_watchlist_group(const struct lb_quote_context_t *ctx,
                                             const struct lb_update_watchlist_group_t *req,
                                             lb_async_callback_t callback,
                                             void *userdata);

/**
 * Get quote of securities
 *
 * Get real-time quotes of the subscribed symbols, it always returns the data
 * in the local storage.
 */
void lb_quote_context_realtime_quote(const struct lb_quote_context_t *ctx,
                                     const char *const *symbols,
                                     uintptr_t num_symbols,
                                     lb_async_callback_t callback,
                                     void *userdata);

/**
 * Get real-time depth
 *
 * Get real-time depth of the subscribed symbols, it always returns the data in
 * the local storage.
 */
void lb_quote_context_realtime_depth(const struct lb_quote_context_t *ctx,
                                     const char *symbol,
                                     lb_async_callback_t callback,
                                     void *userdata);

/**
 * Get real-time trades
 *
 * Get real-time trades of the subscribed symbols, it always returns the data
 * in the local storage.
 */
void lb_quote_context_realtime_trades(const struct lb_quote_context_t *ctx,
                                      const char *symbol,
                                      uintptr_t count,
                                      lb_async_callback_t callback,
                                      void *userdata);

/**
 * Get real-time broker queue
 *
 * Get real-time broker queue of the subscribed symbols, it always returns the
 * data in the local storage.
 */
void lb_quote_context_realtime_brokers(const struct lb_quote_context_t *ctx,
                                       const char *symbol,
                                       lb_async_callback_t callback,
                                       void *userdata);

/**
 * Get real-time candlesticks
 *
 * Get real-time candlesticks of the subscribed symbols, it always returns the
 * data in the local storage.
 */
void lb_quote_context_realtime_candlesticks(const struct lb_quote_context_t *ctx,
                                            const char *symbol,
                                            enum lb_period_t period,
                                            uintptr_t count,
                                            lb_async_callback_t callback,
                                            void *userdata);

/**
 * Get filings
 */
void lb_quote_context_filings(const struct lb_quote_context_t *ctx,
                              const char *symbol,
                              lb_async_callback_t callback,
                              void *userdata);

/**
 * Get security list
 */
void lb_quote_context_security_list(const struct lb_quote_context_t *ctx,
                                    enum lb_market_t market,
                                    const enum lb_security_list_category_t *category,
                                    lb_async_callback_t callback,
                                    void *userdata);

/**
 * Get current market temperature
 */
void lb_quote_context_market_temperature(const struct lb_quote_context_t *ctx,
                                         enum lb_market_t market,
                                         lb_async_callback_t callback,
                                         void *userdata);

/**
 * Get historical market temperature
 */
void lb_quote_context_history_market_temperature(const struct lb_quote_context_t *ctx,
                                                 enum lb_market_t market,
                                                 const struct lb_date_t *start,
                                                 const struct lb_date_t *end,
                                                 lb_async_callback_t callback,
                                                 void *userdata);

/**
 * Get short interest data for a US or HK security. Returns
 * `CShortPositionsResponse`. Market is inferred from symbol suffix.
 */
void lb_quote_context_short_positions(const struct lb_quote_context_t *ctx,
                                      const char *symbol,
                                      uint32_t count,
                                      lb_async_callback_t callback,
                                      void *userdata);

/**
 * Get short trade records for a HK or US security. Returns
 * `CShortTradesResponse`. Market is inferred from symbol suffix.
 */
void lb_quote_context_short_trades(const struct lb_quote_context_t *ctx,
                                   const char *symbol,
                                   uint32_t count,
                                   lb_async_callback_t callback,
                                   void *userdata);

/**
 * Get real-time option call/put volume. Returns `COptionVolumeStats`.
 */
void lb_quote_context_option_volume(const struct lb_quote_context_t *ctx,
                                    const char *symbol,
                                    lb_async_callback_t callback,
                                    void *userdata);

/**
 * Get daily historical option volume. Returns `COptionVolumeDaily`.
 */
void lb_quote_context_option_volume_daily(const struct lb_quote_context_t *ctx,
                                          const char *symbol,
                                          int64_t timestamp,
                                          uint32_t count,
                                          lb_async_callback_t callback,
                                          void *userdata);

const struct lb_screener_context_t *lb_screener_context_new(const struct lb_config_t *config);

void lb_screener_context_retain(const struct lb_screener_context_t *ctx);

void lb_screener_context_release(const struct lb_screener_context_t *ctx);

/**
 * Get recommended built-in screener strategies.
 * Returns `CScreenerRecommendStrategiesResponse`.
 */
void lb_screener_context_recommend_strategies(const struct lb_screener_context_t *ctx,
                                              const char *market,
                                              lb_async_callback_t callback,
                                              void *userdata);

/**
 * Get the current user's saved screener strategies.
 * Returns `CScreenerUserStrategiesResponse`.
 */
void lb_screener_context_user_strategies(const struct lb_screener_context_t *ctx,
                                         const char *market,
                                         lb_async_callback_t callback,
                                         void *userdata);

/**
 * Get detail for one screener strategy by ID.
 * Returns `CScreenerStrategyResponse`.
 */
void lb_screener_context_strategy(const struct lb_screener_context_t *ctx,
                                  int64_t id,
                                  lb_async_callback_t callback,
                                  void *userdata);

/**
 * Search / screen securities using a strategy.
 * Returns `CScreenerSearchResponse`.
 */
void lb_screener_context_search(const struct lb_screener_context_t *ctx,
                                const char *market,
                                int64_t strategy_id,
                                bool has_strategy_id,
                                uint32_t page,
                                uint32_t size,
                                lb_async_callback_t callback,
                                void *userdata);

/**
 * Get all available screener indicator definitions.
 * Returns `CScreenerIndicatorsResponse`.
 */
void lb_screener_context_indicators(const struct lb_screener_context_t *ctx,
                                    lb_async_callback_t callback,
                                    void *userdata);

const struct lb_sharelist_context_t *lb_sharelist_context_new(const struct lb_config_t *config);

void lb_sharelist_context_retain(const struct lb_sharelist_context_t *ctx);

void lb_sharelist_context_release(const struct lb_sharelist_context_t *ctx);

/**
 * List user's sharelists. Returns `CSharelistList`.
 */
void lb_sharelist_context_list(const struct lb_sharelist_context_t *ctx,
                               uint32_t count,
                               lb_async_callback_t callback,
                               void *userdata);

/**
 * Get sharelist detail. Returns `CSharelistDetail`.
 */
void lb_sharelist_context_detail(const struct lb_sharelist_context_t *ctx,
                                 int64_t id,
                                 lb_async_callback_t callback,
                                 void *userdata);

/**
 * Get popular sharelists. Returns `CSharelistList`.
 */
void lb_sharelist_context_popular(const struct lb_sharelist_context_t *ctx,
                                  uint32_t count,
                                  lb_async_callback_t callback,
                                  void *userdata);

/**
 * Add securities to a sharelist.
 */
void lb_sharelist_context_add_securities(const struct lb_sharelist_context_t *ctx,
                                         int64_t id,
                                         const char *const *symbols,
                                         uintptr_t num_symbols,
                                         lb_async_callback_t callback,
                                         void *userdata);

/**
 * Remove securities from a sharelist.
 */
void lb_sharelist_context_remove_securities(const struct lb_sharelist_context_t *ctx,
                                            int64_t id,
                                            const char *const *symbols,
                                            uintptr_t num_symbols,
                                            lb_async_callback_t callback,
                                            void *userdata);

/**
 * Create a new sharelist. Returns no data (empty response).
 */
void lb_sharelist_context_create(const struct lb_sharelist_context_t *ctx,
                                 const char *name,
                                 const char *description,
                                 lb_async_callback_t callback,
                                 void *userdata);

/**
 * Delete a sharelist.
 */
void lb_sharelist_context_delete(const struct lb_sharelist_context_t *ctx,
                                 int64_t id,
                                 lb_async_callback_t callback,
                                 void *userdata);

/**
 * Reorder securities in a sharelist.
 */
void lb_sharelist_context_sort_securities(const struct lb_sharelist_context_t *ctx,
                                          int64_t id,
                                          const char *const *symbols,
                                          uintptr_t num_symbols,
                                          lb_async_callback_t callback,
                                          void *userdata);

const struct lb_trade_context_t *lb_trade_context_new(const struct lb_config_t *config);

void lb_trade_context_retain(const struct lb_trade_context_t *ctx);

void lb_trade_context_release(const struct lb_trade_context_t *ctx);

uintptr_t lb_trade_context_ref_count(const struct lb_trade_context_t *ctx);

void lb_trade_context_set_userdata(const struct lb_trade_context_t *ctx, void *userdata);

void *lb_trade_context_userdata(const struct lb_trade_context_t *ctx);

void lb_trade_context_set_free_userdata_func(const struct lb_trade_context_t *ctx,
                                             lb_free_userdata_func_t f);

/**
 * Set order changed callback, after receiving the order changed event, it will
 * call back to this function.
 */
void lb_trade_context_set_on_order_changed(const struct lb_trade_context_t *ctx,
                                           lb_order_changed_callback_t callback,
                                           void *userdata,
                                           lb_free_userdata_func_t free_userdata);

void lb_trade_context_subscribe(const struct lb_trade_context_t *ctx,
                                const enum lb_topic_type_t *topics,
                                uintptr_t num_topics,
                                lb_async_callback_t callback,
                                void *userdata);

void lb_trade_context_unsubscribe(const struct lb_trade_context_t *ctx,
                                  const enum lb_topic_type_t *topics,
                                  uintptr_t num_topics,
                                  lb_async_callback_t callback,
                                  void *userdata);

/**
 * Get history executions
 *
 * @param[in] opts Options for get history executions request (can be null)
 */
void lb_trade_context_history_executions(const struct lb_trade_context_t *ctx,
                                         const struct lb_get_history_executions_options_t *opts,
                                         lb_async_callback_t callback,
                                         void *userdata);

/**
 * Get today executions
 *
 * @param[in] opts Options for get today executions request (can be null)
 */
void lb_trade_context_today_executions(const struct lb_trade_context_t *ctx,
                                       const struct lb_get_today_executions_options_t *opts,
                                       lb_async_callback_t callback,
                                       void *userdata);

/**
 * Get history orders
 *
 * @param[in] opts Options for get history orders request (can be null)
 */
void lb_trade_context_history_orders(const struct lb_trade_context_t *ctx,
                                     const struct lb_get_history_orders_options_t *opts,
                                     lb_async_callback_t callback,
                                     void *userdata);

/**
 * Get today orders
 *
 * @param[in] opts Options for get today orders request (can be null)
 */
void lb_trade_context_today_orders(const struct lb_trade_context_t *ctx,
                                   const struct lb_get_today_orders_options_t *opts,
                                   lb_async_callback_t callback,
                                   void *userdata);

/**
 * Replace order
 *
 * @param[in] opts Options for replace order request
 */
void lb_trade_context_replace_order(const struct lb_trade_context_t *ctx,
                                    const struct lb_replace_order_options_t *opts,
                                    lb_async_callback_t callback,
                                    void *userdata);

/**
 * Submit order
 *
 * @param[in] opts Options for submit order request
 */
void lb_trade_context_submit_order(const struct lb_trade_context_t *ctx,
                                   const struct lb_submit_order_options_t *opts,
                                   lb_async_callback_t callback,
                                   void *userdata);

/**
 * Cancel order
 */
void lb_trade_context_cancel_order(const struct lb_trade_context_t *ctx,
                                   const char *order_id,
                                   lb_async_callback_t callback,
                                   void *userdata);

/**
 * Get account balance
 */
void lb_trade_context_account_balance(const struct lb_trade_context_t *ctx,
                                      const char *currency,
                                      lb_async_callback_t callback,
                                      void *userdata);

/**
 * Get cash flow
 *
 * @param[in] opts Options for get cash flow request
 */
void lb_trade_context_cash_flow(const struct lb_trade_context_t *ctx,
                                const struct lb_get_cash_flow_options_t *opts,
                                lb_async_callback_t callback,
                                void *userdata);

/**
 * Get fund positions
 *
 * @param[in] opts Options for get fund positions request
 */
void lb_trade_context_fund_positions(const struct lb_trade_context_t *ctx,
                                     const struct lb_get_fund_positions_options_t *opts,
                                     lb_async_callback_t callback,
                                     void *userdata);

/**
 * Get stock positions
 *
 * @param[in] opts Options for get stock positions request
 */
void lb_trade_context_stock_positions(const struct lb_trade_context_t *ctx,
                                      const struct lb_get_stock_positions_options_t *opts,
                                      lb_async_callback_t callback,
                                      void *userdata);

/**
 * Get margin ratio
 */
void lb_trade_context_margin_ratio(const struct lb_trade_context_t *ctx,
                                   const char *symbol,
                                   lb_async_callback_t callback,
                                   void *userdata);

/**
 * Get order detail
 */
void lb_trade_context_order_detail(const struct lb_trade_context_t *ctx,
                                   const char *order_id,
                                   lb_async_callback_t callback,
                                   void *userdata);

/**
 * Get order detail for attached order
 */
void lb_trade_context_order_detail_attached(const struct lb_trade_context_t *ctx,
                                            const char *order_id,
                                            lb_async_callback_t callback,
                                            void *userdata);

/**
 * Get order detail
 */
void lb_trade_context_estimate_max_purchase_quantity(const struct lb_trade_context_t *ctx,
                                                     const struct lb_estimate_max_purchase_quantity_options_t *opts,
                                                     lb_async_callback_t callback,
                                                     void *userdata);

/**
 * Create a decimal value with a 64 bit `m` representation and corresponding
 * `e` scale.
 */
struct lb_decimal_t *lb_decimal_new(int64_t num, uint32_t scale);

/**
 * Clone the decimal value
 */
struct lb_decimal_t *lb_decimal_clone(const struct lb_decimal_t *value);

/**
 * Create a decimal value from string
 */
struct lb_decimal_t *lb_decimal_from_str(const char *value);

/**
 * Create a decimal value from double
 */
struct lb_decimal_t *lb_decimal_from_double(double value);

/**
 * Free the decimal value
 */
void lb_decimal_free(struct lb_decimal_t *value);

double lb_decimal_to_double(const struct lb_decimal_t *value);

/**
 * Computes the absolute value.
 */
void lb_decimal_abs(struct lb_decimal_t *value);

/**
 * Returns the smallest integer greater than or equal to a number.
 */
void lb_decimal_ceil(struct lb_decimal_t *value);

/**
 * Returns the largest integer less than or equal to a number.
 */
void lb_decimal_floor(struct lb_decimal_t *value);

/**
 * Returns a new Decimal representing the fractional portion of the number.
 */
void lb_decimal_fract(struct lb_decimal_t *value);

/**
 * Returns `true` if the decimal is negative.
 */
bool lb_decimal_is_negative(const struct lb_decimal_t *value);

/**
 * Returns `true` if the decimal is positive.
 */
bool lb_decimal_is_positive(const struct lb_decimal_t *value);

/**
 * Returns `true` if this Decimal number is equivalent to zero.
 */
bool lb_decimal_is_zero(const struct lb_decimal_t *value);

/**
 * Returns the maximum of the two numbers.
 */
const struct lb_decimal_t *lb_decimal_max(const struct lb_decimal_t *a,
                                          const struct lb_decimal_t *b);

/**
 * Returns the minimum of the two numbers.
 */
const struct lb_decimal_t *lb_decimal_min(const struct lb_decimal_t *a,
                                          const struct lb_decimal_t *b);

/**
 * Strips any trailing zero’s from a Decimal and converts `-0` to `0`.
 */
void lb_decimal_normalize(struct lb_decimal_t *value);

/**
 * Returns a new Decimal number with no fractional portion (i.e. an
 * integer). Rounding currently follows “Bankers Rounding” rules. e.g.
 * `6.5` -> `6`, `7.5` -> `8`
 */
void lb_decimal_round(struct lb_decimal_t *value);

/**
 * Returns a new Decimal number with the specified number of decimal points for
 * fractional portion. Rounding currently follows “Bankers Rounding” rules.
 * e.g. 6.5 -> 6, 7.5 -> 8
 */
void lb_decimal_round_dp(struct lb_decimal_t *value, uint32_t dp);

/**
 * Returns a new Decimal integral with no fractional portion. This is a
 * true truncation whereby no rounding is performed.
 */
void lb_decimal_trunc(struct lb_decimal_t *value);

/**
 * Performs the `+` operation.
 */
void lb_decimal_add(struct lb_decimal_t *a, const struct lb_decimal_t *b);

/**
 * Performs the `-` operation.
 */
void lb_decimal_sub(struct lb_decimal_t *a, const struct lb_decimal_t *b);

/**
 * Performs the `*` operation.
 */
void lb_decimal_mul(struct lb_decimal_t *a, const struct lb_decimal_t *b);

/**
 * Performs the `/` operation.
 */
void lb_decimal_div(struct lb_decimal_t *a, const struct lb_decimal_t *b);

/**
 * Performs the `%` operation.
 */
void lb_decimal_rem(struct lb_decimal_t *a, const struct lb_decimal_t *b);

/**
 * Performs the unary `-` operation.
 */
void lb_decimal_neg(struct lb_decimal_t *value);

/**
 * Returns `true` if the value of this Decimal is greater than the value of
 * `x`, otherwise returns `false`.
 */
bool lb_decimal_gt(const struct lb_decimal_t *a, const struct lb_decimal_t *b);

/**
 * Returns `true` if the value of this Decimal is greater than or equal to
 * the value of `x`, otherwise returns `false`.
 */
bool lb_decimal_gte(const struct lb_decimal_t *a, const struct lb_decimal_t *b);

/**
 * Returns `true` if the value of this Decimal equals the value of `x`,
 * otherwise returns `false`.
 */
bool lb_decimal_eq(const struct lb_decimal_t *a, const struct lb_decimal_t *b);

/**
 * Returns `true` if the value of this Decimal is less than the value of
 * `x`, otherwise returns `false`.
 */
bool lb_decimal_lt(const struct lb_decimal_t *a, const struct lb_decimal_t *b);

/**
 * Returns `true` if the value of this Decimal is less than or equal to the
 * value of `x`, otherwise returns `false`.
 */
bool lb_decimal_lte(const struct lb_decimal_t *a, const struct lb_decimal_t *b);

/**
 * Compares the values of two Decimals.
 *
 * Returns `-1` if the value of this Decimal is less than the value of
 * `x`.
 *
 * Returns `1` if the value of this Decimal is greater than the value of
 * `x`.
 *
 * Returns `0` if the value of this Decimal equals the value of `x`.
 */
int32_t lb_decimal_cmp(const struct lb_decimal_t *a, const struct lb_decimal_t *b);

/**
 * Computes the sine of a number (in radians)
 */
void lb_decimal_sin(struct lb_decimal_t *value);

/**
 * Computes the cosine of a number (in radians)
 */
void lb_decimal_cos(struct lb_decimal_t *value);

/**
 * Computes the tangent of a number (in radians). Panics upon overflow or
 * upon approaching a limit.
 */
void lb_decimal_tan(struct lb_decimal_t *value);

/**
 * The square root of a Decimal. Uses a standard Babylonian method.
 */
void lb_decimal_sqrt(struct lb_decimal_t *value);

/**
 * Raise self to the given Decimal exponent: x<sup>y</sup>. If `exp` is not
 * whole then the approximation e<sup>y*ln(x)</sup> is used.
 */
void lb_decimal_pow(struct lb_decimal_t *value, const struct lb_decimal_t *exp);

/**
 * Calculates the natural logarithm for a Decimal calculated using Taylor’s
 * series.
 */
void lb_decimal_ln(struct lb_decimal_t *value);

/**
 * Calculates the base 10 logarithm of a specified Decimal number.
 */
void lb_decimal_log10(struct lb_decimal_t *value);

/**
 * The estimated exponential function, ex. Stops calculating when it is
 * within tolerance of roughly `0.0000002`.
 */
void lb_decimal_exp(struct lb_decimal_t *value);

/**
 * The estimated exponential function, e<sup>x</sup> using the `tolerance`
 * provided as a hint as to when to stop calculating. A larger
 * tolerance will cause the number to stop calculating sooner at the
 * potential cost of a slightly less accurate result.
 */
void lb_decimal_exp_with_tolerance(struct lb_decimal_t *value,
                                   const struct lb_decimal_t *tolerance);

/**
 * Abramowitz Approximation of Error Function from [wikipedia](https://en.wikipedia.org/wiki/Error_function#Numerical_approximations)
 */
void lb_decimal_erf(struct lb_decimal_t *value);

/**
 * The Cumulative distribution function for a Normal distribution
 */
void lb_decimal_normal_cdf(struct lb_decimal_t *value);

/**
 * The Probability density function for a Normal distribution.
 */
void lb_decimal_norm_pdf(struct lb_decimal_t *value);

/**
 * The Probability density function for a Normal distribution.
 */
const char *lb_decimal_to_string(const struct lb_decimal_t *value);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus

#endif  /* _LONGBRIDGE_H_ */
