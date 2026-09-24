#include "fund_context.hpp"
#include "longbridge.h"
#include "convert.hpp"
#include "fund_ffi.hpp"
#include "utils.hpp"
#include <algorithm>
#include <iterator>
#include <string>
#include <vector>

namespace longbridge {
namespace fund {

using longbridge::convert::convert;

FundContext::FundContext()
  : ctx_(nullptr)
{
}

FundContext::FundContext(const CFundContext* ctx)
{
  ctx_ = ctx;
  if (ctx_)
    lb_fund_context_retain(ctx_);
}

FundContext::FundContext(const FundContext& ctx)
{
  ctx_ = ctx.ctx_;
  if (ctx_)
    lb_fund_context_retain(ctx_);
}

FundContext::FundContext(FundContext&& ctx)
{
  ctx_ = ctx.ctx_;
  ctx.ctx_ = nullptr;
}

FundContext::~FundContext()
{
  if (ctx_)
    lb_fund_context_release(ctx_);
}

FundContext&
FundContext::operator=(const FundContext& ctx)
{
  ctx_ = ctx.ctx_;
  if (ctx_)
    lb_fund_context_retain(ctx_);
  return *this;
}

FundContext
FundContext::create(const Config& config)
{
  auto* ptr = lb_fund_context_new(config);
  FundContext ctx(ptr);
  if (ptr)
    lb_fund_context_release(ptr);
  return ctx;
}

// A vector-returning callback: `res->data` is a `const CElem*` array of
// `res->length` items.
#define FUND_VEC_CALLBACK(Elem, Value)                                         \
  [](auto res) {                                                               \
    auto callback_ptr =                                                         \
      callback::get_async_callback<FundContext, std::vector<Value>>(           \
        res->userdata);                                                         \
    FundContext ctx((const CFundContext*)res->ctx);                            \
    Status status(res->error);                                                  \
    if (status) {                                                               \
      auto rows = (const fund::ffi::Elem*)res->data;                            \
      std::vector<Value> rows2;                                                 \
      std::transform(rows,                                                      \
                     rows + res->length,                                        \
                     std::back_inserter(rows2),                                 \
                     [](auto& row) { return convert(&row); });                  \
      (*callback_ptr)(AsyncResult<FundContext, std::vector<Value>>(            \
        ctx, std::move(status), &rows2));                                       \
    } else {                                                                    \
      (*callback_ptr)(AsyncResult<FundContext, std::vector<Value>>(            \
        ctx, std::move(status), nullptr));                                      \
    }                                                                           \
  }

// A single-object-returning callback: `res->data` is a `const CElem*`.
#define FUND_OBJ_CALLBACK(Elem, Value)                                         \
  [](auto res) {                                                               \
    auto callback_ptr =                                                         \
      callback::get_async_callback<FundContext, Value>(res->userdata);         \
    FundContext ctx((const CFundContext*)res->ctx);                            \
    Status status(res->error);                                                  \
    if (status) {                                                               \
      Value resp = convert((const fund::ffi::Elem*)res->data);                 \
      (*callback_ptr)(                                                          \
        AsyncResult<FundContext, Value>(ctx, std::move(status), &resp));       \
    } else {                                                                    \
      (*callback_ptr)(                                                          \
        AsyncResult<FundContext, Value>(ctx, std::move(status), nullptr));     \
    }                                                                           \
  }

void
FundContext::hot_funds(
  AsyncCallback<FundContext, std::vector<HotFund>> callback) const
{
  lb_fund_context_hot_funds(
    ctx_,
    FUND_VEC_CALLBACK(CHotFund, HotFund),
    new AsyncCallback<FundContext, std::vector<HotFund>>(callback));
}

void
FundContext::funds(
  const std::optional<GetFundsOptions>& opts,
  AsyncCallback<FundContext, std::vector<FundBrief>> callback) const
{
  CGetFundsOptions opts2 = { nullptr, nullptr, 0, nullptr, 0 };
  std::vector<const char*> time_interval;

  if (opts) {
    opts2.filter = opts->filter ? opts->filter->c_str() : nullptr;
    if (!opts->quick_ids.empty()) {
      opts2.quick_ids = opts->quick_ids.data();
      opts2.num_quick_ids = opts->quick_ids.size();
    }
    std::transform(opts->time_interval.cbegin(),
                   opts->time_interval.cend(),
                   std::back_inserter(time_interval),
                   [](auto& s) { return s.c_str(); });
    if (!time_interval.empty()) {
      opts2.time_interval = time_interval.data();
      opts2.num_time_interval = time_interval.size();
    }
  }

  lb_fund_context_funds(
    ctx_,
    &opts2,
    FUND_VEC_CALLBACK(CFundBrief, FundBrief),
    new AsyncCallback<FundContext, std::vector<FundBrief>>(callback));
}

void
FundContext::filters(AsyncCallback<FundContext, FundFilters> callback) const
{
  lb_fund_context_filters(
    ctx_,
    FUND_OBJ_CALLBACK(CFundFilters, FundFilters),
    new AsyncCallback<FundContext, FundFilters>(callback));
}

void
FundContext::detail(const std::string& counter_id,
                    AsyncCallback<FundContext, FundDetail> callback) const
{
  lb_fund_context_detail(
    ctx_,
    counter_id.c_str(),
    FUND_OBJ_CALLBACK(CFundDetail, FundDetail),
    new AsyncCallback<FundContext, FundDetail>(callback));
}

void
FundContext::analysis(const std::string& counter_id,
                      const std::optional<GetFundAnalysisOptions>& opts,
                      AsyncCallback<FundContext, FundAnalysis> callback) const
{
  CGetFundAnalysisOptions opts2 = { nullptr };
  if (opts)
    opts2.period = opts->period ? &opts->period.value() : nullptr;

  lb_fund_context_analysis(
    ctx_,
    counter_id.c_str(),
    &opts2,
    FUND_OBJ_CALLBACK(CFundAnalysis, FundAnalysis),
    new AsyncCallback<FundContext, FundAnalysis>(callback));
}

void
FundContext::analysis_detail(
  const std::string& counter_id,
  const std::optional<GetFundAnalysisOptions>& opts,
  AsyncCallback<FundContext, FundAnalysisDetail> callback) const
{
  CGetFundAnalysisOptions opts2 = { nullptr };
  if (opts)
    opts2.period = opts->period ? &opts->period.value() : nullptr;

  lb_fund_context_analysis_detail(
    ctx_,
    counter_id.c_str(),
    &opts2,
    FUND_OBJ_CALLBACK(CFundAnalysisDetail, FundAnalysisDetail),
    new AsyncCallback<FundContext, FundAnalysisDetail>(callback));
}

void
FundContext::trend(const std::string& counter_id,
                   const std::optional<GetFundAnalysisOptions>& opts,
                   AsyncCallback<FundContext, FundTrend> callback) const
{
  CGetFundAnalysisOptions opts2 = { nullptr };
  if (opts)
    opts2.period = opts->period ? &opts->period.value() : nullptr;

  lb_fund_context_trend(
    ctx_,
    counter_id.c_str(),
    &opts2,
    FUND_OBJ_CALLBACK(CFundTrend, FundTrend),
    new AsyncCallback<FundContext, FundTrend>(callback));
}

void
FundContext::annual_returns(
  const std::string& counter_id,
  const std::optional<FundPageOptions>& opts,
  AsyncCallback<FundContext, std::vector<FundAnnualReturn>> callback) const
{
  CFundPageOptions opts2 = { nullptr, nullptr };
  if (opts) {
    opts2.page = opts->page ? &opts->page.value() : nullptr;
    opts2.size = opts->size ? &opts->size.value() : nullptr;
  }

  lb_fund_context_annual_returns(
    ctx_,
    counter_id.c_str(),
    &opts2,
    FUND_VEC_CALLBACK(CFundAnnualReturn, FundAnnualReturn),
    new AsyncCallback<FundContext, std::vector<FundAnnualReturn>>(callback));
}

void
FundContext::quarterly_returns(
  const std::string& counter_id,
  const std::optional<FundPageOptions>& opts,
  AsyncCallback<FundContext, std::vector<FundQuarterlyReturn>> callback) const
{
  CFundPageOptions opts2 = { nullptr, nullptr };
  if (opts) {
    opts2.page = opts->page ? &opts->page.value() : nullptr;
    opts2.size = opts->size ? &opts->size.value() : nullptr;
  }

  lb_fund_context_quarterly_returns(
    ctx_,
    counter_id.c_str(),
    &opts2,
    FUND_VEC_CALLBACK(CFundQuarterlyReturn, FundQuarterlyReturn),
    new AsyncCallback<FundContext, std::vector<FundQuarterlyReturn>>(callback));
}

void
FundContext::performance(
  const std::string& counter_id,
  AsyncCallback<FundContext, std::vector<FundPerformance>> callback) const
{
  lb_fund_context_performance(
    ctx_,
    counter_id.c_str(),
    FUND_VEC_CALLBACK(CFundPerformance, FundPerformance),
    new AsyncCallback<FundContext, std::vector<FundPerformance>>(callback));
}

void
FundContext::performance_comparison(
  const std::string& counter_id,
  const std::optional<GetFundAnalysisOptions>& opts,
  AsyncCallback<FundContext, FundPerformanceComparison> callback) const
{
  CGetFundAnalysisOptions opts2 = { nullptr };
  if (opts)
    opts2.period = opts->period ? &opts->period.value() : nullptr;

  lb_fund_context_performance_comparison(
    ctx_,
    counter_id.c_str(),
    &opts2,
    FUND_OBJ_CALLBACK(CFundPerformanceComparison, FundPerformanceComparison),
    new AsyncCallback<FundContext, FundPerformanceComparison>(callback));
}

void
FundContext::nav(
  const std::string& counter_id,
  AsyncCallback<FundContext, std::vector<FundNavValue>> callback) const
{
  lb_fund_context_nav(
    ctx_,
    counter_id.c_str(),
    FUND_VEC_CALLBACK(CFundNavValue, FundNavValue),
    new AsyncCallback<FundContext, std::vector<FundNavValue>>(callback));
}

void
FundContext::nav_history(
  const std::string& counter_id,
  const std::optional<FundPageOptions>& opts,
  AsyncCallback<FundContext, std::vector<FundNavValue>> callback) const
{
  CFundPageOptions opts2 = { nullptr, nullptr };
  if (opts) {
    opts2.page = opts->page ? &opts->page.value() : nullptr;
    opts2.size = opts->size ? &opts->size.value() : nullptr;
  }

  lb_fund_context_nav_history(
    ctx_,
    counter_id.c_str(),
    &opts2,
    FUND_VEC_CALLBACK(CFundNavValue, FundNavValue),
    new AsyncCallback<FundContext, std::vector<FundNavValue>>(callback));
}

void
FundContext::nav_range(
  const std::string& counter_id,
  const std::optional<FundNavRangeOptions>& opts,
  AsyncCallback<FundContext, std::vector<FundNavValue>> callback) const
{
  CFundNavRangeOptions opts2 = { nullptr, nullptr };
  if (opts) {
    opts2.month_before =
      opts->month_before ? &opts->month_before.value() : nullptr;
    opts2.year_before = opts->year_before ? &opts->year_before.value() : nullptr;
  }

  lb_fund_context_nav_range(
    ctx_,
    counter_id.c_str(),
    &opts2,
    FUND_VEC_CALLBACK(CFundNavValue, FundNavValue),
    new AsyncCallback<FundContext, std::vector<FundNavValue>>(callback));
}

void
FundContext::holdings(const std::string& counter_id,
                      const std::optional<GetFundHoldingsOptions>& opts,
                      AsyncCallback<FundContext, FundHoldings> callback) const
{
  CGetFundHoldingsOptions opts2 = { nullptr };
  if (opts)
    opts2.scene = opts->scene ? &opts->scene.value() : nullptr;

  lb_fund_context_holdings(
    ctx_,
    counter_id.c_str(),
    &opts2,
    FUND_OBJ_CALLBACK(CFundHoldings, FundHoldings),
    new AsyncCallback<FundContext, FundHoldings>(callback));
}

void
FundContext::stock_holdings(
  const std::string& counter_id,
  const std::optional<GetFundStockHoldingsOptions>& opts,
  AsyncCallback<FundContext, std::vector<FundStockHolding>> callback) const
{
  CGetFundStockHoldingsOptions opts2 = { nullptr };
  if (opts)
    opts2.limit = opts->limit ? &opts->limit.value() : nullptr;

  lb_fund_context_stock_holdings(
    ctx_,
    counter_id.c_str(),
    &opts2,
    FUND_VEC_CALLBACK(CFundStockHolding, FundStockHolding),
    new AsyncCallback<FundContext, std::vector<FundStockHolding>>(callback));
}

void
FundContext::positions(const std::optional<GetFundPositionsOptions>& opts,
                       AsyncCallback<FundContext, FundPositions> callback) const
{
  lb_fund_positions_options_t opts2 = { nullptr, nullptr };
  if (opts) {
    opts2.account_channel =
      opts->account_channel ? opts->account_channel->c_str() : nullptr;
    opts2.aaid = opts->aaid ? &opts->aaid.value() : nullptr;
  }

  lb_fund_context_positions(
    ctx_,
    &opts2,
    FUND_OBJ_CALLBACK(CFundPositions, FundPositions),
    new AsyncCallback<FundContext, FundPositions>(callback));
}

void
FundContext::position(
  const std::string& counter_id,
  const std::optional<GetFundPositionOptions>& opts,
  AsyncCallback<FundContext, FundPositionDetail> callback) const
{
  CGetFundPositionOptions opts2 = { nullptr, nullptr, nullptr, nullptr };
  if (opts) {
    opts2.account_channel =
      opts->account_channel ? opts->account_channel->c_str() : nullptr;
    opts2.aaid = opts->aaid ? &opts->aaid.value() : nullptr;
    opts2.start = opts->start ? opts->start->c_str() : nullptr;
    opts2.end = opts->end ? opts->end->c_str() : nullptr;
  }

  lb_fund_context_position(
    ctx_,
    counter_id.c_str(),
    &opts2,
    FUND_OBJ_CALLBACK(CFundPositionDetail, FundPositionDetail),
    new AsyncCallback<FundContext, FundPositionDetail>(callback));
}

void
FundContext::position_performance(
  const std::string& counter_id,
  AsyncCallback<FundContext, std::vector<FundPositionPerformance>> callback)
  const
{
  lb_fund_context_position_performance(
    ctx_,
    counter_id.c_str(),
    FUND_VEC_CALLBACK(CFundPositionPerformance, FundPositionPerformance),
    new AsyncCallback<FundContext, std::vector<FundPositionPerformance>>(
      callback));
}

void
FundContext::position_profits(
  const std::string& counter_id,
  const std::optional<GetFundPositionProfitsOptions>& opts,
  AsyncCallback<FundContext, FundPositionProfits> callback) const
{
  CGetFundPositionProfitsOptions opts2 = {
    nullptr, nullptr, nullptr, nullptr, nullptr, nullptr,
  };
  if (opts) {
    opts2.account_channel =
      opts->account_channel ? opts->account_channel->c_str() : nullptr;
    opts2.aaid = opts->aaid ? &opts->aaid.value() : nullptr;
    opts2.start = opts->start ? opts->start->c_str() : nullptr;
    opts2.end = opts->end ? opts->end->c_str() : nullptr;
    opts2.page = opts->page ? &opts->page.value() : nullptr;
    opts2.size = opts->size ? &opts->size.value() : nullptr;
  }

  lb_fund_context_position_profits(
    ctx_,
    counter_id.c_str(),
    &opts2,
    FUND_OBJ_CALLBACK(CFundPositionProfits, FundPositionProfits),
    new AsyncCallback<FundContext, FundPositionProfits>(callback));
}

void
FundContext::position_nav(
  const std::string& counter_id,
  const std::optional<FundNavRangeOptions>& opts,
  AsyncCallback<FundContext, std::vector<FundPositionNav>> callback) const
{
  CFundNavRangeOptions opts2 = { nullptr, nullptr };
  if (opts) {
    opts2.month_before =
      opts->month_before ? &opts->month_before.value() : nullptr;
    opts2.year_before = opts->year_before ? &opts->year_before.value() : nullptr;
  }

  lb_fund_context_position_nav(
    ctx_,
    counter_id.c_str(),
    &opts2,
    FUND_VEC_CALLBACK(CFundPositionNav, FundPositionNav),
    new AsyncCallback<FundContext, std::vector<FundPositionNav>>(callback));
}

void
FundContext::position_dividends(
  const std::string& counter_id,
  const std::optional<GetFundPositionDividendsOptions>& opts,
  AsyncCallback<FundContext, FundDividends> callback) const
{
  CGetFundPositionDividendsOptions opts2 = {
    nullptr, nullptr, nullptr, nullptr, nullptr, nullptr, nullptr,
  };
  if (opts) {
    opts2.account_channel =
      opts->account_channel ? opts->account_channel->c_str() : nullptr;
    opts2.aaid = opts->aaid ? &opts->aaid.value() : nullptr;
    opts2.currency = opts->currency ? opts->currency->c_str() : nullptr;
    opts2.start = opts->start ? &opts->start.value() : nullptr;
    opts2.end = opts->end ? &opts->end.value() : nullptr;
    opts2.page = opts->page ? &opts->page.value() : nullptr;
    opts2.size = opts->size ? &opts->size.value() : nullptr;
  }

  lb_fund_context_position_dividends(
    ctx_,
    counter_id.c_str(),
    &opts2,
    FUND_OBJ_CALLBACK(CFundDividends, FundDividends),
    new AsyncCallback<FundContext, FundDividends>(callback));
}

void
FundContext::orders(
  const std::optional<GetFundOrdersOptions>& opts,
  AsyncCallback<FundContext, std::vector<FundOrder>> callback) const
{
  CGetFundOrdersOptions opts2 = {
    nullptr, 0, nullptr, nullptr, nullptr, nullptr, nullptr, nullptr, nullptr,
  };
  std::vector<const char*> symbols;

  if (opts) {
    std::transform(opts->symbols.cbegin(),
                   opts->symbols.cend(),
                   std::back_inserter(symbols),
                   [](auto& s) { return s.c_str(); });
    if (!symbols.empty()) {
      opts2.symbols = symbols.data();
      opts2.num_symbols = symbols.size();
    }
    opts2.actions = opts->actions ? opts->actions->c_str() : nullptr;
    opts2.states = opts->states ? opts->states->c_str() : nullptr;
    opts2.currency = opts->currency ? opts->currency->c_str() : nullptr;
    opts2.start = opts->start ? &opts->start.value() : nullptr;
    opts2.end = opts->end ? &opts->end.value() : nullptr;
    opts2.page = opts->page ? &opts->page.value() : nullptr;
    opts2.size = opts->size ? &opts->size.value() : nullptr;
  }

  lb_fund_context_orders(
    ctx_,
    &opts2,
    FUND_VEC_CALLBACK(CFundOrder, FundOrder),
    new AsyncCallback<FundContext, std::vector<FundOrder>>(callback));
}

void
FundContext::order(int64_t order_id,
                   AsyncCallback<FundContext, FundOrderDetail> callback) const
{
  lb_fund_context_order(
    ctx_,
    order_id,
    FUND_OBJ_CALLBACK(CFundOrderDetail, FundOrderDetail),
    new AsyncCallback<FundContext, FundOrderDetail>(callback));
}

void
FundContext::transactions(
  const std::optional<GetFundTransactionsOptions>& opts,
  AsyncCallback<FundContext, std::vector<FundTransaction>> callback) const
{
  CGetFundTransactionsOptions opts2 = {
    nullptr, nullptr, nullptr, nullptr, nullptr, nullptr, nullptr, nullptr,
  };
  if (opts) {
    opts2.account_channel =
      opts->account_channel ? opts->account_channel->c_str() : nullptr;
    opts2.business_type =
      opts->business_type ? opts->business_type->c_str() : nullptr;
    opts2.category = opts->category ? opts->category->c_str() : nullptr;
    opts2.currencies = opts->currencies ? opts->currencies->c_str() : nullptr;
    opts2.start = opts->start ? &opts->start.value() : nullptr;
    opts2.end = opts->end ? &opts->end.value() : nullptr;
    opts2.page = opts->page ? &opts->page.value() : nullptr;
    opts2.size = opts->size ? &opts->size.value() : nullptr;
  }

  lb_fund_context_transactions(
    ctx_,
    &opts2,
    FUND_VEC_CALLBACK(CFundTransaction, FundTransaction),
    new AsyncCallback<FundContext, std::vector<FundTransaction>>(callback));
}

void
FundContext::validate_order(
  const ValidateFundOrderOptions& opts,
  AsyncCallback<FundContext, FundOrderValidation> callback) const
{
  CValidateFundOrderOptions opts2 = {
    opts.symbol.c_str(),
    opts.action.c_str(),
    opts.currency.c_str(),
    opts.amount ? opts.amount->c_str() : nullptr,
    opts.units ? opts.units->c_str() : nullptr,
    opts.dividend_option ? &opts.dividend_option.value() : nullptr,
    opts.fund_source ? &opts.fund_source.value() : nullptr,
    opts.account_channel ? opts.account_channel->c_str() : nullptr,
  };

  lb_fund_context_validate_order(
    ctx_,
    &opts2,
    FUND_OBJ_CALLBACK(CFundOrderValidation, FundOrderValidation),
    new AsyncCallback<FundContext, FundOrderValidation>(callback));
}

void
FundContext::submit_order(
  const SubmitFundOrderOptions& opts,
  AsyncCallback<FundContext, FundOrderSubmitResponse> callback) const
{
  CSubmitFundOrderOptions opts2 = {
    opts.symbol.c_str(),
    opts.action.c_str(),
    opts.currency.c_str(),
    opts.amount ? opts.amount->c_str() : nullptr,
    opts.units ? opts.units->c_str() : nullptr,
    opts.dividend_option ? &opts.dividend_option.value() : nullptr,
    opts.fee ? opts.fee->c_str() : nullptr,
    opts.is_sell_all ? &opts.is_sell_all.value() : nullptr,
    opts.remark ? opts.remark->c_str() : nullptr,
    opts.trade_method ? &opts.trade_method.value() : nullptr,
  };

  lb_fund_context_submit_order(
    ctx_,
    &opts2,
    FUND_OBJ_CALLBACK(CFundOrderSubmitResponse, FundOrderSubmitResponse),
    new AsyncCallback<FundContext, FundOrderSubmitResponse>(callback));
}

void
FundContext::cancel_order(int64_t order_id,
                          AsyncCallback<FundContext, void> callback) const
{
  lb_fund_context_cancel_order(
    ctx_,
    order_id,
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<FundContext, void>(res->userdata);
      (*callback_ptr)(AsyncResult<FundContext, void>(
        FundContext((const CFundContext*)res->ctx),
        Status(res->error),
        nullptr));
    },
    new AsyncCallback<FundContext, void>(callback));
}

} // namespace fund
} // namespace longbridge
