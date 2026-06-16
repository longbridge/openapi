#pragma once

#include "longbridge.h"
#include "types.hpp"
#include "portfolio_context.hpp"
#include <algorithm>
#include <iterator>
#include <stdexcept>


namespace longbridge {
namespace convert {

using longbridge::ErrorKind;
using longbridge::quote::AdjustType;
using longbridge::quote::Brokers;
using longbridge::quote::CalcIndex;
using longbridge::quote::Candlestick;
using longbridge::quote::CapitalDistribution;
using longbridge::quote::CapitalDistributionResponse;
using longbridge::quote::CapitalFlowLine;
using longbridge::quote::Depth;
using longbridge::quote::DerivativeType;
using longbridge::quote::FilterWarrantExpiryDate;
using longbridge::quote::FilterWarrantInOutBoundsType;
using longbridge::quote::Granularity;
using longbridge::quote::HistoryMarketTemperatureResponse;
using longbridge::quote::IntradayLine;
using longbridge::quote::IssuerInfo;
using longbridge::quote::MarketTemperature;
using longbridge::quote::MarketTradingDays;
using longbridge::quote::MarketTradingSession;
using longbridge::quote::OptionDirection;
using longbridge::quote::OptionQuote;
using longbridge::quote::OptionType;
using longbridge::quote::ParticipantInfo;
using longbridge::quote::Period;
using longbridge::quote::PrePostQuote;
using longbridge::quote::PushBrokers;
using longbridge::quote::PushCandlestick;
using longbridge::quote::PushDepth;
using longbridge::quote::PushQuote;
using longbridge::quote::PushTrades;
using longbridge::quote::QuotePackageDetail;
using longbridge::quote::RealtimeQuote;
using longbridge::quote::SecuritiesUpdateMode;
using longbridge::quote::Security;
using longbridge::quote::SecurityBoard;
using longbridge::quote::SecurityBrokers;
using longbridge::quote::SecurityCalcIndex;
using longbridge::quote::SecurityDepth;
using longbridge::quote::SecurityListCategory;
using longbridge::quote::SecurityQuote;
using longbridge::quote::SecurityStaticInfo;
using longbridge::quote::SortOrderType;
using longbridge::quote::StrikePriceInfo;
using longbridge::quote::SubFlags;
using longbridge::quote::Subscription;
using longbridge::quote::Trade;
using longbridge::quote::TradeDirection;
using longbridge::quote::TradeSession;
using longbridge::quote::TradeSessions;
using longbridge::quote::TradeStatus;
using longbridge::quote::TradingSessionInfo;
using longbridge::quote::WarrantInfo;
using longbridge::quote::WarrantQuote;
using longbridge::quote::WarrantSortBy;
using longbridge::quote::WarrantStatus;
using longbridge::quote::WarrantType;
using longbridge::quote::WatchlistGroup;
using longbridge::quote::WatchlistSecurity;
using longbridge::trade::AccountBalance;
using longbridge::trade::AttachedOrderDetail;
using longbridge::trade::AttachedOrderType;
using longbridge::trade::BalanceType;
using longbridge::trade::CashFlow;
using longbridge::trade::CashFlowDirection;
using longbridge::trade::CashInfo;
using longbridge::trade::ChargeCategoryCode;
using longbridge::trade::CommissionFreeStatus;
using longbridge::trade::DeductionStatus;
using longbridge::trade::Execution;
using longbridge::trade::FrozenTransactionFee;
using longbridge::trade::FundPosition;
using longbridge::trade::FundPositionChannel;
using longbridge::trade::FundPositionsResponse;
using longbridge::trade::GetHistoryExecutionsOptions;
using longbridge::trade::GetHistoryOrdersOptions;
using longbridge::trade::GetTodayExecutionsOptions;
using longbridge::trade::MarginRatio;
using longbridge::trade::Order;
using longbridge::trade::OrderChargeDetail;
using longbridge::trade::OrderChargeFee;
using longbridge::trade::OrderChargeItem;
using longbridge::trade::OrderDetail;
using longbridge::trade::OrderHistoryDetail;
using longbridge::trade::OrderSide;
using longbridge::trade::OrderStatus;
using longbridge::trade::OrderTag;
using longbridge::trade::OrderType;
using longbridge::trade::OutsideRTH;
using longbridge::trade::PushOrderChanged;
using longbridge::trade::ReplaceAttachedParams;
using longbridge::trade::StockPosition;
using longbridge::trade::StockPositionChannel;
using longbridge::trade::StockPositionsResponse;
using longbridge::trade::SubmitAttachedParams;
using longbridge::trade::SubmitOrderResponse;
using longbridge::trade::TimeInForceType;
using longbridge::trade::TopicType;
using longbridge::trade::TriggerStatus;
using longbridge::quote::FilingItem;
using longbridge::content::OwnedTopic;
using longbridge::content::NewsItem;
using longbridge::content::TopicAuthor;
using longbridge::content::TopicImage;
using longbridge::content::TopicItem;

inline lb_language_t
convert(Language language)
{
  switch (language) {
    case Language::ZH_CN:
      return Language_ZH_CN;
    case Language::ZH_HK:
      return Language_ZH_HK;
    case Language::EN:
      return Language_EN;
    default:
      throw std::invalid_argument("unreachable");
  }
}

inline ErrorKind
convert(lb_error_kind_t kind)
{
  switch (kind) {
    case ErrorKindHttp:
      return ErrorKind::Http;
    case ErrorKindOpenApi:
      return ErrorKind::OpenApi;
    case ErrorKindOther:
      return ErrorKind::Other;
    case ErrorKindOAuth:
      return ErrorKind::OAuth;
    default:
      throw std::invalid_argument("unreachable");
  }
}

inline lb_push_candlestick_mode_t
convert(PushCandlestickMode mode)
{
  switch (mode) {
    case PushCandlestickMode::Realtime:
      return PushCandlestickMode_Realtime;
    case PushCandlestickMode::Confirmed:
      return PushCandlestickMode_Confirmed;
    default:
      throw std::invalid_argument("unreachable");
  }
}

inline Market
convert(lb_market_t market)
{
  switch (market) {
    case MarketUnknown:
      return Market::Unknown;
    case MarketUS:
      return Market::US;
    case MarketHK:
      return Market::HK;
    case MarketCN:
      return Market::CN;
    case MarketSG:
      return Market::SG;
    case MarketCrypto:
      return Market::Crypto;
    default:
      throw std::invalid_argument("unreachable");
  }
}

inline lb_market_t
convert(Market market)
{
  switch (market) {
    case Market::Unknown:
      return MarketUnknown;
    case Market::US:
      return MarketUS;
    case Market::HK:
      return MarketHK;
    case Market::CN:
      return MarketCN;
    case Market::SG:
      return MarketSG;
    default:
      throw std::invalid_argument("unreachable");
  }
}

inline lb_period_t
convert(Period period)
{
  switch (period) {
    case Period::Unknown:
      return PeriodUnknown;
    case Period::Min1:
      return PeriodMin1;
    case Period::Min2:
      return PeriodMin2;
    case Period::Min3:
      return PeriodMin3;
    case Period::Min5:
      return PeriodMin5;
    case Period::Min10:
      return PeriodMin10;
    case Period::Min15:
      return PeriodMin15;
    case Period::Min20:
      return PeriodMin20;
    case Period::Min30:
      return PeriodMin30;
    case Period::Min45:
      return PeriodMin45;
    case Period::Min60:
      return PeriodMin60;
    case Period::Min120:
      return PeriodMin120;
    case Period::Min180:
      return PeriodMin180;
    case Period::Min240:
      return PeriodMin240;
    case Period::Day:
      return PeriodDay;
    case Period::Week:
      return PeriodWeek;
    case Period::Month:
      return PeriodMonth;
    case Period::Quarter:
      return PeriodQuarter;
    case Period::Year:
      return PeriodYear;
    default:
      throw std::invalid_argument("unreachable");
  }
}

inline Period
convert(lb_period_t period)
{
  switch (period) {
    case PeriodUnknown:
      return Period::Unknown;
    case PeriodMin1:
      return Period::Min1;
    case PeriodMin2:
      return Period::Min2;
    case PeriodMin3:
      return Period::Min3;
    case PeriodMin5:
      return Period::Min5;
    case PeriodMin15:
      return Period::Min15;
    case PeriodMin20:
      return Period::Min20;
    case PeriodMin30:
      return Period::Min30;
    case PeriodMin45:
      return Period::Min45;
    case PeriodMin60:
      return Period::Min60;
    case PeriodMin120:
      return Period::Min120;
    case PeriodMin180:
      return Period::Min180;
    case PeriodMin240:
      return Period::Min240;
    case PeriodDay:
      return Period::Day;
    case PeriodWeek:
      return Period::Week;
    case PeriodMonth:
      return Period::Month;
    case PeriodQuarter:
      return Period::Quarter;
    case PeriodYear:
      return Period::Year;
    default:
      throw std::invalid_argument("unreachable");
  }
}

inline SecurityBoard
convert(lb_security_board_t ty)
{
  switch (ty) {
    case SecurityBoardUnknown:
      return SecurityBoard::Unknown;
    case SecurityBoardUSMain:
      return SecurityBoard::USMain;
    case SecurityBoardUSPink:
      return SecurityBoard::USPink;
    case SecurityBoardUSDJI:
      return SecurityBoard::USDJI;
    case SecurityBoardUSNSDQ:
      return SecurityBoard::USNSDQ;
    case SecurityBoardUSSector:
      return SecurityBoard::USSector;
    case SecurityBoardUSOption:
      return SecurityBoard::USOption;
    case SecurityBoardUSOptionS:
      return SecurityBoard::USOptionS;
    case SecurityBoardHKEquity:
      return SecurityBoard::HKEquity;
    case SecurityBoardHKPreIPO:
      return SecurityBoard::HKPreIPO;
    case SecurityBoardHKWarrant:
      return SecurityBoard::HKWarrant;
    case SecurityBoardHKHS:
      return SecurityBoard::HKHS;
    case SecurityBoardHKSector:
      return SecurityBoard::HKSector;
    case SecurityBoardSHMainConnect:
      return SecurityBoard::SHMainConnect;
    case SecurityBoardSHMainNonConnect:
      return SecurityBoard::SHMainNonConnect;
    case SecurityBoardSHSTAR:
      return SecurityBoard::SHSTAR;
    case SecurityBoardCNIX:
      return SecurityBoard::CNIX;
    case SecurityBoardCNSector:
      return SecurityBoard::CNSector;
    case SecurityBoardSZMainConnect:
      return SecurityBoard::SZMainConnect;
    case SecurityBoardSZMainNonConnect:
      return SecurityBoard::SZMainNonConnect;
    case SecurityBoardSZGEMConnect:
      return SecurityBoard::SZGEMConnect;
    case SecurityBoardSZGEMNonConnect:
      return SecurityBoard::SZGEMNonConnect;
    case SecurityBoardSGMain:
      return SecurityBoard::SGMain;
    case SecurityBoardSTI:
      return SecurityBoard::STI;
    case SecurityBoardSGSector:
      return SecurityBoard::SGSector;
    case SecurityBoardSPXIndex:
      return SecurityBoard::SPXIndex;
    case SecurityBoardVIXIndex:
      return SecurityBoard::VIXIndex;
    default:
      throw std::invalid_argument("unreachable");
  }
}

inline SecurityStaticInfo
convert(const lb_security_static_info_t* info)
{
  return SecurityStaticInfo{ info->symbol,
                             info->name_cn,
                             info->name_en,
                             info->name_hk,
                             info->exchange,
                             info->currency,
                             info->lot_size,
                             info->total_shares,
                             info->circulating_shares,
                             info->hk_shares,
                             Decimal(info->eps),
                             Decimal(info->eps_ttm),
                             Decimal(info->bps),
                             Decimal(info->dividend_yield),
                             DerivativeType{ info->stock_derivatives },
                             convert(info->board) };
}

inline Subscription
convert(const lb_subscription_t* info)
{
  std::vector<Period> candlesticks;
  std::transform(info->candlesticks,
                 info->candlesticks + info->num_candlesticks,
                 std::back_inserter(candlesticks),
                 [](auto period) { return convert(period); });
  return Subscription{ info->symbol, SubFlags(info->sub_types), candlesticks };
};

inline TradeStatus
convert(lb_trade_status_t status)
{
  switch (status) {
    case TradeStatusNormal:
      return TradeStatus::Normal;
    case TradeStatusHalted:
      return TradeStatus::Halted;
    case TradeStatusDelisted:
      return TradeStatus::Delisted;
    case TradeStatusFuse:
      return TradeStatus::Fuse;
    case TradeStatusPrepareList:
      return TradeStatus::PrepareList;
    case TradeStatusCodeMoved:
      return TradeStatus::CodeMoved;
    case TradeStatusToBeOpened:
      return TradeStatus::ToBeOpened;
    case TradeStatusSplitStockHalts:
      return TradeStatus::SplitStockHalts;
    case TradeStatusExpired:
      return TradeStatus::Expired;
    case TradeStatusWarrantPrepareList:
      return TradeStatus::WarrantPrepareList;
    case TradeStatusSuspendTrade:
      return TradeStatus::SuspendTrade;
    default:
      throw std::invalid_argument("unreachable");
  }
}

inline std::optional<PrePostQuote>
convert(const lb_prepost_quote_t* info)
{
  if (info) {
    return std::optional<PrePostQuote>{ PrePostQuote{
      Decimal(info->last_done),
      info->timestamp,
      info->volume,
      Decimal(info->turnover),
      Decimal(info->high),
      Decimal(info->low),
      Decimal(info->prev_close),
    } };
  } else {
    return std::optional<PrePostQuote>();
  }
}

inline SecurityQuote
convert(const lb_security_quote_t* info)
{
  return SecurityQuote{
    info->symbol,
    Decimal(info->last_done),
    Decimal(info->prev_close),
    Decimal(info->open),
    Decimal(info->high),
    Decimal(info->low),
    info->timestamp,
    info->volume,
    Decimal(info->turnover),
    convert(info->trade_status),
    convert(info->pre_market_quote),
    convert(info->post_market_quote),
    convert(info->overnight_quote),
  };
}

inline OptionType
convert(lb_option_type_t ty)
{
  switch (ty) {
    case OptionTypeUnknown:
      return OptionType::Unknown;
    case OptionTypeAmerican:
      return OptionType::American;
    case OptionTypeEurope:
      return OptionType::Europe;
    default:
      throw std::invalid_argument("unreachable");
  }
}

inline OptionDirection
convert(lb_option_direction_t ty)
{
  switch (ty) {
    case OptionDirectionUnknown:
      return OptionDirection::Unknown;
    case OptionDirectionCall:
      return OptionDirection::Call;
    case OptionDirectionPut:
      return OptionDirection::Put;
    default:
      throw std::invalid_argument("unreachable");
  }
}

inline Date
convert(const lb_date_t* date)
{
  return Date{
    date->year,
    date->month,
    date->day,
  };
}

inline lb_date_t
convert(const Date* date)
{
  return lb_date_t{
    date->year,
    date->month,
    date->day,
  };
}

inline lb_time_t
convert(const Time* time)
{
  return lb_time_t{ time->hour, time->minute, time->second };
}

inline Time
convert(const lb_time_t* time)
{
  return Time{
    time->hour,
    time->minute,
    time->second,
  };
}

inline lb_datetime_t
convert(const DateTime* datetime)
{
  return lb_datetime_t{
    convert(&datetime->date),
    convert(&datetime->time),
  };
}

inline OptionQuote
convert(const lb_option_quote_t* info)
{
  return OptionQuote{
    info->symbol,
    Decimal(info->last_done),
    Decimal(info->prev_close),
    Decimal(info->open),
    Decimal(info->high),
    Decimal(info->low),
    info->timestamp,
    info->volume,
    Decimal(info->turnover),
    convert(info->trade_status),
    Decimal(info->implied_volatility),
    info->open_interest,
    convert(&info->expiry_date),
    Decimal(info->strike_price),
    Decimal(info->contract_multiplier),
    convert(info->contract_type),
    Decimal(info->contract_size),
    convert(info->direction),
    Decimal(info->historical_volatility),
    info->underlying_symbol,
  };
}

inline TradeSession
convert(lb_trade_session_t ty)
{
  switch (ty) {
    case TradeSessionIntraday:
      return TradeSession::Intraday;
    case TradeSessionPre:
      return TradeSession::Pre;
    case TradeSessionPost:
      return TradeSession::Post;
    case TradeSessionOvernight:
      return TradeSession::Overnight;
    default:
      throw std::invalid_argument("unreachable");
  }
}

inline PushQuote
convert(const lb_push_quote_t* info)
{
  return PushQuote{
    info->symbol,
    Decimal(info->last_done),
    Decimal(info->open),
    Decimal(info->high),
    Decimal(info->low),
    info->timestamp,
    info->volume,
    Decimal(info->turnover),
    convert(info->trade_status),
    convert(info->trade_session),
    info->current_volume,
    Decimal(info->current_turnover),
  };
}

inline Depth
convert(const lb_depth_t* depth)
{
  return Depth{
    depth->position,
    depth->price ? std::optional{ Decimal(depth->price) } : std::nullopt,
    depth->volume,
    depth->order_num,
  };
}

inline PushDepth
convert(const lb_push_depth_t* info)
{
  std::vector<Depth> asks;
  std::vector<Depth> bids;

  std::transform(info->asks,
                 info->asks + info->num_asks,
                 std::back_inserter(asks),
                 [](auto depth) { return convert(&depth); });
  std::transform(info->bids,
                 info->bids + info->num_bids,
                 std::back_inserter(bids),
                 [](auto depth) { return convert(&depth); });
  return PushDepth{ info->symbol, asks, bids };
}

inline Brokers
convert(const lb_brokers_t* brokers)
{
  std::vector<int32_t> broker_ids;
  std::transform(brokers->broker_ids,
                 brokers->broker_ids + brokers->num_broker_ids,
                 std::back_inserter(broker_ids),
                 [](auto id) { return id; });
  return Brokers{ brokers->position, broker_ids };
}

inline PushBrokers
convert(const lb_push_brokers_t* info)
{
  std::vector<Brokers> ask_brokers;
  std::vector<Brokers> bid_brokers;

  std::transform(info->ask_brokers,
                 info->ask_brokers + info->num_ask_brokers,
                 std::back_inserter(ask_brokers),
                 [](auto brokers) { return convert(&brokers); });
  std::transform(info->bid_brokers,
                 info->bid_brokers + info->num_bid_brokers,
                 std::back_inserter(bid_brokers),
                 [](auto brokers) { return convert(&brokers); });
  return PushBrokers{ info->symbol, ask_brokers, bid_brokers };
}

inline TradeDirection
convert(lb_trade_direction_t direction)
{
  switch (direction) {
    case TradeDirectionNeutral:
      return TradeDirection::Neutral;
    case TradeDirectionDown:
      return TradeDirection::Down;
    case TradeDirectionUp:
      return TradeDirection::Up;
    default:
      throw std::invalid_argument("unreachable");
  }
}

inline Trade
convert(const lb_trade_t* trade)
{
  return Trade{
    Decimal(trade->price),     trade->volume,
    trade->timestamp,          trade->trade_type,
    convert(trade->direction), convert(trade->trade_session),
  };
}

inline PushTrades
convert(const lb_push_trades_t* trades)
{
  std::vector<Trade> trades2;
  std::transform(trades->trades,
                 trades->trades + trades->num_trades,
                 std::back_inserter(trades2),
                 [](auto trade) { return convert(&trade); });
  return PushTrades{ trades->symbol, trades2 };
}

inline Candlestick
convert(const lb_candlestick_t* candlestick)
{
  return Candlestick{
    Decimal(candlestick->close), Decimal(candlestick->open),
    Decimal(candlestick->low),   Decimal(candlestick->high),
    candlestick->volume,         Decimal(candlestick->turnover),
    candlestick->timestamp,      convert(candlestick->trade_session)
  };
}

inline PushCandlestick
convert(const lb_push_candlestick_t* info)
{
  return PushCandlestick{
    info->symbol,
    convert(info->period),
    convert(&info->candlestick),
    info->is_confirmed,
  };
}

inline WarrantType
convert(lb_warrant_type_t ty)
{
  switch (ty) {
    case WarrantTypeUnknown:
      return WarrantType::Unknown;
    case WarrantTypePut:
      return WarrantType::Put;
    case WarrantTypeCall:
      return WarrantType::Call;
    case WarrantTypeBull:
      return WarrantType::Bull;
    case WarrantTypeBear:
      return WarrantType::Bear;
    case WarrantTypeInline:
      return WarrantType::Inline;
    default:
      throw std::invalid_argument("unreachable");
  }
}

inline lb_warrant_type_t
convert(WarrantType ty)
{
  switch (ty) {
    case WarrantType::Unknown:
      return WarrantTypeUnknown;
    case WarrantType::Put:
      return WarrantTypePut;
    case WarrantType::Call:
      return WarrantTypeCall;
    case WarrantType::Bull:
      return WarrantTypeBull;
    case WarrantType::Bear:
      return WarrantTypeBear;
    case WarrantType::Inline:
      return WarrantTypeInline;
    default:
      throw std::invalid_argument("unreachable");
  }
}

inline WarrantQuote
convert(const lb_warrant_quote_t* quote)
{
  return WarrantQuote{
    quote->symbol,
    Decimal(quote->last_done),
    Decimal(quote->prev_close),
    Decimal(quote->open),
    Decimal(quote->high),
    Decimal(quote->low),
    quote->timestamp,
    quote->volume,
    Decimal(quote->turnover),
    convert(quote->trade_status),
    Decimal(quote->implied_volatility),
    convert(&quote->expiry_date),
    convert(&quote->last_trade_date),
    Decimal(quote->outstanding_ratio),
    quote->outstanding_quantity,
    Decimal(quote->conversion_ratio),
    convert(quote->category),
    Decimal(quote->strike_price),
    Decimal(quote->upper_strike_price),
    Decimal(quote->lower_strike_price),
    Decimal(quote->call_price),
    quote->underlying_symbol,
  };
}

inline SecurityDepth
convert(const lb_security_depth_t* info)
{
  std::vector<Depth> asks;
  std::vector<Depth> bids;

  std::transform(info->asks,
                 info->asks + info->num_asks,
                 std::back_inserter(asks),
                 [](auto depth) { return convert(&depth); });
  std::transform(info->bids,
                 info->bids + info->num_bids,
                 std::back_inserter(bids),
                 [](auto depth) { return convert(&depth); });
  return SecurityDepth{ asks, bids };
}

inline SecurityBrokers
convert(const lb_security_brokers_t* info)
{
  std::vector<Brokers> ask_brokers;
  std::vector<Brokers> bid_brokers;

  std::transform(info->ask_brokers,
                 info->ask_brokers + info->num_ask_brokers,
                 std::back_inserter(ask_brokers),
                 [](auto brokers) { return convert(&brokers); });
  std::transform(info->bid_brokers,
                 info->bid_brokers + info->num_bid_brokers,
                 std::back_inserter(bid_brokers),
                 [](auto brokers) { return convert(&brokers); });
  return SecurityBrokers{ ask_brokers, bid_brokers };
}

inline ParticipantInfo
convert(const lb_participant_info_t* info)
{
  std::vector<int32_t> broker_ids(info->broker_ids,
                                  info->broker_ids + info->num_broker_ids);
  return ParticipantInfo{
    broker_ids,
    info->name_cn,
    info->name_en,
    info->name_hk,
  };
}

inline IntradayLine
convert(const lb_intraday_line_t* info)
{
  return IntradayLine{
    Decimal(info->price),    info->timestamp,          info->volume,
    Decimal(info->turnover), Decimal(info->avg_price),
  };
}

inline lb_adjust_type_t
convert(AdjustType ty)
{
  switch (ty) {
    case AdjustType::NoAdjust:
      return AdjustTypeNoAdjust;
    case AdjustType::ForwardAdjust:
      return AdjustTypeForward;
    default:
      throw std::invalid_argument("unreachable");
  }
}

inline StrikePriceInfo
convert(const lb_strike_price_info_t* info)
{
  return StrikePriceInfo{
    Decimal(info->price),
    info->call_symbol,
    info->put_symbol,
    info->standard,
  };
}

inline IssuerInfo
convert(const lb_issuer_info_t* info)
{
  return IssuerInfo{
    info->issuer_id,
    info->name_cn,
    info->name_en,
    info->name_hk,
  };
}

inline TradingSessionInfo
convert(const lb_trading_session_info_t* info)
{
  return TradingSessionInfo{
    convert(&info->begin_time),
    convert(&info->end_time),
    convert(info->trade_session),
  };
}

inline MarketTradingSession
convert(const lb_market_trading_session_t* info)
{
  std::vector<TradingSessionInfo> sessions;
  std::transform(info->trade_sessions,
                 info->trade_sessions + info->num_trade_sessions,
                 std::back_inserter(sessions),
                 [](auto item) { return convert(&item); });
  return MarketTradingSession{
    convert(info->market),
    sessions,
  };
}

inline MarketTradingDays
convert(const lb_market_trading_days_t* info)
{
  std::vector<Date> trading_days;
  std::vector<Date> half_trading_days;

  std::transform(info->trading_days,
                 info->trading_days + info->num_trading_days,
                 std::back_inserter(trading_days),
                 [](auto item) { return convert(&item); });
  std::transform(info->half_trading_days,
                 info->half_trading_days + info->num_half_trading_days,
                 std::back_inserter(half_trading_days),
                 [](auto item) { return convert(&item); });
  return MarketTradingDays{ trading_days, half_trading_days };
}

inline CapitalFlowLine
convert(const lb_capital_flow_line_t* info)
{
  return CapitalFlowLine{
    Decimal(info->inflow),
    info->timestamp,
  };
}

inline CapitalDistribution
convert(const lb_capital_distribution_t* info)
{
  return CapitalDistribution{
    Decimal(info->large),
    Decimal(info->medium),
    Decimal(info->small),
  };
}

inline CapitalDistributionResponse
convert(const lb_capital_distribution_response_t* resp)
{
  return CapitalDistributionResponse{
    resp->timestamp,
    convert(&resp->capital_in),
    convert(&resp->capital_out),
  };
}

inline RealtimeQuote
convert(const lb_realtime_quote_t* info)
{
  return RealtimeQuote{
    info->symbol,        Decimal(info->last_done), Decimal(info->open),
    Decimal(info->high), Decimal(info->low),       info->timestamp,
    info->volume,        Decimal(info->turnover),  convert(info->trade_status),
  };
}

inline lb_topic_type_t
convert(TopicType ty)
{
  switch (ty) {
    case TopicType::Private:
      return TopicPrivate;
    default:
      throw std::invalid_argument("unreachable");
  }
}

inline Execution
convert(const lb_execution_t* info)
{
  return Execution{
    info->order_id,      info->trade_id, info->symbol,
    info->trade_done_at, info->quantity, Decimal(info->price),
  };
}

inline OrderStatus
convert(lb_order_status_t status)
{
  switch (status) {
    case OrderStatusUnknown:
      return OrderStatus::Unknown;
    case OrderStatusNotReported:
      return OrderStatus::NotReported;
    case OrderStatusReplacedNotReported:
      return OrderStatus::ReplacedNotReported;
    case OrderStatusProtectedNotReported:
      return OrderStatus::ProtectedNotReported;
    case OrderStatusVarietiesNotReported:
      return OrderStatus::VarietiesNotReported;
    case OrderStatusFilled:
      return OrderStatus::Filled;
    case OrderStatusWaitToNew:
      return OrderStatus::WaitToNew;
    case OrderStatusNew:
      return OrderStatus::New;
    case OrderStatusWaitToReplace:
      return OrderStatus::WaitToReplace;
    case OrderStatusPendingReplace:
      return OrderStatus::PendingReplace;
    case OrderStatusReplaced:
      return OrderStatus::Replaced;
    case OrderStatusPartialFilled:
      return OrderStatus::PartialFilled;
    case OrderStatusWaitToCancel:
      return OrderStatus::WaitToCancel;
    case OrderStatusPendingCancel:
      return OrderStatus::PendingCancel;
    case OrderStatusRejected:
      return OrderStatus::Rejected;
    case OrderStatusCanceled:
      return OrderStatus::Canceled;
    case OrderStatusExpired:
      return OrderStatus::Expired;
    case OrderStatusPartialWithdrawal:
      return OrderStatus::PartialWithdrawal;
    default:
      throw std::invalid_argument("unreachable");
  }
}

inline lb_order_status_t
convert(OrderStatus status)
{
  switch (status) {
    case OrderStatus::Unknown:
      return OrderStatusUnknown;
    case OrderStatus::NotReported:
      return OrderStatusNotReported;
    case OrderStatus::ReplacedNotReported:
      return OrderStatusReplacedNotReported;
    case OrderStatus::ProtectedNotReported:
      return OrderStatusProtectedNotReported;
    case OrderStatus::VarietiesNotReported:
      return OrderStatusVarietiesNotReported;
    case OrderStatus::Filled:
      return OrderStatusFilled;
    case OrderStatus::WaitToNew:
      return OrderStatusWaitToNew;
    case OrderStatus::New:
      return OrderStatusNew;
    case OrderStatus::WaitToReplace:
      return OrderStatusWaitToReplace;
    case OrderStatus::PendingReplace:
      return OrderStatusPendingReplace;
    case OrderStatus::Replaced:
      return OrderStatusReplaced;
    case OrderStatus::PartialFilled:
      return OrderStatusPartialFilled;
    case OrderStatus::WaitToCancel:
      return OrderStatusWaitToCancel;
    case OrderStatus::PendingCancel:
      return OrderStatusPendingCancel;
    case OrderStatus::Rejected:
      return OrderStatusRejected;
    case OrderStatus::Canceled:
      return OrderStatusCanceled;
    case OrderStatus::Expired:
      return OrderStatusExpired;
    case OrderStatus::PartialWithdrawal:
      return OrderStatusPartialWithdrawal;
    default:
      throw std::invalid_argument("unreachable");
  }
}

inline OrderSide
convert(lb_order_side_t side)
{
  switch (side) {
    case OrderSideUnknown:
      return OrderSide::Unknown;
    case OrderSideBuy:
      return OrderSide::Buy;
    case OrderSideSell:
      return OrderSide::Sell;
    default:
      throw std::invalid_argument("unreachable");
  }
}

inline lb_order_side_t
convert(OrderSide side)
{
  switch (side) {
    case OrderSide::Unknown:
      return OrderSideUnknown;
    case OrderSide::Buy:
      return OrderSideBuy;
    case OrderSide::Sell:
      return OrderSideSell;
    default:
      throw std::invalid_argument("unreachable");
  }
}

inline OrderType
convert(lb_order_type_t ty)
{
  switch (ty) {
    case OrderTypeUnknown:
      return OrderType::Unknown;
    case OrderTypeLO:
      return OrderType::LO;
    case OrderTypeELO:
      return OrderType::ELO;
    case OrderTypeMO:
      return OrderType::MO;
    case OrderTypeAO:
      return OrderType::AO;
    case OrderTypeALO:
      return OrderType::ALO;
    case OrderTypeODD:
      return OrderType::ODD;
    case OrderTypeLIT:
      return OrderType::LIT;
    case OrderTypeMIT:
      return OrderType::MIT;
    case OrderTypeTSLPAMT:
      return OrderType::TSLPAMT;
    case OrderTypeTSLPPCT:
      return OrderType::TSLPPCT;
    case OrderTypeTSMAMT:
      return OrderType::TSMAMT;
    case OrderTypeTSMPCT:
      return OrderType::TSMPCT;
    case OrderTypeSLO:
      return OrderType::SLO;
    default:
      throw std::invalid_argument("unreachable");
  }
}

inline lb_order_type_t
convert(OrderType ty)
{
  switch (ty) {
    case OrderType::Unknown:
      return OrderTypeUnknown;
    case OrderType::LO:
      return OrderTypeLO;
    case OrderType::ELO:
      return OrderTypeELO;
    case OrderType::MO:
      return OrderTypeMO;
    case OrderType::AO:
      return OrderTypeAO;
    case OrderType::ALO:
      return OrderTypeALO;
    case OrderType::ODD:
      return OrderTypeODD;
    case OrderType::LIT:
      return OrderTypeLIT;
    case OrderType::MIT:
      return OrderTypeMIT;
    case OrderType::TSLPAMT:
      return OrderTypeTSLPAMT;
    case OrderType::TSLPPCT:
      return OrderTypeTSLPPCT;
    case OrderType::TSMAMT:
      return OrderTypeTSMAMT;
    case OrderType::TSMPCT:
      return OrderTypeTSMPCT;
    case OrderType::SLO:
      return OrderTypeSLO;
    default:
      throw std::invalid_argument("unreachable");
  }
}

inline OrderTag
convert(lb_order_tag_t tag)
{
  switch (tag) {
    case OrderTagUnknown:
      return OrderTag::Unknown;
    case OrderTagNormal:
      return OrderTag::Normal;
    case OrderTagLongTerm:
      return OrderTag::LongTerm;
    case OrderTagGrey:
      return OrderTag::Grey;
    case OrderTagMarginCall:
      return OrderTag::Grey;
    case OrderTagOffline:
      return OrderTag::Offline;
    case OrderTagCreditor:
      return OrderTag::Creditor;
    case OrderTagDebtor:
      return OrderTag::Debtor;
    case OrderTagNonExercise:
      return OrderTag::NonExercise;
    case OrderTagAllocatedSub:
      return OrderTag::AllocatedSub;
    /// Trade Allocation
    default:
      throw std::invalid_argument("unreachable");
  }
}

inline TimeInForceType
convert(lb_time_in_force_type_t ty)
{
  switch (ty) {
    case TimeInForceUnknown:
      return TimeInForceType::Unknown;
    case TimeInForceDay:
      return TimeInForceType::Day;
    case TimeInForceGoodTilCanceled:
      return TimeInForceType::GoodTilCanceled;
    case TimeInForceGoodTilDate:
      return TimeInForceType::GoodTilDate;
    default:
      throw std::invalid_argument("unreachable");
  }
}

inline lb_time_in_force_type_t
convert(TimeInForceType ty)
{
  switch (ty) {
    case TimeInForceType::Unknown:
      return TimeInForceUnknown;
    case TimeInForceType::Day:
      return TimeInForceDay;
    case TimeInForceType::GoodTilCanceled:
      return TimeInForceGoodTilCanceled;
    case TimeInForceType::GoodTilDate:
      return TimeInForceGoodTilDate;
    default:
      throw std::invalid_argument("unreachable");
  }
}

inline TriggerStatus
convert(lb_trigger_status_t status)
{
  switch (status) {
    case TriggerStatusUnknown:
      return TriggerStatus::Unknown;
    case TriggerStatusDeactive:
      return TriggerStatus::Deactive;
    case TriggerStatusActive:
      return TriggerStatus::Active;
    case TriggerStatusReleased:
      return TriggerStatus::Released;
    default:
      throw std::invalid_argument("unreachable");
  }
}

inline OutsideRTH
convert(lb_outside_rth_t status)
{
  switch (status) {
    case OutsideRTHUnknown:
      return OutsideRTH::Unknown;
    case OutsideRTHOnly:
      return OutsideRTH::RTHOnly;
    case OutsideRTHAnyTime:
      return OutsideRTH::AnyTime;
    case OutsideRTHOvernight:
      return OutsideRTH::Overnight;
    default:
      throw std::invalid_argument("unreachable");
  }
}

inline lb_outside_rth_t
convert(OutsideRTH status)
{
  switch (status) {
    case OutsideRTH::Unknown:
      return OutsideRTHUnknown;
    case OutsideRTH::RTHOnly:
      return OutsideRTHOnly;
    case OutsideRTH::AnyTime:
      return OutsideRTHAnyTime;
    case OutsideRTH::Overnight:
      return OutsideRTHOvernight;
    default:
      throw std::invalid_argument("unreachable");
  }
}

inline AttachedOrderType
convert(lb_attached_order_type_t ty)
{
  switch (ty) {
    case AttachedOrderTypeUnknown:
      return AttachedOrderType::Unknown;
    case AttachedOrderTypeProfitTaker:
      return AttachedOrderType::ProfitTaker;
    case AttachedOrderTypeStopLoss:
      return AttachedOrderType::StopLoss;
    case AttachedOrderTypeBracket:
      return AttachedOrderType::Bracket;
    default:
      throw std::invalid_argument("unreachable");
  }
}

inline lb_attached_order_type_t
convert(AttachedOrderType ty)
{
  switch (ty) {
    case AttachedOrderType::Unknown:
      return AttachedOrderTypeUnknown;
    case AttachedOrderType::ProfitTaker:
      return AttachedOrderTypeProfitTaker;
    case AttachedOrderType::StopLoss:
      return AttachedOrderTypeStopLoss;
    case AttachedOrderType::Bracket:
      return AttachedOrderTypeBracket;
    default:
      throw std::invalid_argument("unreachable");
  }
}

inline AttachedOrderDetail
convert(const lb_attached_order_detail_t* detail)
{
  return AttachedOrderDetail{
    detail->order_id,
    detail->attached_type_display,
    detail->trigger_price ? std::optional{ Decimal(detail->trigger_price) }
                          : std::nullopt,
    Decimal(detail->quantity),
    Decimal(detail->executed_qty),
    convert(detail->status),
    detail->updated_at,
    detail->withdrawn,
    detail->gtd ? std::optional{ convert(detail->gtd) } : std::nullopt,
    convert(detail->time_in_force),
    detail->counter_id,
    detail->trigger_status,
    Decimal(detail->executed_amount),
    detail->tag,
    detail->submitted_at,
    Decimal(detail->executed_price),
    detail->force_only_rth ? std::optional{ convert(*detail->force_only_rth) }
                           : std::nullopt,
    detail->reviewed,
    convert(detail->activate_order_type),
    detail->activate_rth ? std::optional{ convert(*detail->activate_rth) }
                         : std::nullopt,
    detail->submit_price ? std::optional{ Decimal(detail->submit_price) }
                         : std::nullopt,
  };
}

struct CSubmitAttachedParamsStorage
{
  lb_time_in_force_type_t time_in_force;
  lb_order_type_t activate_order_type;
  lb_outside_rth_t activate_rth;
  lb_submit_attached_params_t params;
};

inline CSubmitAttachedParamsStorage
convert_submit_attached(const SubmitAttachedParams& src)
{
  CSubmitAttachedParamsStorage s{};
  s.params.attached_order_type = convert(src.attached_order_type);
  s.params.profit_taker_price =
    src.profit_taker_price
      ? (const lb_decimal_t*)src.profit_taker_price.value()
      : nullptr;
  s.params.stop_loss_price =
    src.stop_loss_price ? (const lb_decimal_t*)src.stop_loss_price.value()
                        : nullptr;
  if (src.time_in_force) {
    s.time_in_force = convert(*src.time_in_force);
    s.params.time_in_force = &s.time_in_force;
  }
  s.params.expire_time = src.expire_time ? &*src.expire_time : nullptr;
  if (src.activate_order_type) {
    s.activate_order_type = convert(*src.activate_order_type);
    s.params.activate_order_type = &s.activate_order_type;
  }
  s.params.profit_taker_submit_price =
    src.profit_taker_submit_price
      ? (const lb_decimal_t*)src.profit_taker_submit_price.value()
      : nullptr;
  s.params.stop_loss_submit_price =
    src.stop_loss_submit_price
      ? (const lb_decimal_t*)src.stop_loss_submit_price.value()
      : nullptr;
  if (src.activate_rth) {
    s.activate_rth = convert(*src.activate_rth);
    s.params.activate_rth = &s.activate_rth;
  }
  return s;
}

struct CReplaceAttachedParamsStorage
{
  lb_time_in_force_type_t time_in_force;
  lb_order_type_t activate_order_type;
  lb_outside_rth_t activate_rth;
  lb_replace_attached_params_t params;
};

inline CReplaceAttachedParamsStorage
convert_replace_attached(const ReplaceAttachedParams& src)
{
  CReplaceAttachedParamsStorage s{};
  s.params.attached_order_type = convert(src.attached_order_type);
  s.params.profit_taker_price =
    src.profit_taker_price
      ? (const lb_decimal_t*)src.profit_taker_price.value()
      : nullptr;
  s.params.stop_loss_price =
    src.stop_loss_price ? (const lb_decimal_t*)src.stop_loss_price.value()
                        : nullptr;
  if (src.time_in_force) {
    s.time_in_force = convert(*src.time_in_force);
    s.params.time_in_force = &s.time_in_force;
  }
  s.params.expire_time = src.expire_time ? &*src.expire_time : nullptr;
  if (src.activate_order_type) {
    s.activate_order_type = convert(*src.activate_order_type);
    s.params.activate_order_type = &s.activate_order_type;
  }
  s.params.profit_taker_submit_price =
    src.profit_taker_submit_price
      ? (const lb_decimal_t*)src.profit_taker_submit_price.value()
      : nullptr;
  s.params.stop_loss_submit_price =
    src.stop_loss_submit_price
      ? (const lb_decimal_t*)src.stop_loss_submit_price.value()
      : nullptr;
  if (src.activate_rth) {
    s.activate_rth = convert(*src.activate_rth);
    s.params.activate_rth = &s.activate_rth;
  }
  s.params.profit_taker_id =
    src.profit_taker_id ? &*src.profit_taker_id : nullptr;
  s.params.stop_loss_id = src.stop_loss_id ? &*src.stop_loss_id : nullptr;
  s.params.cancel_all_attached =
    src.cancel_all_attached ? &*src.cancel_all_attached : nullptr;
  s.params.main_id = src.main_id ? &*src.main_id : nullptr;
  s.params.quantity =
    src.quantity ? (const lb_decimal_t*)src.quantity.value() : nullptr;
  s.params.market_price =
    src.market_price ? (const lb_decimal_t*)src.market_price.value() : nullptr;
  return s;
}

inline Order
convert(const lb_order_t* order)
{
  std::vector<AttachedOrderDetail> attached_orders;
  std::transform(order->attached_orders,
                 order->attached_orders + order->num_attached_orders,
                 std::back_inserter(attached_orders),
                 [](auto& item) { return convert(&item); });
  return Order{
    order->order_id,
    convert(order->status),
    order->stock_name,
    order->quantity,
    order->executed_quantity,
    order->price ? std::optional{ Decimal(order->price) } : std::nullopt,
    order->executed_price ? std::optional{ Decimal(order->executed_price) }
                          : std::nullopt,
    order->submitted_at,
    convert(order->side),
    order->symbol,
    convert(order->order_type),
    order->last_done ? std::optional{ Decimal(order->last_done) }
                     : std::nullopt,
    order->trigger_price ? std::optional{ Decimal(order->trigger_price) }
                         : std::nullopt,
    order->msg,
    convert(order->tag),
    convert(order->time_in_force),
    order->expire_date ? std::optional{ convert(order->expire_date) }
                       : std::nullopt,
    order->updated_at ? std::optional{ *order->updated_at } : std::nullopt,
    order->trigger_at ? std::optional{ *order->trigger_at } : std::nullopt,
    order->trailing_amount ? std::optional{ Decimal(order->trailing_amount) }
                           : std::nullopt,
    order->trailing_percent ? std::optional{ Decimal(order->trailing_percent) }
                            : std::nullopt,
    order->limit_offset ? std::optional{ Decimal(order->limit_offset) }
                        : std::nullopt,
    order->trigger_status ? std::optional{ convert(*order->trigger_status) }
                          : std::nullopt,
    order->currency,
    order->outside_rth ? std::optional{ convert(*order->outside_rth) }
                       : std::nullopt,
    order->limit_depth_level ? std::optional{ *order->limit_depth_level }
                             : std::nullopt,
    order->trigger_count ? std::optional{ *order->trigger_count }
                         : std::nullopt,
    order->monitor_price ? std::optional{ Decimal(order->monitor_price) }
                         : std::nullopt,
    order->remark,
    attached_orders,
  };
}

inline SubmitOrderResponse
convert(const lb_submit_order_response_t* info)
{
  return SubmitOrderResponse{ info->order_id };
}

inline CashInfo
convert(const lb_cash_info_t* info)
{
  return CashInfo{
    Decimal(info->withdraw_cash),
    Decimal(info->available_cash),
    Decimal(info->frozen_cash),
    Decimal(info->settling_cash),
    info->currency,
  };
}

inline FrozenTransactionFee
convert(const lb_frozen_transaction_fee_t* info)
{
  return FrozenTransactionFee{
    info->currency,
    Decimal(info->frozen_transaction_fee),
  };
}

inline AccountBalance
convert(const lb_account_balance_t* info)
{
  std::vector<CashInfo> cash_infos;
  std::transform(info->cash_infos,
                 info->cash_infos + info->num_cash_infos,
                 std::back_inserter(cash_infos),
                 [](auto item) { return convert(&item); });

  std::vector<FrozenTransactionFee> frozen_transaction_fees;
  std::transform(info->frozen_transaction_fees,
                 info->frozen_transaction_fees +
                   info->num_frozen_transaction_fees,
                 std::back_inserter(frozen_transaction_fees),
                 [](auto item) { return convert(&item); });

  return AccountBalance{
    Decimal(info->total_cash),
    Decimal(info->max_finance_amount),
    Decimal(info->remaining_finance_amount),
    info->risk_level,
    Decimal(info->margin_call),
    info->currency,
    cash_infos,
    Decimal(info->net_assets),
    Decimal(info->init_margin),
    Decimal(info->maintenance_margin),
    Decimal(info->buy_power),
    frozen_transaction_fees,
  };
}

inline CashFlowDirection
convert(lb_cash_flow_direction_t ty)
{
  switch (ty) {
    case CashFlowDirectionUnknown:
      return CashFlowDirection::Unknown;
    case CashFlowDirectionOut:
      return CashFlowDirection::Out;
    case CashFlowDirectionIn:
      return CashFlowDirection::In;
    default:
      throw std::invalid_argument("unreachable");
  }
}

inline BalanceType
convert(lb_balance_type_t ty)
{
  switch (ty) {
    case BalanceTypeUnknown:
      return BalanceType::Unknown;
    case BalanceTypeCash:
      return BalanceType::Cash;
    case BalanceTypeStock:
      return BalanceType::Stock;
    case BalanceTypeFund:
      return BalanceType::Fund;
    default:
      throw std::invalid_argument("unreachable");
  }
}

inline lb_balance_type_t
convert(BalanceType ty)
{
  switch (ty) {
    case BalanceType::Unknown:
      return BalanceTypeUnknown;
    case BalanceType::Cash:
      return BalanceTypeCash;
    case BalanceType::Stock:
      return BalanceTypeStock;
    case BalanceType::Fund:
      return BalanceTypeFund;
    default:
      throw std::invalid_argument("unreachable");
  }
}

inline CashFlow
convert(const lb_cash_flow_t* flow)
{
  return CashFlow{
    flow->transaction_flow_name,
    convert(flow->direction),
    convert(flow->business_type),
    Decimal(flow->balance),
    flow->currency,
    flow->business_time,
    flow->symbol ? std::optional{ flow->symbol } : std::nullopt,
    flow->description,
  };
}

inline FundPosition
convert(const lb_fund_position_t* position)
{
  return FundPosition{
    position->symbol,
    Decimal(position->current_net_asset_value),
    position->net_asset_value_day,
    position->symbol_name,
    position->currency,
    Decimal(position->cost_net_asset_value),
    Decimal(position->holding_units),
  };
}

inline FundPositionChannel
convert(const lb_fund_position_channel_t* channel)
{
  std::vector<FundPosition> positions;
  std::transform(channel->positions,
                 channel->positions + channel->num_positions,
                 std::back_inserter(positions),
                 [](auto position) { return convert(&position); });
  return FundPositionChannel{ channel->account_channel, positions };
}

inline FundPositionsResponse
convert(const lb_fund_position_response_t* resp)
{
  std::vector<FundPositionChannel> channels;
  std::transform(resp->channels,
                 resp->channels + resp->num_channels,
                 std::back_inserter(channels),
                 [](auto channel) { return convert(&channel); });
  return FundPositionsResponse{ channels };
}

inline StockPosition
convert(const lb_stock_position_t* position)
{
  return StockPosition{
    position->symbol,
    position->symbol_name,
    position->quantity,
    position->available_quantity,
    position->currency,
    Decimal(position->cost_price),
    convert(position->market),
    position->init_quantity ? std::optional{ position->init_quantity }
                            : std::nullopt,
  };
}

inline StockPositionChannel
convert(const lb_stock_position_channel_t* channel)
{
  std::vector<StockPosition> positions;
  std::transform(channel->positions,
                 channel->positions + channel->num_positions,
                 std::back_inserter(positions),
                 [](auto position) { return convert(&position); });
  return StockPositionChannel{ channel->account_channel, positions };
}

inline StockPositionsResponse
convert(const lb_stock_position_response_t* resp)
{
  std::vector<StockPositionChannel> channels;
  std::transform(resp->channels,
                 resp->channels + resp->num_channels,
                 std::back_inserter(channels),
                 [](auto channel) { return convert(&channel); });
  return StockPositionsResponse{ channels };
}

inline PushOrderChanged
convert(const lb_push_order_changed_t* info)
{
  return PushOrderChanged{
    convert(info->side),
    info->stock_name,
    info->submitted_quantity,
    info->symbol,
    convert(info->order_type),
    Decimal(info->submitted_price),
    info->executed_quantity,
    info->executed_price ? std::optional{ Decimal(info->executed_price) }
                         : std::nullopt,
    info->order_id,
    info->currency,
    convert(info->status),
    info->submitted_at,
    info->updated_at,
    info->trigger_price ? std::optional{ Decimal(info->trigger_price) }
                        : std::nullopt,
    info->msg,
    convert(info->tag),
    info->trigger_status ? std::optional{ convert(*info->trigger_status) }
                         : std::nullopt,
    info->trigger_at ? std::optional{ *info->trigger_at } : std::nullopt,
    info->trailing_amount ? std::optional{ Decimal(info->trailing_amount) }
                          : std::nullopt,
    info->trailing_percent ? std::optional{ Decimal(info->trailing_percent) }
                           : std::nullopt,
    info->limit_offset ? std::optional{ Decimal(info->limit_offset) }
                       : std::nullopt,
    info->account_no,
    info->last_share ? std::optional{ Decimal(info->last_share) }
                     : std::nullopt,
    info->last_price ? std::optional{ Decimal(info->last_price) }
                     : std::nullopt,
    info->remark,
  };
}

inline WatchlistSecurity
convert(const lb_watchlist_security_t* info)
{
  return WatchlistSecurity{
    info->symbol,
    convert(info->market),
    info->name,
    info->watched_price ? std::optional{ Decimal(info->watched_price) }
                        : std::nullopt,
    info->watched_at,
  };
}

inline WatchlistGroup
convert(const lb_watchlist_group_t* info)
{
  std::vector<WatchlistSecurity> securities;
  std::transform(info->securities,
                 info->securities + info->num_securities,
                 std::back_inserter(securities),
                 [](auto item) { return convert(&item); });
  return WatchlistGroup{ info->id, info->name, securities };
}

inline MarginRatio
convert(const lb_margin_ratio_t* info)
{
  return MarginRatio{ Decimal(info->im_factor),
                      Decimal(info->mm_factor),
                      Decimal(info->fm_factor) };
}

inline OrderHistoryDetail
convert(const lb_order_history_detail_t* history)
{
  return OrderHistoryDetail{
    Decimal(history->price),   history->quantity, convert(history->status),
    std::string(history->msg), history->time,
  };
}

inline OrderChargeFee
convert(const lb_order_charge_fee_t* item)
{
  return OrderChargeFee{
    std::string(item->code),
    std::string(item->name),
    Decimal(item->amount),
    std::string(item->currency),
  };
}

inline ChargeCategoryCode
convert(lb_charge_category_code_t code)
{
  switch (code) {
    case ChargeCategoryCodeBroker:
      return ChargeCategoryCode::Broker;
    case ChargeCategoryCodeThird:
      return ChargeCategoryCode::Third;
    default:
      return ChargeCategoryCode::Unknown;
  }
}

inline CommissionFreeStatus
convert(lb_commission_free_status_t status)
{
  switch (status) {
    case CommissionFreeStatusNone:
      return CommissionFreeStatus::None;
    case CommissionFreeStatusCalculated:
      return CommissionFreeStatus::Calculated;
    case CommissionFreeStatusPending:
      return CommissionFreeStatus::Pending;
    case CommissionFreeStatusReady:
      return CommissionFreeStatus::Ready;
    default:
      return CommissionFreeStatus::Unknown;
  }
}

inline DeductionStatus
convert(lb_deduction_status_t status)
{
  switch (status) {
    case DeductionStatusNone:
      return DeductionStatus::None;
    case DeductionStatusNoData:
      return DeductionStatus::NoData;
    case DeductionStatusPending:
      return DeductionStatus::Pending;
    case DeductionStatusDone:
      return DeductionStatus::Done;
    default:
      return DeductionStatus::Unknown;
  }
}

inline OrderChargeItem
convert(const lb_order_charge_item_t* item)
{
  std::vector<OrderChargeFee> fees;
  std::transform(item->fees,
                 item->fees + item->num_fees,
                 std::back_inserter(fees),
                 [](auto item) { return convert(&item); });

  return OrderChargeItem{
    convert(item->code),
    std::string(item->name),
    fees,
  };
}

inline OrderChargeDetail
convert(const lb_order_charge_detail_t* detail)
{
  std::vector<OrderChargeItem> items;
  std::transform(detail->items,
                 detail->items + detail->num_items,
                 std::back_inserter(items),
                 [](auto item) { return convert(&item); });

  return OrderChargeDetail{
    Decimal(detail->total_amount),
    std::string(detail->currency),
    items,
  };
}

inline OrderDetail
convert(const lb_order_detail_t* order)
{
  std::vector<OrderHistoryDetail> history;
  std::transform(order->history,
                 order->history + order->num_history,
                 std::back_inserter(history),
                 [](auto item) { return convert(&item); });

  std::vector<AttachedOrderDetail> attached_orders;
  std::transform(order->attached_orders,
                 order->attached_orders + order->num_attached_orders,
                 std::back_inserter(attached_orders),
                 [](auto& item) { return convert(&item); });

  return OrderDetail{
    order->order_id,
    convert(order->status),
    order->stock_name,
    order->quantity,
    order->executed_quantity,
    order->price ? std::optional{ Decimal(order->price) } : std::nullopt,
    order->executed_price ? std::optional{ Decimal(order->executed_price) }
                          : std::nullopt,
    order->submitted_at,
    convert(order->side),
    order->symbol,
    convert(order->order_type),
    order->last_done ? std::optional{ Decimal(order->last_done) }
                     : std::nullopt,
    order->trigger_price ? std::optional{ Decimal(order->trigger_price) }
                         : std::nullopt,
    order->msg,
    convert(order->tag),
    convert(order->time_in_force),
    order->expire_date ? std::optional{ convert(order->expire_date) }
                       : std::nullopt,
    order->updated_at ? std::optional{ *order->updated_at } : std::nullopt,
    order->trigger_at ? std::optional{ *order->trigger_at } : std::nullopt,
    order->trailing_amount ? std::optional{ Decimal(order->trailing_amount) }
                           : std::nullopt,
    order->trailing_percent ? std::optional{ Decimal(order->trailing_percent) }
                            : std::nullopt,
    order->limit_offset ? std::optional{ Decimal(order->limit_offset) }
                        : std::nullopt,
    order->trigger_status ? std::optional{ convert(*order->trigger_status) }
                          : std::nullopt,
    order->currency,
    order->outside_rth ? std::optional{ convert(*order->outside_rth) }
                       : std::nullopt,
    order->limit_depth_level ? std::optional{ *order->limit_depth_level }
                             : std::nullopt,
    order->trigger_count ? std::optional{ *order->trigger_count }
                         : std::nullopt,
    order->monitor_price ? std::optional{ Decimal(order->monitor_price) }
                         : std::nullopt,
    order->remark,
    convert(order->free_status),
    order->free_amount ? std::optional{ Decimal(order->free_amount) }
                       : std::nullopt,
    order->free_currency ? std::optional{ std::string(order->free_currency) }
                         : std::nullopt,
    convert(order->deductions_status),
    order->deductions_amount
      ? std::optional{ Decimal(order->deductions_amount) }
      : std::nullopt,
    order->deductions_currency
      ? std::optional{ std::string(order->deductions_currency) }
      : std::nullopt,
    convert(order->platform_deducted_status),
    order->platform_deducted_amount
      ? std::optional{ Decimal(order->platform_deducted_amount) }
      : std::nullopt,
    order->platform_deducted_currency
      ? std::optional{ std::string(order->platform_deducted_currency) }
      : std::nullopt,
    history,
    convert(&order->charge_detail),
    attached_orders,
  };
}

inline lb_securities_update_mode_t
convert(SecuritiesUpdateMode mode)
{
  switch (mode) {
    case SecuritiesUpdateMode::Add:
      return SecuritiesUpdateModeAdd;
    case SecuritiesUpdateMode::Remove:
      return SecuritiesUpdateModeRemove;
    case SecuritiesUpdateMode::Replace:
      return SecuritiesUpdateModeReplace;
    default:
      throw std::invalid_argument("unreachable");
  }
}

inline lb_calc_index_t
convert(CalcIndex index)
{
  switch (index) {
    case CalcIndex::LastDone:
      return CalcIndexLastDone;
    case CalcIndex::ChangeValue:
      return CalcIndexChangeValue;
    case CalcIndex::ChangeRate:
      return CalcIndexChangeRate;
    case CalcIndex::Volume:
      return CalcIndexVolume;
    case CalcIndex::Turnover:
      return CalcIndexTurnover;
    case CalcIndex::YtdChangeRate:
      return CalcIndexYtdChangeRate;
    case CalcIndex::TurnoverRate:
      return CalcIndexTurnoverRate;
    case CalcIndex::TotalMarketValue:
      return CalcIndexTotalMarketValue;
    case CalcIndex::CapitalFlow:
      return CalcIndexCapitalFlow;
    case CalcIndex::Amplitude:
      return CalcIndexAmplitude;
    case CalcIndex::VolumeRatio:
      return CalcIndexVolumeRatio;
    case CalcIndex::PeTtmRatio:
      return CalcIndexPeTtmRatio;
    case CalcIndex::PbRatio:
      return CalcIndexPbRatio;
    case CalcIndex::DividendRatioTtm:
      return CalcIndexDividendRatioTtm;
    case CalcIndex::FiveDayChangeRate:
      return CalcIndexFiveDayChangeRate;
    case CalcIndex::TenDayChangeRate:
      return CalcIndexTenDayChangeRate;
    case CalcIndex::HalfYearChangeRate:
      return CalcIndexHalfYearChangeRate;
    case CalcIndex::FiveMinutesChangeRate:
      return CalcIndexFiveMinutesChangeRate;
    case CalcIndex::ExpiryDate:
      return CalcIndexExpiryDate;
    case CalcIndex::StrikePrice:
      return CalcIndexStrikePrice;
    case CalcIndex::UpperStrikePrice:
      return CalcIndexUpperStrikePrice;
    case CalcIndex::LowerStrikePrice:
      return CalcIndexLowerStrikePrice;
    case CalcIndex::OutstandingQty:
      return CalcIndexOutstandingQty;
    case CalcIndex::OutstandingRatio:
      return CalcIndexOutstandingRatio;
    case CalcIndex::Premium:
      return CalcIndexPremium;
    case CalcIndex::ItmOtm:
      return CalcIndexItmOtm;
    case CalcIndex::ImpliedVolatility:
      return CalcIndexImpliedVolatility;
    case CalcIndex::WarrantDelta:
      return CalcIndexWarrantDelta;
    case CalcIndex::CallPrice:
      return CalcIndexCallPrice;
    case CalcIndex::ToCallPrice:
      return CalcIndexToCallPrice;
    case CalcIndex::EffectiveLeverage:
      return CalcIndexEffectiveLeverage;
    case CalcIndex::LeverageRatio:
      return CalcIndexLeverageRatio;
    case CalcIndex::ConversionRatio:
      return CalcIndexConversionRatio;
    case CalcIndex::BalancePoint:
      return CalcIndexBalancePoint;
    case CalcIndex::OpenInterest:
      return CalcIndexOpenInterest;
    case CalcIndex::Delta:
      return CalcIndexDelta;
    case CalcIndex::Gamma:
      return CalcIndexGamma;
    case CalcIndex::Theta:
      return CalcIndexTheta;
    case CalcIndex::Vega:
      return CalcIndexVega;
    case CalcIndex::Rho:
      return CalcIndexRho;
    default:
      throw std::invalid_argument("unreachable");
  }
}

inline SecurityCalcIndex
convert(const lb_security_calc_index_t* resp)
{
  return SecurityCalcIndex{
    resp->symbol,
    resp->last_done ? std::optional{ Decimal(resp->last_done) } : std::nullopt,
    resp->change_value ? std::optional{ Decimal(resp->change_value) }
                       : std::nullopt,
    resp->change_rate ? std::optional{ Decimal(resp->change_rate) }
                      : std::nullopt,
    resp->volume ? std::optional{ *resp->volume } : std::nullopt,
    resp->turnover ? std::optional{ Decimal(resp->turnover) } : std::nullopt,
    resp->ytd_change_rate ? std::optional{ Decimal(resp->ytd_change_rate) }
                          : std::nullopt,
    resp->turnover_rate ? std::optional{ Decimal(resp->turnover_rate) }
                        : std::nullopt,
    resp->total_market_value
      ? std::optional{ Decimal(resp->total_market_value) }
      : std::nullopt,
    resp->capital_flow ? std::optional{ Decimal(resp->capital_flow) }
                       : std::nullopt,
    resp->amplitude ? std::optional{ Decimal(resp->amplitude) } : std::nullopt,
    resp->volume_ratio ? std::optional{ Decimal(resp->volume_ratio) }
                       : std::nullopt,
    resp->pe_ttm_ratio ? std::optional{ Decimal(resp->pe_ttm_ratio) }
                       : std::nullopt,
    resp->pb_ratio ? std::optional{ Decimal(resp->pb_ratio) } : std::nullopt,
    resp->dividend_ratio_ttm
      ? std::optional{ Decimal(resp->dividend_ratio_ttm) }
      : std::nullopt,
    resp->five_day_change_rate
      ? std::optional{ Decimal(resp->five_day_change_rate) }
      : std::nullopt,
    resp->ten_day_change_rate
      ? std::optional{ Decimal(resp->ten_day_change_rate) }
      : std::nullopt,
    resp->half_year_change_rate
      ? std::optional{ Decimal(resp->half_year_change_rate) }
      : std::nullopt,
    resp->five_minutes_change_rate
      ? std::optional{ Decimal(resp->five_minutes_change_rate) }
      : std::nullopt,
    resp->expiry_date ? std::optional{ convert(resp->expiry_date) }
                      : std::nullopt,
    resp->strike_price ? std::optional{ Decimal(resp->strike_price) }
                       : std::nullopt,
    resp->upper_strike_price
      ? std::optional{ Decimal(resp->upper_strike_price) }
      : std::nullopt,
    resp->lower_strike_price
      ? std::optional{ Decimal(resp->lower_strike_price) }
      : std::nullopt,
    resp->outstanding_qty ? std::optional{ *resp->outstanding_qty }
                          : std::nullopt,
    resp->outstanding_ratio ? std::optional{ Decimal(resp->outstanding_ratio) }
                            : std::nullopt,
    resp->premium ? std::optional{ Decimal(resp->premium) } : std::nullopt,
    resp->itm_otm ? std::optional{ Decimal(resp->itm_otm) } : std::nullopt,
    resp->implied_volatility
      ? std::optional{ Decimal(resp->implied_volatility) }
      : std::nullopt,
    resp->warrant_delta ? std::optional{ Decimal(resp->warrant_delta) }
                        : std::nullopt,
    resp->call_price ? std::optional{ Decimal(resp->call_price) }
                     : std::nullopt,
    resp->to_call_price ? std::optional{ Decimal(resp->to_call_price) }
                        : std::nullopt,
    resp->effective_leverage
      ? std::optional{ Decimal(resp->effective_leverage) }
      : std::nullopt,
    resp->leverage_ratio ? std::optional{ Decimal(resp->leverage_ratio) }
                         : std::nullopt,
    resp->conversion_ratio ? std::optional{ Decimal(resp->conversion_ratio) }
                           : std::nullopt,
    resp->balance_point ? std::optional{ Decimal(resp->balance_point) }
                        : std::nullopt,
    resp->open_interest ? std::optional{ *resp->open_interest } : std::nullopt,
    resp->delta ? std::optional{ Decimal(resp->delta) } : std::nullopt,
    resp->gamma ? std::optional{ Decimal(resp->gamma) } : std::nullopt,
    resp->theta ? std::optional{ Decimal(resp->theta) } : std::nullopt,
    resp->vega ? std::optional{ Decimal(resp->vega) } : std::nullopt,
    resp->rho ? std::optional{ Decimal(resp->rho) } : std::nullopt,
  };
}

inline lb_sort_order_type_t
convert(SortOrderType ty)
{
  switch (ty) {
    case SortOrderType::Ascending:
      return SortOrderAscending;
    case SortOrderType::Descending:
      return SortOrderDescending;
    default:
      throw std::invalid_argument("unreachable");
  }
}

inline lb_warrant_sort_by_t
convert(WarrantSortBy ty)
{
  switch (ty) {
    case WarrantSortBy::LastDone:
      return WarrantSortByLastDone;
    case WarrantSortBy::ChangeRate:
      return WarrantSortByChangeRate;
    case WarrantSortBy::ChangeValue:
      return WarrantSortByChangeValue;
    case WarrantSortBy::Volume:
      return WarrantSortByVolume;
    case WarrantSortBy::Turnover:
      return WarrantSortByTurnover;
    case WarrantSortBy::ExpiryDate:
      return WarrantSortByExpiryDate;
    case WarrantSortBy::StrikePrice:
      return WarrantSortByStrikePrice;
    case WarrantSortBy::UpperStrikePrice:
      return WarrantSortByUpperStrikePrice;
    case WarrantSortBy::LowerStrikePrice:
      return WarrantSortByLowerStrikePrice;
    case WarrantSortBy::OutstandingQuantity:
      return WarrantSortByOutstandingQuantity;
    case WarrantSortBy::OutstandingRatio:
      return WarrantSortByOutstandingRatio;
    case WarrantSortBy::Premium:
      return WarrantSortByPremium;
    case WarrantSortBy::ItmOtm:
      return WarrantSortByItmOtm;
    case WarrantSortBy::ImpliedVolatility:
      return WarrantSortByImpliedVolatility;
    case WarrantSortBy::Delta:
      return WarrantSortByDelta;
    case WarrantSortBy::CallPrice:
      return WarrantSortByCallPrice;
    case WarrantSortBy::ToCallPrice:
      return WarrantSortByToCallPrice;
    case WarrantSortBy::EffectiveLeverage:
      return WarrantSortByEffectiveLeverage;
    case WarrantSortBy::LeverageRatio:
      return WarrantSortByLeverageRatio;
    case WarrantSortBy::ConversionRatio:
      return WarrantSortByConversionRatio;
    case WarrantSortBy::BalancePoint:
      return WarrantSortByBalancePoint;
    case WarrantSortBy::Status:
      return WarrantSortByStatus;
    default:
      throw std::invalid_argument("unreachable");
  }
}

inline lb_filter_warrant_expiry_date_t
convert(FilterWarrantExpiryDate ty)
{
  switch (ty) {
    case FilterWarrantExpiryDate::LT_3:
      return WarrantExpiryDate_LT_3;
    case FilterWarrantExpiryDate::Between_3_6:
      return WarrantExpiryDate_Between_3_6;
    case FilterWarrantExpiryDate::Between_6_12:
      return WarrantExpiryDate_Between_6_12;
    case FilterWarrantExpiryDate::GT_12:
      return WarrantExpiryDate_GT_12;
    default:
      throw std::invalid_argument("unreachable");
  }
}

inline lb_filter_warrant_in_out_bounds_type_t
convert(FilterWarrantInOutBoundsType ty)
{
  switch (ty) {
    case FilterWarrantInOutBoundsType::In:
      return WarrantInOutBoundsType_In;
    case FilterWarrantInOutBoundsType::Out:
      return WarrantInOutBoundsType_Out;
    default:
      throw std::invalid_argument("unreachable");
  }
}

inline lb_warrant_status_t
convert(WarrantStatus ty)
{
  switch (ty) {
    case WarrantStatus::Suspend:
      return WarrantStatusSuspend;
    case WarrantStatus::PrepareList:
      return WarrantStatusPrepareList;
    case WarrantStatus::Normal:
      return WarrantStatusNormal;
    default:
      throw std::invalid_argument("unreachable");
  }
}

inline WarrantStatus
convert(lb_warrant_status_t ty)
{
  switch (ty) {
    case WarrantStatusSuspend:
      return WarrantStatus::Suspend;
    case WarrantStatusPrepareList:
      return WarrantStatus::PrepareList;
    case WarrantStatusNormal:
      return WarrantStatus::Normal;
    default:
      throw std::invalid_argument("unreachable");
  }
}

inline WarrantInfo
convert(lb_warrant_info_t info)
{
  return WarrantInfo{
    info.symbol,
    convert(info.warrant_type),
    info.name,
    Decimal(info.last_done),
    Decimal(info.change_rate),
    Decimal(info.change_value),
    info.volume,
    Decimal(info.turnover),
    convert(&info.expiry_date),
    info.strike_price ? std::optional{ Decimal(info.strike_price) }
                      : std::nullopt,
    info.upper_strike_price ? std::optional{ Decimal(info.upper_strike_price) }
                            : std::nullopt,
    info.lower_strike_price ? std::optional{ Decimal(info.lower_strike_price) }
                            : std::nullopt,
    info.outstanding_qty,
    Decimal(info.outstanding_ratio),
    Decimal(info.premium),
    info.itm_otm ? std::optional{ Decimal(info.itm_otm) } : std::nullopt,
    info.implied_volatility ? std::optional{ Decimal(info.implied_volatility) }
                            : std::nullopt,
    info.delta ? std::optional{ Decimal(info.delta) } : std::nullopt,
    info.call_price ? std::optional{ Decimal(info.call_price) } : std::nullopt,
    info.to_call_price ? std::optional{ Decimal(info.to_call_price) }
                       : std::nullopt,
    info.effective_leverage ? std::optional{ Decimal(info.effective_leverage) }
                            : std::nullopt,
    Decimal(info.leverage_ratio),
    info.conversion_ratio ? std::optional{ Decimal(info.conversion_ratio) }
                          : std::nullopt,
    info.balance_point ? std::optional{ Decimal(info.balance_point) }
                       : std::nullopt,
    convert(info.status),
  };
}

inline lb_security_list_category_t
convert(SecurityListCategory category)
{
  switch (category) {
    case SecurityListCategory::Overnight:
      return SecurityListCategoryOvernight;
    default:
      throw std::invalid_argument("unreachable");
  }
}

inline Security
convert(const lb_security_t* info)
{
  return Security{
    info->symbol,
    info->name_cn,
    info->name_en,
    info->name_hk,
  };
}

inline QuotePackageDetail
convert(const lb_quote_package_detail_t* detail)
{
  return QuotePackageDetail{
    detail->key,      detail->name,   detail->description,
    detail->start_at, detail->end_at,
  };
}

inline lb_trade_sessions_t
convert(TradeSessions ts)
{
  switch (ts) {
    case TradeSessions::Intraday:
      return TradeSessionsIntraday;
    case TradeSessions::All:
      return TradeSessionsAll;
    default:
      throw std::invalid_argument("unreachable");
  }
}

inline MarketTemperature
convert(const lb_market_temperature_t* mt)
{
  return MarketTemperature{ mt->temperature,
                            mt->description,
                            mt->valuation,
                            mt->sentiment,
                            mt->timestamp };
}

inline Granularity
convert(lb_granularity_t granularity)
{
  switch (granularity) {
    case GranularityUnknown:
      return Granularity::Unknown;
    case GranularityDaily:
      return Granularity::Daily;
    case GranularityWeekly:
      return Granularity::Weekly;
    case GranularityMonthly:
      return Granularity::Monthly;
    default:
      throw std::invalid_argument("unreachable");
  }
}

inline HistoryMarketTemperatureResponse
convert(const lb_history_market_temperature_response_t* resp)
{
  std::vector<MarketTemperature> records;
  std::transform(resp->records,
                 resp->records + resp->num_records,
                 std::back_inserter(records),
                 [](auto item) { return convert(&item); });
  return HistoryMarketTemperatureResponse{ convert(resp->granularity),
                                           records };
}

inline FilingItem
convert(const lb_filing_item_t* item)
{
  std::vector<std::string> file_urls;
  std::transform(item->file_urls,
                 item->file_urls + item->num_file_urls,
                 std::back_inserter(file_urls),
                 [](auto url) { return std::string(url); });
  return FilingItem{ item->id,
                     item->title,
                     item->description,
                     item->file_name,
                     file_urls,
                     item->published_at };
}

inline TopicItem
convert(const lb_topic_item_t* item)
{
  return TopicItem{ item->id,
                    item->title,
                    item->description,
                    item->url,
                    item->published_at,
                    item->comments_count,
                    item->likes_count,
                    item->shares_count };
}

inline NewsItem
convert(const lb_news_item_t* item)
{
  return NewsItem{ item->id,
                   item->title,
                   item->description,
                   item->url,
                   item->published_at,
                   item->comments_count,
                   item->likes_count,
                   item->shares_count };
}

inline TopicAuthor
convert(const lb_topic_author_t* a)
{
  return TopicAuthor{ a->member_id, a->name, a->avatar };
}

inline TopicImage
convert(const lb_topic_image_t* img)
{
  return TopicImage{ img->url, img->sm, img->lg };
}

inline OwnedTopic
convert(const lb_owned_topic_t* item)
{
  std::vector<std::string> tickers(item->tickers,
                                   item->tickers + item->num_tickers);
  std::vector<std::string> hashtags(item->hashtags,
                                    item->hashtags + item->num_hashtags);
  std::vector<TopicImage> images;
  std::transform(item->images,
                 item->images + item->num_images,
                 std::back_inserter(images),
                 [](const lb_topic_image_t& img) { return convert(&img); });
  return OwnedTopic{ item->id,
                      item->title,
                      item->description,
                      item->body,
                      convert(&item->author),
                      std::move(tickers),
                      std::move(hashtags),
                      std::move(images),
                      item->likes_count,
                      item->comments_count,
                      item->views_count,
                      item->shares_count,
                      item->topic_type,
                      item->detail_url,
                      item->created_at,
                      item->updated_at };
}

// ── QuoteContext extension types ──────────────────────────────────

inline quote::ShortPositionsItem convert(const lb_short_positions_item_t* item) {
  return { item->timestamp ? item->timestamp : "",
           item->rate ? item->rate : "",
           item->close ? item->close : "",
           item->current_shares_short ? item->current_shares_short : "",
           item->avg_daily_share_volume ? item->avg_daily_share_volume : "",
           item->days_to_cover ? item->days_to_cover : "",
           item->amount ? item->amount : "",
           item->balance ? item->balance : "",
           item->cost ? item->cost : "" };
}
inline quote::ShortPositionsResponse convert(const lb_short_positions_response_t* r) {
  std::vector<quote::ShortPositionsItem> items;
  for (size_t i = 0; i < r->num_data; ++i) items.push_back(convert(&r->data[i]));
  return { std::move(items) };
}
inline quote::ShortTradesItem convert(const lb_short_trades_item_t* item) {
  return { item->timestamp ? item->timestamp : "",
           item->rate ? item->rate : "",
           item->close ? item->close : "",
           item->nus_amount ? item->nus_amount : "",
           item->ny_amount ? item->ny_amount : "",
           item->total_amount ? item->total_amount : "",
           item->amount ? item->amount : "",
           item->balance ? item->balance : "" };
}
inline quote::ShortTradesResponse convert(const lb_short_trades_response_t* r) {
  std::vector<quote::ShortTradesItem> items;
  for (size_t i = 0; i < r->num_data; ++i) items.push_back(convert(&r->data[i]));
  return { std::move(items) };
}
inline quote::OptionVolumeStats convert(const lb_option_volume_stats_t* s) {
  return { s->c, s->p };
}
inline quote::OptionVolumeDailyStat convert(const lb_option_volume_daily_stat_t* s) {
  return { s->symbol, s->timestamp, s->total_volume, s->total_put_volume, s->total_call_volume,
           s->put_call_volume_ratio, s->total_open_interest, s->total_put_open_interest,
           s->total_call_open_interest, s->put_call_open_interest_ratio };
}
inline quote::OptionVolumeDaily convert(const lb_option_volume_daily_t* r) {
  std::vector<quote::OptionVolumeDailyStat> stats;
  for (size_t i = 0; i < r->num_stats; ++i) stats.push_back(convert(&r->stats[i]));
  return { std::move(stats) };
}
inline fundamental::ElementType convert(lb_element_type_t ty) {
  switch (ty) {
    case ElementTypeUnknown:
      return fundamental::ElementType::Unknown;
    case ElementTypeHoldings:
      return fundamental::ElementType::Holdings;
    case ElementTypeRegional:
      return fundamental::ElementType::Regional;
    case ElementTypeAssetClass:
      return fundamental::ElementType::AssetClass;
    case ElementTypeIndustry:
      return fundamental::ElementType::Industry;
    default:
      throw std::invalid_argument("unreachable");
  }
}
inline fundamental::HoldingDetail convert(const lb_holding_detail_t* d) {
  return { d->industry_id ? d->industry_id : "",
           d->industry_name ? d->industry_name : "",
           d->index ? d->index : "",
           d->index_name ? d->index_name : "",
           d->holding_type ? d->holding_type : "",
           d->holding_type_name ? d->holding_type_name : "" };
}
inline fundamental::AssetAllocationItem convert(const lb_asset_allocation_item_t* item) {
  std::map<std::string, std::string> name_locales;
  for (size_t i = 0; i < item->num_name_locales; ++i) {
    const auto& entry = item->name_locales[i];
    name_locales.emplace(entry.locale ? entry.locale : "", entry.name ? entry.name : "");
  }
  return { item->name ? item->name : "",
           item->code ? item->code : "",
           item->position_ratio ? item->position_ratio : "",
           item->symbol ? item->symbol : "",
           std::move(name_locales),
           item->holding_detail ? std::make_optional(convert(item->holding_detail))
                                : std::nullopt };
}
inline fundamental::AssetAllocationGroup convert(const lb_asset_allocation_group_t* g) {
  std::vector<fundamental::AssetAllocationItem> lists;
  for (size_t i = 0; i < g->num_lists; ++i) lists.push_back(convert(&g->lists[i]));
  return { g->report_date ? g->report_date : "", convert(g->asset_type), std::move(lists) };
}
inline fundamental::AssetAllocationResponse convert(const lb_asset_allocation_response_t* r) {
  std::vector<fundamental::AssetAllocationGroup> info;
  for (size_t i = 0; i < r->num_info; ++i) info.push_back(convert(&r->info[i]));
  return { std::move(info) };
}

// ── MarketContext conversions ─────────────────────────────────────

inline market::MarketTimeItem convert(const lb_market_time_item_t* item) {
  return { convert(item->market), item->trade_status, item->timestamp, item->delay_trade_status, item->delay_timestamp, item->sub_status, item->delay_sub_status };
}
inline market::MarketStatusResponse convert(const lb_market_status_response_t* r) {
  std::vector<market::MarketTimeItem> v;
  for (size_t i = 0; i < r->num_market_time; ++i) v.push_back(convert(&r->market_time[i]));
  return { std::move(v) };
}
inline market::BrokerHoldingEntry convert(const lb_broker_holding_entry_t* e) { return { e->name, e->parti_number, e->chg, e->strong }; }
inline market::BrokerHoldingTop convert(const lb_broker_holding_top_t* t) {
  std::vector<market::BrokerHoldingEntry> buy, sell;
  for (size_t i = 0; i < t->num_buy; ++i) buy.push_back(convert(&t->buy[i]));
  for (size_t i = 0; i < t->num_sell; ++i) sell.push_back(convert(&t->sell[i]));
  return { std::move(buy), std::move(sell), t->updated_at };
}
inline market::BrokerHoldingChanges convert(const lb_broker_holding_changes_t& c) { return { c.value, c.chg_1, c.chg_5, c.chg_20, c.chg_60 }; }
inline market::BrokerHoldingDetailItem convert(const lb_broker_holding_detail_item_t* item) { return { item->name, item->parti_number, convert(item->ratio), convert(item->shares), item->strong }; }
inline market::BrokerHoldingDetail convert(const lb_broker_holding_detail_t* d) {
  std::vector<market::BrokerHoldingDetailItem> list;
  for (size_t i = 0; i < d->num_list; ++i) list.push_back(convert(&d->list[i]));
  return { std::move(list), d->updated_at };
}
inline market::BrokerHoldingDailyItem convert(const lb_broker_holding_daily_item_t* item) { return { item->date, item->holding, item->ratio, item->chg }; }
inline market::BrokerHoldingDailyHistory convert(const lb_broker_holding_daily_history_t* h) {
  std::vector<market::BrokerHoldingDailyItem> list;
  for (size_t i = 0; i < h->num_list; ++i) list.push_back(convert(&h->list[i]));
  return { std::move(list) };
}
inline market::AhPremiumKline convert(const lb_ah_premium_kline_t* k) { return { k->aprice, k->apreclose, k->hprice, k->hpreclose, k->currency_rate, k->ahpremium_rate, k->price_spread, k->timestamp }; }
inline market::AhPremiumKlines convert(const lb_ah_premium_klines_t* r) {
  std::vector<market::AhPremiumKline> klines;
  for (size_t i = 0; i < r->num_klines; ++i) klines.push_back(convert(&r->klines[i]));
  return { std::move(klines) };
}
inline market::AhPremiumIntraday convert(const lb_ah_premium_intraday_t* r) {
  std::vector<market::AhPremiumKline> klines;
  for (size_t i = 0; i < r->num_klines; ++i) klines.push_back(convert(&r->klines[i]));
  return { std::move(klines) };
}
inline market::TradePriceLevel convert(const lb_trade_price_level_t* p) { return { p->buy_amount, p->neutral_amount, p->price, p->sell_amount }; }
inline market::TradeStatistics convert(const lb_trade_statistics_t* s) {
  std::vector<std::string> td;
  for (size_t i = 0; i < s->num_trade_date; ++i) td.push_back(s->trade_date[i]);
  return { s->avgprice, s->buy, s->neutral, s->preclose, s->sell, s->timestamp, s->total_amount, std::move(td), s->trades_count };
}
inline market::TradeStatsResponse convert(const lb_trade_stats_response_t* r) {
  std::vector<market::TradePriceLevel> trades;
  for (size_t i = 0; i < r->num_trades; ++i) trades.push_back(convert(&r->trades[i]));
  return { convert(&r->statistics), std::move(trades) };
}
inline market::AnomalyItem convert(const lb_anomaly_item_t* item) {
  std::vector<std::string> cv;
  for (size_t i = 0; i < item->num_change_values; ++i) cv.push_back(item->change_values[i]);
  return { item->symbol, item->name, item->alert_name, item->alert_time, std::move(cv), item->emotion };
}
inline market::AnomalyResponse convert(const lb_anomaly_response_t* r) {
  std::vector<market::AnomalyItem> changes;
  for (size_t i = 0; i < r->num_changes; ++i) changes.push_back(convert(&r->changes[i]));
  return { r->all_off, std::move(changes) };
}
inline market::ConstituentStock convert(const lb_constituent_stock_t* s) {
  std::vector<std::string> tags;
  for (size_t i = 0; i < s->num_tags; ++i) tags.push_back(s->tags[i]);
  return { s->symbol, s->name, s->last_done, s->prev_close, s->inflow, s->balance, s->amount, s->total_shares, std::move(tags), s->intro, s->market, s->circulating_shares, s->delay, s->chg, s->trade_status };
}
inline market::IndexConstituents convert(const lb_index_constituents_t* r) {
  std::vector<market::ConstituentStock> stocks;
  for (size_t i = 0; i < r->num_stocks; ++i) stocks.push_back(convert(&r->stocks[i]));
  return { r->fall_num, r->flat_num, r->rise_num, std::move(stocks) };
}
inline market::TopMoversStock convert(const lb_top_movers_stock_t* s) {
  std::vector<std::string> labels;
  for (size_t i = 0; i < s->num_labels; ++i) labels.push_back(s->labels[i] ? s->labels[i] : "");
  return { s->symbol ? s->symbol : "", s->code ? s->code : "", s->name ? s->name : "",
           s->full_name ? s->full_name : "", s->change ? s->change : "",
           s->last_done ? s->last_done : "", s->market ? s->market : "",
           s->logo ? s->logo : "", std::move(labels) };
}
inline market::TopMoversEvent convert(const lb_top_movers_event_t* e) {
  return { e->timestamp ? e->timestamp : "", e->alert_reason ? e->alert_reason : "",
           e->alert_type, convert(&e->stock), e->post ? e->post : "" };
}
inline market::TopMoversResponse convert(const lb_top_movers_response_t* r) {
  std::vector<market::TopMoversEvent> events;
  for (size_t i = 0; i < r->num_events; ++i) events.push_back(convert(&r->events[i]));
  return { std::move(events), r->next_params ? r->next_params : "" };
}
inline market::RankListItem convert(const lb_rank_list_item_t* item) {
  return { item->symbol ? item->symbol : "", item->code ? item->code : "",
           item->name ? item->name : "", item->last_done ? item->last_done : "",
           item->chg ? item->chg : "", item->change ? item->change : "",
           item->inflow ? item->inflow : "", item->market_cap ? item->market_cap : "",
           item->industry ? item->industry : "",
           item->pre_post_price ? item->pre_post_price : "",
           item->pre_post_chg ? item->pre_post_chg : "",
           item->amplitude ? item->amplitude : "",
           item->five_day_chg ? item->five_day_chg : "",
           item->turnover_rate ? item->turnover_rate : "",
           item->volume_rate ? item->volume_rate : "",
           item->pb_ttm ? item->pb_ttm : "" };
}
inline market::RankListResponse convert(const lb_rank_list_response_t* r) {
  std::vector<market::RankListItem> lists;
  for (size_t i = 0; i < r->num_lists; ++i) lists.push_back(convert(&r->lists[i]));
  return { r->bmp, std::move(lists) };
}

// ── FundamentalContext conversions ────────────────────────────────

inline fundamental::InstitutionRatingDetailEvaluateItem convert(const lb_institution_rating_detail_evaluate_item_t* e) {
  return { e->buy, e->date, e->hold, e->sell, e->strong_buy, e->under };
}
inline fundamental::InstitutionRatingDetailTargetItem convert(const lb_institution_rating_detail_target_item_t* t) {
  return { t->avg_target, t->date, t->max_target, t->min_target, t->meet, t->price, t->timestamp };
}
inline fundamental::InstitutionRatingDetail convert(const lb_institution_rating_detail_t* r) {
  std::vector<fundamental::InstitutionRatingDetailEvaluateItem> ev;
  for (size_t i = 0; i < r->num_evaluate_list; ++i) ev.push_back(convert(&r->evaluate_list[i]));
  std::vector<fundamental::InstitutionRatingDetailTargetItem> tg;
  for (size_t i = 0; i < r->num_target_list; ++i) tg.push_back(convert(&r->target_list[i]));
  return { r->ccy_symbol, std::move(ev), r->data_percent ? r->data_percent : "", r->prediction_accuracy, r->updated_at, std::move(tg) };
}
inline fundamental::ForecastEpsItem convert(const lb_forecast_eps_item_t* item) {
  return { item->forecast_eps_median, item->forecast_eps_mean, item->forecast_eps_lowest,
           item->forecast_eps_highest, item->institution_total, item->institution_up,
           item->institution_down, item->forecast_start_date, item->forecast_end_date };
}
inline fundamental::ForecastEps convert(const lb_forecast_eps_t* r) {
  std::vector<fundamental::ForecastEpsItem> items;
  for (size_t i = 0; i < r->num_items; ++i) items.push_back(convert(&r->items[i]));
  return { std::move(items) };
}
// ValuationData and ValuationHistoryResponse defined after convert_opt_metric below

inline fundamental::DividendItem convert(const lb_dividend_item_t* item) { return { item->symbol, item->id, item->desc, item->record_date, item->ex_date, item->payment_date }; }
inline fundamental::DividendList convert(const lb_dividend_list_t* r) {
  std::vector<fundamental::DividendItem> list;
  for (size_t i = 0; i < r->num_list; ++i) list.push_back(convert(&r->list[i]));
  return { std::move(list) };
}
inline fundamental::RatingEvaluate convert(const lb_rating_evaluate_t& e) { return { e.buy, e.over, e.hold, e.under, e.sell, e.no_opinion, e.total, e.start_date, e.end_date }; }
inline fundamental::RatingTarget convert(const lb_rating_target_t& t) { return { t.highest_price, t.lowest_price, t.prev_close, t.start_date, t.end_date }; }
inline fundamental::RatingSummaryEvaluate convert(const lb_rating_summary_evaluate_t& e) { return { e.buy, e.date, e.hold, e.sell, e.strong_buy, e.under }; }
inline fundamental::InstitutionRating convert(const lb_institution_rating_t* r) {
  fundamental::InstitutionRatingLatest latest { convert(r->latest.evaluate), convert(r->latest.target), r->latest.industry_id, r->latest.industry_name, r->latest.industry_rank, r->latest.industry_total, r->latest.industry_mean, r->latest.industry_median };
  fundamental::InstitutionRatingSummary summary { r->summary.ccy_symbol, r->summary.change, convert(r->summary.evaluate), static_cast<fundamental::InstitutionRecommend>(r->summary.recommend), r->summary.target, r->summary.updated_at };
  return { std::move(latest), std::move(summary) };
}
inline fundamental::ValuationPoint convert(const lb_valuation_point_t* p) { return { p->timestamp, p->value }; }
inline fundamental::ValuationMetricData convert(const lb_valuation_metric_data_t* m) {
  std::vector<fundamental::ValuationPoint> list;
  for (size_t i = 0; i < m->num_list; ++i) list.push_back(convert(&m->list[i]));
  return { m->desc, m->high, m->low, m->median, std::move(list) };
}
inline std::optional<fundamental::ValuationMetricData> convert_opt_metric(const lb_valuation_metric_data_t* m) {
  if (!m) return std::nullopt;
  return convert(m);
}
inline fundamental::ValuationData convert(const lb_valuation_data_t* r) {
  fundamental::ValuationMetricsData metrics {
    convert_opt_metric(r->metrics.pe), convert_opt_metric(r->metrics.pb),
    convert_opt_metric(r->metrics.ps), convert_opt_metric(r->metrics.dvd_yld)
  };
  return { std::move(metrics) };
}
inline fundamental::ValuationHistoryResponse convert(const lb_valuation_history_response_t* r) {
  return { convert_opt_metric(r->pe), convert_opt_metric(r->pb), convert_opt_metric(r->ps) };
}
inline fundamental::CompanyOverview convert(const lb_company_overview_t* c) { return { c->name, c->company_name, c->founded, c->listing_date, c->market, c->region, c->address, c->office_address, c->website, c->issue_price, c->shares_offered, c->chairman, c->secretary, c->audit_inst, c->category, c->year_end, c->employees, c->phone, c->fax, c->email, c->legal_repr, c->manager, c->ticker, c->profile, c->sector }; }
inline fundamental::ShareholderStock convert(const lb_shareholder_stock_t* s) { return { s->symbol, s->code, s->market, s->chg }; }
inline fundamental::Shareholder convert(const lb_shareholder_t* s) {
  std::vector<fundamental::ShareholderStock> stocks;
  for (size_t i = 0; i < s->num_stocks; ++i) stocks.push_back(convert(&s->stocks[i]));
  return { s->shareholder_id, s->shareholder_name, s->institution_type, s->percent_of_shares, s->shares_changed, s->report_date, std::move(stocks) };
}
inline fundamental::ShareholderList convert(const lb_shareholder_list_t* r) {
  std::vector<fundamental::Shareholder> list;
  for (size_t i = 0; i < r->num_shareholder_list; ++i) list.push_back(convert(&r->shareholder_list[i]));
  return { std::move(list), r->forward_url, r->total };
}
inline fundamental::FundHolder convert(const lb_fund_holder_t* h) { return { h->code, h->symbol, h->currency, h->name, h->position_ratio, h->report_date }; }
inline fundamental::FundHolders convert(const lb_fund_holders_t* r) {
  std::vector<fundamental::FundHolder> lists;
  for (size_t i = 0; i < r->num_lists; ++i) lists.push_back(convert(&r->lists[i]));
  return { std::move(lists) };
}
inline fundamental::CorpActionItem convert(const lb_corp_action_item_t* item) { return { item->id, item->date, item->date_str, item->date_type, item->date_zone, item->act_type, item->act_desc, item->action, item->recent, item->is_delay, item->delay_content }; }
inline fundamental::CorpActions convert(const lb_corp_actions_t* r) {
  std::vector<fundamental::CorpActionItem> items;
  for (size_t i = 0; i < r->num_items; ++i) items.push_back(convert(&r->items[i]));
  return { std::move(items) };
}
inline fundamental::InvestSecurity convert(const lb_invest_security_t* s) { return { s->company_id, s->company_name, s->company_name_en, s->company_name_zhcn, s->symbol, s->currency, s->percent_of_shares, s->shares_rank, s->shares_value }; }
inline fundamental::InvestRelations convert(const lb_invest_relations_t* r) {
  std::vector<fundamental::InvestSecurity> secs;
  for (size_t i = 0; i < r->num_invest_securities; ++i) secs.push_back(convert(&r->invest_securities[i]));
  return { r->forward_url, std::move(secs) };
}
inline fundamental::OperatingIndicator convert(const lb_operating_indicator_t* ind) { return { ind->field_name, ind->indicator_name, ind->indicator_value, ind->yoy }; }
inline fundamental::OperatingItem convert(const lb_operating_item_t* item) {
  std::vector<fundamental::OperatingIndicator> inds;
  for (size_t i = 0; i < item->num_indicators; ++i) inds.push_back(convert(&item->indicators[i]));
  return { item->id, item->report, item->title, item->txt, item->latest, item->web_url, item->financial_currency, item->financial_name, item->financial_region, item->financial_report, std::move(inds) };
}
inline fundamental::OperatingList convert(const lb_operating_list_t* r) {
  std::vector<fundamental::OperatingItem> list;
  for (size_t i = 0; i < r->num_list; ++i) list.push_back(convert(&r->list[i]));
  return { std::move(list) };
}

// New fundamental conversions

inline fundamental::ConsensusDetail convert(const lb_consensus_detail_t* d) {
  return { d->key, d->name, d->description, d->actual, d->estimate, d->comp_value, d->comp_desc, d->comp, d->is_released };
}
inline fundamental::ConsensusReport convert(const lb_consensus_report_t* r) {
  std::vector<fundamental::ConsensusDetail> details;
  for (size_t i = 0; i < r->num_details; ++i) details.push_back(convert(&r->details[i]));
  return { r->fiscal_year, r->fiscal_period, r->period_text, std::move(details) };
}
inline fundamental::FinancialConsensus convert(const lb_financial_consensus_t* r) {
  std::vector<fundamental::ConsensusReport> list;
  for (size_t i = 0; i < r->num_list; ++i) list.push_back(convert(&r->list[i]));
  std::vector<std::string> opt_periods;
  for (size_t i = 0; i < r->num_opt_periods; ++i) opt_periods.push_back(r->opt_periods[i]);
  return { std::move(list), r->current_index, r->currency, std::move(opt_periods), r->current_period };
}
inline fundamental::IndustryValuationHistory convert(const lb_industry_valuation_history_t* h) {
  return { h->date, h->pe, h->pb, h->ps };
}
inline fundamental::IndustryValuationItem convert(const lb_industry_valuation_item_t* item) {
  std::vector<fundamental::IndustryValuationHistory> hist;
  for (size_t i = 0; i < item->num_history; ++i) hist.push_back(convert(&item->history[i]));
  return { item->symbol, item->name, item->currency, item->assets, item->bps, item->eps,
           item->dps, item->div_yld, item->div_payout_ratio, item->five_y_avg_dps, item->pe, std::move(hist) };
}
inline fundamental::IndustryValuationList convert(const lb_industry_valuation_list_t* r) {
  std::vector<fundamental::IndustryValuationItem> list;
  for (size_t i = 0; i < r->num_list; ++i) list.push_back(convert(&r->list[i]));
  return { std::move(list) };
}
inline fundamental::ValuationDist convert_valuation_dist(const lb_valuation_dist_t* d) {
  return { d->low, d->high, d->median, d->value, d->ranking, d->rank_index, d->rank_total };
}
inline fundamental::IndustryValuationDist convert(const lb_industry_valuation_dist_t* r) {
  std::optional<fundamental::ValuationDist> pe, pb, ps;
  if (r->pe) pe = convert_valuation_dist(r->pe);
  if (r->pb) pb = convert_valuation_dist(r->pb);
  if (r->ps) ps = convert_valuation_dist(r->ps);
  return { std::move(pe), std::move(pb), std::move(ps) };
}
inline fundamental::Professional convert(const lb_professional_t* p) {
  return { p->id, p->name, p->name_zhcn, p->name_en, p->title, p->biography, p->photo, p->wiki_url };
}
inline fundamental::ExecutiveGroup convert(const lb_executive_group_t* g) {
  std::vector<fundamental::Professional> profs;
  for (size_t i = 0; i < g->num_professionals; ++i) profs.push_back(convert(&g->professionals[i]));
  return { g->symbol, g->forward_url, g->total, std::move(profs) };
}
inline fundamental::ExecutiveList convert(const lb_executive_list_t* r) {
  std::vector<fundamental::ExecutiveGroup> list;
  for (size_t i = 0; i < r->num_professional_list; ++i) list.push_back(convert(&r->professional_list[i]));
  return { std::move(list) };
}
inline fundamental::BuybackHistoryItem convert(const lb_buyback_history_item_t* item) {
  return { item->fiscal_year, item->fiscal_year_range, item->net_buyback, item->net_buyback_yield, item->net_buyback_growth_rate, item->currency };
}
inline fundamental::BuybackRatios convert(const lb_buyback_ratios_t* r) {
  return { r->net_buyback_payout_ratio, r->net_buyback_to_cashflow_ratio };
}
inline fundamental::BuybackData convert(const lb_buyback_data_t* r) {
  std::optional<fundamental::RecentBuybacks> recent;
  if (r->recent_buybacks) {
    recent = fundamental::RecentBuybacks{ r->recent_buybacks->currency, r->recent_buybacks->net_buyback_ttm, r->recent_buybacks->net_buyback_yield_ttm };
  }
  std::vector<fundamental::BuybackHistoryItem> hist;
  for (size_t i = 0; i < r->num_buyback_history; ++i) hist.push_back(convert(&r->buyback_history[i]));
  std::vector<fundamental::BuybackRatios> ratios;
  for (size_t i = 0; i < r->num_buyback_ratios; ++i) ratios.push_back(convert(&r->buyback_ratios[i]));
  return { std::move(recent), std::move(hist), std::move(ratios) };
}
inline fundamental::RatingLeafIndicator convert(const lb_rating_leaf_indicator_t* leaf) {
  return { leaf->name, leaf->value, leaf->value_type, leaf->score, leaf->letter };
}
inline fundamental::RatingIndicator convert(const lb_rating_indicator_t* ind) {
  return { ind->name, ind->score, ind->letter };
}
inline fundamental::RatingSubIndicatorGroup convert(const lb_rating_sub_indicator_group_t* g) {
  std::vector<fundamental::RatingLeafIndicator> subs;
  for (size_t i = 0; i < g->num_sub_indicators; ++i) subs.push_back(convert(&g->sub_indicators[i]));
  return { convert(&g->indicator), std::move(subs) };
}
inline fundamental::RatingCategory convert(const lb_rating_category_t* cat) {
  std::vector<fundamental::RatingSubIndicatorGroup> subs;
  for (size_t i = 0; i < cat->num_sub_indicators; ++i) subs.push_back(convert(&cat->sub_indicators[i]));
  return { cat->kind, std::move(subs) };
}
inline fundamental::StockRatings convert(const lb_stock_ratings_t* r) {
  std::vector<fundamental::RatingCategory> ratings;
  for (size_t i = 0; i < r->num_ratings; ++i) ratings.push_back(convert(&r->ratings[i]));
  return { r->style_txt_name, r->scale_txt_name, r->report_period_txt, r->multi_score, r->multi_letter,
           r->multi_score_change, r->industry_name, r->industry_rank, r->industry_total,
           r->industry_mean_score, r->industry_median_score, std::move(ratings) };
}

// ── business_segments conversions ────────────────────────────────
inline fundamental::BusinessSegmentItem convert(const lb_business_segment_item_t* item) {
  return { item->name, item->percent };
}
inline fundamental::BusinessSegments convert(const lb_business_segments_t* r) {
  std::vector<fundamental::BusinessSegmentItem> business;
  for (size_t i = 0; i < r->num_business; ++i) business.push_back(convert(&r->business[i]));
  return { r->date, r->total, r->currency, std::move(business) };
}
inline fundamental::BusinessSegmentHistoryItem convert(const lb_business_segment_history_item_t* item) {
  return { item->name, item->percent, item->value };
}
inline fundamental::BusinessSegmentsHistoricalItem convert(const lb_business_segments_historical_item_t* item) {
  std::vector<fundamental::BusinessSegmentHistoryItem> business, regionals;
  for (size_t i = 0; i < item->num_business; ++i) business.push_back(convert(&item->business[i]));
  for (size_t i = 0; i < item->num_regionals; ++i) regionals.push_back(convert(&item->regionals[i]));
  return { item->date, item->total, item->currency, std::move(business), std::move(regionals) };
}
inline fundamental::BusinessSegmentsHistory convert(const lb_business_segments_history_t* r) {
  std::vector<fundamental::BusinessSegmentsHistoricalItem> hist;
  for (size_t i = 0; i < r->num_historical; ++i) hist.push_back(convert(&r->historical[i]));
  return { std::move(hist) };
}

// ── institution_rating_views conversion ──────────────────────────
inline fundamental::InstitutionRatingViewItem convert(const lb_institution_rating_view_item_t* item) {
  return { item->date, item->buy, item->over, item->hold, item->under, item->sell, item->total };
}
inline fundamental::InstitutionRatingViews convert(const lb_institution_rating_views_t* r) {
  std::vector<fundamental::InstitutionRatingViewItem> elist;
  for (size_t i = 0; i < r->num_elist; ++i) elist.push_back(convert(&r->elist[i]));
  return { std::move(elist) };
}

// ── industry_rank conversions ─────────────────────────────────────
inline fundamental::IndustryRankItem convert(const lb_industry_rank_item_t* item) {
  return { item->name, item->counter_id, item->chg, item->leading_name, item->leading_ticker,
           item->leading_chg, item->value_name, item->value_data };
}
inline fundamental::IndustryRankGroup convert(const lb_industry_rank_group_t* g) {
  std::vector<fundamental::IndustryRankItem> lists;
  for (size_t i = 0; i < g->num_lists; ++i) lists.push_back(convert(&g->lists[i]));
  return { std::move(lists) };
}
inline fundamental::IndustryRankResponse convert(const lb_industry_rank_response_t* r) {
  std::vector<fundamental::IndustryRankGroup> items;
  for (size_t i = 0; i < r->num_items; ++i) items.push_back(convert(&r->items[i]));
  return { std::move(items) };
}

// ── industry_peers conversions ────────────────────────────────────
inline fundamental::IndustryPeerNode convert(const lb_industry_peer_node_t* node) {
  return { node->name, node->counter_id, node->stock_num, node->chg, node->ytd_chg,
           node->next_json ? node->next_json : "" };
}
inline fundamental::IndustryPeersResponse convert(const lb_industry_peers_response_t* r) {
  fundamental::IndustryPeersTop top{ r->top.name, r->top.market };
  std::optional<fundamental::IndustryPeerNode> chain;
  if (r->chain) chain = convert(r->chain);
  return { std::move(top), std::move(chain) };
}

// ── financial_report_snapshot conversions ────────────────────────
inline fundamental::SnapshotForecastMetric convert(const lb_snapshot_forecast_metric_t* m) {
  return { m->value, m->yoy, m->cmp_desc, m->est_value };
}
inline fundamental::SnapshotReportedMetric convert(const lb_snapshot_reported_metric_t* m) {
  return { m->value, m->yoy };
}
inline fundamental::FinancialReportSnapshot convert(const lb_financial_report_snapshot_t* r) {
  std::optional<fundamental::SnapshotForecastMetric> fo_revenue, fo_ebit, fo_eps;
  if (r->fo_revenue) fo_revenue = convert(r->fo_revenue);
  if (r->fo_ebit) fo_ebit = convert(r->fo_ebit);
  if (r->fo_eps) fo_eps = convert(r->fo_eps);
  std::optional<fundamental::SnapshotReportedMetric> fr_revenue, fr_profit, fr_operate_cash,
      fr_invest_cash, fr_finance_cash, fr_total_assets, fr_total_liability;
  if (r->fr_revenue) fr_revenue = convert(r->fr_revenue);
  if (r->fr_profit) fr_profit = convert(r->fr_profit);
  if (r->fr_operate_cash) fr_operate_cash = convert(r->fr_operate_cash);
  if (r->fr_invest_cash) fr_invest_cash = convert(r->fr_invest_cash);
  if (r->fr_finance_cash) fr_finance_cash = convert(r->fr_finance_cash);
  if (r->fr_total_assets) fr_total_assets = convert(r->fr_total_assets);
  if (r->fr_total_liability) fr_total_liability = convert(r->fr_total_liability);
  return { r->name, r->ticker, r->fp_start, r->fp_end, r->currency, r->report_desc,
           std::move(fo_revenue), std::move(fo_ebit), std::move(fo_eps),
           std::move(fr_revenue), std::move(fr_profit), std::move(fr_operate_cash),
           std::move(fr_invest_cash), std::move(fr_finance_cash),
           std::move(fr_total_assets), std::move(fr_total_liability),
           r->fr_roe_ttm, r->fr_profit_margin, r->fr_profit_margin_ttm,
           r->fr_asset_turn_ttm, r->fr_leverage_ttm, r->fr_debt_assets_ratio };
}
inline fundamental::ValuationHistoryPoint convert(const lb_valuation_history_point_t* p) {
  return { p->date ? p->date : "", p->pe ? p->pe : "", p->pb ? p->pb : "", p->ps ? p->ps : "" };
}
inline fundamental::ValuationComparisonItem convert(const lb_valuation_comparison_item_t* item) {
  std::vector<fundamental::ValuationHistoryPoint> history;
  for (size_t i = 0; i < item->num_history; ++i) history.push_back(convert(&item->history[i]));
  return { item->symbol ? item->symbol : "", item->name ? item->name : "",
           item->currency ? item->currency : "", item->market_value ? item->market_value : "",
           item->price_close ? item->price_close : "", item->pe ? item->pe : "",
           item->pb ? item->pb : "", item->ps ? item->ps : "",
           item->roe ? item->roe : "", item->eps ? item->eps : "",
           item->bps ? item->bps : "", item->dps ? item->dps : "",
           item->div_yld ? item->div_yld : "", item->assets ? item->assets : "",
           std::move(history) };
}
inline fundamental::ValuationComparisonResponse convert(const lb_valuation_comparison_response_t* r) {
  std::vector<fundamental::ValuationComparisonItem> list;
  for (size_t i = 0; i < r->num_list; ++i) list.push_back(convert(&r->list[i]));
  return { std::move(list) };
}

// ── Portfolio conversions ─────────────────────────────────────────

inline portfolio::ProfitSummaryInfo convert(const lb_profit_summary_info_t* item) {
  return { static_cast<portfolio::AssetType>(item->asset_type), item->profit_max, item->profit_max_name, item->loss_max, item->loss_max_name };
}
inline portfolio::ProfitSummaryBreakdown convert(const lb_profit_summary_breakdown_t& b) {
  std::vector<portfolio::ProfitSummaryInfo> info;
  for (size_t i = 0; i < b.num_summary_info; ++i) info.push_back(convert(&b.summary_info[i]));
  return { b.stock, b.fund, b.crypto, b.mmf, b.other, b.cumulative_transaction_amount,
           b.trade_order_num, b.trade_stock_num, b.ipo_hit, b.ipo_subscription, std::move(info) };
}
inline portfolio::ProfitAnalysisSummary convert(const lb_profit_analysis_summary_t& s) {
  return { s.currency, s.current_total_asset, s.start_date, s.end_date, s.start_time, s.end_time,
           s.ending_asset_value, s.initial_asset_value, s.invest_amount, s.is_traded,
           s.sum_profit, s.sum_profit_rate, convert(s.profits) };
}
inline portfolio::ProfitAnalysisItem convert(const lb_profit_analysis_item_t* item) {
  return { item->name, item->market, item->is_holding, item->profit, item->profit_rate,
           item->clearance_times, static_cast<portfolio::AssetType>(item->item_type), item->currency, item->symbol,
           item->holding_period, item->security_code, item->isin,
           item->underlying_profit, item->derivatives_profit, item->order_profit };
}
inline portfolio::ProfitAnalysisSublist convert(const lb_profit_analysis_sublist_t& sl) {
  std::vector<portfolio::ProfitAnalysisItem> items;
  for (size_t i = 0; i < sl.num_items; ++i) items.push_back(convert(&sl.items[i]));
  return { sl.start, sl.end, sl.start_date, sl.end_date, sl.updated_at, sl.updated_date, std::move(items) };
}
inline portfolio::ProfitAnalysis convert(const lb_profit_analysis_t* r) {
  return { convert(r->summary), convert(r->sublist) };
}
inline portfolio::ProfitAnalysisByMarketItem convert(const lb_profit_analysis_by_market_item_t* item) {
  return { item->code, item->name, item->market, item->profit };
}
inline portfolio::ProfitAnalysisByMarket convert(const lb_profit_analysis_by_market_t* r) {
  std::vector<portfolio::ProfitAnalysisByMarketItem> items;
  for (size_t i = 0; i < r->num_stock_items; ++i) items.push_back(convert(&r->stock_items[i]));
  return { r->profit, r->has_more, std::move(items) };
}
inline portfolio::FlowItem convert(const lb_flow_item_t* item) {
  return { item->executed_date, item->executed_timestamp, item->code, static_cast<portfolio::FlowDirection>(item->direction),
           item->executed_quantity, item->executed_price, item->executed_cost, item->describe };
}
inline portfolio::ProfitAnalysisFlows convert(const lb_profit_analysis_flows_t* r) {
  std::vector<portfolio::FlowItem> flows;
  for (size_t i = 0; i < r->num_flows_list; ++i) flows.push_back(convert(&r->flows_list[i]));
  return { std::move(flows), r->has_more };
}
inline portfolio::ProfitDetailEntry convert(const lb_profit_detail_entry_t* e) {
  return { e->describe, e->amount };
}
inline portfolio::ProfitDetails convert_profit_details(const lb_profit_details_t& d) {
  std::vector<portfolio::ProfitDetailEntry> credited, debited, fee;
  for (size_t i = 0; i < d.num_credited_details; ++i) credited.push_back(convert(&d.credited_details[i]));
  for (size_t i = 0; i < d.num_debited_details; ++i) debited.push_back(convert(&d.debited_details[i]));
  for (size_t i = 0; i < d.num_fee_details; ++i) fee.push_back(convert(&d.fee_details[i]));
  return { d.holding_value, d.profit, d.cumulative_credited_amount, std::move(credited),
           d.cumulative_debited_amount, std::move(debited), d.cumulative_fee_amount, std::move(fee),
           d.short_holding_value, d.long_holding_value, d.holding_value_at_beginning, d.holding_value_at_ending };
}
inline portfolio::ProfitAnalysisDetail convert(const lb_profit_analysis_detail_t* r) {
  return { r->profit, convert_profit_details(r->underlying_details), convert_profit_details(r->derivative_pnl_details),
           r->name, r->updated_at, r->updated_date, r->currency, r->default_tag,
           r->start, r->end, r->start_date, r->end_date };
}

// ── AlertContext ──────────────────────────────────────────────────

inline alert::AlertItem convert(const lb_alert_item_t* item) {
  std::vector<int32_t> state(item->state, item->state + item->num_state);
  return { item->id, item->indicator_id, item->enabled, item->frequency, item->scope,
           item->text, std::move(state), item->value_map };
}
inline alert::AlertSymbolGroup convert(const lb_alert_symbol_group_t* g) {
  std::vector<alert::AlertItem> inds;
  for (size_t i = 0; i < g->num_indicators; ++i) inds.push_back(convert(&g->indicators[i]));
  return { g->symbol, g->code, g->market, g->name, g->price, g->chg, g->p_chg, g->product,
           std::move(inds) };
}
inline alert::AlertList convert(const lb_alert_list_t* r) {
  std::vector<alert::AlertSymbolGroup> lists;
  for (size_t i = 0; i < r->num_lists; ++i) lists.push_back(convert(&r->lists[i]));
  return { std::move(lists) };
}

// ── DCAContext ────────────────────────────────────────────────────

inline dca::DcaPlan convert(const lb_dca_plan_t* p) {
  return { p->plan_id, static_cast<dca::DCAStatus>(p->status), p->symbol, p->member_id, p->aaid, p->account_channel,
           p->display_account, convert(p->market), p->per_invest_amount, static_cast<dca::DCAFrequency>(p->invest_frequency),
           p->invest_day_of_week, p->invest_day_of_month, p->allow_margin_finance,
           p->alter_hours, p->created_at, p->updated_at, p->next_trd_date, p->stock_name,
           p->cum_amount, p->issue_number, p->average_cost, p->cum_profit };
}
inline dca::DcaList convert(const lb_dca_list_t* r) {
  std::vector<dca::DcaPlan> plans;
  for (size_t i = 0; i < r->num_plans; ++i) plans.push_back(convert(&r->plans[i]));
  return { std::move(plans) };
}
inline dca::DcaStats convert(const lb_dca_stats_t* r) {
  std::vector<dca::DcaPlan> np;
  for (size_t i = 0; i < r->num_nearest_plans; ++i) np.push_back(convert(&r->nearest_plans[i]));
  return { r->active_count, r->finished_count, r->suspended_count, std::move(np),
           r->rest_days, r->total_amount, r->total_profit };
}
inline dca::DcaSupportInfo convert(const lb_dca_support_info_t* s) {
  return { s->symbol, s->support_regular_saving };
}
inline dca::DcaSupportList convert(const lb_dca_support_list_t* r) {
  std::vector<dca::DcaSupportInfo> infos;
  for (size_t i = 0; i < r->num_infos; ++i) infos.push_back(convert(&r->infos[i]));
  return { std::move(infos) };
}
inline dca::DcaCalcDateResult convert(const lb_dca_calc_date_result_t* r) {
  return { r->trade_date };
}
inline dca::DcaCreateResult convert(const lb_dca_create_result_t* r) {
  return { r->plan_id };
}
inline dca::DcaHistoryRecord convert(const lb_dca_history_record_t* r) {
  return {
    r->created_at ? r->created_at : "",
    r->order_id ? r->order_id : "",
    r->status ? r->status : "",
    r->action ? r->action : "",
    r->order_type ? r->order_type : "",
    r->executed_qty ? r->executed_qty : "",
    r->executed_price ? r->executed_price : "",
    r->executed_amount ? r->executed_amount : "",
    r->rejected_reason ? r->rejected_reason : "",
    r->symbol ? r->symbol : ""
  };
}
inline dca::DcaHistoryResponse convert(const lb_dca_history_response_t* r) {
  std::vector<dca::DcaHistoryRecord> records;
  for (size_t i = 0; i < r->num_records; i++) records.push_back(convert(&r->records[i]));
  return { std::move(records), r->has_more };
}

// ── SharelistContext ──────────────────────────────────────────────

inline sharelist::SharelistStock convert(const lb_sharelist_stock_t* s) {
  std::optional<std::string> change = s->change ? std::optional<std::string>(s->change) : std::nullopt;
  std::optional<std::string> last_done = s->last_done ? std::optional<std::string>(s->last_done) : std::nullopt;
  std::optional<int32_t> trade_status = s->has_trade_status ? std::optional<int32_t>(s->trade_status) : std::nullopt;
  return { s->symbol, s->name, s->market, s->code, s->intro, s->unread_change_log_category,
           std::move(change), std::move(last_done), std::move(trade_status) };
}
inline sharelist::SharelistScopes convert(const lb_sharelist_scopes_t* s) {
  return { s->subscription, s->is_self };
}
inline sharelist::SharelistInfo convert(const lb_sharelist_info_t* info) {
  std::vector<sharelist::SharelistStock> stocks;
  for (size_t i = 0; i < info->num_stocks; ++i) stocks.push_back(convert(&info->stocks[i]));
  return { info->id, info->name, info->description, info->cover, info->subscribers_count,
           info->created_at, info->edited_at, info->this_year_chg, info->creator,
           std::move(stocks), info->subscribed, info->chg, info->sharelist_type,
           info->industry_code };
}
inline sharelist::SharelistList convert(const lb_sharelist_list_t* r) {
  std::vector<sharelist::SharelistInfo> sl, ssl;
  for (size_t i = 0; i < r->num_sharelists; ++i) sl.push_back(convert(&r->sharelists[i]));
  for (size_t i = 0; i < r->num_subscribed_sharelists; ++i)
    ssl.push_back(convert(&r->subscribed_sharelists[i]));
  return { std::move(sl), std::move(ssl), r->tail_mark };
}
inline sharelist::SharelistDetail convert(const lb_sharelist_detail_t* r) {
  return { convert(&r->sharelist), convert(&r->scopes) };
}

} // namespace convert

} // namespace longbridge
