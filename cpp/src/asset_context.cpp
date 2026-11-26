#include "asset_context.hpp"
#include "convert.hpp"
#include "longbridge.h"
#include "utils.hpp"
#include <algorithm>
#include <iterator>

namespace longbridge {
namespace asset {

using longbridge::convert::convert;

namespace {

/// Async callback delivering an array of `C` items converted to
/// `std::vector<T>`
template<typename T, typename C>
void
on_list_result(const lb_async_result_t* res)
{
  auto callback_ptr =
    callback::get_async_callback<AssetContext, std::vector<T>>(res->userdata);
  AssetContext ctx((const lb_asset_context_t*)res->ctx);
  Status status(res->error);

  if (status) {
    auto rows = (const C*)res->data;
    std::vector<T> rows2;
    rows2.reserve(res->length);
    for (uintptr_t i = 0; i < res->length; ++i) {
      rows2.push_back(convert(&rows[i]));
    }
    (*callback_ptr)(
      AsyncResult<AssetContext, std::vector<T>>(ctx, std::move(status), &rows2));
  } else {
    (*callback_ptr)(
      AsyncResult<AssetContext, std::vector<T>>(ctx, std::move(status), nullptr));
  }
}

/// Async callback delivering a single `C` value converted to `T`
template<typename T, typename C>
void
on_value_result(const lb_async_result_t* res)
{
  auto callback_ptr =
    callback::get_async_callback<AssetContext, T>(res->userdata);
  AssetContext ctx((const lb_asset_context_t*)res->ctx);
  Status status(res->error);

  if (status) {
    T value = convert((const C*)res->data);
    (*callback_ptr)(
      AsyncResult<AssetContext, T>(ctx, std::move(status), &value));
  } else {
    (*callback_ptr)(
      AsyncResult<AssetContext, T>(ctx, std::move(status), nullptr));
  }
}

/// Owns the C view of `GetStockTransfersInOptions` /
/// `GetStockTransfersOutOptions` for the duration of a call
template<typename Options, typename COptions, typename CState>
struct StockTransfersOptionsC
{
  COptions opts = {
    nullptr, nullptr, nullptr, nullptr, nullptr, nullptr,
  };
  CState state;

  explicit StockTransfersOptionsC(const std::optional<Options>& value)
  {
    if (!value) {
      return;
    }
    if (value->market) {
      opts.market = value->market->c_str();
    }
    if (value->state) {
      state = convert(*value->state);
      opts.state = &state;
    }
    if (value->start) {
      opts.start = &value->start.value();
    }
    if (value->end) {
      opts.end = &value->end.value();
    }
    if (value->page) {
      opts.page = &value->page.value();
    }
    if (value->size) {
      opts.size = &value->size.value();
    }
  }
};

} // namespace

AssetContext::AssetContext()
  : ctx_(nullptr)
{
}

AssetContext::AssetContext(const lb_asset_context_t* ctx)
{
  ctx_ = ctx;
  if (ctx_) {
    lb_asset_context_retain(ctx_);
  }
}

AssetContext::AssetContext(const AssetContext& ctx)
{
  ctx_ = ctx.ctx_;
  if (ctx_) {
    lb_asset_context_retain(ctx_);
  }
}

AssetContext::AssetContext(AssetContext&& ctx)
{
  ctx_ = ctx.ctx_;
  ctx.ctx_ = nullptr;
}

AssetContext::~AssetContext()
{
  if (ctx_) {
    lb_asset_context_release(ctx_);
  }
}

AssetContext&
AssetContext::operator=(const AssetContext& ctx)
{
  ctx_ = ctx.ctx_;
  if (ctx_) {
    lb_asset_context_retain(ctx_);
  }
  return *this;
}

AssetContext
AssetContext::create(const Config& config)
{
  auto* ctx_ptr = lb_asset_context_new(config);
  AssetContext ctx(ctx_ptr);
  if (ctx_ptr) {
    lb_asset_context_release(ctx_ptr);
  }
  return ctx;
}

void
AssetContext::statements(
  int32_t statement_type,
  int32_t start_date,
  int32_t limit,
  AsyncCallback<AssetContext, std::vector<StatementItem>> callback) const
{
  lb_asset_context_statements(
    ctx_,
    statement_type,
    start_date,
    limit,
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<AssetContext,
                                     std::vector<StatementItem>>(
          res->userdata);
      AssetContext ctx((const lb_asset_context_t*)res->ctx);
      Status status(res->error);

      if (status) {
        auto rows = (const lb_statement_item_t*)res->data;
        std::vector<StatementItem> rows2;
        std::transform(rows,
                       rows + res->length,
                       std::back_inserter(rows2),
                       [](const auto& row) {
                         StatementItem item;
                         item.dt = row.dt;
                         item.file_key = row.file_key;
                         return item;
                       });

        (*callback_ptr)(
          AsyncResult<AssetContext, std::vector<StatementItem>>(
            ctx, std::move(status), &rows2));
      } else {
        (*callback_ptr)(
          AsyncResult<AssetContext, std::vector<StatementItem>>(
            ctx, std::move(status), nullptr));
      }
    },
    new AsyncCallback<AssetContext, std::vector<StatementItem>>(callback));
}

void
AssetContext::statement_download_url(
  const std::string& file_key,
  AsyncCallback<AssetContext, StatementDownloadUrlResponse> callback) const
{
  lb_asset_context_download_url(
    ctx_,
    file_key.c_str(),
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<AssetContext,
                                     StatementDownloadUrlResponse>(
          res->userdata);
      AssetContext ctx((const lb_asset_context_t*)res->ctx);
      Status status(res->error);

      if (status) {
        StatementDownloadUrlResponse result;
        result.url = (const char*)res->data;

        (*callback_ptr)(
          AsyncResult<AssetContext, StatementDownloadUrlResponse>(
            ctx, std::move(status), &result));
      } else {
        (*callback_ptr)(
          AsyncResult<AssetContext, StatementDownloadUrlResponse>(
            ctx, std::move(status), nullptr));
      }
    },
    new AsyncCallback<AssetContext, StatementDownloadUrlResponse>(
      callback));
}

void
AssetContext::bank_cards(
  AsyncCallback<AssetContext, std::vector<BankCard>> callback) const
{
  lb_asset_context_bank_cards(
    ctx_,
    on_list_result<BankCard, lb_bank_card_t>,
    new AsyncCallback<AssetContext, std::vector<BankCard>>(callback));
}

void
AssetContext::edda_applications(
  const std::string& card_id,
  const std::string& currency,
  AsyncCallback<AssetContext, std::vector<EddaApplication>> callback) const
{
  lb_asset_context_edda_applications(
    ctx_,
    card_id.c_str(),
    currency.c_str(),
    on_list_result<EddaApplication, lb_edda_application_t>,
    new AsyncCallback<AssetContext, std::vector<EddaApplication>>(callback));
}

void
AssetContext::stock_transfers_in(
  const std::optional<GetStockTransfersInOptions>& opts,
  AsyncCallback<AssetContext, StockTransferInResponse> callback) const
{
  StockTransfersOptionsC<GetStockTransfersInOptions,
                         lb_get_stock_transfers_in_options_t,
                         lb_stock_transfer_in_state_t>
    opts2(opts);
  lb_asset_context_stock_transfers_in(
    ctx_,
    &opts2.opts,
    on_value_result<StockTransferInResponse, lb_stock_transfer_in_response_t>,
    new AsyncCallback<AssetContext, StockTransferInResponse>(callback));
}

void
AssetContext::stock_transfers_out(
  const std::optional<GetStockTransfersOutOptions>& opts,
  AsyncCallback<AssetContext, std::vector<StockTransferOut>> callback) const
{
  StockTransfersOptionsC<GetStockTransfersOutOptions,
                         lb_get_stock_transfers_out_options_t,
                         lb_stock_transfer_out_state_t>
    opts2(opts);
  lb_asset_context_stock_transfers_out(
    ctx_,
    &opts2.opts,
    on_list_result<StockTransferOut, lb_stock_transfer_out_t>,
    new AsyncCallback<AssetContext, std::vector<StockTransferOut>>(callback));
}

void
AssetContext::reward_records(
  const std::optional<GetRewardRecordsOptions>& opts,
  AsyncCallback<AssetContext, RewardRecordsResponse> callback) const
{
  lb_get_reward_records_options_t opts2 = { nullptr, nullptr, nullptr };
  lb_reward_type_t reward_type;

  if (opts) {
    if (opts->page) {
      opts2.page = &opts->page.value();
    }
    if (opts->page_size) {
      opts2.page_size = &opts->page_size.value();
    }
    if (opts->reward_type) {
      reward_type = convert(*opts->reward_type);
      opts2.reward_type = &reward_type;
    }
  }

  lb_asset_context_reward_records(
    ctx_,
    &opts2,
    on_value_result<RewardRecordsResponse, lb_reward_records_response_t>,
    new AsyncCallback<AssetContext, RewardRecordsResponse>(callback));
}

void
AssetContext::coupons(
  const std::optional<GetCouponsOptions>& opts,
  AsyncCallback<AssetContext, CouponsResponse> callback) const
{
  lb_get_coupons_options_t opts2 = { nullptr, nullptr, false, nullptr };

  if (opts) {
    if (opts->page) {
      opts2.page = &opts->page.value();
    }
    if (opts->page_size) {
      opts2.page_size = &opts->page_size.value();
    }
    opts2.expired = opts->expired;
    if (opts->card_type) {
      opts2.card_type = opts->card_type->c_str();
    }
  }

  lb_asset_context_coupons(
    ctx_,
    &opts2,
    on_value_result<CouponsResponse, lb_coupons_response_t>,
    new AsyncCallback<AssetContext, CouponsResponse>(callback));
}

void
AssetContext::coupon_orders(
  const std::string& card_id,
  CouponOrderType order_type,
  AsyncCallback<AssetContext, std::vector<CouponOrder>> callback) const
{
  lb_asset_context_coupon_orders(
    ctx_,
    card_id.c_str(),
    convert(order_type),
    on_list_result<CouponOrder, lb_coupon_order_t>,
    new AsyncCallback<AssetContext, std::vector<CouponOrder>>(callback));
}

void
AssetContext::deposits(
  const std::optional<GetDepositsOptions>& opts,
  AsyncCallback<AssetContext, DepositsResponse> callback) const
{
  lb_get_deposits_options_t opts2 = {
    nullptr, nullptr, nullptr, 0, nullptr, 0, nullptr, nullptr,
  };
  std::vector<lb_deposit_state_t> states;
  std::vector<const char*> currencies;

  if (opts) {
    if (opts->page) {
      opts2.page = &opts->page.value();
    }
    if (opts->size) {
      opts2.size = &opts->size.value();
    }
    if (opts->states) {
      for (auto state : opts->states.value()) {
        states.push_back(convert(state));
      }
      opts2.states = states.data();
      opts2.num_states = states.size();
    }
    if (opts->currencies) {
      currencies = utils::get_cstring_vector(opts->currencies.value());
      opts2.currencies = currencies.data();
      opts2.num_currencies = currencies.size();
    }
    if (opts->start) {
      opts2.start = &opts->start.value();
    }
    if (opts->end) {
      opts2.end = &opts->end.value();
    }
  }

  lb_asset_context_deposits(
    ctx_,
    &opts2,
    on_value_result<DepositsResponse, lb_deposits_response_t>,
    new AsyncCallback<AssetContext, DepositsResponse>(callback));
}

void
AssetContext::withdrawals(
  const std::optional<GetWithdrawalsOptions>& opts,
  AsyncCallback<AssetContext, WithdrawalsResponse> callback) const
{
  lb_get_withdrawals_options_t opts2 = { nullptr, nullptr, nullptr, nullptr };

  if (opts) {
    if (opts->page) {
      opts2.page = &opts->page.value();
    }
    if (opts->size) {
      opts2.size = &opts->size.value();
    }
    if (opts->start) {
      opts2.start = &opts->start.value();
    }
    if (opts->end) {
      opts2.end = &opts->end.value();
    }
  }

  lb_asset_context_withdrawals(
    ctx_,
    &opts2,
    on_value_result<WithdrawalsResponse, lb_withdrawals_response_t>,
    new AsyncCallback<AssetContext, WithdrawalsResponse>(callback));
}

} // namespace asset
} // namespace longbridge
