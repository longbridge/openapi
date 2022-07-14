#include "quote_context.hpp"
#include "convert.hpp"
#include "utils.hpp"
#include <algorithm>
#include <iterator>

namespace longbridge {
namespace quote {

using longbridge::convert::convert;

QuoteContext::QuoteContext()
  : ctx_(nullptr)
{
}

QuoteContext::QuoteContext(const lb_quote_context_t* ctx)
{
  ctx_ = ctx;
  if (ctx_) {
    lb_quote_context_retain(ctx_);
  }
}

QuoteContext::QuoteContext(const QuoteContext& ctx)
{
  ctx_ = ctx.ctx_;
  if (ctx_) {
    lb_quote_context_retain(ctx_);
  }
}

QuoteContext::~QuoteContext()
{
  if (ctx_) {
    lb_quote_context_release(ctx_);
  }
}

void
QuoteContext::create(const Config& config,
                     AsyncCallback<QuoteContext, void> callback)
{
  lb_quote_context_new(
    config,
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<QuoteContext, void>(res->userdata);
      (*callback_ptr)(AsyncResult<QuoteContext, void>(
        QuoteContext((const lb_quote_context_t*)res->ctx),
        Status(res->error),
        nullptr));
    },
    new AsyncCallback<QuoteContext, void>(callback));
}

void
QuoteContext::subscribe(const std::vector<std::string>& symbols,
                        SubFlags sub_flags,
                        bool is_first_push,
                        AsyncCallback<QuoteContext, void> callback) const
{
  auto c_symbols = utils::get_cstring_vector(symbols);

  lb_quote_context_subscribe(
    ctx_,
    c_symbols.data(),
    c_symbols.size(),
    sub_flags,
    is_first_push,
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<QuoteContext, void>(res->userdata);
      (*callback_ptr)(AsyncResult<QuoteContext, void>(
        QuoteContext((const lb_quote_context_t*)res->ctx),
        Status(res->error),
        nullptr));
    },
    new AsyncCallback<QuoteContext, void>(callback));
}

void
QuoteContext::unsubscribe(const std::vector<std::string>& symbols,
                          SubFlags sub_flags,
                          AsyncCallback<QuoteContext, void> callback) const
{
  auto c_symbols = utils::get_cstring_vector(symbols);

  lb_quote_context_unsubscribe(
    ctx_,
    c_symbols.data(),
    c_symbols.size(),
    sub_flags,
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<QuoteContext, void>(res->userdata);
      (*callback_ptr)(AsyncResult<QuoteContext, void>(
        QuoteContext((const lb_quote_context_t*)res->ctx),
        Status(res->error),
        nullptr));
    },
    new AsyncCallback<QuoteContext, void>(callback));
}

void
QuoteContext::subscribe_candlesticks(
  const std::string& symbol,
  Period period,
  AsyncCallback<QuoteContext, void> callback) const
{
  lb_quote_context_subscribe_candlesticks(
    ctx_,
    symbol.c_str(),
    convert(period),
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<QuoteContext, void>(res->userdata);
      (*callback_ptr)(AsyncResult<QuoteContext, void>(
        QuoteContext((const lb_quote_context_t*)res->ctx),
        Status(res->error),
        nullptr));
    },
    new AsyncCallback<QuoteContext, void>(callback));
}

void
QuoteContext::unsubscribe_candlesticks(
  const std::string& symbol,
  Period period,
  AsyncCallback<QuoteContext, void> callback) const
{
  lb_quote_context_unsubscribe_candlesticks(
    ctx_,
    symbol.c_str(),
    convert(period),
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<QuoteContext, void>(res->userdata);
      (*callback_ptr)(AsyncResult<QuoteContext, void>(
        QuoteContext((const lb_quote_context_t*)res->ctx),
        Status(res->error),
        nullptr));
    },
    new AsyncCallback<QuoteContext, void>(callback));
}

void
QuoteContext::subscriptions(
  AsyncCallback<QuoteContext, std::vector<Subscription>> callback) const
{
  lb_quote_context_subscrptions(
    ctx_,
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<QuoteContext, std::vector<Subscription>>(
          res->userdata);
      QuoteContext ctx((const lb_quote_context_t*)res->ctx);
      Status status(res->error);

      if (status) {
        auto rows = (const lb_subscription_t*)res->data;
        std::vector<Subscription> rows2;
        std::transform(rows,
                       rows + res->length,
                       std::back_inserter(rows2),
                       [](auto row) { return convert(&row); });

        (*callback_ptr)(AsyncResult<QuoteContext, std::vector<Subscription>>(
          ctx, std::move(status), &rows2));
      } else {
        (*callback_ptr)(AsyncResult<QuoteContext, std::vector<Subscription>>(
          ctx, std::move(status), nullptr));
      }
    },
    new AsyncCallback<QuoteContext, std::vector<Subscription>>(callback));
}

void
QuoteContext::set_on_quote(PushCallback<QuoteContext, PushQuote> callback) const
{
  lb_quote_context_set_on_quote(
    ctx_,
    [](auto ctx, auto event, auto userdata) {
      auto callback_ptr =
        callback::get_push_callback<QuoteContext, PushQuote>(userdata);
      PushQuote event2 = convert(event);
      (*callback_ptr)(
        PushEvent<QuoteContext, PushQuote>(QuoteContext(ctx), &event2));
    },
    new PushCallback<QuoteContext, PushQuote>(callback),
    [](auto p) { delete (PushCallback<QuoteContext, PushQuote>*)p; });
}

void
QuoteContext::set_on_depth(PushCallback<QuoteContext, PushDepth> callback) const
{
  lb_quote_context_set_on_depth(
    ctx_,
    [](auto ctx, auto event, auto userdata) {
      auto callback_ptr =
        callback::get_push_callback<QuoteContext, PushDepth>(userdata);
      PushDepth event2 = convert(event);
      (*callback_ptr)(
        PushEvent<QuoteContext, PushDepth>(QuoteContext(ctx), &event2));
    },
    new PushCallback<QuoteContext, PushDepth>(callback),
    [](auto p) { delete (PushCallback<QuoteContext, PushDepth>*)p; });
}

void
QuoteContext::set_on_brokers(
  PushCallback<QuoteContext, PushBrokers> callback) const
{
  lb_quote_context_set_on_brokers(
    ctx_,
    [](auto ctx, auto event, auto userdata) {
      auto callback_ptr =
        callback::get_push_callback<QuoteContext, PushBrokers>(userdata);
      PushBrokers event2 = convert(event);
      (*callback_ptr)(
        PushEvent<QuoteContext, PushBrokers>(QuoteContext(ctx), &event2));
    },
    new PushCallback<QuoteContext, PushBrokers>(callback),
    [](auto p) { delete (PushCallback<QuoteContext, PushBrokers>*)p; });
}

void
QuoteContext::set_on_trades(
  PushCallback<QuoteContext, PushTrades> callback) const
{
  lb_quote_context_set_on_trades(
    ctx_,
    [](auto ctx, auto event, auto userdata) {
      auto callback_ptr =
        callback::get_push_callback<QuoteContext, PushTrades>(userdata);
      PushTrades event2 = convert(event);
      (*callback_ptr)(
        PushEvent<QuoteContext, PushTrades>(QuoteContext(ctx), &event2));
    },
    new PushCallback<QuoteContext, PushTrades>(callback),
    [](auto p) { delete (PushCallback<QuoteContext, PushTrades>*)p; });
}

/// Set candlestick callback, after receiving the trades data push, it will call
/// back to this function.
void
QuoteContext::set_on_candlestick(
  PushCallback<QuoteContext, PushCandlestick> callback) const
{
  lb_quote_context_set_on_candlestick(
    ctx_,
    [](auto ctx, auto event, auto userdata) {
      auto callback_ptr =
        callback::get_push_callback<QuoteContext, PushCandlestick>(userdata);
      PushCandlestick event2 = convert(event);
      (*callback_ptr)(
        PushEvent<QuoteContext, PushCandlestick>(QuoteContext(ctx), &event2));
    },
    new PushCallback<QuoteContext, PushCandlestick>(callback),
    [](auto p) { delete (PushCallback<QuoteContext, PushCandlestick>*)p; });
}

void
QuoteContext::static_info(
  const std::vector<std::string>& symbols,
  AsyncCallback<QuoteContext, std::vector<SecurityStaticInfo>> callback) const
{
  auto c_symbols = utils::get_cstring_vector(symbols);

  lb_quote_context_static_info(
    ctx_,
    c_symbols.data(),
    c_symbols.size(),
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<QuoteContext,
                                     std::vector<SecurityStaticInfo>>(
          res->userdata);
      QuoteContext ctx((const lb_quote_context_t*)res->ctx);
      Status status(res->error);

      if (status) {
        auto rows = (const lb_security_static_info_t*)res->data;
        std::vector<SecurityStaticInfo> rows2;
        std::transform(rows,
                       rows + res->length,
                       std::back_inserter(rows2),
                       [](auto row) { return convert(&row); });

        (*callback_ptr)(
          AsyncResult<QuoteContext, std::vector<SecurityStaticInfo>>(
            ctx, std::move(status), &rows2));
      } else {
        (*callback_ptr)(
          AsyncResult<QuoteContext, std::vector<SecurityStaticInfo>>(
            ctx, std::move(status), nullptr));
      }
    },
    new AsyncCallback<QuoteContext, std::vector<SecurityStaticInfo>>(callback));
}

void
QuoteContext::quote(
  const std::vector<std::string>& symbols,
  AsyncCallback<QuoteContext, std::vector<SecurityQuote>> callback) const
{
  auto c_symbols = utils::get_cstring_vector(symbols);

  lb_quote_context_quote(
    ctx_,
    c_symbols.data(),
    c_symbols.size(),
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<QuoteContext, std::vector<SecurityQuote>>(
          res->userdata);
      QuoteContext ctx((const lb_quote_context_t*)res->ctx);
      Status status(res->error);

      if (status) {
        auto rows = (const lb_security_quote_t*)res->data;
        std::vector<SecurityQuote> rows2;
        std::transform(rows,
                       rows + res->length,
                       std::back_inserter(rows2),
                       [](auto row) { return convert(&row); });

        (*callback_ptr)(AsyncResult<QuoteContext, std::vector<SecurityQuote>>(
          ctx, std::move(status), &rows2));
      } else {
        (*callback_ptr)(AsyncResult<QuoteContext, std::vector<SecurityQuote>>(
          ctx, std::move(status), nullptr));
      }
    },
    new AsyncCallback<QuoteContext, std::vector<SecurityQuote>>(callback));
}

void
QuoteContext::option_quote(
  const std::vector<std::string>& symbols,
  AsyncCallback<QuoteContext, std::vector<OptionQuote>> callback) const
{
  auto c_symbols = utils::get_cstring_vector(symbols);

  lb_quote_context_option_quote(
    ctx_,
    c_symbols.data(),
    c_symbols.size(),
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<QuoteContext, std::vector<OptionQuote>>(
          res->userdata);
      QuoteContext ctx((const lb_quote_context_t*)res->ctx);
      Status status(res->error);

      if (status) {
        auto rows = (const lb_option_quote_t*)res->data;
        std::vector<OptionQuote> rows2;
        std::transform(rows,
                       rows + res->length,
                       std::back_inserter(rows2),
                       [](auto row) { return convert(&row); });

        (*callback_ptr)(AsyncResult<QuoteContext, std::vector<OptionQuote>>(
          ctx, std::move(status), &rows2));
      } else {
        (*callback_ptr)(AsyncResult<QuoteContext, std::vector<OptionQuote>>(
          ctx, std::move(status), nullptr));
      }
    },
    new AsyncCallback<QuoteContext, std::vector<OptionQuote>>(callback));
}

void
QuoteContext::warrant_quote(
  const std::vector<std::string>& symbols,
  AsyncCallback<QuoteContext, std::vector<WarrantQuote>> callback) const
{
  auto c_symbols = utils::get_cstring_vector(symbols);

  lb_quote_context_warrant_quote(
    ctx_,
    c_symbols.data(),
    c_symbols.size(),
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<QuoteContext, std::vector<WarrantQuote>>(
          res->userdata);
      QuoteContext ctx((const lb_quote_context_t*)res->ctx);
      Status status(res->error);

      if (status) {
        auto rows = (const lb_warrant_quote_t*)res->data;
        std::vector<WarrantQuote> rows2;
        std::transform(rows,
                       rows + res->length,
                       std::back_inserter(rows2),
                       [](auto row) { return convert(&row); });

        (*callback_ptr)(AsyncResult<QuoteContext, std::vector<WarrantQuote>>(
          ctx, std::move(status), &rows2));
      } else {
        (*callback_ptr)(AsyncResult<QuoteContext, std::vector<WarrantQuote>>(
          ctx, std::move(status), nullptr));
      }
    },
    new AsyncCallback<QuoteContext, std::vector<WarrantQuote>>(callback));
}

void
QuoteContext::depth(const std::string& symbol,
                    AsyncCallback<QuoteContext, SecurityDepth> callback) const
{
  lb_quote_context_depth(
    ctx_,
    symbol.c_str(),
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<QuoteContext, SecurityDepth>(
          res->userdata);
      QuoteContext ctx((const lb_quote_context_t*)res->ctx);
      Status status(res->error);

      if (status) {
        SecurityDepth depth = convert((const lb_security_depth_t*)res->data);
        (*callback_ptr)(AsyncResult<QuoteContext, SecurityDepth>(
          ctx, std::move(status), &depth));
      } else {
        (*callback_ptr)(AsyncResult<QuoteContext, SecurityDepth>(
          ctx, std::move(status), nullptr));
      }
    },
    new AsyncCallback<QuoteContext, SecurityDepth>(callback));
}

void
QuoteContext::brokers(
  const std::string& symbol,
  AsyncCallback<QuoteContext, SecurityBrokers> callback) const
{
  lb_quote_context_brokers(
    ctx_,
    symbol.c_str(),
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<QuoteContext, SecurityBrokers>(
          res->userdata);
      QuoteContext ctx((const lb_quote_context_t*)res->ctx);
      Status status(res->error);

      if (status) {
        SecurityBrokers brokers =
          convert((const lb_security_brokers_t*)res->data);
        (*callback_ptr)(AsyncResult<QuoteContext, SecurityBrokers>(
          ctx, std::move(status), &brokers));
      } else {
        (*callback_ptr)(AsyncResult<QuoteContext, SecurityBrokers>(
          ctx, std::move(status), nullptr));
      }
    },
    new AsyncCallback<QuoteContext, SecurityBrokers>(callback));
}

void
QuoteContext::participants(
  AsyncCallback<QuoteContext, std::vector<ParticipantInfo>> callback) const
{
  lb_quote_context_participants(
    ctx_,
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<QuoteContext,
                                     std::vector<ParticipantInfo>>(
          res->userdata);
      QuoteContext ctx((const lb_quote_context_t*)res->ctx);
      Status status(res->error);

      if (status) {
        auto rows = (const lb_participant_info_t*)res->data;
        std::vector<ParticipantInfo> rows2;
        std::transform(rows,
                       rows + res->length,
                       std::back_inserter(rows2),
                       [](auto row) { return convert(&row); });

        (*callback_ptr)(AsyncResult<QuoteContext, std::vector<ParticipantInfo>>(
          ctx, std::move(status), &rows2));
      } else {
        (*callback_ptr)(AsyncResult<QuoteContext, std::vector<ParticipantInfo>>(
          ctx, std::move(status), nullptr));
      }
    },
    new AsyncCallback<QuoteContext, std::vector<ParticipantInfo>>(callback));
}

void
QuoteContext::trades(
  const std::string& symbol,
  uintptr_t count,
  AsyncCallback<QuoteContext, std::vector<Trade>> callback) const
{
  lb_quote_context_trades(
    ctx_,
    symbol.c_str(),
    count,
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<QuoteContext, std::vector<Trade>>(
          res->userdata);
      QuoteContext ctx((const lb_quote_context_t*)res->ctx);
      Status status(res->error);

      if (status) {
        auto rows = (const lb_trade_t*)res->data;
        std::vector<Trade> rows2;
        std::transform(rows,
                       rows + res->length,
                       std::back_inserter(rows2),
                       [](auto row) { return convert(&row); });

        (*callback_ptr)(AsyncResult<QuoteContext, std::vector<Trade>>(
          ctx, std::move(status), &rows2));
      } else {
        (*callback_ptr)(AsyncResult<QuoteContext, std::vector<Trade>>(
          ctx, std::move(status), nullptr));
      }
    },
    new AsyncCallback<QuoteContext, std::vector<Trade>>(callback));
}

void
QuoteContext::intraday(
  const std::string& symbol,
  AsyncCallback<QuoteContext, std::vector<IntradayLine>> callback) const
{
  lb_quote_context_intraday(
    ctx_,
    symbol.c_str(),
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<QuoteContext, std::vector<IntradayLine>>(
          res->userdata);
      QuoteContext ctx((const lb_quote_context_t*)res->ctx);
      Status status(res->error);

      if (status) {
        auto rows = (const lb_intraday_line_t*)res->data;
        std::vector<IntradayLine> rows2;
        std::transform(rows,
                       rows + res->length,
                       std::back_inserter(rows2),
                       [](auto row) { return convert(&row); });

        (*callback_ptr)(AsyncResult<QuoteContext, std::vector<IntradayLine>>(
          ctx, std::move(status), &rows2));
      } else {
        (*callback_ptr)(AsyncResult<QuoteContext, std::vector<IntradayLine>>(
          ctx, std::move(status), nullptr));
      }
    },
    new AsyncCallback<QuoteContext, std::vector<IntradayLine>>(callback));
}

void
QuoteContext::candlesticks(
  const std::string& symbol,
  Period period,
  uintptr_t count,
  AdjustType adjust_type,
  AsyncCallback<QuoteContext, std::vector<Candlestick>> callback) const
{
  lb_quote_context_candlesticks(
    ctx_,
    symbol.c_str(),
    convert(period),
    count,
    convert(adjust_type),
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<QuoteContext, std::vector<Candlestick>>(
          res->userdata);
      QuoteContext ctx((const lb_quote_context_t*)res->ctx);
      Status status(res->error);

      if (status) {
        auto rows = (const lb_candlestick_t*)res->data;
        std::vector<Candlestick> rows2;
        std::transform(rows,
                       rows + res->length,
                       std::back_inserter(rows2),
                       [](auto row) { return convert(&row); });

        (*callback_ptr)(AsyncResult<QuoteContext, std::vector<Candlestick>>(
          ctx, std::move(status), &rows2));
      } else {
        (*callback_ptr)(AsyncResult<QuoteContext, std::vector<Candlestick>>(
          ctx, std::move(status), nullptr));
      }
    },
    new AsyncCallback<QuoteContext, std::vector<Candlestick>>(callback));
}

void
QuoteContext::option_chain_expiry_date_list(
  const std::string& symbol,
  AsyncCallback<QuoteContext, std::vector<Date>> callback) const
{
  lb_quote_context_option_chain_expiry_date_list(
    ctx_,
    symbol.c_str(),
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<QuoteContext, std::vector<Date>>(
          res->userdata);
      QuoteContext ctx((const lb_quote_context_t*)res->ctx);
      Status status(res->error);

      if (status) {
        auto rows = (const lb_date_t*)res->data;
        std::vector<Date> rows2;
        std::transform(rows,
                       rows + res->length,
                       std::back_inserter(rows2),
                       [](auto row) { return convert(&row); });

        (*callback_ptr)(AsyncResult<QuoteContext, std::vector<Date>>(
          ctx, std::move(status), &rows2));
      } else {
        (*callback_ptr)(AsyncResult<QuoteContext, std::vector<Date>>(
          ctx, std::move(status), nullptr));
      }
    },
    new AsyncCallback<QuoteContext, std::vector<Date>>(callback));
}

void
QuoteContext::option_chain_info_by_date(
  const std::string& symbol,
  Date expiry_date,
  AsyncCallback<QuoteContext, std::vector<StrikePriceInfo>> callback) const
{
  auto expiry_date2 = convert(&expiry_date);

  lb_quote_context_option_chain_info_by_date(
    ctx_,
    symbol.c_str(),
    &expiry_date2,
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<QuoteContext,
                                     std::vector<StrikePriceInfo>>(
          res->userdata);
      QuoteContext ctx((const lb_quote_context_t*)res->ctx);
      Status status(res->error);

      if (status) {
        auto rows = (const lb_strike_price_info_t*)res->data;
        std::vector<StrikePriceInfo> rows2;
        std::transform(rows,
                       rows + res->length,
                       std::back_inserter(rows2),
                       [](auto row) { return convert(&row); });

        (*callback_ptr)(AsyncResult<QuoteContext, std::vector<StrikePriceInfo>>(
          ctx, std::move(status), &rows2));
      } else {
        (*callback_ptr)(AsyncResult<QuoteContext, std::vector<StrikePriceInfo>>(
          ctx, std::move(status), nullptr));
      }
    },
    new AsyncCallback<QuoteContext, std::vector<StrikePriceInfo>>(callback));
}

void
QuoteContext::warrant_issuers(
  AsyncCallback<QuoteContext, std::vector<IssuerInfo>> callback) const
{
  lb_quote_context_warrant_issuers(
    ctx_,
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<QuoteContext, std::vector<IssuerInfo>>(
          res->userdata);
      QuoteContext ctx((const lb_quote_context_t*)res->ctx);
      Status status(res->error);

      if (status) {
        auto rows = (const lb_issuer_info_t*)res->data;
        std::vector<IssuerInfo> rows2;
        std::transform(rows,
                       rows + res->length,
                       std::back_inserter(rows2),
                       [](auto row) { return convert(&row); });

        (*callback_ptr)(AsyncResult<QuoteContext, std::vector<IssuerInfo>>(
          ctx, std::move(status), &rows2));
      } else {
        (*callback_ptr)(AsyncResult<QuoteContext, std::vector<IssuerInfo>>(
          ctx, std::move(status), nullptr));
      }
    },
    new AsyncCallback<QuoteContext, std::vector<IssuerInfo>>(callback));
}

void
QuoteContext::trading_session(
  AsyncCallback<QuoteContext, std::vector<MarketTradingSession>> callback) const
{
  lb_quote_context_trading_session(
    ctx_,
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<QuoteContext,
                                     std::vector<MarketTradingSession>>(
          res->userdata);
      QuoteContext ctx((const lb_quote_context_t*)res->ctx);
      Status status(res->error);

      if (status) {
        auto rows = (const lb_market_trading_session_t*)res->data;
        std::vector<MarketTradingSession> rows2;
        std::transform(rows,
                       rows + res->length,
                       std::back_inserter(rows2),
                       [](auto row) { return convert(&row); });

        (*callback_ptr)(
          AsyncResult<QuoteContext, std::vector<MarketTradingSession>>(
            ctx, std::move(status), &rows2));
      } else {
        (*callback_ptr)(
          AsyncResult<QuoteContext, std::vector<MarketTradingSession>>(
            ctx, std::move(status), nullptr));
      }
    },
    new AsyncCallback<QuoteContext, std::vector<MarketTradingSession>>(
      callback));
}

void
QuoteContext::trading_days(
  Market market,
  Date begin,
  Date end,
  AsyncCallback<QuoteContext, MarketTradingDays> callback)
{
  auto begin2 = convert(&begin);
  auto end2 = convert(&end);

  lb_quote_context_trading_days(
    ctx_,
    convert(market),
    &begin2,
    &end2,
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<QuoteContext, MarketTradingDays>(
          res->userdata);
      QuoteContext ctx((const lb_quote_context_t*)res->ctx);
      Status status(res->error);

      if (status) {
        MarketTradingDays days =
          convert((const lb_market_trading_days_t*)res->data);
        (*callback_ptr)(AsyncResult<QuoteContext, MarketTradingDays>(
          ctx, std::move(status), &days));
      } else {
        (*callback_ptr)(AsyncResult<QuoteContext, MarketTradingDays>(
          ctx, std::move(status), nullptr));
      }
    },
    new AsyncCallback<QuoteContext, MarketTradingDays>(callback));
}

void
QuoteContext::capital_flow(
  const std::string& symbol,
  AsyncCallback<QuoteContext, std::vector<CapitalFlowLine>> callback) const
{
  lb_quote_context_capital_flow(
    ctx_,
    symbol.c_str(),
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<QuoteContext,
                                     std::vector<CapitalFlowLine>>(
          res->userdata);
      QuoteContext ctx((const lb_quote_context_t*)res->ctx);
      Status status(res->error);

      if (status) {
        auto rows = (const lb_capital_flow_line_t*)res->data;
        std::vector<CapitalFlowLine> rows2;
        std::transform(rows,
                       rows + res->length,
                       std::back_inserter(rows2),
                       [](auto row) { return convert(&row); });

        (*callback_ptr)(AsyncResult<QuoteContext, std::vector<CapitalFlowLine>>(
          ctx, std::move(status), &rows2));
      } else {
        (*callback_ptr)(AsyncResult<QuoteContext, std::vector<CapitalFlowLine>>(
          ctx, std::move(status), nullptr));
      }
    },
    new AsyncCallback<QuoteContext, std::vector<CapitalFlowLine>>(callback));
}

void
QuoteContext::capital_distribution(
  const std::string& symbol,
  AsyncCallback<QuoteContext, CapitalDistributionResponse> callback) const
{
  lb_quote_context_capital_distribution(
    ctx_,
    symbol.c_str(),
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<QuoteContext, CapitalDistributionResponse>(
          res->userdata);
      QuoteContext ctx((const lb_quote_context_t*)res->ctx);
      Status status(res->error);

      if (status) {
        CapitalDistributionResponse resp =
          convert((const lb_capital_distribution_response_t*)res->data);
        (*callback_ptr)(AsyncResult<QuoteContext, CapitalDistributionResponse>(
          ctx, std::move(status), &resp));
      } else {
        (*callback_ptr)(AsyncResult<QuoteContext, CapitalDistributionResponse>(
          ctx, std::move(status), nullptr));
      }
    },
    new AsyncCallback<QuoteContext, CapitalDistributionResponse>(callback));
}

void
QuoteContext::realtime_quote(
  const std::vector<std::string>& symbols,
  AsyncCallback<QuoteContext, std::vector<RealtimeQuote>> callback) const
{
  auto c_symbols = utils::get_cstring_vector(symbols);

  lb_quote_context_realtime_quote(
    ctx_,
    c_symbols.data(),
    c_symbols.size(),
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<QuoteContext, std::vector<RealtimeQuote>>(
          res->userdata);
      QuoteContext ctx((const lb_quote_context_t*)res->ctx);
      Status status(res->error);

      if (status) {
        auto rows = (const lb_realtime_quote_t*)res->data;
        std::vector<RealtimeQuote> rows2;
        std::transform(rows,
                       rows + res->length,
                       std::back_inserter(rows2),
                       [](auto row) { return convert(&row); });

        (*callback_ptr)(AsyncResult<QuoteContext, std::vector<RealtimeQuote>>(
          ctx, std::move(status), &rows2));
      } else {
        (*callback_ptr)(AsyncResult<QuoteContext, std::vector<RealtimeQuote>>(
          ctx, std::move(status), nullptr));
      }
    },
    new AsyncCallback<QuoteContext, std::vector<RealtimeQuote>>(callback));
}

void
QuoteContext::realtime_depth(
  const std::string& symbol,
  AsyncCallback<QuoteContext, SecurityDepth> callback) const
{
  lb_quote_context_realtime_depth(
    ctx_,
    symbol.c_str(),
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<QuoteContext, SecurityDepth>(
          res->userdata);
      QuoteContext ctx((const lb_quote_context_t*)res->ctx);
      Status status(res->error);

      if (status) {
        SecurityDepth resp = convert((const lb_security_depth_t*)res->data);
        (*callback_ptr)(AsyncResult<QuoteContext, SecurityDepth>(
          ctx, std::move(status), &resp));
      } else {
        (*callback_ptr)(AsyncResult<QuoteContext, SecurityDepth>(
          ctx, std::move(status), nullptr));
      }
    },
    new AsyncCallback<QuoteContext, SecurityDepth>(callback));
}

void
QuoteContext::realtime_trades(
  const std::string& symbol,
  uint64_t count,
  AsyncCallback<QuoteContext, std::vector<Trade>> callback) const
{
  lb_quote_context_realtime_trades(
    ctx_,
    symbol.c_str(),
    count,
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<QuoteContext, std::vector<Trade>>(
          res->userdata);
      QuoteContext ctx((const lb_quote_context_t*)res->ctx);
      Status status(res->error);

      if (status) {
        auto rows = (const lb_trade_t*)res->data;
        std::vector<Trade> rows2;
        std::transform(rows,
                       rows + res->length,
                       std::back_inserter(rows2),
                       [](auto row) { return convert(&row); });

        (*callback_ptr)(AsyncResult<QuoteContext, std::vector<Trade>>(
          ctx, std::move(status), &rows2));
      } else {
        (*callback_ptr)(AsyncResult<QuoteContext, std::vector<Trade>>(
          ctx, std::move(status), nullptr));
      }
    },
    new AsyncCallback<QuoteContext, std::vector<Trade>>(callback));
}

void
QuoteContext::realtime_brokers(
  const std::string& symbol,
  AsyncCallback<QuoteContext, SecurityBrokers> callback) const
{
  lb_quote_context_realtime_brokers(
    ctx_,
    symbol.c_str(),
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<QuoteContext, SecurityBrokers>(
          res->userdata);
      QuoteContext ctx((const lb_quote_context_t*)res->ctx);
      Status status(res->error);

      if (status) {
        SecurityBrokers resp = convert((const lb_security_brokers_t*)res->data);
        (*callback_ptr)(AsyncResult<QuoteContext, SecurityBrokers>(
          ctx, std::move(status), &resp));
      } else {
        (*callback_ptr)(AsyncResult<QuoteContext, SecurityBrokers>(
          ctx, std::move(status), nullptr));
      }
    },
    new AsyncCallback<QuoteContext, SecurityBrokers>(callback));
}

void
QuoteContext::realtime_candlesticks(
  const std::string& symbol,
  Period period,
  uintptr_t count,
  AsyncCallback<QuoteContext, std::vector<Candlestick>> callback) const
{
  lb_quote_context_realtime_candlesticks(
    ctx_,
    symbol.c_str(),
    convert(period),
    count,
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<QuoteContext, std::vector<Candlestick>>(
          res->userdata);
      QuoteContext ctx((const lb_quote_context_t*)res->ctx);
      Status status(res->error);

      if (status) {
        auto rows = (const lb_candlestick_t*)res->data;
        std::vector<Candlestick> rows2;
        std::transform(rows,
                       rows + res->length,
                       std::back_inserter(rows2),
                       [](auto row) { return convert(&row); });

        (*callback_ptr)(AsyncResult<QuoteContext, std::vector<Candlestick>>(
          ctx, std::move(status), &rows2));
      } else {
        (*callback_ptr)(AsyncResult<QuoteContext, std::vector<Candlestick>>(
          ctx, std::move(status), nullptr));
      }
    },
    new AsyncCallback<QuoteContext, std::vector<Candlestick>>(callback));
}

} // namespace quote
} // namespace longbridge