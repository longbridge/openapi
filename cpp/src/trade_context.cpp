#include "trade_context.hpp"
#include "convert.hpp"
#include "utils.hpp"
#include <algorithm>
#include <iterator>

namespace longport {
namespace trade {

using longport::convert::convert;

TradeContext::TradeContext()
  : ctx_(nullptr)
{
}

TradeContext::TradeContext(const lb_trade_context_t* ctx)
{

  ctx_ = ctx;
  if (ctx_) {
    lb_trade_context_retain(ctx_);
  }
}

TradeContext::TradeContext(const TradeContext& ctx)
{
  ctx_ = ctx.ctx_;
  if (ctx_) {
    lb_trade_context_retain(ctx_);
  }
}

TradeContext::TradeContext(TradeContext&& ctx)
{
  ctx_ = ctx.ctx_;
  ctx.ctx_ = nullptr;
}

TradeContext::~TradeContext()
{
  if (ctx_) {
    lb_trade_context_release(ctx_);
  }
}

TradeContext&
TradeContext::operator=(const TradeContext& ctx)
{
  ctx_ = ctx.ctx_;
  if (ctx_) {
    lb_trade_context_retain(ctx_);
  }
  return *this;
}

size_t
TradeContext::ref_count() const
{
  return ctx_ ? lb_trade_context_ref_count(ctx_) : 0;
}

void
TradeContext::create(const Config& config,
                     AsyncCallback<TradeContext, void> callback)
{
  lb_trade_context_new(
    config,
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<TradeContext, void>(res->userdata);
      auto* ctx_ptr = (lb_trade_context_t*)res->ctx;
      TradeContext ctx(ctx_ptr);
      if (ctx_ptr) {
        lb_trade_context_release(ctx_ptr);
      }
      (*callback_ptr)(
        AsyncResult<TradeContext, void>(ctx, Status(res->error), nullptr));
    },
    new AsyncCallback<TradeContext, void>(callback));
}

void
TradeContext::subscribe(const std::vector<TopicType>& topics,
                        AsyncCallback<TradeContext, void> callback) const
{
  std::vector<lb_topic_type_t> topics2;
  std::transform(topics.cbegin(),
                 topics.cend(),
                 std::back_inserter(topics2),
                 [](auto topic) { return convert(topic); });

  lb_trade_context_subscribe(
    ctx_,
    topics2.data(),
    topics2.size(),
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<TradeContext, void>(res->userdata);
      (*callback_ptr)(AsyncResult<TradeContext, void>(
        TradeContext((const lb_trade_context_t*)res->ctx),
        Status(res->error),
        nullptr));
    },
    new AsyncCallback<TradeContext, void>(callback));
}

void
TradeContext::unsubscribe(const std::vector<TopicType>& topics,
                          AsyncCallback<TradeContext, void> callback) const
{
  std::vector<lb_topic_type_t> topics2;
  std::transform(topics.cbegin(),
                 topics.cend(),
                 std::back_inserter(topics2),
                 [](auto topic) { return convert(topic); });

  lb_trade_context_unsubscribe(
    ctx_,
    topics2.data(),
    topics2.size(),
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<TradeContext, void>(res->userdata);
      (*callback_ptr)(AsyncResult<TradeContext, void>(
        TradeContext((const lb_trade_context_t*)res->ctx),
        Status(res->error),
        nullptr));
    },
    new AsyncCallback<TradeContext, void>(callback));
}

void
TradeContext::set_on_order_changed(
  PushCallback<TradeContext, PushOrderChanged> callback) const
{
  lb_trade_context_set_on_order_changed(
    ctx_,
    [](auto ctx, auto event, auto userdata) {
      auto callback_ptr =
        callback::get_push_callback<TradeContext, PushOrderChanged>(userdata);
      PushOrderChanged event2 = convert(event);
      (*callback_ptr)(
        PushEvent<TradeContext, PushOrderChanged>(TradeContext(ctx), &event2));
    },
    new PushCallback<TradeContext, PushOrderChanged>(callback),
    [](auto p) { delete (PushCallback<TradeContext, PushOrderChanged>*)p; });
}

void
TradeContext::history_executions(
  const std::optional<GetHistoryExecutionsOptions>& opts,
  AsyncCallback<TradeContext, std::vector<Execution>> callback) const
{
  lb_get_history_executions_options_t opts2 = { nullptr, nullptr, nullptr };

  if (opts) {
    opts2.start_at = opts->start_at ? &opts->start_at.value() : nullptr;
    opts2.end_at = opts->end_at ? &opts->end_at.value() : nullptr;
    opts2.symbol = opts->symbol ? opts->symbol->c_str() : nullptr;
  }

  lb_trade_context_history_executions(
    ctx_,
    &opts2,
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<TradeContext, std::vector<Execution>>(
          res->userdata);
      TradeContext ctx((const lb_trade_context_t*)res->ctx);
      Status status(res->error);

      if (status) {
        auto rows = (const lb_execution_t*)res->data;
        std::vector<Execution> rows2;
        std::transform(rows,
                       rows + res->length,
                       std::back_inserter(rows2),
                       [](auto row) { return convert(&row); });

        (*callback_ptr)(AsyncResult<TradeContext, std::vector<Execution>>(
          ctx, std::move(status), &rows2));
      } else {
        (*callback_ptr)(AsyncResult<TradeContext, std::vector<Execution>>(
          ctx, std::move(status), nullptr));
      }
    },
    new AsyncCallback<TradeContext, std::vector<Execution>>(callback));
}

void
TradeContext::today_executions(
  const std::optional<GetTodayExecutionsOptions>& opts,
  AsyncCallback<TradeContext, std::vector<Execution>> callback) const
{
  lb_get_today_executions_options_t opts2 = { nullptr, nullptr };

  if (opts) {
    opts2.symbol = opts->symbol ? opts->symbol->c_str() : nullptr;
    opts2.order_id = opts->order_id ? opts->order_id->c_str() : nullptr;
  }

  lb_trade_context_today_executions(
    ctx_,
    &opts2,
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<TradeContext, std::vector<Execution>>(
          res->userdata);
      TradeContext ctx((const lb_trade_context_t*)res->ctx);
      Status status(res->error);

      if (status) {
        auto rows = (const lb_execution_t*)res->data;
        std::vector<Execution> rows2;
        std::transform(rows,
                       rows + res->length,
                       std::back_inserter(rows2),
                       [](auto row) { return convert(&row); });

        (*callback_ptr)(AsyncResult<TradeContext, std::vector<Execution>>(
          ctx, std::move(status), &rows2));
      } else {
        (*callback_ptr)(AsyncResult<TradeContext, std::vector<Execution>>(
          ctx, std::move(status), nullptr));
      }
    },
    new AsyncCallback<TradeContext, std::vector<Execution>>(callback));
}

/// Get history orders
void
TradeContext::history_orders(
  const std::optional<GetHistoryOrdersOptions>& opts,
  AsyncCallback<TradeContext, std::vector<Order>> callback) const
{
  lb_get_history_orders_options_t opts2 = {
    nullptr, nullptr, 0, nullptr, nullptr, nullptr, nullptr,
  };
  std::vector<lb_order_status_t> order_status;
  lb_order_side_t side;
  lb_market_t market;

  if (opts) {
    opts2.symbol = opts->symbol ? opts->symbol->c_str() : nullptr;

    if (opts->status) {
      std::transform(opts->status->cbegin(),
                     opts->status->cend(),
                     std::back_inserter(order_status),
                     [](auto status) { return convert(status); });
      if (order_status.size() > 0) {
        opts2.status = order_status.data();
        opts2.num_status = order_status.size();
      }
    }

    if (opts->side) {
      side = convert(*(opts->side));
      opts2.side = &side;
    }

    if (opts->market) {
      market = convert(*(opts->market));
      opts2.market = &market;
    }

    opts2.start_at = opts->start_at ? &opts->start_at.value() : nullptr;
    opts2.end_at = opts->end_at ? &opts->end_at.value() : nullptr;
  }

  lb_trade_context_history_orders(
    ctx_,
    &opts2,
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<TradeContext, std::vector<Order>>(
          res->userdata);
      TradeContext ctx((const lb_trade_context_t*)res->ctx);
      Status status(res->error);

      if (status) {
        auto rows = (const lb_order_t*)res->data;
        std::vector<Order> rows2;
        std::transform(rows,
                       rows + res->length,
                       std::back_inserter(rows2),
                       [](auto row) { return convert(&row); });

        (*callback_ptr)(AsyncResult<TradeContext, std::vector<Order>>(
          ctx, std::move(status), &rows2));
      } else {
        (*callback_ptr)(AsyncResult<TradeContext, std::vector<Order>>(
          ctx, std::move(status), nullptr));
      }
    },
    new AsyncCallback<TradeContext, std::vector<Order>>(callback));
}

void
TradeContext::today_orders(
  const std::optional<GetTodayOrdersOptions>& opts,
  AsyncCallback<TradeContext, std::vector<Order>> callback) const
{
  lb_get_today_orders_options_t opts2 = {
    nullptr, nullptr, 0, nullptr, nullptr, nullptr,
  };
  std::vector<lb_order_status_t> order_status;
  lb_order_side_t side;
  lb_market_t market;

  if (opts) {
    opts2.symbol = opts->symbol ? opts->symbol->c_str() : nullptr;

    if (opts->status) {
      std::transform(opts->status->cbegin(),
                     opts->status->cend(),
                     std::back_inserter(order_status),
                     [](auto status) { return convert(status); });
      if (order_status.size() > 0) {
        opts2.status = order_status.data();
        opts2.num_status = order_status.size();
      }
    }

    if (opts->side) {
      side = convert(*(opts->side));
      opts2.side = &side;
    }

    if (opts->market) {
      market = convert(*(opts->market));
      opts2.market = &market;
    }

    opts2.order_id = opts->order_id ? opts->order_id->c_str() : nullptr;
  }

  lb_trade_context_today_orders(
    ctx_,
    &opts2,
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<TradeContext, std::vector<Order>>(
          res->userdata);
      TradeContext ctx((const lb_trade_context_t*)res->ctx);
      Status status(res->error);

      if (status) {
        auto rows = (const lb_order_t*)res->data;
        std::vector<Order> rows2;
        std::transform(rows,
                       rows + res->length,
                       std::back_inserter(rows2),
                       [](auto row) { return convert(&row); });

        (*callback_ptr)(AsyncResult<TradeContext, std::vector<Order>>(
          ctx, std::move(status), &rows2));
      } else {
        (*callback_ptr)(AsyncResult<TradeContext, std::vector<Order>>(
          ctx, std::move(status), nullptr));
      }
    },
    new AsyncCallback<TradeContext, std::vector<Order>>(callback));
}

void
TradeContext::replace_order(const ReplaceOrderOptions& opts,
                            AsyncCallback<TradeContext, void> callback) const
{
  lb_replace_order_options_t opts2 = {
    opts.order_id.c_str(),
    opts.quantity,
    nullptr,
    nullptr,
    nullptr,
    nullptr,
    nullptr,
    nullptr,
  };

  opts2.price = opts.price ? (const lb_decimal_t*)opts.price.value() : nullptr;
  opts2.trigger_price = opts.trigger_price
                          ? (const lb_decimal_t*)opts.trigger_price.value()
                          : nullptr;
  opts2.limit_offset = opts.limit_offset
                         ? (const lb_decimal_t*)opts.limit_offset.value()
                         : nullptr;
  opts2.trailing_amount = opts.trailing_amount
                            ? (const lb_decimal_t*)opts.trailing_amount.value()
                            : nullptr;
  opts2.trailing_percent =
    opts.trailing_percent ? (const lb_decimal_t*)opts.trailing_percent.value()
                          : nullptr;
  opts2.remark = opts.remark ? opts.remark->c_str() : nullptr;

  lb_trade_context_replace_order(
    ctx_,
    &opts2,
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<TradeContext, void>(res->userdata);
      (*callback_ptr)(AsyncResult<TradeContext, void>(
        TradeContext((const lb_trade_context_t*)res->ctx),
        Status(res->error),
        nullptr));
    },
    new AsyncCallback<TradeContext, void>(callback));
}

void
TradeContext::submit_order(
  const SubmitOrderOptions& opts,
  AsyncCallback<TradeContext, SubmitOrderResponse> callback) const
{
  lb_submit_order_options_t opts2 = {
    opts.symbol.c_str(),
    convert(opts.order_type),
    convert(opts.side),
    opts.submitted_quantity,
    convert(opts.time_in_force),
    nullptr,
    nullptr,
    nullptr,
    nullptr,
    nullptr,
    nullptr,
    nullptr,
    opts.remark ? opts.remark->c_str() : nullptr,
  };
  lb_date_t expire_date;
  lb_outside_rth_t outside_rth;

  if (opts.submitted_price) {
    opts2.submitted_price = (const lb_decimal_t*)opts.submitted_price.value();
  }
  if (opts.trigger_price) {
    opts2.trigger_price = (const lb_decimal_t*)opts.trigger_price.value();
  }
  if (opts.limit_offset) {
    opts2.limit_offset = (const lb_decimal_t*)opts.limit_offset.value();
  }
  if (opts.trailing_amount) {
    opts2.trailing_amount = (const lb_decimal_t*)opts.trailing_amount.value();
  }
  if (opts.trailing_percent) {
    opts2.trailing_percent = (const lb_decimal_t*)opts.trailing_percent.value();
  }
  if (opts.expire_date) {
    expire_date = convert(&opts.expire_date.value());
    opts2.expire_date = &expire_date;
  }
  if (opts.outside_rth) {
    outside_rth = convert(*opts.outside_rth);
    opts2.outside_rth = &outside_rth;
  }

  lb_trade_context_submit_order(
    ctx_,
    &opts2,
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<TradeContext, SubmitOrderResponse>(
          res->userdata);
      TradeContext ctx((const lb_trade_context_t*)res->ctx);
      Status status(res->error);

      if (status) {
        SubmitOrderResponse resp =
          convert((const lb_submit_order_response_t*)res->data);
        (*callback_ptr)(AsyncResult<TradeContext, SubmitOrderResponse>(
          ctx, std::move(status), &resp));
      } else {
        (*callback_ptr)(AsyncResult<TradeContext, SubmitOrderResponse>(
          ctx, std::move(status), nullptr));
      }
    },
    new AsyncCallback<TradeContext, SubmitOrderResponse>(callback));
}

void
TradeContext::cancel_order(const std::string& order_id,
                           AsyncCallback<TradeContext, void> callback) const
{
  lb_trade_context_cancel_order(
    ctx_,
    order_id.c_str(),
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<TradeContext, void>(res->userdata);
      (*callback_ptr)(AsyncResult<TradeContext, void>(
        TradeContext((const lb_trade_context_t*)res->ctx),
        Status(res->error),
        nullptr));
    },
    new AsyncCallback<TradeContext, void>(callback));
}

void
TradeContext::account_balance(
  const std::string& currency,
  AsyncCallback<TradeContext, std::vector<AccountBalance>> callback) const
{
  lb_trade_context_account_balance(
    ctx_,
    currency.c_str(),
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<TradeContext, std::vector<AccountBalance>>(
          res->userdata);
      TradeContext ctx((const lb_trade_context_t*)res->ctx);
      Status status(res->error);

      if (status) {
        auto rows = (const lb_account_balance_t*)res->data;
        std::vector<AccountBalance> rows2;
        std::transform(rows,
                       rows + res->length,
                       std::back_inserter(rows2),
                       [](auto row) { return convert(&row); });

        (*callback_ptr)(AsyncResult<TradeContext, std::vector<AccountBalance>>(
          ctx, std::move(status), &rows2));
      } else {
        (*callback_ptr)(AsyncResult<TradeContext, std::vector<AccountBalance>>(
          ctx, std::move(status), nullptr));
      }
    },
    new AsyncCallback<TradeContext, std::vector<AccountBalance>>(callback));
}

void
TradeContext::account_balance(
  AsyncCallback<TradeContext, std::vector<AccountBalance>> callback) const
{
  lb_trade_context_account_balance(
    ctx_,
    nullptr,
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<TradeContext, std::vector<AccountBalance>>(
          res->userdata);
      TradeContext ctx((const lb_trade_context_t*)res->ctx);
      Status status(res->error);

      if (status) {
        auto rows = (const lb_account_balance_t*)res->data;
        std::vector<AccountBalance> rows2;
        std::transform(rows,
                       rows + res->length,
                       std::back_inserter(rows2),
                       [](auto row) { return convert(&row); });

        (*callback_ptr)(AsyncResult<TradeContext, std::vector<AccountBalance>>(
          ctx, std::move(status), &rows2));
      } else {
        (*callback_ptr)(AsyncResult<TradeContext, std::vector<AccountBalance>>(
          ctx, std::move(status), nullptr));
      }
    },
    new AsyncCallback<TradeContext, std::vector<AccountBalance>>(callback));
}

void
TradeContext::account_balance(
  const GetCashFlowOptions& opts,
  AsyncCallback<TradeContext, std::vector<CashFlow>> callback) const
{
  lb_get_cash_flow_options_t opts2 = {
    opts.start_at, opts.end_at, nullptr, nullptr, nullptr, nullptr,
  };
  lb_balance_type_t balance_type;

  if (opts.business_type) {
    balance_type = convert(*opts.business_type);
    opts2.business_type = &balance_type;
  }
  if (opts.symbol) {
    opts2.symbol = opts.symbol->c_str();
  }
  if (opts.page) {
    opts2.page = &opts.page.value();
  }
  if (opts.size) {
    opts2.size = &opts.size.value();
  }

  lb_trade_context_cash_flow(
    ctx_,
    &opts2,
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<TradeContext, std::vector<CashFlow>>(
          res->userdata);
      TradeContext ctx((const lb_trade_context_t*)res->ctx);
      Status status(res->error);

      if (status) {
        auto rows = (const lb_cash_flow_t*)res->data;
        std::vector<CashFlow> rows2;
        std::transform(rows,
                       rows + res->length,
                       std::back_inserter(rows2),
                       [](auto row) { return convert(&row); });

        (*callback_ptr)(AsyncResult<TradeContext, std::vector<CashFlow>>(
          ctx, std::move(status), &rows2));
      } else {
        (*callback_ptr)(AsyncResult<TradeContext, std::vector<CashFlow>>(
          ctx, std::move(status), nullptr));
      }
    },
    new AsyncCallback<TradeContext, std::vector<CashFlow>>(callback));
}

void
TradeContext::fund_positions(
  const std::optional<GetFundPositionsOptions>& opts,
  AsyncCallback<TradeContext, FundPositionsResponse> callback) const
{
  lb_get_fund_positions_options_t opts2 = { nullptr, 0 };
  std::vector<const char*> symbols;

  if (opts) {
    if (opts->symbols) {
      symbols = utils::get_cstring_vector(opts->symbols.value());
      opts2.symbols = symbols.data();
      opts2.num_symbols = symbols.size();
    }
  }

  lb_trade_context_fund_positions(
    ctx_,
    &opts2,
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<TradeContext, FundPositionsResponse>(
          res->userdata);
      TradeContext ctx((const lb_trade_context_t*)res->ctx);
      Status status(res->error);

      if (status) {
        FundPositionsResponse resp =
          convert((const lb_fund_position_response_t*)res->data);
        (*callback_ptr)(AsyncResult<TradeContext, FundPositionsResponse>(
          ctx, std::move(status), &resp));
      } else {
        (*callback_ptr)(AsyncResult<TradeContext, FundPositionsResponse>(
          ctx, std::move(status), nullptr));
      }
    },
    new AsyncCallback<TradeContext, FundPositionsResponse>(callback));
}

void
TradeContext::stock_positions(
  const std::optional<GetStockPositionsOptions>& opts,
  AsyncCallback<TradeContext, StockPositionsResponse> callback) const
{
  lb_get_stock_positions_options_t opts2 = { nullptr, 0 };
  std::vector<const char*> symbols;

  if (opts) {
    if (opts->symbols) {
      symbols = utils::get_cstring_vector(opts->symbols.value());
      opts2.symbols = symbols.data();
      opts2.num_symbols = symbols.size();
    }
  }

  lb_trade_context_stock_positions(
    ctx_,
    &opts2,
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<TradeContext, StockPositionsResponse>(
          res->userdata);
      TradeContext ctx((const lb_trade_context_t*)res->ctx);
      Status status(res->error);

      if (status) {
        StockPositionsResponse resp =
          convert((const lb_stock_position_response_t*)res->data);
        (*callback_ptr)(AsyncResult<TradeContext, StockPositionsResponse>(
          ctx, std::move(status), &resp));
      } else {
        (*callback_ptr)(AsyncResult<TradeContext, StockPositionsResponse>(
          ctx, std::move(status), nullptr));
      }
    },
    new AsyncCallback<TradeContext, StockPositionsResponse>(callback));
}

void
TradeContext::margin_ratio(
  const std::string& symbol,
  AsyncCallback<TradeContext, MarginRatio> callback) const
{
  lb_trade_context_margin_ratio(
    ctx_,
    symbol.c_str(),
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<TradeContext, MarginRatio>(res->userdata);
      TradeContext ctx((const lb_trade_context_t*)res->ctx);
      Status status(res->error);

      if (status) {
        MarginRatio resp = convert((const lb_margin_ratio_t*)res->data);
        (*callback_ptr)(AsyncResult<TradeContext, MarginRatio>(
          ctx, std::move(status), &resp));
      } else {
        (*callback_ptr)(AsyncResult<TradeContext, MarginRatio>(
          ctx, std::move(status), nullptr));
      }
    },
    new AsyncCallback<TradeContext, MarginRatio>(callback));
}

void
TradeContext::order_detail(
  const std::string& order_id,
  AsyncCallback<TradeContext, OrderDetail> callback) const
{
  lb_trade_context_order_detail(
    ctx_,
    order_id.c_str(),
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<TradeContext, OrderDetail>(res->userdata);
      TradeContext ctx((const lb_trade_context_t*)res->ctx);
      Status status(res->error);

      if (status) {
        OrderDetail resp = convert((const lb_order_detail_t*)res->data);
        (*callback_ptr)(AsyncResult<TradeContext, OrderDetail>(
          ctx, std::move(status), &resp));
      } else {
        (*callback_ptr)(AsyncResult<TradeContext, OrderDetail>(
          ctx, std::move(status), nullptr));
      }
    },
    new AsyncCallback<TradeContext, OrderDetail>(callback));
}

void
TradeContext::estimate_max_purchase_quantity(
  const EstimateMaxPurchaseQuantityOptions& opts,
  AsyncCallback<TradeContext, EstimateMaxPurchaseQuantityResponse> callback)
  const
{
  lb_estimate_max_purchase_quantity_options_t opts2 = {
    opts.symbol.c_str(),
    convert(opts.order_type),
    opts.price ? (const lb_decimal_t*)opts.price.value() : nullptr,
    convert(opts.side),
    opts.currency ? opts.currency->c_str() : nullptr,
    opts.order_id ? opts.order_id->c_str() : nullptr,
    opts.fractional_shares,
  };

  lb_trade_context_estimate_max_purchase_quantity(
    ctx_,
    &opts2,
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<TradeContext,
                                     EstimateMaxPurchaseQuantityResponse>(
          res->userdata);
      TradeContext ctx((const lb_trade_context_t*)res->ctx);
      Status status(res->error);

      if (status) {
        auto res_data =
          (const lb_estimate_max_purchase_quantity_response_t*)res->data;
        EstimateMaxPurchaseQuantityResponse resp = {
          res_data->cash_max_qty,
          res_data->margin_max_qty,
        };
        (*callback_ptr)(
          AsyncResult<TradeContext, EstimateMaxPurchaseQuantityResponse>(
            ctx, std::move(status), &resp));
      } else {
        (*callback_ptr)(
          AsyncResult<TradeContext, EstimateMaxPurchaseQuantityResponse>(
            ctx, std::move(status), nullptr));
      }
    },
    new AsyncCallback<TradeContext, EstimateMaxPurchaseQuantityResponse>(
      callback));
}

} // namespace trade
} // namespace longport
