#include "grid_context.hpp"
#include "longbridge.h"
#include "convert.hpp"
#include "utils.hpp"
#include <algorithm>
#include <iterator>

namespace longbridge {
namespace grid {

using longbridge::convert::convert;
using longbridge::convert::convert_grid_trade_rule;

GridContext::GridContext()
  : ctx_(nullptr)
{
}

GridContext::GridContext(const lb_grid_context_t* ctx)
{
  ctx_ = ctx;
  if (ctx_)
    lb_grid_context_retain(ctx_);
}

GridContext::GridContext(const GridContext& ctx)
{
  ctx_ = ctx.ctx_;
  if (ctx_)
    lb_grid_context_retain(ctx_);
}

GridContext::GridContext(GridContext&& ctx)
{
  ctx_ = ctx.ctx_;
  ctx.ctx_ = nullptr;
}

GridContext::~GridContext()
{
  if (ctx_)
    lb_grid_context_release(ctx_);
}

GridContext&
GridContext::operator=(const GridContext& ctx)
{
  ctx_ = ctx.ctx_;
  if (ctx_)
    lb_grid_context_retain(ctx_);
  return *this;
}

GridContext
GridContext::create(const Config& config)
{
  auto* ptr = lb_grid_context_new(config);
  GridContext ctx(ptr);
  if (ptr)
    lb_grid_context_release(ptr);
  return ctx;
}

void
GridContext::submit(
  const SubmitGridOrderOptions& opts,
  AsyncCallback<GridContext, SubmitGridOrderResponse> callback) const
{
  lb_submit_grid_order_options_t opts2 = {
    opts.symbol.c_str(),
    opts.settlement_currency.c_str(),
    convert_grid_trade_rule(opts.grid_trading_rule),
  };

  lb_grid_context_submit(
    ctx_,
    &opts2,
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<GridContext, SubmitGridOrderResponse>(
          res->userdata);
      GridContext ctx((const lb_grid_context_t*)res->ctx);
      Status status(res->error);

      if (status) {
        SubmitGridOrderResponse resp =
          convert((const lb_submit_grid_order_response_t*)res->data);
        (*callback_ptr)(AsyncResult<GridContext, SubmitGridOrderResponse>(
          ctx, std::move(status), &resp));
      } else {
        (*callback_ptr)(AsyncResult<GridContext, SubmitGridOrderResponse>(
          ctx, std::move(status), nullptr));
      }
    },
    new AsyncCallback<GridContext, SubmitGridOrderResponse>(callback));
}

void
GridContext::replace(
  const ReplaceGridOrderOptions& opts,
  AsyncCallback<GridContext, void> callback) const
{
  lb_replace_grid_order_options_t opts2 = {
    opts.order_id.c_str(),
    convert_grid_trade_rule(opts.grid_trading_rule),
  };

  lb_grid_context_replace(
    ctx_,
    &opts2,
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<GridContext, void>(res->userdata);
      (*callback_ptr)(AsyncResult<GridContext, void>(
        GridContext((const lb_grid_context_t*)res->ctx),
        Status(res->error),
        nullptr));
    },
    new AsyncCallback<GridContext, void>(callback));
}

void
GridContext::list(
  const std::optional<GetGridOrdersOptions>& opts,
  AsyncCallback<GridContext, GridOrdersResponse> callback) const
{
  lb_get_grid_orders_options_t opts2 = {
    nullptr, nullptr, nullptr, nullptr, nullptr, nullptr, nullptr,
  };
  lb_market_t market;

  if (opts) {
    opts2.page = opts->page ? &opts->page.value() : nullptr;
    opts2.limit = opts->limit ? &opts->limit.value() : nullptr;
    if (opts->market) {
      market = convert(*opts->market);
      opts2.market = &market;
    }
    opts2.status = opts->status ? opts->status->c_str() : nullptr;
    opts2.symbol = opts->symbol ? opts->symbol->c_str() : nullptr;
    opts2.sort_by = opts->sort_by ? opts->sort_by->c_str() : nullptr;
    opts2.sort_order = opts->sort_order ? opts->sort_order->c_str() : nullptr;
  }

  lb_grid_context_list(
    ctx_,
    &opts2,
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<GridContext, GridOrdersResponse>(
          res->userdata);
      GridContext ctx((const lb_grid_context_t*)res->ctx);
      Status status(res->error);

      if (status) {
        GridOrdersResponse resp =
          convert((const lb_grid_orders_response_t*)res->data);
        (*callback_ptr)(AsyncResult<GridContext, GridOrdersResponse>(
          ctx, std::move(status), &resp));
      } else {
        (*callback_ptr)(AsyncResult<GridContext, GridOrdersResponse>(
          ctx, std::move(status), nullptr));
      }
    },
    new AsyncCallback<GridContext, GridOrdersResponse>(callback));
}

void
GridContext::list_by_ids(
  const GetGridOrdersByIdsOptions& opts,
  AsyncCallback<GridContext, std::vector<GridOrder>> callback) const
{
  std::vector<const char*> order_ids;
  std::transform(opts.order_ids.cbegin(),
                 opts.order_ids.cend(),
                 std::back_inserter(order_ids),
                 [](auto& id) { return id.c_str(); });
  lb_get_grid_orders_by_ids_options_t opts2 = {
    order_ids.data(),
    order_ids.size(),
  };

  lb_grid_context_list_by_ids(
    ctx_,
    &opts2,
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<GridContext, std::vector<GridOrder>>(
          res->userdata);
      GridContext ctx((const lb_grid_context_t*)res->ctx);
      Status status(res->error);

      if (status) {
        auto rows = (const lb_grid_order_t*)res->data;
        std::vector<GridOrder> rows2;
        std::transform(rows,
                       rows + res->length,
                       std::back_inserter(rows2),
                       [](auto& row) { return convert(&row); });

        (*callback_ptr)(AsyncResult<GridContext, std::vector<GridOrder>>(
          ctx, std::move(status), &rows2));
      } else {
        (*callback_ptr)(AsyncResult<GridContext, std::vector<GridOrder>>(
          ctx, std::move(status), nullptr));
      }
    },
    new AsyncCallback<GridContext, std::vector<GridOrder>>(callback));
}

void
GridContext::detail(
  const GetGridOrderDetailOptions& opts,
  AsyncCallback<GridContext, GridOrderDetail> callback) const
{
  lb_get_grid_order_detail_options_t opts2 = {
    opts.order_id.c_str(),
    opts.history_id ? opts.history_id->c_str() : nullptr,
    opts.limit ? &opts.limit.value() : nullptr,
  };

  lb_grid_context_detail(
    ctx_,
    &opts2,
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<GridContext, GridOrderDetail>(
          res->userdata);
      GridContext ctx((const lb_grid_context_t*)res->ctx);
      Status status(res->error);

      if (status) {
        GridOrderDetail resp =
          convert((const lb_grid_order_detail_t*)res->data);
        (*callback_ptr)(AsyncResult<GridContext, GridOrderDetail>(
          ctx, std::move(status), &resp));
      } else {
        (*callback_ptr)(AsyncResult<GridContext, GridOrderDetail>(
          ctx, std::move(status), nullptr));
      }
    },
    new AsyncCallback<GridContext, GridOrderDetail>(callback));
}

void
GridContext::trigger_history(
  const GetGridTriggerHistoryOptions& opts,
  AsyncCallback<GridContext, GridTriggerHistoryResponse> callback) const
{
  lb_get_grid_trigger_history_options_t opts2 = {
    opts.grid_order_id.c_str(),
    opts.page ? &opts.page.value() : nullptr,
    opts.limit ? &opts.limit.value() : nullptr,
  };

  lb_grid_context_trigger_history(
    ctx_,
    &opts2,
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<GridContext,
                                     GridTriggerHistoryResponse>(
          res->userdata);
      GridContext ctx((const lb_grid_context_t*)res->ctx);
      Status status(res->error);

      if (status) {
        GridTriggerHistoryResponse resp =
          convert((const lb_grid_trigger_history_response_t*)res->data);
        (*callback_ptr)(
          AsyncResult<GridContext, GridTriggerHistoryResponse>(
            ctx, std::move(status), &resp));
      } else {
        (*callback_ptr)(
          AsyncResult<GridContext, GridTriggerHistoryResponse>(
            ctx, std::move(status), nullptr));
      }
    },
    new AsyncCallback<GridContext, GridTriggerHistoryResponse>(callback));
}

void
GridContext::cancel(
  const std::string& order_id,
  AsyncCallback<GridContext, void> callback) const
{
  lb_grid_context_cancel(
    ctx_,
    order_id.c_str(),
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<GridContext, void>(res->userdata);
      (*callback_ptr)(AsyncResult<GridContext, void>(
        GridContext((const lb_grid_context_t*)res->ctx),
        Status(res->error),
        nullptr));
    },
    new AsyncCallback<GridContext, void>(callback));
}

void
GridContext::suspend(
  const std::string& order_id,
  AsyncCallback<GridContext, void> callback) const
{
  lb_grid_context_suspend(
    ctx_,
    order_id.c_str(),
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<GridContext, void>(res->userdata);
      (*callback_ptr)(AsyncResult<GridContext, void>(
        GridContext((const lb_grid_context_t*)res->ctx),
        Status(res->error),
        nullptr));
    },
    new AsyncCallback<GridContext, void>(callback));
}

void
GridContext::restart(
  const std::string& order_id,
  AsyncCallback<GridContext, void> callback) const
{
  lb_grid_context_restart(
    ctx_,
    order_id.c_str(),
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<GridContext, void>(res->userdata);
      (*callback_ptr)(AsyncResult<GridContext, void>(
        GridContext((const lb_grid_context_t*)res->ctx),
        Status(res->error),
        nullptr));
    },
    new AsyncCallback<GridContext, void>(callback));
}

void
GridContext::symbol_info(
  const std::string& symbol,
  AsyncCallback<GridContext, GridSymbolInfo> callback) const
{
  lb_grid_context_symbol_info(
    ctx_,
    symbol.c_str(),
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<GridContext, GridSymbolInfo>(
          res->userdata);
      GridContext ctx((const lb_grid_context_t*)res->ctx);
      Status status(res->error);

      if (status) {
        GridSymbolInfo resp =
          convert((const lb_grid_symbol_info_t*)res->data);
        (*callback_ptr)(AsyncResult<GridContext, GridSymbolInfo>(
          ctx, std::move(status), &resp));
      } else {
        (*callback_ptr)(AsyncResult<GridContext, GridSymbolInfo>(
          ctx, std::move(status), nullptr));
      }
    },
    new AsyncCallback<GridContext, GridSymbolInfo>(callback));
}

} // namespace grid
} // namespace longbridge
