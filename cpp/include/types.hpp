#pragma once

#include "decimal.hpp"
#include <optional>
#include <vector>

namespace longport {

struct Date
{
  int32_t year;
  uint8_t month;
  uint8_t day;
};

struct Time
{
  uint8_t hour;
  uint8_t minute;
  uint8_t second;
};

struct DateTime
{
  Date date;
  Time time;
};

/// Language identifer
enum class Language
{
  /// zh-CN
  ZH_CN,
  /// zh-HK
  ZH_HK,
  /// en
  EN,
};

/// Push candlestick mode
enum class PushCandlestickMode
{
  /// Real-time
  Realtime,
  /// Confirmed
  Confirmed,
};

/// Market
enum class Market
{
  /// Unknown
  Unknown,
  /// US market
  US,
  /// HK market
  HK,
  /// CN market
  CN,
  /// SG market
  SG,
  /// Crypto market
  Crypto,
};

namespace quote {

enum class TradeStatus;

/// Subscription flags
class SubFlags
{
private:
  uint8_t value_;

public:
  inline SubFlags(uint8_t value) { value_ = value; }

  inline operator uint8_t() const { return value_; }

  inline SubFlags operator|(const SubFlags& other) const
  {
    return SubFlags(value_ | other.value_);
  }

  inline SubFlags& operator|=(const SubFlags& other)
  {
    value_ |= other.value_;
    return *this;
  }

  inline SubFlags operator&(const SubFlags& other) const
  {
    return SubFlags(value_ & other.value_);
  }

  inline SubFlags& operator&=(const SubFlags& other)
  {
    value_ &= other.value_;
    return *this;
  }

  inline bool operator==(const SubFlags& other) const
  {
    return value_ == other.value_;
  }

  inline bool contains(const SubFlags& other) const
  {
    return (value_ & other.value_) > 0;
  }

  static SubFlags QUOTE();
  static SubFlags DEPTH();
  static SubFlags BROKER();
  static SubFlags TRADE();
};

/// Derivative type
struct DerivativeType
{
  uint8_t value;

  bool has_option();
  bool has_warrant();
};

/// Candlestick period
enum class Period
{
  /// Unknown
  Unknown,
  /// One Minute
  Min1,
  /// Two Minutes
  Min2,
  /// Three Minutes
  Min3,
  /// Five Minutes
  Min5,
  /// Ten Minutes
  Min10,
  /// Fifteen Minutes
  Min15,
  /// Twenty Minutes
  Min20,
  /// Thirty Minutes
  Min30,
  /// Forty-Five Minutes
  Min45,
  /// One Hour
  Min60,
  /// Two Hours
  Min120,
  /// Three Hours
  Min180,
  /// Four Hours
  Min240,
  /// Daily
  Day,
  /// Weekly
  Week,
  /// Monthly
  Month,
  /// Quarterly
  Quarter,
  /// Yearly
  Year,
};

/// Subscription
struct Subscription
{
  /// Security code
  std::string symbol;
  /// Subscription flags
  SubFlags sub_types;
  /// Candlesticks
  std::vector<Period> candlesticks;
};

/// Trade session
enum class TradeSession
{
  /// Intraday
  Intraday,
  /// Pre-Market
  Pre,
  /// Post-Market
  Post,
  /// Overnight
  Overnight,
};

/// Quote message
struct PushQuote
{
  /// Security code
  std::string symbol;
  /// Latest price
  Decimal last_done;
  /// Open
  Decimal open;
  /// High
  Decimal high;
  /// Low
  Decimal low;
  /// Time of latest price
  int64_t timestamp;
  /// Volume
  int64_t volume;
  /// Turnover
  Decimal turnover;
  /// Security trading status
  TradeStatus trade_status;
  /// Trade session
  TradeSession trade_session;
  /// Increase volume between pushes
  int64_t current_volume;
  /// Increase turnover between pushes
  Decimal urrent_turnover;
};

struct Depth
{
  /// Position
  int32_t position;
  /// Price
  std::optional<Decimal> price;
  /// Volume
  int64_t volume;
  /// Number of orders
  int64_t order_num;
};

/// Depth message
struct PushDepth
{
  /// Security code
  std::string symbol;
  /// Ask depth
  std::vector<Depth> asks;
  /// Bid depth
  std::vector<Depth> bids;
};

/// Brokers
struct Brokers
{
  /// Position
  int32_t position;
  /// Broker IDs
  std::vector<int32_t> broker_ids;
};

/// Brokers message
struct PushBrokers
{
  /// Security code
  std::string symbol;
  /// Ask brokers
  std::vector<Brokers> ask_brokers;
  /// Bid brokers
  std::vector<Brokers> bid_brokers;
};

/// Security board
enum class SecurityBoard
{
  /// Unknown
  Unknown,
  /// US Main Board
  USMain,
  /// US Pink Board
  USPink,
  /// Dow Jones Industrial Average
  USDJI,
  /// Nasdsaq Index
  USNSDQ,
  /// US Industry Board
  USSector,
  /// US Option
  USOption,
  /// US Sepecial Option
  USOptionS,
  /// Hong Kong Equity Securities
  HKEquity,
  /// HK PreIPO Security
  HKPreIPO,
  /// HK Warrant
  HKWarrant,
  /// Hang Seng Index
  HKHS,
  /// HK Industry Board
  HKSector,
  /// SH Main Board(Connect)
  SHMainConnect,
  /// SH Main Board(Non Connect)
  SHMainNonConnect,
  /// SH Science and Technology Innovation Board
  SHSTAR,
  /// CN Index
  CNIX,
  /// CN Industry Board
  CNSector,
  /// SZ Main Board(Connect)
  SZMainConnect,
  /// SZ Main Board(Non Connect)
  SZMainNonConnect,
  /// SZ Gem Board(Connect)
  SZGEMConnect,
  /// SZ Gem Board(Non Connect)
  SZGEMNonConnect,
  /// SG Main Board
  SGMain,
  /// Singapore Straits Index
  STI,
  /// SG Industry Board
  SGSector,
  /// S&P 500 Index
  SPXIndex,
  /// CBOE Volatility Index
  VIXIndex,
};

/// Security
struct Security
{
  /// Security code
  std::string symbol;
  /// Security name (zh-CN)
  std::string name_cn;
  /// Security name (en)
  std::string name_en;
  /// Security name (zh-HK)
  std::string name_hk;
};

/// The basic information of securities
struct SecurityStaticInfo
{
  /// Security code
  std::string symbol;
  /// Security name (zh-CN)
  std::string name_cn;
  /// Security name (en)
  std::string name_en;
  /// Security name (zh-HK)
  std::string name_hk;
  /// Exchange which the security belongs to
  std::string exchange;
  /// Trading currency
  std::string currency;
  /// Lot size
  int32_t lot_size;
  /// Total shares
  int64_t total_shares;
  /// Circulating shares
  int64_t circulating_shares;
  /// HK shares (only HK stocks)
  int64_t hk_shares;
  /// Earnings per share
  Decimal eps;
  /// Earnings per share (TTM)
  Decimal eps_ttm;
  /// Net assets per share
  Decimal bps;
  /// Dividend yield
  Decimal dividend_yield;
  /// Types of supported derivatives
  DerivativeType stock_derivatives;
  /// Board
  SecurityBoard board;
};

/// Trade status
enum class TradeStatus
{
  /// Normal
  Normal,
  /// Suspension
  Halted,
  /// Delisted
  Delisted,
  /// Fuse
  Fuse,
  /// Papare List
  PrepareList,
  /// Code Moved
  CodeMoved,
  /// To Be Opened
  ToBeOpened,
  /// Split Stock Halts
  SplitStockHalts,
  /// Expired
  Expired,
  /// Warrant To BeListed
  WarrantPrepareList,
  /// Suspend
  SuspendTrade,
};

/// Quote of US pre/post market
struct PrePostQuote
{
  /// Latest price
  Decimal last_done;
  /// Time of latest price
  int64_t timestamp;
  /// Volume
  int64_t volume;
  /// Turnover
  Decimal turnover;
  /// High
  Decimal high;
  /// Low
  Decimal low;
  /// Close of the last trade session
  Decimal prev_close;
};

/// Quote of securitity
struct SecurityQuote
{
  /// Security code
  std::string symbol;
  /// Latest price
  Decimal last_done;
  /// Yesterday's close
  Decimal prev_close;
  /// Open
  Decimal open;
  /// High
  Decimal high;
  /// Low
  Decimal low;
  /// Time of latest price
  int64_t timestamp;
  /// Volume
  int64_t volume;
  /// Turnover
  Decimal turnover;
  /// Security trading status
  TradeStatus trade_status;
  /// Quote of US pre market
  std::optional<PrePostQuote> pre_market_quote;
  /// Quote of US post market
  std::optional<PrePostQuote> post_market_quote;
  /// Quote of US overnight market
  std::optional<PrePostQuote> overnight_quote;
};

/// Option type
enum class OptionType
{
  /// Unknown
  Unknown,
  /// American
  American,
  /// Europe
  Europe,
};

/// Option direction
enum class OptionDirection
{
  /// Unknown
  Unknown,
  /// Put
  Put,
  /// Call
  Call,
};

/// Quote of option
struct OptionQuote
{ /// Security code
  std::string symbol;
  /// Latest price
  Decimal last_done;
  /// Yesterday's close
  Decimal prev_close;
  /// Open
  Decimal open;
  /// High
  Decimal high;
  /// Low
  Decimal low;
  /// Time of latest price
  int64_t timestamp;
  /// Volume
  int64_t volume;
  /// Turnover
  Decimal turnover;
  /// Security trading status
  TradeStatus trade_status;
  /// Implied volatility
  Decimal implied_volatility;
  /// Number of open positions
  int64_t open_interest;
  /// Exprity date
  Date expiry_date;
  /// Strike price
  Decimal strike_price;
  /// Contract multiplier
  Decimal contract_multiplier;
  /// Option type
  OptionType contract_type;
  /// Contract size
  Decimal contract_size;
  /// Option direction
  OptionDirection direction;
  /// Underlying security historical volatility of the option
  Decimal historical_volatility;
  /// Underlying security symbol of the option
  std::string underlying_symbol;
};

/// Trade direction
enum class TradeDirection
{
  /// Neutral
  Neutral,
  /// Down
  Down,
  /// Up
  Up
};

/// Trade
struct Trade
{
  Decimal price;
  int64_t volume;
  int64_t timestamp;
  std::string trade_type;
  TradeDirection direction;
  TradeSession trade_session;
};

/// Trades message
struct PushTrades
{
  /// Security code
  std::string symbol;
  /// Trades data
  std::vector<Trade> trades;
};

/// Candlestick
struct Candlestick
{
  /// Close price
  Decimal close;
  /// Open price
  Decimal open;
  /// Low price
  Decimal low;
  /// High price
  Decimal high;
  /// Volume
  int64_t volume;
  /// Turnover
  Decimal turnover;
  /// Timestamp
  int64_t timestamp;
  /// Trade session
  TradeSession trade_session;
};

/// Candlestick updated message
struct PushCandlestick
{
  /// Security code
  std::string symbol;
  /// Period type
  Period period;
  /// Candlestick
  Candlestick candlestick;
  /// Is confirmed
  bool is_confirmed;
};

/// Warrant type
enum class WarrantType
{
  /// Unknown
  Unknown,
  /// Call
  Call,
  /// Put
  Put,
  /// Bull
  Bull,
  /// Bear
  Bear,
  /// Inline
  Inline
};

/// Quote of warrant
struct WarrantQuote
{ /// Security code
  std::string symbol;
  /// Latest price
  Decimal last_done;
  /// Yesterday's close
  Decimal prev_close;
  /// Open
  Decimal open;
  /// High
  Decimal high;
  /// Low
  Decimal low;
  /// Time of latest price
  int64_t timestamp;
  /// Volume
  int64_t volume;
  /// Turnover
  Decimal turnover;
  /// Security trading status
  TradeStatus trade_status;
  /// Implied volatility
  Decimal implied_volatility;
  /// Exprity date
  Date expiry_date;
  /// Last tradalbe date
  Date last_trade_date;
  /// Outstanding ratio
  Decimal outstanding_ratio;
  /// Outstanding quantity
  int64_t outstanding_quantity;
  /// Conversion ratio
  Decimal conversion_ratio;
  /// Warrant type
  WarrantType category;
  /// Strike price
  Decimal strike_price;
  /// Upper bound price
  Decimal upper_strike_price;
  /// Lower bound price
  Decimal lower_strike_price;
  /// Call price
  Decimal call_price;
  /// Underlying security symbol of the warrant
  std::string underlying_symbol;
};

/// Security depth
struct SecurityDepth
{
  /// Ask depth
  std::vector<Depth> asks;
  /// Bid depth
  std::vector<Depth> bids;
};

/// Security brokers
struct SecurityBrokers
{
  /// Ask brokers
  std::vector<Brokers> ask_brokers;
  /// Bid brokers
  std::vector<Brokers> bid_brokers;
};

struct ParticipantInfo
{
  /// Broker IDs
  std::vector<int32_t> broker_ids;
  /// Participant name (zh-CN)
  std::string name_cn;
  /// Participant name (en)
  std::string name_en;
  /// Participant name (zh-HK)
  std::string name_hk;
};

/// Intraday line
struct IntradayLine
{
  Decimal price;
  int64_t timestamp;
  int64_t volume;
  Decimal turnover;
  Decimal avg_price;
};

/// Adjust type
enum class AdjustType
{
  NoAdjust,
  ForwardAdjust
};

/// Strike price info
struct StrikePriceInfo
{
  /// Strike price
  Decimal price;
  /// Security code of call option
  std::string call_symbol;
  /// Security code of put option
  std::string put_symbol;
  /// Is standard
  bool standard;
};

/// Issuer info
struct IssuerInfo
{
  /// Issuer ID
  int32_t issuer_id;
  /// Issuer name (zh-CN)
  std::string name_cn;
  /// Issuer name (en)
  std::string name_en;
  /// Issuer name (zh-HK)
  std::string name_hk;
};

struct TradingSessionInfo
{
  /// Being trading time
  Time begin_time;
  /// End trading time
  Time end_time;
  /// Trading session
  TradeSession trade_session;
};

/// Market trading session
struct MarketTradingSession
{
  /// Market
  Market market;
  /// Trading session
  std::vector<TradingSessionInfo> trade_session;
};

/// Market trading days
struct MarketTradingDays
{
  /// Trading days
  std::vector<Date> trading_days;
  /// Half trading days
  std::vector<Date> half_trading_days;
};

/// Capital flow line
struct CapitalFlowLine
{
  /// Inflow capital data
  Decimal inflow;
  /// Time
  int64_t timestamp;
};

/// Capital distribution
struct CapitalDistribution
{
  /// Large order
  Decimal large;
  /// Medium order
  Decimal medium;
  /// Small order
  Decimal small;
};

/// Capital distribution response
struct CapitalDistributionResponse
{
  /// Time
  int64_t timestamp;
  /// Inflow capital data
  CapitalDistribution capital_in;
  /// Outflow capital data
  CapitalDistribution capital_out;
};

/// Watchlist security
struct WatchlistSecurity
{
  /// Security symbol
  std::string symbol;
  /// Market
  Market market;
  /// Security name
  std::string name;
  /// Watched price
  std::optional<Decimal> watched_price;
  /// Watched time
  int64_t watched_at;
};

/// Watchlist group
struct WatchlistGroup
{
  /// Group id
  int64_t id;
  /// Group name
  std::string name;
  /// Securities
  std::vector<WatchlistSecurity> securities;
};

/// Securities update mode
enum class SecuritiesUpdateMode
{
  /// Add
  Add,
  /// Remove
  Remove,
  /// Replace
  Replace,
};

/// An request for create watchlist group
struct CreateWatchlistGroup
{
  /// Group name
  std::string name;
  /// Securities
  std::vector<std::string> securities;
};

/// An request for update watchlist group
struct UpdateWatchlistGroup
{
  /// Group id
  int64_t id;
  /// Group name
  std::optional<std::string> name;
  /// Securities
  std::optional<std::vector<std::string>> securities;
  /// Securities Update Mode
  SecuritiesUpdateMode mode;
};

/// Real-time quote
struct RealtimeQuote
{
  /// Security code
  std::string symbol;
  /// Latest price
  Decimal last_done;
  /// Open
  Decimal open;
  /// High
  Decimal high;
  /// Low
  Decimal low;
  /// Time of latest price
  int64_t timestamp;
  /// Volume
  int64_t volume;
  /// Turnover
  Decimal turnover;
  /// Security trading status
  TradeStatus trade_status;
};

/// Calc index
enum class CalcIndex
{
  /// Latest price
  LastDone,
  /// Change value
  ChangeValue,
  /// Change rate
  ChangeRate,
  /// Volume
  Volume,
  /// Turnover
  Turnover,
  /// Year-to-date change ratio
  YtdChangeRate,
  /// Turnover rate
  TurnoverRate,
  /// Total market value
  TotalMarketValue,
  /// Capital flow
  CapitalFlow,
  /// Amplitude
  Amplitude,
  /// Volume ratio
  VolumeRatio,
  /// PE (TTM)
  PeTtmRatio,
  /// PB
  PbRatio,
  /// Dividend ratio (TTM)
  DividendRatioTtm,
  /// Five days change ratio
  FiveDayChangeRate,
  /// Ten days change ratio
  TenDayChangeRate,
  /// Half year change ratio
  HalfYearChangeRate,
  /// Five minutes change ratio
  FiveMinutesChangeRate,
  /// Expiry date
  ExpiryDate,
  /// Strike price
  StrikePrice,
  /// Upper bound price
  UpperStrikePrice,
  /// Lower bound price
  LowerStrikePrice,
  /// Outstanding quantity
  OutstandingQty,
  /// Outstanding ratio
  OutstandingRatio,
  /// Premium
  Premium,
  /// In/out of the bound
  ItmOtm,
  /// Implied volatility
  ImpliedVolatility,
  /// Warrant delta
  WarrantDelta,
  /// Call price
  CallPrice,
  /// Price interval from the call price
  ToCallPrice,
  /// Effective leverage
  EffectiveLeverage,
  /// Leverage ratio
  LeverageRatio,
  /// Conversion ratio
  ConversionRatio,
  /// Breakeven point
  BalancePoint,
  /// Open interest
  OpenInterest,
  /// Delta
  Delta,
  /// Gamma
  Gamma,
  /// Theta
  Theta,
  /// Vega
  Vega,
  /// Rho
  Rho,
};

/// Security calc index response
struct SecurityCalcIndex
{
  /// Security code
  std::string symbol;
  /// Latest price
  std::optional<Decimal> last_done;
  /// Change value
  std::optional<Decimal> change_value;
  /// Change ratio
  std::optional<Decimal> change_rate;
  /// Volume
  std::optional<int64_t> volume;
  /// Turnover
  std::optional<Decimal> turnover;
  /// Year-to-date change ratio
  std::optional<Decimal> ytd_change_rate;
  /// Turnover rate
  std::optional<Decimal> turnover_rate;
  /// Total market value
  std::optional<Decimal> total_market_value;
  /// Capital flow
  std::optional<Decimal> capital_flow;
  /// Amplitude
  std::optional<Decimal> amplitude;
  /// Volume ratio
  std::optional<Decimal> volume_ratio;
  /// PE (TTM)
  std::optional<Decimal> pe_ttm_ratio;
  /// PB
  std::optional<Decimal> pb_ratio;
  /// Dividend ratio (TTM)
  std::optional<Decimal> dividend_ratio_ttm;
  /// Five days change ratio
  std::optional<Decimal> five_day_change_rate;
  /// Ten days change ratio
  std::optional<Decimal> ten_day_change_rate;
  /// Half year change ratio
  std::optional<Decimal> half_year_change_rate;
  /// Five minutes change ratio
  std::optional<Decimal> five_minutes_change_rate;
  /// Expiry date
  std::optional<Date> expiry_date;
  /// Strike price
  std::optional<Decimal> strike_price;
  /// Upper bound price
  std::optional<Decimal> upper_strike_price;
  /// Lower bound price
  std::optional<Decimal> lower_strike_price;
  /// Outstanding quantity
  std::optional<int64_t> outstanding_qty;
  /// Outstanding ratio
  std::optional<Decimal> outstanding_ratio;
  /// Premium
  std::optional<Decimal> premium;
  /// In/out of the bound
  std::optional<Decimal> itm_otm;
  /// Implied volatility
  std::optional<Decimal> implied_volatility;
  /// Warrant delta
  std::optional<Decimal> warrant_delta;
  /// Call price
  std::optional<Decimal> call_price;
  /// Price interval from the call price
  std::optional<Decimal> to_call_price;
  /// Effective leverage
  std::optional<Decimal> effective_leverage;
  /// Leverage ratio
  std::optional<Decimal> leverage_ratio;
  /// Conversion ratio
  std::optional<Decimal> conversion_ratio;
  /// Breakeven point
  std::optional<Decimal> balance_point;
  /// Open interest
  std::optional<int64_t> open_interest;
  /// Delta
  std::optional<Decimal> delta;
  /// Gamma
  std::optional<Decimal> gamma;
  /// Theta
  std::optional<Decimal> theta;
  /// Vega
  std::optional<Decimal> vega;
  /// Rho
  std::optional<Decimal> rho;
};

/// Sort order type
enum class SortOrderType
{
  /// Ascending
  Ascending,
  /// Descending
  Descending,
};

/// Warrant sort by
enum class WarrantSortBy
{
  /// Last done
  LastDone,
  /// Change rate
  ChangeRate,
  /// Change value
  ChangeValue,
  /// Volume
  Volume,
  /// Turnover
  Turnover,
  /// Expiry date
  ExpiryDate,
  /// Strike price
  StrikePrice,
  /// Upper strike price
  UpperStrikePrice,
  /// Lower strike price
  LowerStrikePrice,
  /// Outstanding quantity
  OutstandingQuantity,
  /// Outstanding ratio
  OutstandingRatio,
  /// Premium
  Premium,
  /// In/out of the bound
  ItmOtm,
  /// Implied volatility
  ImpliedVolatility,
  /// Greek value Delta
  Delta,
  /// Call price
  CallPrice,
  /// Price interval from the call price
  ToCallPrice,
  /// Effective leverage
  EffectiveLeverage,
  /// Leverage ratio
  LeverageRatio,
  /// Conversion ratio
  ConversionRatio,
  /// Breakeven point
  BalancePoint,
  /// Status
  Status,
};

/// Filter warrant expiry date type
enum class FilterWarrantExpiryDate
{
  /// Less than 3 months
  LT_3,
  /// 3 - 6 months
  Between_3_6,
  /// 6 - 12 months
  Between_6_12,
  /// Greater than 12 months
  GT_12,
};

/// Filter warrant in/out of the bounds type
enum class FilterWarrantInOutBoundsType
{
  /// In bounds
  In,
  /// Out bounds
  Out,
};

/// Warrant status
enum class WarrantStatus
{
  /// Suspend
  Suspend,
  /// Prepare List
  PrepareList,
  /// Normal
  Normal,
};

/// Warrant info
struct WarrantInfo
{
  /// Security code
  std::string symbol;
  /// Warrant type
  WarrantType warrant_type;
  /// Security name
  std::string name;
  /// Latest price
  Decimal last_done;
  /// Quote change rate
  Decimal change_rate;
  /// Quote change
  Decimal change_value;
  /// Volume
  int64_t volume;
  /// Turnover
  Decimal turnover;
  /// Expiry date
  Date expiry_date;
  /// Strike price
  std::optional<Decimal> strike_price;
  /// Upper strike price
  std::optional<Decimal> upper_strike_price;
  /// Lower strike price
  std::optional<Decimal> lower_strike_price;
  /// Outstanding quantity
  int64_t outstanding_qty;
  /// Outstanding ratio
  Decimal outstanding_ratio;
  /// Premium
  Decimal premium;
  /// In/out of the bound
  std::optional<Decimal> itm_otm;
  /// Implied volatility
  std::optional<Decimal> implied_volatility;
  /// Delta
  std::optional<Decimal> delta;
  /// Call price
  std::optional<Decimal> call_price;
  /// Price interval from the call price
  std::optional<Decimal> to_call_price;
  /// Effective leverage
  std::optional<Decimal> effective_leverage;
  /// Leverage ratio
  Decimal leverage_ratio;
  /// Conversion ratio
  std::optional<Decimal> conversion_ratio;
  /// Breakeven point
  std::optional<Decimal> balance_point;
  /// Status
  WarrantStatus status;
};

/// Security list category
enum class SecurityListCategory
{
  /// Overnight
  Overnight,
};

/// Quote package detail
struct QuotePackageDetail
{
  /// Key
  std::string key;
  /// Name
  std::string name;
  /// Description
  std::string description;
  /// Start at
  int64_t start_at;
  /// End at
  int64_t end_at;
};

/// Trade sessions
enum class TradeSessions
{
  /// Intraday
  Intraday,
  /// All
  All,
};

/// Market temperature
struct MarketTemperature
{
  /// Temperature value
  int32_t temperature;
  /// Temperature description
  std::string description;
  /// Market valuation
  int32_t valuation;
  /// Market sentiment
  int32_t sentiment;
  /// Time
  int64_t timestamp;
};

/// Data granularity
enum Granularity
{
  /// Unknown
  Unknown,
  /// Daily
  Daily,
  /// Weekly
  Weekly,
  /// Monthly
  Monthly,
};

/// History market temperature response
struct HistoryMarketTemperatureResponse
{
  /// Granularity
  Granularity granularity;
  /// Records
  std::vector<MarketTemperature> records;
};

} // namespace quote

namespace trade {

/// Topic type
enum class TopicType
{
  /// Private notification for trade
  Private,
};

/// Exexution
struct Execution
{
  std::string order_id;
  std::string trade_id;
  std::string symbol;
  int64_t trade_done_at;
  Decimal quantity;
  Decimal price;
};

/// Options for get history executions request
struct GetHistoryExecutionsOptions
{
  /// Start time
  std::optional<int64_t> start_at;
  /// End time
  std::optional<int64_t> end_at;
  /// Security code
  std::optional<std::string> symbol;
};

/// Options for get today executions request
struct GetTodayExecutionsOptions
{
  /// Security code
  std::optional<std::string> symbol;
  /// Order id
  std::optional<std::string> order_id;
};

/// Order status
enum class OrderStatus
{
  /// Unknown
  Unknown,
  /// Not reported
  NotReported,
  /// Not reported (Replaced Order)
  ReplacedNotReported,
  /// Not reported (Protected Order)
  ProtectedNotReported,
  /// Not reported (Conditional Order)
  VarietiesNotReported,
  /// Filled
  Filled,
  /// Wait To New
  WaitToNew,
  /// New
  New,
  /// Wait To Replace
  WaitToReplace,
  /// Pending Replace
  PendingReplace,
  /// Replaced
  Replaced,
  /// Partial Filled
  PartialFilled,
  /// Wait To Cancel
  WaitToCancel,
  /// Pending Cancel
  PendingCancel,
  /// Rejected
  Rejected,
  /// Canceled
  Canceled,
  /// Expired
  Expired,
  /// Partial Withdrawal
  PartialWithdrawal,
};

/// Order side
enum class OrderSide
{
  /// Unknown
  Unknown,
  /// Buy
  Buy,
  /// Sell
  Sell,
};

/// Order type
enum class OrderType
{
  /// Unknown
  Unknown,
  /// Limit Order
  LO,
  /// Enhanced Limit Order
  ELO,
  /// Market Order
  MO,
  /// At-auction Order
  AO,
  /// At-auction Limit Order
  ALO,
  /// Odd Lots
  ODD,
  /// Limit If Touched
  LIT,
  /// Market If Touched
  MIT,
  /// Trailing Limit If Touched (Trailing Amount)
  TSLPAMT,
  /// Trailing Limit If Touched (Trailing Percent)
  TSLPPCT,
  /// Trailing Market If Touched (Trailing Amount)
  TSMAMT,
  /// Trailing Market If Touched (Trailing Percent)
  TSMPCT,
  /// Special Limit Order
  SLO,
};

/// Order tag
enum class OrderTag
{
  /// Unknown
  Unknown,
  /// Normal Order
  Normal,
  /// Long term Order
  LongTerm,
  /// Grey Order
  Grey,
  /// Force Selling
  MarginCall,
  /// OTC
  Offline,
  /// Option Exercise Long
  Creditor,
  /// Option Exercise Short
  Debtor,
  /// Wavier Of Option Exercise
  NonExercise,
  /// Trade Allocation
  AllocatedSub,
};

/// Time in force Type
enum class TimeInForceType
{
  /// Unknown
  Unknown,
  /// Day Order
  Day,
  /// Good Til Canceled Order
  GoodTilCanceled,
  /// Good Til Date Order
  GoodTilDate,
};

/// Trigger status
enum class TriggerStatus
{
  /// Unknown
  Unknown,
  /// Deactive
  Deactive,
  /// Active
  Active,
  /// Released
  Released,
};

/// Enable or disable outside regular trading hours
enum class OutsideRTH
{
  /// Unknown
  Unknown,
  /// Regular trading hour only
  RTHOnly,
  /// Any time
  AnyTime,
  /// Overnight
  Overnight,
};

/// Order
struct Order
{
  /// Order ID
  std::string order_id;
  /// Order status
  OrderStatus status;
  /// Stock name
  std::string stock_name;
  /// Submitted quantity
  Decimal quantity;
  /// Executed quantity
  Decimal executed_quantity;
  /// Submitted price
  std::optional<Decimal> price;
  /// Executed price
  std::optional<Decimal> executed_price;
  /// Submitted time
  int64_t submitted_at;
  /// Order side
  OrderSide side;
  /// Security code
  std::string symbol;
  /// Order type
  OrderType order_type;
  /// Last done
  std::optional<Decimal> last_done;
  /// `LIT` / `MIT` Order Trigger Price
  std::optional<Decimal> trigger_price;
  /// Rejected Message or remark
  std::string msg;
  /// Order tag
  OrderTag tag;
  /// Time in force type
  TimeInForceType time_in_force;
  /// Long term order expire date
  std::optional<Date> expire_date;
  /// Last updated time
  std::optional<int64_t> updated_at;
  /// Conditional order trigger time
  std::optional<int64_t> trigger_at;
  /// `TSMAMT` / `TSLPAMT` order trailing amount
  std::optional<Decimal> trailing_amount;
  /// `TSMPCT` / `TSLPPCT` order trailing percent
  std::optional<Decimal> trailing_percent;
  /// `TSLPAMT` / `TSLPPCT` order limit offset amount
  std::optional<Decimal> limit_offset;
  /// Conditional order trigger status
  std::optional<TriggerStatus> trigger_status;
  /// Currency
  std::string currency;
  /// Enable or disable outside regular trading hours
  std::optional<OutsideRTH> outside_rth;
  /// Remark
  std::string remark;
};

/// Order changed message
struct PushOrderChanged
{
  /// Order side
  OrderSide side;
  /// Stock name
  std::string stock_name;
  /// Submitted quantity
  Decimal submitted_quantity;
  /// Order symbol
  std::string symbol;
  /// Order type
  OrderType order_type;
  /// Submitted price
  Decimal submitted_price;
  /// Executed quantity
  Decimal executed_quantity;
  /// Executed price
  std::optional<Decimal> executed_price;
  /// Order ID
  std::string order_id;
  /// Currency
  std::string currency;
  /// Order status
  OrderStatus status;
  /// Submitted time
  int64_t submitted_at;
  /// Last updated time
  int64_t updated_at;
  /// Order trigger price
  std::optional<Decimal> trigger_price;
  /// Rejected message or remark
  std::string msg;
  /// Order tag
  OrderTag tag;
  /// Conditional order trigger status
  std::optional<TriggerStatus> trigger_status;
  /// Conditional order trigger time
  std::optional<int64_t> trigger_at;
  /// Trailing amount
  std::optional<Decimal> trailing_amount;
  /// Trailing percent
  std::optional<Decimal> trailing_percent;
  /// Limit offset amount
  std::optional<Decimal> limit_offset;
  /// Account no
  std::string account_no;
  /// Last share
  std::optional<Decimal> last_share;
  /// Last price
  std::optional<Decimal> last_price;
  /// Remark message
  std::string remark;
};

/// Options for get history orders request
struct GetHistoryOrdersOptions
{
  /// Security symbol
  std::optional<std::string> symbol;
  /// Order status
  std::optional<std::vector<OrderStatus>> status;
  /// Order side
  std::optional<OrderSide> side;
  /// Market
  std::optional<Market> market;
  /// Start time
  std::optional<int64_t> start_at;
  /// End time
  std::optional<int64_t> end_at;
};

/// Options for get today orders request
struct GetTodayOrdersOptions
{
  /// Security symbol
  std::optional<std::string> symbol;
  /// Order status
  std::optional<std::vector<OrderStatus>> status;
  /// Order side
  std::optional<OrderSide> side;
  /// Market
  std::optional<Market> market;
  /// Order id
  std::optional<std::string> order_id;
};

/// Options for replace order request
struct ReplaceOrderOptions
{
  /// Order ID
  std::string order_id;
  /// Quantity
  Decimal quantity;
  /// Price
  std::optional<Decimal> price;
  /// Trigger price
  std::optional<Decimal> trigger_price;
  /// Limit offset
  std::optional<Decimal> limit_offset;
  /// Trailing amount
  std::optional<Decimal> trailing_amount;
  /// Trailing percent
  std::optional<Decimal> trailing_percent;
  /// Remark
  std::optional<std::string> remark;
};

/// Options for submit order request
struct SubmitOrderOptions
{
  /// Security symbol
  std::string symbol;
  /// Order type
  OrderType order_type;
  /// Order side
  OrderSide side;
  /// Submitted price
  Decimal submitted_quantity;
  /// Time in force type
  TimeInForceType time_in_force;
  /// Submitted price
  std::optional<Decimal> submitted_price;
  /// Trigger price (`LIT` / `MIT` Required)
  std::optional<Decimal> trigger_price;
  /// Limit offset amount (`TSLPAMT` / `TSLPPCT` Required)
  std::optional<Decimal> limit_offset;
  /// Trailing amount (`TSLPAMT` / `TSMAMT` Required)
  std::optional<Decimal> trailing_amount;
  /// Trailing percent (`TSLPPCT` / `TSMAPCT` Required)
  std::optional<Decimal> trailing_percent;
  /// Long term order expire date (Required when `time_in_force` is
  /// `GoodTilDate`)
  std::optional<Date> expire_date;
  /// Enable or disable outside regular trading hours
  std::optional<OutsideRTH> outside_rth;
  /// Remark (Maximum 64 characters)
  std::optional<std::string> remark;
};

/// Response for submit order request
struct SubmitOrderResponse
{
  /// Order id
  std::string order_id;
};

/// Cash info
struct CashInfo
{
  /// Withdraw cash
  Decimal withdraw_cash;
  /// Available cash
  Decimal available_cash;
  /// Frozen cash
  Decimal frozen_cash;
  /// Cash to be settled
  Decimal settling_cash;
  /// Currency
  std::string currency;
};

/// Frozen transaction fee
struct FrozenTransactionFee
{
  std::string currency;
  Decimal frozen_transaction_fee;
};

/// Account balance
struct AccountBalance
{
  /// Total cash
  Decimal total_cash;
  /// Maximum financing amount
  Decimal max_finance_amount;
  /// Remaining financing amount
  Decimal remaining_finance_amount;
  /// Risk control level
  int32_t risk_level;
  /// Margin call
  Decimal margin_call;
  /// Currency
  std::string currency;
  /// Cash details
  std::vector<CashInfo> cash_infos;
  /// Net assets
  Decimal net_assets;
  /// Initial margin
  Decimal init_margin;
  /// Maintenance margin
  Decimal maintenance_margin;
  /// Buy power
  Decimal buy_power;
  /// Frozen transaction fees
  std::vector<FrozenTransactionFee> frozen_transaction_fees;
};

/// Cash flow direction
enum class CashFlowDirection
{
  /// Unknown
  Unknown,
  /// Out
  Out,
  /// In
  In,
};

/// Balance type
enum class BalanceType
{
  /// Unknown
  Unknown,
  /// Cash
  Cash,
  /// Stock
  Stock,
  /// Fund
  Fund,
};

/// Cash flow
struct CashFlow
{
  /// Cash flow name
  std::string transaction_flow_name;
  /// Outflow direction
  CashFlowDirection direction;
  /// Balance type
  BalanceType business_type;
  /// Cash amount
  Decimal balance;
  /// Cash currency
  std::string currency;
  /// Business time
  int64_t business_time;
  /// Associated Stock code information
  std::optional<std::string> symbol;
  /// Cash flow description
  std::string description;
};

/// Options for submit order request
struct GetCashFlowOptions
{
  /// Start time
  int64_t start_at;
  /// End time
  int64_t end_at;
  /// Business type
  std::optional<BalanceType> business_type;
  /// Security symbol
  std::optional<std::string> symbol;
  /// Page number
  std::optional<uintptr_t> page;
  /// Page size
  std::optional<uintptr_t> size;
};

/// Options for get fund positions request
struct GetFundPositionsOptions
{
  /// Fund symbols
  std::optional<std::vector<std::string>> symbols;
};

/// Options for get stock positions request
struct GetStockPositionsOptions
{
  /// Stock symbols
  std::optional<std::vector<std::string>> symbols;
};

/// Fund position
struct FundPosition
{
  /// Fund ISIN code
  std::string symbol;
  /// Current equity
  Decimal current_net_asset_value;
  /// Current equity time
  int64_t net_asset_value_day;
  /// Fund name
  std::string symbol_name;
  /// Currency
  std::string currency;
  /// Net cost
  Decimal cost_net_asset_value;
  /// Holding units
  Decimal holding_units;
};

/// Fund position channel
struct FundPositionChannel
{
  /// Account type
  std::string account_channel;
  /// Fund positions
  std::vector<FundPosition> positions;
};

/// Fund positions response
struct FundPositionsResponse
{
  /// Channels
  std::vector<FundPositionChannel> channels;
};

/// Stock position
struct StockPosition
{
  /// Stock code
  std::string symbol;
  /// Stock name
  std::string symbol_name;
  /// The number of holdings
  Decimal quantity;
  /// Available quantity
  Decimal available_quantity;
  /// Currency
  std::string currency;
  /// Cost Price(According to the client's choice of average purchase or diluted
  /// cost)
  Decimal cost_price;
  /// Market
  Market market;
  /// Initial position before market opening
  std::optional<Decimal> init_quantity;
};

/// Stock position channel
struct StockPositionChannel
{
  /// Account type
  std::string account_channel;
  /// Stock positions
  std::vector<StockPosition> positions;
};

/// Stock positions response
struct StockPositionsResponse
{
  /// Channels
  std::vector<StockPositionChannel> channels;
};

/// Margin ratio
struct MarginRatio
{
  /// Initial margin ratio
  Decimal im_factor;
  /// Maintain the initial margin ratio
  Decimal mm_factor;
  /// Forced close-out margin ratio
  Decimal fm_factor;
};

/// Commission-free Status
enum class CommissionFreeStatus
{
  Unknown,
  None,
  Calculated,
  Pending,
  Ready,
};

/// Deduction status
enum class DeductionStatus
{
  Unknown,
  None,
  NoData,
  Pending,
  Done,
};

/// Charge category code
enum class ChargeCategoryCode
{
  Unknown,
  Broker,
  Third,
};

/// Order history detail
struct OrderHistoryDetail
{
  /// Executed price for executed orders, submitted price for expired, canceled,
  /// rejected orders, etc.
  Decimal price;
  /// Executed quantity for executed orders, remaining quantity for expired,
  /// canceled, rejected orders, etc.
  Decimal quantity;
  /// Order status
  OrderStatus status;
  /// Execution or error message
  std::string msg;
  /// Occurrence time
  int64_t time;
};

/// Order charge fee
struct OrderChargeFee
{
  /// Charge code
  std::string code;
  /// Charge name
  std::string name;
  /// Charge amount
  Decimal amount;
  /// Charge currency
  std::string currency;
};

/// Order charge item
struct OrderChargeItem
{
  /// Charge category code
  ChargeCategoryCode code;
  /// Charge category name
  std::string name;
  /// Charge details
  std::vector<OrderChargeFee> fees;
};

/// Order charge detail
struct OrderChargeDetail
{
  /// Total charges amount
  Decimal total_amount;
  /// Settlement currency
  std::string currency;
  /// Order charge items
  std::vector<OrderChargeItem> items;
};

/// Order detail
struct OrderDetail
{
  /// Order ID
  std::string order_id;
  /// Order status
  OrderStatus status;
  /// Stock name
  std::string stock_name;
  /// Submitted quantity
  Decimal quantity;
  /// Executed quantity
  Decimal executed_quantity;
  /// Submitted price
  std::optional<Decimal> price;
  /// Executed price
  std::optional<Decimal> executed_price;
  /// Submitted time
  int64_t submitted_at;
  /// Order side
  OrderSide side;
  /// Security code
  std::string symbol;
  /// Order type
  OrderType order_type;
  /// Last done
  std::optional<Decimal> last_done;
  /// `LIT` / `MIT` Order Trigger Price
  std::optional<Decimal> trigger_price;
  /// Rejected Message or remark
  std::string msg;
  /// Order tag
  OrderTag tag;
  /// Time in force type
  TimeInForceType time_in_force;
  /// Long term order expire date
  std::optional<Date> expire_date;
  /// Last updated time
  std::optional<int64_t> updated_at;
  /// Conditional order trigger time
  std::optional<int64_t> trigger_at;
  /// `TSMAMT` / `TSLPAMT` order trailing amount
  std::optional<Decimal> trailing_amount;
  /// `TSMPCT` / `TSLPPCT` order trailing percent
  std::optional<Decimal> trailing_percent;
  /// `TSLPAMT` / `TSLPPCT` order limit offset amount
  std::optional<Decimal> limit_offset;
  /// Conditional order trigger status
  std::optional<TriggerStatus> trigger_status;
  /// Currency
  std::string currency;
  /// Enable or disable outside regular trading hours
  std::optional<OutsideRTH> outside_rth;
  /// Remark
  std::string remark;
  /// Commission-free Status
  CommissionFreeStatus free_status;
  /// Commission-free amount
  std::optional<Decimal> free_amount;
  /// Commission-free currency
  std::optional<std::string> free_currency;
  /// Deduction status
  DeductionStatus deductions_status;
  /// Deduction amount
  std::optional<Decimal> deductions_amount;
  /// Deduction currency
  std::optional<std::string> deductions_currency;
  /// Platform fee deduction status
  DeductionStatus platform_deducted_status;
  /// Platform deduction amount
  std::optional<Decimal> platform_deducted_amount;
  /// Platform deduction currency
  std::optional<std::string> platform_deducted_currency;
  /// Order history details
  std::vector<OrderHistoryDetail> history;
  /// Order charges
  OrderChargeDetail charge_detail;
};

/// Options for estimate maximum purchase quantity
struct EstimateMaxPurchaseQuantityOptions
{
  /// Security code
  std::string symbol;
  /// Order type
  OrderType order_type;
  /// Order side
  OrderSide side;
  /// Estimated order price
  std::optional<Decimal> price;
  /// Settlement currency
  std::optional<std::string> currency;
  /// Order ID, required when estimating the maximum purchase quantity for a
  /// modified order
  std::optional<std::string> order_id;
  /// Get the maximum fractional share buying power
  bool fractional_shares;
};

/// Response for estimate maximum purchase quantity
struct EstimateMaxPurchaseQuantityResponse
{
  /// Cash available quantity
  Decimal cash_max_qty;
  /// Margin available quantity
  Decimal margin_max_qty;
};

} // namespace trade

} // namespace longport