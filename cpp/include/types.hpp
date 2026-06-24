#pragma once

#include "decimal.hpp"
#include <map>
#include <optional>
#include <vector>

namespace longbridge {

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
  /// Dividend (per share), **not** the dividend yield (ratio).
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

/// Filing item
struct FilingItem
{
  /// Filing ID
  std::string id;
  /// Title
  std::string title;
  /// Description
  std::string description;
  /// File name
  std::string file_name;
  /// File URLs
  std::vector<std::string> file_urls;
  /// Published time (Unix timestamp)
  int64_t published_at;
};

/// One short-position record, unified for US and HK markets.
struct ShortPositionsItem
{
  /// Trading date in RFC 3339 format
  std::string timestamp;
  /// Short ratio
  std::string rate;
  /// Closing price
  std::string close;
  /// [US] Number of short shares outstanding
  std::string current_shares_short;
  /// [US] Average daily share volume
  std::string avg_daily_share_volume;
  /// [US] Days-to-cover ratio
  std::string days_to_cover;
  /// [HK] Short sale amount (HKD)
  std::string amount;
  /// [HK] Short position balance
  std::string balance;
  /// [HK] Closing price (HK naming)
  std::string cost;
};

/// Short interest / positions response (HK or US).
struct ShortPositionsResponse
{
  /// Short position records
  std::vector<ShortPositionsItem> data;
};

/// One short-trade record, unified for US and HK markets.
struct ShortTradesItem
{
  /// Trading date in RFC 3339 format
  std::string timestamp;
  /// Short ratio
  std::string rate;
  /// Closing price
  std::string close;
  /// [US] NASDAQ short sale volume
  std::string nus_amount;
  /// [US] NYSE short sale volume
  std::string ny_amount;
  /// [US] Total short amount
  std::string total_amount;
  /// [HK] Short sale turnover amount (HKD)
  std::string amount;
  /// [HK] Short position balance
  std::string balance;
};

/// Short trade records response (HK or US).
struct ShortTradesResponse
{
  /// Short trade records
  std::vector<ShortTradesItem> data;
};

struct OptionVolumeStats
{
  std::string c;
  std::string p;
};

struct OptionVolumeDailyStat
{
  std::string symbol;
  std::string timestamp;
  std::string total_volume;
  std::string total_put_volume;
  std::string total_call_volume;
  std::string put_call_volume_ratio;
  std::string total_open_interest;
  std::string total_put_open_interest;
  std::string total_call_open_interest;
  std::string put_call_open_interest_ratio;
};

struct OptionVolumeDaily
{
  std::vector<OptionVolumeDailyStat> stats;
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

/// Attached order type
enum class AttachedOrderType
{
  /// Unknown
  Unknown,
  /// Take profit
  ProfitTaker,
  /// Stop loss
  StopLoss,
  /// Bracket order
  Bracket,
};

/// Attached order detail
struct AttachedOrderDetail
{
  std::string order_id;
  AttachedOrderType attached_type_display;
  std::optional<Decimal> trigger_price;
  Decimal quantity;
  Decimal executed_qty;
  OrderStatus status;
  int64_t updated_at;
  bool withdrawn;
  std::optional<Date> gtd;
  TimeInForceType time_in_force;
  std::string counter_id;
  std::optional<TriggerStatus> trigger_status;
  Decimal executed_amount;
  OrderTag tag;
  int64_t submitted_at;
  std::optional<Decimal> executed_price;
  std::optional<OutsideRTH> force_only_rth;
  bool reviewed;
  OrderType activate_order_type;
  std::optional<OutsideRTH> activate_rth;
  std::optional<Decimal> submit_price;
};

/// Submit attached order params
struct SubmitAttachedParams
{
  AttachedOrderType attached_order_type;
  std::optional<Decimal> profit_taker_price;
  std::optional<Decimal> stop_loss_price;
  std::optional<TimeInForceType> time_in_force;
  std::optional<int64_t> expire_time;
  std::optional<OrderType> activate_order_type;
  std::optional<Decimal> profit_taker_submit_price;
  std::optional<Decimal> stop_loss_submit_price;
  std::optional<OutsideRTH> activate_rth;
};

/// Replace attached order params
struct ReplaceAttachedParams
{
  AttachedOrderType attached_order_type;
  std::optional<Decimal> profit_taker_price;
  std::optional<Decimal> stop_loss_price;
  std::optional<TimeInForceType> time_in_force;
  std::optional<int64_t> expire_time;
  std::optional<OrderType> activate_order_type;
  std::optional<Decimal> profit_taker_submit_price;
  std::optional<Decimal> stop_loss_submit_price;
  std::optional<OutsideRTH> activate_rth;
  std::optional<int64_t> profit_taker_id;
  std::optional<int64_t> stop_loss_id;
  std::optional<bool> cancel_all_attached;
  std::optional<int64_t> main_id;
  std::optional<Decimal> quantity;
  std::optional<Decimal> market_price;
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
  /// Limit depth level
  std::optional<int32_t> limit_depth_level;
  /// Trigger count
  std::optional<int32_t> trigger_count;
  /// Monitor price
  std::optional<Decimal> monitor_price;
  /// Remark
  std::string remark;
  /// Attached orders
  std::vector<AttachedOrderDetail> attached_orders;
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
  /// Whether to include attached orders
  std::optional<bool> is_attached;
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
  /// Limit depth level
  std::optional<int32_t> limit_depth_level;
  /// Trigger count
  std::optional<int32_t> trigger_count;
  /// Monitor price
  std::optional<Decimal> monitor_price;
  /// Remark
  std::optional<std::string> remark;
  /// Attached order params
  std::optional<ReplaceAttachedParams> attached_params;
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
  /// Limit depth level
  std::optional<int32_t> limit_depth_level;
  /// Trigger count
  std::optional<int32_t> trigger_count;
  /// Monitor price
  std::optional<Decimal> monitor_price;
  /// Remark (Maximum 64 characters)
  std::optional<std::string> remark;
  /// Attached order params
  std::optional<SubmitAttachedParams> attached_params;
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
  /// Limit depth level
  std::optional<int32_t> limit_depth_level;
  /// Trigger count
  std::optional<int32_t> trigger_count;
  /// Monitor price
  std::optional<Decimal> monitor_price;
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
  /// Order charges (maybe null)
  std::optional<OrderChargeDetail> charge_detail;
  /// Attached orders
  std::vector<AttachedOrderDetail> attached_orders;
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

namespace content {

/// Topic item
struct TopicItem
{
  /// Topic ID
  std::string id;
  /// Title
  std::string title;
  /// Description
  std::string description;
  /// URL
  std::string url;
  /// Published time (Unix timestamp)
  int64_t published_at;
  /// Comments count
  int32_t comments_count;
  /// Likes count
  int32_t likes_count;
  /// Shares count
  int32_t shares_count;
};

/// News item
struct NewsItem
{
  /// News ID
  std::string id;
  /// Title
  std::string title;
  /// Description
  std::string description;
  /// URL
  std::string url;
  /// Published time (Unix timestamp)
  int64_t published_at;
  /// Comments count
  int32_t comments_count;
  /// Likes count
  int32_t likes_count;
  /// Shares count
  int32_t shares_count;
};

/// Topic author
struct TopicAuthor
{
  /// Member ID
  std::string member_id;
  /// Display name
  std::string name;
  /// Avatar URL
  std::string avatar;
};

/// Topic image
struct TopicImage
{
  /// Original image URL
  std::string url;
  /// Small thumbnail URL
  std::string sm;
  /// Large image URL
  std::string lg;
};

/// My topic item (created by the current authenticated user)
struct OwnedTopic
{
  /// Topic ID
  std::string id;
  /// Title
  std::string title;
  /// Plain text excerpt
  std::string description;
  /// Markdown body
  std::string body;
  /// Author
  TopicAuthor author;
  /// Related stock tickers
  std::vector<std::string> tickers;
  /// Hashtag names
  std::vector<std::string> hashtags;
  /// Images
  std::vector<TopicImage> images;
  /// Likes count
  int32_t likes_count;
  /// Comments count
  int32_t comments_count;
  /// Views count
  int32_t views_count;
  /// Shares count
  int32_t shares_count;
  /// Content type: "article" or "post"
  std::string topic_type;
  /// URL to the full topic page
  std::string detail_url;
  /// Created time (Unix timestamp)
  int64_t created_at;
  /// Updated time (Unix timestamp)
  int64_t updated_at;
};

/// Options for listing topics created by the current authenticated user
struct MyTopicsOptions
{
  /// Page number (0 = default 1)
  int32_t page = 0;
  /// Records per page, range 1~500 (0 = default 50)
  int32_t size = 0;
  /// Filter by content type: "article" or "post" (empty = all)
  std::string topic_type;
};

/// Options for creating a topic
struct CreateTopicOptions
{
  /// Topic title (required)
  std::string title;
  /// Topic body in Markdown format (required)
  std::string body;
  /// Content type: "article" or "post" (empty = default "post")
  std::string topic_type;
  /// Related stock tickers, format: {symbol}.{market}, max 10
  std::vector<std::string> tickers;
  /// Hashtag names, max 5
  std::vector<std::string> hashtags;
};

} // namespace content

// ── MarketContext types ───────────────────────────────────────────
namespace market {

/// Current trading status and timestamps for one market.
struct MarketTimeItem
{
  longbridge::Market market;
  int32_t trade_status;
  std::string timestamp;
  int32_t delay_trade_status;
  std::string delay_timestamp;
  int32_t sub_status;
  int32_t delay_sub_status;
};

/// Response containing trading status for all markets.
struct MarketStatusResponse
{
  std::vector<MarketTimeItem> market_time;
};

/// One broker's holding entry in a top-holders list.
struct BrokerHoldingEntry
{
  std::string name;
  std::string parti_number;
  std::string chg;
  bool strong;
};

/// Top broker holders (buy and sell sides).
struct BrokerHoldingTop
{
  std::vector<BrokerHoldingEntry> buy;
  std::vector<BrokerHoldingEntry> sell;
  std::string updated_at;
};

/// Holding change figures over multiple periods for a broker.
struct BrokerHoldingChanges
{
  std::string value;
  std::string chg_1;
  std::string chg_5;
  std::string chg_20;
  std::string chg_60;
};

/// Detailed holding entry for one broker including ratio and share changes.
struct BrokerHoldingDetailItem
{
  std::string name;
  std::string parti_number;
  BrokerHoldingChanges ratio;
  BrokerHoldingChanges shares;
  bool strong;
};

/// Full broker holding detail with historical change data.
struct BrokerHoldingDetail
{
  std::vector<BrokerHoldingDetailItem> list;
  std::string updated_at;
};

/// One day's broker holding snapshot.
struct BrokerHoldingDailyItem
{
  std::string date;
  std::string holding;
  std::string ratio;
  std::string chg;
};

/// Historical daily broker holding series.
struct BrokerHoldingDailyHistory
{
  std::vector<BrokerHoldingDailyItem> list;
};

/// A/H premium candlestick data point.
struct AhPremiumKline
{
  std::string aprice;
  std::string apreclose;
  std::string hprice;
  std::string hpreclose;
  std::string currency_rate;
  std::string ahpremium_rate;
  std::string price_spread;
  int64_t timestamp;
};

/// Historical A/H premium kline series.
struct AhPremiumKlines
{
  std::vector<AhPremiumKline> klines;
};

/// Intraday A/H premium kline series.
struct AhPremiumIntraday
{
  std::vector<AhPremiumKline> klines;
};

/// Trade volume and amount aggregated at one price level.
struct TradePriceLevel
{
  std::string buy_amount;
  std::string neutral_amount;
  std::string price;
  std::string sell_amount;
};

/// Aggregate buy/sell/neutral trade statistics for a security.
struct TradeStatistics
{
  std::string avgprice;
  std::string buy;
  std::string neutral;
  std::string preclose;
  std::string sell;
  std::string timestamp;
  std::string total_amount;
  std::vector<std::string> trade_date;
  std::string trades_count;
};

/// Response for trade statistics including per-price-level breakdown.
struct TradeStatsResponse
{
  TradeStatistics statistics;
  std::vector<TradePriceLevel> trades;
};

/// Stock information within a top-movers event.
struct TopMoversStock
{
  std::string symbol;
  std::string code;
  std::string name;
  std::string full_name;
  std::string change;
  std::string last_done;
  std::string market;
  std::string logo;
  std::vector<std::string> labels;
};

/// One top-movers event entry.
struct TopMoversEvent
{
  std::string timestamp;
  std::string alert_reason;
  int64_t alert_type;
  TopMoversStock stock;
  std::string post;
};

/// Response for top_movers.
struct TopMoversResponse
{
  std::vector<TopMoversEvent> events;
  /// Pagination cursor as a JSON string
  std::string next_params;
};

/// One ranked security item.
struct RankListItem
{
  std::string symbol;
  std::string code;
  std::string name;
  std::string last_done;
  std::string chg;
  std::string change;
  std::string inflow;
  std::string market_cap;
  std::string industry;
  std::string pre_post_price;
  std::string pre_post_chg;
  std::string amplitude;
  std::string five_day_chg;
  std::string turnover_rate;
  std::string volume_rate;
  std::string pb_ttm;
};

/// Response for rank_list.
struct RankListResponse
{
  bool bmp;
  std::vector<RankListItem> lists;
};

/// A single anomaly (unusual market movement) alert item.
struct AnomalyItem
{
  std::string symbol;
  std::string name;
  std::string alert_name;
  int64_t alert_time;
  std::vector<std::string> change_values;
  int32_t emotion;
};

/// Response containing anomaly alert items.
struct AnomalyResponse
{
  bool all_off;
  std::vector<AnomalyItem> changes;
};

/// One constituent stock of an index.
struct ConstituentStock
{
  std::string symbol;
  std::string name;
  std::string last_done;
  std::string prev_close;
  std::string inflow;
  std::string balance;
  std::string amount;
  std::string total_shares;
  std::vector<std::string> tags;
  std::string intro;
  std::string market;
  std::string circulating_shares;
  bool delay;
  std::string chg;
  int32_t trade_status;
};

/// Index constituent stocks with rise/fall/flat counts.
struct IndexConstituents
{
  int32_t fall_num;
  int32_t flat_num;
  int32_t rise_num;
  std::vector<ConstituentStock> stocks;
};

} // namespace market

// ── FundamentalContext types ──────────────────────────────────────
namespace fundamental {

/// Institutional analyst recommendation.
enum class InstitutionRecommend
{
  Unknown     = 0,
  StrongBuy   = 1,
  Buy         = 2,
  Hold        = 3,
  Sell        = 4,
  StrongSell  = 5,
  Underperform = 6,
};

/// One dividend event for a security.
struct DividendItem
{
  std::string symbol;
  std::string id;
  std::string desc;
  std::string record_date;
  std::string ex_date;
  std::string payment_date;
};

/// List of dividend events.
struct DividendList
{
  std::vector<DividendItem> list;
};

/// Buy/sell/hold evaluation counts from institutional analysts.
struct RatingEvaluate
{
  int32_t buy;
  int32_t over;
  int32_t hold;
  int32_t under;
  int32_t sell;
  int32_t no_opinion;
  int32_t total;
  std::string start_date;
  std::string end_date;
};

/// Analyst price target range.
struct RatingTarget
{
  std::string highest_price;
  std::string lowest_price;
  std::string prev_close;
  std::string start_date;
  std::string end_date;
};

/// Summary evaluation counts for one rating period.
struct RatingSummaryEvaluate
{
  int32_t buy;
  std::string date;
  int32_t hold;
  int32_t sell;
  int32_t strong_buy;
  int32_t under;
};

/// Latest institutional rating data including industry comparison.
struct InstitutionRatingLatest
{
  RatingEvaluate evaluate;
  RatingTarget target;
  int64_t industry_id;
  std::string industry_name;
  int32_t industry_rank;
  int32_t industry_total;
  int32_t industry_mean;
  int32_t industry_median;
};

/// Institutional rating summary with current recommendation and target price.
struct InstitutionRatingSummary
{
  std::string ccy_symbol;
  std::string change;
  RatingSummaryEvaluate evaluate;
  InstitutionRecommend recommend;
  std::string target;
  std::string updated_at;
};

/// Combined latest and summary institutional rating data.
struct InstitutionRating
{
  InstitutionRatingLatest latest;
  InstitutionRatingSummary summary;
};

/// One evaluation data point in an institutional rating detail series.
struct InstitutionRatingDetailEvaluateItem
{
  int32_t buy;
  std::string date;
  int32_t hold;
  int32_t sell;
  int32_t strong_buy;
  int32_t under;
};

/// One target price data point in an institutional rating detail series.
struct InstitutionRatingDetailTargetItem
{
  std::string avg_target;
  std::string date;
  std::string max_target;
  std::string min_target;
  bool meet;
  std::string price;
  std::string timestamp;
};

/// Detailed institutional rating including historical evaluate and target series.
struct InstitutionRatingDetail
{
  std::string ccy_symbol;
  std::vector<InstitutionRatingDetailEvaluateItem> evaluate_list;
  std::string data_percent;
  std::string prediction_accuracy;
  std::string updated_at;
  std::vector<InstitutionRatingDetailTargetItem> target_list;
};

/// Forecast EPS data point from institutional analysts.
struct ForecastEpsItem
{
  std::string forecast_eps_median;
  std::string forecast_eps_mean;
  std::string forecast_eps_lowest;
  std::string forecast_eps_highest;
  int32_t institution_total;
  int32_t institution_up;
  int32_t institution_down;
  int64_t forecast_start_date;
  int64_t forecast_end_date;
};

/// Collection of forecast EPS items.
struct ForecastEps
{
  std::vector<ForecastEpsItem> items;
};

/// One data point in a valuation time series.
struct ValuationPoint
{
  int64_t timestamp;
  std::string value;
};

/// Historical data for one valuation metric (PE, PB, PS, or dividend yield).
struct ValuationMetricData
{
  std::string desc;
  std::string high;
  std::string low;
  std::string median;
  std::vector<ValuationPoint> list;
};

/// All valuation metrics for a security.
struct ValuationMetricsData
{
  std::optional<ValuationMetricData> pe;
  std::optional<ValuationMetricData> pb;
  std::optional<ValuationMetricData> ps;
  std::optional<ValuationMetricData> dvd_yld;
};

/// Valuation data container.
struct ValuationData
{
  ValuationMetricsData metrics;
};

/// Historical valuation response (PE, PB, PS without dividend yield).
struct ValuationHistoryResponse
{
  std::optional<ValuationMetricData> pe;
  std::optional<ValuationMetricData> pb;
  std::optional<ValuationMetricData> ps;
};

/// Company overview and profile information.
struct CompanyOverview
{
  std::string name;
  std::string company_name;
  std::string founded;
  std::string listing_date;
  std::string market;
  std::string region;
  std::string address;
  std::string office_address;
  std::string website;
  std::string issue_price;
  std::string shares_offered;
  std::string chairman;
  std::string secretary;
  std::string audit_inst;
  std::string category;
  std::string year_end;
  std::string employees;
  std::string phone;
  std::string fax;
  std::string email;
  std::string legal_repr;
  std::string manager;
  std::string ticker;
  std::string profile;
  int32_t sector;
};

/// A security held by a shareholder.
struct ShareholderStock
{
  std::string symbol;
  std::string code;
  std::string market;
  std::string chg;
};

/// One institutional or major shareholder record.
struct Shareholder
{
  std::string shareholder_id;
  std::string shareholder_name;
  std::string institution_type;
  std::string percent_of_shares;
  std::string shares_changed;
  std::string report_date;
  std::vector<ShareholderStock> stocks;
};

/// Paginated list of shareholders.
struct ShareholderList
{
  std::vector<Shareholder> shareholder_list;
  std::string forward_url;
  int32_t total;
};

/// One fund's holding record for a security.
struct FundHolder
{
  std::string code;
  std::string symbol;
  std::string currency;
  std::string name;
  std::string position_ratio;
  std::string report_date;
};

/// Collection of fund holders for a security.
struct FundHolders
{
  std::vector<FundHolder> lists;
};

/// One corporate action event (dividend, split, etc.).
struct CorpActionItem
{
  std::string id;
  std::string date;
  std::string date_str;
  std::string date_type;
  std::string date_zone;
  std::string act_type;
  std::string act_desc;
  std::string action;
  bool recent;
  bool is_delay;
  std::string delay_content;
};

/// Collection of corporate action events.
struct CorpActions
{
  std::vector<CorpActionItem> items;
};

/// One security in an investment relationship (parent/subsidiary holding).
struct InvestSecurity
{
  std::string company_id;
  std::string company_name;
  std::string company_name_en;
  std::string company_name_zhcn;
  std::string symbol;
  std::string currency;
  std::string percent_of_shares;
  std::string shares_rank;
  std::string shares_value;
};

/// Investment relationship data including parent/subsidiary securities.
struct InvestRelations
{
  std::string forward_url;
  std::vector<InvestSecurity> invest_securities;
};

/// One operating indicator from a financial report.
struct OperatingIndicator
{
  std::string field_name;
  std::string indicator_name;
  std::string indicator_value;
  std::string yoy;
};

/// One operating report item with associated indicators.
struct OperatingItem
{
  std::string id;
  std::string report;
  std::string title;
  std::string txt;
  bool latest;
  std::string web_url;
  std::string financial_currency;
  std::string financial_name;
  std::string financial_region;
  std::string financial_report;
  std::vector<OperatingIndicator> indicators;
};

/// List of operating report items.
struct OperatingList
{
  std::vector<OperatingItem> list;
};

/// Financial reports — list_json contains serialized JSON
struct FinancialReports
{
  std::string list_json;
};

/// One consensus estimate detail for a financial metric.
struct ConsensusDetail
{
  std::string key;
  std::string name;
  std::string description;
  std::string actual;
  std::string estimate;
  std::string comp_value;
  std::string comp_desc;
  std::string comp;
  bool is_released;
};

/// Consensus report for one fiscal period.
struct ConsensusReport
{
  int32_t fiscal_year;
  std::string fiscal_period;
  std::string period_text;
  std::vector<ConsensusDetail> details;
};

/// Financial consensus response.
struct FinancialConsensus
{
  std::vector<ConsensusReport> list;
  int32_t current_index;
  std::string currency;
  std::vector<std::string> opt_periods;
  std::string current_period;
};

/// Historical valuation snapshot for an industry peer.
struct IndustryValuationHistory
{
  std::string date;
  std::string pe;
  std::string pb;
  std::string ps;
};

/// Valuation data for one industry peer security.
struct IndustryValuationItem
{
  std::string symbol;
  std::string name;
  std::string currency;
  std::string assets;
  std::string bps;
  std::string eps;
  std::string dps;
  std::string div_yld;
  std::string div_payout_ratio;
  std::string five_y_avg_dps;
  std::string pe;
  std::vector<IndustryValuationHistory> history;
};

/// List of industry valuation items.
struct IndustryValuationList
{
  std::vector<IndustryValuationItem> list;
};

/// Distribution statistics for one valuation metric within an industry.
struct ValuationDist
{
  std::string low;
  std::string high;
  std::string median;
  std::string value;
  std::string ranking;
  std::string rank_index;
  std::string rank_total;
};

/// Industry valuation distribution for PE, PB, PS ratios.
struct IndustryValuationDist
{
  std::optional<ValuationDist> pe;
  std::optional<ValuationDist> pb;
  std::optional<ValuationDist> ps;
};

/// One executive or board member.
struct Professional
{
  std::string id;
  std::string name;
  std::string name_zhcn;
  std::string name_en;
  std::string title;
  std::string biography;
  std::string photo;
  std::string wiki_url;
};

/// Executives for one security.
struct ExecutiveGroup
{
  std::string symbol;
  std::string forward_url;
  int32_t total;
  std::vector<Professional> professionals;
};

/// List of executive groups per security.
struct ExecutiveList
{
  std::vector<ExecutiveGroup> professional_list;
};

/// TTM (trailing twelve months) buyback summary.
struct RecentBuybacks
{
  std::string currency;
  std::string net_buyback_ttm;
  std::string net_buyback_yield_ttm;
};

/// Historical annual buyback data point.
struct BuybackHistoryItem
{
  std::string fiscal_year;
  std::string fiscal_year_range;
  std::string net_buyback;
  std::string net_buyback_yield;
  std::string net_buyback_growth_rate;
  std::string currency;
};

/// Buyback payout and cash-flow ratios.
struct BuybackRatios
{
  std::string net_buyback_payout_ratio;
  std::string net_buyback_to_cashflow_ratio;
};

/// Buyback data response.
struct BuybackData
{
  std::optional<RecentBuybacks> recent_buybacks;
  std::vector<BuybackHistoryItem> buyback_history;
  std::vector<BuybackRatios> buyback_ratios;
};

/// A leaf rating indicator with a raw value.
struct RatingLeafIndicator
{
  std::string name;
  std::string value;
  std::string value_type;
  std::string score;
  std::string letter;
};

/// A rating indicator node (parent or leaf).
struct RatingIndicator
{
  std::string name;
  std::string score;
  std::string letter;
};

/// A group of sub-indicators under one category indicator.
struct RatingSubIndicatorGroup
{
  RatingIndicator indicator;
  std::vector<RatingLeafIndicator> sub_indicators;
};

/// One rating category (e.g. growth, profitability).
struct RatingCategory
{
  int32_t kind;
  std::vector<RatingSubIndicatorGroup> sub_indicators;
};

/// Stock ratings response.
struct StockRatings
{
  std::string style_txt_name;
  std::string scale_txt_name;
  std::string report_period_txt;
  std::string multi_score;
  std::string multi_letter;
  int32_t multi_score_change;
  std::string industry_name;
  std::string industry_rank;
  std::string industry_total;
  std::string industry_mean_score;
  std::string industry_median_score;
  std::vector<RatingCategory> ratings;
};

/// One business segment item (latest snapshot).
struct BusinessSegmentItem
{
  std::string name;
  std::string percent;
};

/// Business segments response.
struct BusinessSegments
{
  std::string date;
  std::string total;
  std::string currency;
  std::vector<BusinessSegmentItem> business;
};

/// One business/regional segment item in a historical snapshot.
struct BusinessSegmentHistoryItem
{
  std::string name;
  std::string percent;
  std::string value;
};

/// One historical business segments snapshot.
struct BusinessSegmentsHistoricalItem
{
  std::string date;
  std::string total;
  std::string currency;
  std::vector<BusinessSegmentHistoryItem> business;
  std::vector<BusinessSegmentHistoryItem> regionals;
};

/// Business segments history response.
struct BusinessSegmentsHistory
{
  std::vector<BusinessSegmentsHistoricalItem> historical;
};

/// One historical rating distribution snapshot.
struct InstitutionRatingViewItem
{
  std::string date;
  std::string buy;
  std::string over;
  std::string hold;
  std::string under;
  std::string sell;
  std::string total;
};

/// Institution rating views response.
struct InstitutionRatingViews
{
  std::vector<InstitutionRatingViewItem> elist;
};

/// One ranked industry item.
struct IndustryRankItem
{
  std::string name;
  std::string counter_id;
  std::string chg;
  std::string leading_name;
  std::string leading_ticker;
  std::string leading_chg;
  std::string value_name;
  std::string value_data;
};

/// A group of ranked industry items.
struct IndustryRankGroup
{
  std::vector<IndustryRankItem> lists;
};

/// Industry rank response.
struct IndustryRankResponse
{
  std::vector<IndustryRankGroup> items;
};

/// Top-level industry info in the peers response.
struct IndustryPeersTop
{
  std::string name;
  std::string market;
};

/// A node in the recursive industry peer chain.
///
/// next_json contains the child nodes serialised as a JSON string.
struct IndustryPeerNode
{
  std::string name;
  std::string counter_id;
  int32_t stock_num;
  std::string chg;
  std::string ytd_chg;
  std::string next_json;
};

/// Industry peers response.
struct IndustryPeersResponse
{
  IndustryPeersTop top;
  std::optional<IndustryPeerNode> chain;
};

/// A forecast metric in the financial report snapshot.
struct SnapshotForecastMetric
{
  std::string value;
  std::string yoy;
  std::string cmp_desc;
  std::string est_value;
};

/// A reported metric in the financial report snapshot.
struct SnapshotReportedMetric
{
  std::string value;
  std::string yoy;
};

/// Financial report snapshot response.
struct FinancialReportSnapshot
{
  std::string name;
  std::string ticker;
  std::string fp_start;
  std::string fp_end;
  std::string currency;
  std::string report_desc;
  std::optional<SnapshotForecastMetric> fo_revenue;
  std::optional<SnapshotForecastMetric> fo_ebit;
  std::optional<SnapshotForecastMetric> fo_eps;
  std::optional<SnapshotReportedMetric> fr_revenue;
  std::optional<SnapshotReportedMetric> fr_profit;
  std::optional<SnapshotReportedMetric> fr_operate_cash;
  std::optional<SnapshotReportedMetric> fr_invest_cash;
  std::optional<SnapshotReportedMetric> fr_finance_cash;
  std::optional<SnapshotReportedMetric> fr_total_assets;
  std::optional<SnapshotReportedMetric> fr_total_liability;
  std::string fr_roe_ttm;
  std::string fr_profit_margin;
  std::string fr_profit_margin_ttm;
  std::string fr_asset_turn_ttm;
  std::string fr_leverage_ttm;
  std::string fr_debt_assets_ratio;
};

/// One historical valuation data point.
struct ValuationHistoryPoint
{
  std::string date;
  std::string pe;
  std::string pb;
  std::string ps;
};

/// One security's valuation comparison item.
struct ValuationComparisonItem
{
  std::string symbol;
  std::string name;
  std::string currency;
  std::string market_value;
  std::string price_close;
  std::string pe;
  std::string pb;
  std::string ps;
  std::string roe;
  std::string eps;
  std::string bps;
  std::string dps;
  std::string div_yld;
  std::string assets;
  std::vector<ValuationHistoryPoint> history;
};

/// Valuation comparison response.
struct ValuationComparisonResponse
{
  std::vector<ValuationComparisonItem> list;
};

/// ETF asset allocation element type
enum class ElementType
{
  /// Unknown
  Unknown,
  /// Holdings
  Holdings,
  /// Regional
  Regional,
  /// Asset class
  AssetClass,
  /// Industry
  Industry,
};

/// Holding detail of an ETF asset allocation element (holdings only)
struct HoldingDetail
{
  /// Industry ID
  std::string industry_id;
  /// Industry name
  std::string industry_name;
  /// Index counter ID (e.g. `BK/US/CP99000`)
  std::string index;
  /// Index name
  std::string index_name;
  /// Holding type (e.g. `E` for stock)
  std::string holding_type;
  /// Holding type name
  std::string holding_type_name;
};

/// One element of an ETF asset allocation group
struct AssetAllocationItem
{
  /// Element name
  std::string name;
  /// Security code (holdings only, e.g. `NVDA`)
  std::string code;
  /// Position ratio (e.g. `0.0861114`)
  std::string position_ratio;
  /// Security symbol (holdings only, e.g. `NVDA.US`)
  std::string symbol;
  /// Localized names (locale → name)
  std::map<std::string, std::string> name_locales;
  /// Holding detail (holdings only)
  std::optional<HoldingDetail> holding_detail;
};

/// One ETF asset allocation group (grouped by element type)
struct AssetAllocationGroup
{
  /// Report date (e.g. `20260601`)
  std::string report_date;
  /// Element type of this group
  ElementType asset_type;
  /// Elements
  std::vector<AssetAllocationItem> lists;
};

/// ETF asset allocation response
struct AssetAllocationResponse
{
  /// Asset allocation groups
  std::vector<AssetAllocationGroup> info;
};

} // namespace fundamental

namespace alert {

/// One price alert rule attached to a security.
struct AlertItem
{
  /// Alert ID
  std::string id;
  /// Condition: "1"=price_rise, "2"=price_fall, "3"=pct_rise, "4"=pct_fall
  std::string indicator_id;
  /// Whether the alert is currently active
  bool enabled;
  /// Trigger frequency: 1=daily, 2=every_time, 3=once
  int32_t frequency;
  /// Scope
  int32_t scope;
  /// Human-readable description of the trigger condition
  std::string text;
  /// Trigger state flags
  std::vector<int32_t> state;
  /// Trigger threshold, serialised as JSON: {"price":"500"} or {"chg":"5"}
  std::string value_map;
};

/// All price alerts for one security.
struct AlertSymbolGroup
{
  /// Security symbol
  std::string symbol;
  /// Ticker code (without market)
  std::string code;
  /// Market, e.g. "HK"
  std::string market;
  /// Security name
  std::string name;
  /// Latest price
  std::string price;
  /// Day change amount
  std::string chg;
  /// Day change percentage
  std::string p_chg;
  /// Product type (may be empty)
  std::string product;
  /// Alert items for this security
  std::vector<AlertItem> indicators;
};

/// Response for AlertContext::list — alerts grouped by security.
struct AlertList
{
  /// Alert groups, one per security
  std::vector<AlertSymbolGroup> lists;
};

} // namespace alert

namespace dca {

/// DCA investment frequency.
enum class DCAFrequency
{
  Daily       = 0,
  Weekly      = 1,
  Fortnightly = 2,
  Monthly     = 3,
};

/// DCA plan status.
enum class DCAStatus
{
  Active    = 0,
  Suspended = 1,
  Finished  = 2,
};

/// One DCA (dollar-cost averaging) investment plan.
struct DcaPlan
{
  /// Plan ID
  std::string plan_id;
  /// Plan status
  DCAStatus status;
  /// Security symbol
  std::string symbol;
  /// Member ID
  std::string member_id;
  /// Account ID
  std::string aaid;
  /// Account channel
  std::string account_channel;
  /// Display account
  std::string display_account;
  /// Market
  longbridge::Market market;
  /// Investment amount per period
  std::string per_invest_amount;
  /// Investment frequency
  DCAFrequency invest_frequency;
  /// Day of week for weekly plans (e.g. "Mon")
  std::string invest_day_of_week;
  /// Day of month for monthly plans
  std::string invest_day_of_month;
  /// Whether margin finance is allowed
  bool allow_margin_finance;
  /// Advance reminder hours ("1", "6", or "12")
  std::string alter_hours;
  /// Creation time
  std::string created_at;
  /// Last updated time
  std::string updated_at;
  /// Next investment date
  std::string next_trd_date;
  /// Security name
  std::string stock_name;
  /// Cumulative invested amount
  std::string cum_amount;
  /// Number of completed investment periods
  int64_t issue_number;
  /// Average cost
  std::string average_cost;
  /// Cumulative profit/loss
  std::string cum_profit;
};

/// Response for DCAContext::list and write operations.
struct DcaList
{
  /// DCA plans
  std::vector<DcaPlan> plans;
};

/// Response for DCAContext::stats — aggregate DCA statistics.
struct DcaStats
{
  /// Number of active plans
  std::string active_count;
  /// Number of finished plans
  std::string finished_count;
  /// Number of suspended plans
  std::string suspended_count;
  /// Nearest upcoming plans
  std::vector<DcaPlan> nearest_plans;
  /// Days until next investment
  std::string rest_days;
  /// Total invested amount
  std::string total_amount;
  /// Total profit/loss
  std::string total_profit;
};

/// DCA support info for one security.
struct DcaSupportInfo
{
  /// Security symbol
  std::string symbol;
  /// Whether DCA is supported for this security
  bool support_regular_saving;
};

/// Response for DCAContext::check_support.
struct DcaSupportList
{
  /// Support info per security
  std::vector<DcaSupportInfo> infos;
};

/// Response for DCAContext::calc_date — next projected trade date.
struct DcaCalcDateResult
{
  /// Next projected trade date (unix timestamp string)
  std::string trade_date;
};

/// Response for DCAContext::create_dca and DCAContext::update_dca.
struct DcaCreateResult
{
  /// The plan ID of the created or updated DCA plan.
  std::string plan_id;
};

/// One DCA execution history record.
struct DcaHistoryRecord
{
  std::string created_at;
  std::string order_id;
  std::string status;
  std::string action;
  std::string order_type;
  std::string executed_qty;
  std::string executed_price;
  std::string executed_amount;
  std::string rejected_reason;
  std::string symbol;
};

/// Paginated DCA execution history response.
struct DcaHistoryResponse
{
  std::vector<DcaHistoryRecord> records;
  bool has_more;
};

} // namespace dca

namespace sharelist {

/// A security constituent of a sharelist.
struct SharelistStock
{
  /// Security symbol
  std::string symbol;
  /// Security name
  std::string name;
  /// Market, e.g. "HK"
  std::string market;
  /// Ticker code
  std::string code;
  /// Brief description
  std::string intro;
  /// Unread change log category
  std::string unread_change_log_category;
  /// Day change percentage (absent when quote unavailable)
  std::optional<std::string> change;
  /// Latest price (absent when quote unavailable)
  std::optional<std::string> last_done;
  /// Trade status code (absent when quote unavailable)
  std::optional<int32_t> trade_status;
};

/// Subscription scope flags for a sharelist.
struct SharelistScopes
{
  /// Whether the current user is subscribed to this sharelist
  bool subscription;
  /// Whether the current user is the creator of this sharelist
  bool is_self;
};

/// Sharelist metadata and constituent stocks.
struct SharelistInfo
{
  /// Sharelist ID
  int64_t id;
  /// Name
  std::string name;
  /// Description
  std::string description;
  /// Cover image URL
  std::string cover;
  /// Number of subscribers
  int64_t subscribers_count;
  /// Creation time (Unix timestamp)
  int64_t created_at;
  /// Last stock edit time (Unix timestamp)
  int64_t edited_at;
  /// YTD change percentage
  std::string this_year_chg;
  /// Creator info (serialised JSON)
  std::string creator;
  /// Constituent stocks
  std::vector<SharelistStock> stocks;
  /// Whether the current user is subscribed
  bool subscribed;
  /// Day change percentage
  std::string chg;
  /// Sharelist type: 0=regular, 3=official, 4=industry
  int32_t sharelist_type;
  /// Industry code (for industry sharelists)
  std::string industry_code;
};

/// Response for SharelistContext::list and SharelistContext::popular.
struct SharelistList
{
  /// User's own and followed sharelists
  std::vector<SharelistInfo> sharelists;
  /// Subscribed sharelists (may be empty in popular response)
  std::vector<SharelistInfo> subscribed_sharelists;
  /// Pagination cursor for the subscribed list
  std::string tail_mark;
};

/// Response for SharelistContext::detail.
struct SharelistDetail
{
  /// Sharelist info including constituent stocks
  SharelistInfo sharelist;
  /// Subscription scope flags for the current user
  SharelistScopes scopes;
};

} // namespace sharelist

} // namespace longbridge