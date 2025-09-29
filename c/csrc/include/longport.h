#ifndef _LONGPORT_H_
#define _LONGPORT_H_

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
} lb_error_kind_t;

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
  /**
   * Force Selling
   */
  OrderTagMarginCall,
  /**
   * OTC
   */
  OrderTagOffline,
  /**
   * Option Exercise Long
   */
  OrderTagCreditor,
  /**
   * Option Exercise Short
   */
  OrderTagDebtor,
  /**
   * Wavier Of Option Exercise
   */
  OrderTagNonExercise,
  /**
   * Trade Allocation
   */
  OrderTagAllocatedSub,
} lb_order_tag_t;

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
} lb_outside_rth_t;

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
} lb_security_board_t;

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
 * Topic type
 */
typedef enum lb_topic_type_t {
  /**
   * Trading
   */
  TopicPrivate,
} lb_topic_type_t;

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
 * Configuration options for LongPort sdk
 */
typedef struct lb_config_t lb_config_t;

typedef struct lb_decimal_t lb_decimal_t;

typedef struct lb_error_t lb_error_t;

/**
 * A HTTP client for LongPort OpenApi
 */
typedef struct lb_http_client_t lb_http_client_t;

typedef struct lb_http_result_t lb_http_result_t;

/**
 * Quote context
 */
typedef struct lb_quote_context_t lb_quote_context_t;

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
} lb_get_today_orders_options_t;

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
   * Remark (can be null)
   */
  const char *remark;
} lb_replace_order_options_t;

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
   * Remark (Maximum 64 characters) (can be null)
   */
  const char *remark;
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
  const char *symbol;
  enum lb_order_type_t order_type;
  const struct lb_decimal_t *price;
  enum lb_order_side_t side;
  const char *currency;
  const char *order_id;
  bool fractional_shares;
} lb_estimate_max_purchase_quantity_options_t;

typedef struct lb_subscription_t {
  const char *symbol;
  uint8_t sub_types;
  const enum lb_period_t *candlesticks;
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
   * Dividend yield
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
 * Market trading days
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
   * Remark
   */
  const char *remark;
} lb_order_t;

/**
 * Account balance
 */
typedef struct lb_frozen_transaction_fee_t {
  /**
   * Total cash
   */
  const char *currency;
  /**
   * Maximum financing amount
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
 * Order detail
 */
typedef struct lb_order_history_detail_t {
  const struct lb_decimal_t *price;
  const struct lb_decimal_t *quantity;
  enum lb_order_status_t status;
  const char *msg;
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
   * Order charges
   */
  struct lb_order_charge_detail_t charge_detail;
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
 * Security
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

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/**
 * Create a new `Config` from the given environment variables
 *
 * It first gets the environment variables from the `.env` file in the
 * current directory.
 *
 * # Variables
 *
 * - `LONGPORT_LANGUAGE` - Language identifier, `zh-CN`, `zh-HK` or `en`
 *   (Default: `en`)
 * - `LONGPORT_APP_KEY` - App key
 * - `LONGPORT_APP_SECRET` - App secret
 * - `LONGPORT_ACCESS_TOKEN` - Access token
 * - `LONGPORT_HTTP_URL` - HTTP endpoint url (Default: `https://openapi.longportapp.com`)
 * - `LONGPORT_QUOTE_WS_URL` - Quote websocket endpoint url (Default:
 *   `wss://openapi-quote.longportapp.com/v2`)
 * - `LONGPORT_TRADE_WS_URL` - Trade websocket endpoint url (Default:
 *   `wss://openapi-trade.longportapp.com/v2`)
 * - `LONGPORT_ENABLE_OVERNIGHT` - Enable overnight quote, `true` or `false`
 *   (Default: `false`)
 * - `LONGPORT_PUSH_CANDLESTICK_MODE` - `realtime` or `confirmed` (Default:
 *   `realtime`)
 * - `LONGPORT_PRINT_QUOTE_PACKAGES` - Print quote packages when connected,
 *   `true` or `false` (Default: `true`)
 * - `LONGPORT_LOG_PATH` - Set the path of the log files (Default: `no logs`)
 */
struct lb_config_t *lb_config_from_env(struct lb_error_t **error);

struct lb_config_t *lb_config_new(const char *app_key,
                                  const char *app_secret,
                                  const char *access_token,
                                  const char *http_url,
                                  const char *quote_ws_url,
                                  const char *trade_ws_url,
                                  const enum lb_language_t *language,
                                  bool enable_overight,
                                  const enum lb_push_candlestick_mode_t *push_candlestick_mode,
                                  bool enable_print_quote_packages,
                                  const char *log_path);

/**
 * Free the config object
 */
void lb_config_free(struct lb_config_t *config);

/**
 * Gets a new `access_token`
 */
void lb_config_refresh_access_token(struct lb_config_t *config,
                                    int64_t expired_at,
                                    lb_async_callback_t callback,
                                    void *userdata);

/**
 * Free the error object
 */
void lb_error_free(struct lb_error_t *error);

const char *lb_error_message(const struct lb_error_t *error);

int64_t lb_error_code(const struct lb_error_t *error);

enum lb_error_kind_t lb_error_kind(const struct lb_error_t *error);

/**
 * Create a HTTP client
 */
struct lb_http_client_t *lb_http_client_new(const char *http_url,
                                            const char *app_key,
                                            const char *app_secret,
                                            const char *access_token);

/**
 * Free the http client
 */
void lb_http_client_free(struct lb_http_client_t *http_client);

/**
 * Create a new `HttpClient` from the given environment variables
 *
 * It first gets the environment variables from the `.env` file in the
 * current directory.
 *
 * # Variables
 *
 * - `LONGPORT_HTTP_URL` - HTTP endpoint url
 * - `LONGPORT_APP_KEY` - App key
 * - `LONGPORT_APP_SECRET` - App secret
 * - `LONGPORT_ACCESS_TOKEN` - Access token
 */
struct lb_http_client_t *lb_http_client_from_env(struct lb_error_t **error);

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

void lb_quote_context_new(const struct lb_config_t *config,
                          lb_async_callback_t callback,
                          void *userdata);

void lb_quote_context_retain(const struct lb_quote_context_t *ctx);

void lb_quote_context_release(const struct lb_quote_context_t *ctx);

uintptr_t lb_quote_context_ref_count(const struct lb_quote_context_t *ctx);

void lb_quote_context_set_userdata(const struct lb_quote_context_t *ctx, void *userdata);

void *lb_quote_context_userdata(const struct lb_quote_context_t *ctx);

void lb_quote_context_set_free_userdata_func(const struct lb_quote_context_t *ctx,
                                             lb_free_userdata_func_t f);

int64_t lb_quote_context_member_id(const struct lb_quote_context_t *ctx);

const char *lb_quote_context_quote_level(const struct lb_quote_context_t *ctx);

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
                                bool is_first_push,
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
void lb_quote_context_subscrptions(const struct lb_quote_context_t *ctx,
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

void lb_trade_context_new(const struct lb_config_t *config,
                          lb_async_callback_t callback,
                          void *userdata);

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

#endif  /* _LONGPORT_H_ */
