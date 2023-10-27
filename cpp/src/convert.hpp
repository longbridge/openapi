#pragma once

#include "longbridge.h"
#include "types.hpp"
#include <algorithm>
#include <iterator>
#include <stdexcept>

namespace longbridge {
namespace convert {

using longbridge::quote::AdjustType;
using longbridge::quote::Brokers;
using longbridge::quote::CalcIndex;
using longbridge::quote::Candlestick;
using longbridge::quote::CapitalDistribution;
using longbridge::quote::CapitalDistributionResponse;
using longbridge::quote::CapitalFlowLine;
using longbridge::quote::Depth;
using longbridge::quote::DerivativeType;
using longbridge::quote::IntradayLine;
using longbridge::quote::IssuerInfo;
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
using longbridge::quote::RealtimeQuote;
using longbridge::quote::SecuritiesUpdateMode;
using longbridge::quote::SecurityBoard;
using longbridge::quote::SecurityBrokers;
using longbridge::quote::SecurityCalcIndex;
using longbridge::quote::SecurityDepth;
using longbridge::quote::SecurityQuote;
using longbridge::quote::SecurityStaticInfo;
using longbridge::quote::StrikePriceInfo;
using longbridge::quote::SubFlags;
using longbridge::quote::Subscription;
using longbridge::quote::Trade;
using longbridge::quote::TradeDirection;
using longbridge::quote::TradeSession;
using longbridge::quote::TradeStatus;
using longbridge::quote::TradingSessionInfo;
using longbridge::quote::WarrantQuote;
using longbridge::quote::WarrantType;
using longbridge::quote::WatchlistGroup;
using longbridge::quote::WatchlistSecurity;
using longbridge::trade::AccountBalance;
using longbridge::trade::BalanceType;
using longbridge::trade::CashFlow;
using longbridge::trade::CashFlowDirection;
using longbridge::trade::CashInfo;
using longbridge::trade::ChargeCategoryCode;
using longbridge::trade::CommissionFreeStatus;
using longbridge::trade::DeductionStatus;
using longbridge::trade::Execution;
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
using longbridge::trade::StockPosition;
using longbridge::trade::StockPositionChannel;
using longbridge::trade::StockPositionsResponse;
using longbridge::trade::SubmitOrderResponse;
using longbridge::trade::TimeInForceType;
using longbridge::trade::TopicType;
using longbridge::trade::TriggerStatus;

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
    case Period::Min5:
      return PeriodMin5;
    case Period::Min15:
      return PeriodMin15;
    case Period::Min30:
      return PeriodMin30;
    case Period::Min60:
      return PeriodMin60;
    case Period::Day:
      return PeriodDay;
    case Period::Week:
      return PeriodWeek;
    case Period::Month:
      return PeriodMonth;
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
    case PeriodMin5:
      return Period::Min5;
    case PeriodMin15:
      return Period::Min15;
    case PeriodMin30:
      return Period::Min30;
    case PeriodMin60:
      return Period::Min60;
    case PeriodDay:
      return Period::Day;
    case PeriodWeek:
      return Period::Week;
    case PeriodMonth:
      return Period::Month;
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
    case TradeSessionNormal:
      return TradeSession::Normal;
    case TradeSessionPre:
      return TradeSession::Pre;
    case TradeSessionPost:
      return TradeSession::Post;
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
  };
}

inline Depth
convert(const lb_depth_t* depth)
{
  return Depth{
    depth->position,
    Decimal(depth->price),
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
    candlestick->timestamp,
  };
}

inline PushCandlestick
convert(const lb_push_candlestick_t* info)
{
  return PushCandlestick{
    info->symbol,
    convert(info->period),
    convert(&info->candlestick),
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
    default:
      throw std::invalid_argument("unreachable");
  }
}

inline Order
convert(const lb_order_t* order)
{
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
    order->remark
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

inline AccountBalance
convert(const lb_account_balance_t* info)
{
  std::vector<CashInfo> cash_infos;
  std::transform(info->cash_infos,
                 info->cash_infos + info->num_cash_infos,
                 std::back_inserter(cash_infos),
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
    position->symbol,          position->symbol_name,
    position->quantity,        position->available_quantity,
    position->currency,        Decimal(position->cost_price),
    convert(position->market),
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
    resp->change_rate ? std::optional{ *resp->change_rate } : std::nullopt,
    resp->volume ? std::optional{ *resp->volume } : std::nullopt,
    resp->turnover ? std::optional{ Decimal(resp->turnover) } : std::nullopt,
    resp->ytd_change_rate ? std::optional{ *resp->ytd_change_rate }
                          : std::nullopt,
    resp->turnover_rate ? std::optional{ *resp->turnover_rate } : std::nullopt,
    resp->total_market_value
      ? std::optional{ Decimal(resp->total_market_value) }
      : std::nullopt,
    resp->capital_flow ? std::optional{ Decimal(resp->capital_flow) }
                       : std::nullopt,
    resp->amplitude ? std::optional{ *resp->amplitude } : std::nullopt,
    resp->volume_ratio ? std::optional{ *resp->volume_ratio } : std::nullopt,
    resp->pe_ttm_ratio ? std::optional{ *resp->pe_ttm_ratio } : std::nullopt,
    resp->pb_ratio ? std::optional{ *resp->pb_ratio } : std::nullopt,
    resp->dividend_ratio_ttm ? std::optional{ *resp->dividend_ratio_ttm }
                             : std::nullopt,
    resp->five_day_change_rate ? std::optional{ *resp->five_day_change_rate }
                               : std::nullopt,
    resp->ten_day_change_rate ? std::optional{ *resp->ten_day_change_rate }
                              : std::nullopt,
    resp->half_year_change_rate ? std::optional{ *resp->half_year_change_rate }
                                : std::nullopt,
    resp->five_minutes_change_rate
      ? std::optional{ *resp->five_minutes_change_rate }
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
    resp->outstanding_ratio ? std::optional{ *resp->outstanding_ratio }
                            : std::nullopt,
    resp->premium ? std::optional{ *resp->premium } : std::nullopt,
    resp->itm_otm ? std::optional{ *resp->itm_otm } : std::nullopt,
    resp->implied_volatility ? std::optional{ *resp->implied_volatility }
                             : std::nullopt,
    resp->warrant_delta ? std::optional{ *resp->warrant_delta } : std::nullopt,
    resp->call_price ? std::optional{ Decimal(resp->call_price) }
                     : std::nullopt,
    resp->to_call_price ? std::optional{ Decimal(resp->to_call_price) }
                        : std::nullopt,
    resp->effective_leverage ? std::optional{ *resp->effective_leverage }
                             : std::nullopt,
    resp->leverage_ratio ? std::optional{ *resp->leverage_ratio }
                         : std::nullopt,
    resp->conversion_ratio ? std::optional{ *resp->conversion_ratio }
                           : std::nullopt,
    resp->balance_point ? std::optional{ *resp->balance_point } : std::nullopt,
    resp->open_interest ? std::optional{ *resp->open_interest } : std::nullopt,
    resp->delta ? std::optional{ *resp->delta } : std::nullopt,
    resp->gamma ? std::optional{ *resp->gamma } : std::nullopt,
    resp->theta ? std::optional{ *resp->theta } : std::nullopt,
    resp->vega ? std::optional{ *resp->vega } : std::nullopt,
    resp->rho ? std::optional{ *resp->rho } : std::nullopt,
  };
}

} // namespace convert
} // namespace longbridge
