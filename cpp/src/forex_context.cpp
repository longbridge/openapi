#include "forex_context.hpp"
#include "longbridge.h"
#include "convert.hpp"
#include "utils.hpp"
#include <string>

namespace longbridge {
namespace forex {

using longbridge::convert::convert;

ForexContext::ForexContext()
  : ctx_(nullptr)
{
}

ForexContext::ForexContext(const lb_forex_context_t* ctx)
{
  ctx_ = ctx;
  if (ctx_)
    lb_forex_context_retain(ctx_);
}

ForexContext::ForexContext(const ForexContext& ctx)
{
  ctx_ = ctx.ctx_;
  if (ctx_)
    lb_forex_context_retain(ctx_);
}

ForexContext::ForexContext(ForexContext&& ctx)
{
  ctx_ = ctx.ctx_;
  ctx.ctx_ = nullptr;
}

ForexContext::~ForexContext()
{
  if (ctx_)
    lb_forex_context_release(ctx_);
}

ForexContext&
ForexContext::operator=(const ForexContext& ctx)
{
  ctx_ = ctx.ctx_;
  if (ctx_)
    lb_forex_context_retain(ctx_);
  return *this;
}

ForexContext
ForexContext::create(const Config& config)
{
  auto* ptr = lb_forex_context_new(config);
  ForexContext ctx(ptr);
  if (ptr)
    lb_forex_context_release(ptr);
  return ctx;
}

// A single-object-returning callback: `res->data` is a `const CElem*`.
#define FOREX_OBJ_CALLBACK(Elem, Value)                                        \
  [](auto res) {                                                               \
    auto callback_ptr =                                                        \
      callback::get_async_callback<ForexContext, Value>(res->userdata);        \
    ForexContext ctx((const lb_forex_context_t*)res->ctx);                     \
    Status status(res->error);                                                 \
    if (status) {                                                              \
      Value resp = convert((const Elem*)res->data);                           \
      (*callback_ptr)(                                                         \
        AsyncResult<ForexContext, Value>(ctx, std::move(status), &resp));      \
    } else {                                                                   \
      (*callback_ptr)(                                                         \
        AsyncResult<ForexContext, Value>(ctx, std::move(status), nullptr));    \
    }                                                                          \
  }

void
ForexContext::quote(const std::string& from,
                    const std::string& to,
                    const std::optional<Decimal>& amount,
                    const std::optional<Decimal>& target_amount,
                    AsyncCallback<ForexContext, ForexQuote> callback) const
{
  lb_forex_context_quote(
    ctx_,
    from.c_str(),
    to.c_str(),
    amount ? (const lb_decimal_t*)amount.value() : nullptr,
    target_amount ? (const lb_decimal_t*)target_amount.value() : nullptr,
    FOREX_OBJ_CALLBACK(lb_forex_quote_t, ForexQuote),
    new AsyncCallback<ForexContext, ForexQuote>(callback));
}

void
ForexContext::submit_order(const std::string& quote_id,
                           const std::string& client_order_id,
                           AsyncCallback<ForexContext, void> callback) const
{
  lb_forex_context_submit_order(
    ctx_,
    quote_id.c_str(),
    client_order_id.c_str(),
    [](auto res) {
      auto callback_ptr =
        callback::get_async_callback<ForexContext, void>(res->userdata);
      (*callback_ptr)(AsyncResult<ForexContext, void>(
        ForexContext((const lb_forex_context_t*)res->ctx),
        Status(res->error),
        nullptr));
    },
    new AsyncCallback<ForexContext, void>(callback));
}

void
ForexContext::order(const std::string& client_order_id,
                    AsyncCallback<ForexContext, ForexOrderDetail> callback) const
{
  lb_forex_context_order(
    ctx_,
    client_order_id.c_str(),
    FOREX_OBJ_CALLBACK(lb_forex_order_detail_t, ForexOrderDetail),
    new AsyncCallback<ForexContext, ForexOrderDetail>(callback));
}

} // namespace forex
} // namespace longbridge
