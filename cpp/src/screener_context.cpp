#include "screener_context.hpp"
#include "longbridge.h"
#include "callback.hpp"
#include "status.hpp"
#include <optional>

extern "C" {
const lb_screener_context_t* lb_screener_context_new(const lb_config_t* config);
void lb_screener_context_retain(const lb_screener_context_t* ctx);
void lb_screener_context_release(const lb_screener_context_t* ctx);
void lb_screener_context_recommend_strategies(const lb_screener_context_t*, lb_async_callback_t, void*);
void lb_screener_context_user_strategies(const lb_screener_context_t*, lb_async_callback_t, void*);
void lb_screener_context_strategy(const lb_screener_context_t*, int64_t, lb_async_callback_t, void*);
void lb_screener_context_search(const lb_screener_context_t*, const char*, int64_t, bool, uint32_t, uint32_t, lb_async_callback_t, void*);
void lb_screener_context_indicators(const lb_screener_context_t*, lb_async_callback_t, void*);
}

namespace longbridge {
namespace screener {

ScreenerContext::ScreenerContext() : ctx_(nullptr) {}
ScreenerContext::ScreenerContext(const lb_screener_context_t* ctx) { ctx_ = ctx; if (ctx_) lb_screener_context_retain(ctx_); }
ScreenerContext::ScreenerContext(const ScreenerContext& ctx) { ctx_ = ctx.ctx_; if (ctx_) lb_screener_context_retain(ctx_); }
ScreenerContext::ScreenerContext(ScreenerContext&& ctx) { ctx_ = ctx.ctx_; ctx.ctx_ = nullptr; }
ScreenerContext::~ScreenerContext() { if (ctx_) lb_screener_context_release(ctx_); }
ScreenerContext& ScreenerContext::operator=(const ScreenerContext& ctx) { ctx_ = ctx.ctx_; if (ctx_) lb_screener_context_retain(ctx_); return *this; }
ScreenerContext ScreenerContext::create(const Config& config) {
  auto* ptr = lb_screener_context_new(config);
  ScreenerContext sctx(ptr);
  if (ptr) lb_screener_context_release(ptr);
  return sctx;
}

// Helper macro: reads .data field of the C response struct as JSON string
#define S_JSON(cfn, CType, ...) cfn(__VA_ARGS__, [](auto res) { \
  auto cb = callback::get_async_callback<ScreenerContext,std::string>(res->userdata); \
  ScreenerContext sctx((const lb_screener_context_t*)res->ctx); Status status(res->error); \
  if(status){const CType* d=(const CType*)res->data; std::string j(d->data ? d->data : ""); (*cb)(AsyncResult<ScreenerContext,std::string>(sctx,std::move(status),&j));} \
  else{(*cb)(AsyncResult<ScreenerContext,std::string>(sctx,std::move(status),nullptr));} \
}, new AsyncCallback<ScreenerContext,std::string>(callback))

void ScreenerContext::screener_recommend_strategies(AsyncCallback<ScreenerContext, std::string> callback) const {
  S_JSON(lb_screener_context_recommend_strategies, lb_screener_recommend_strategies_response_t, ctx_);
}

void ScreenerContext::screener_user_strategies(AsyncCallback<ScreenerContext, std::string> callback) const {
  S_JSON(lb_screener_context_user_strategies, lb_screener_user_strategies_response_t, ctx_);
}

void ScreenerContext::screener_strategy(int64_t id, AsyncCallback<ScreenerContext, std::string> callback) const {
  S_JSON(lb_screener_context_strategy, lb_screener_strategy_response_t, ctx_, id);
}

void ScreenerContext::screener_search(const std::string& market, std::optional<int64_t> strategy_id, uint32_t page, uint32_t size, AsyncCallback<ScreenerContext, std::string> callback) const {
  int64_t sid = strategy_id.value_or(0);
  bool has_sid = strategy_id.has_value();
  S_JSON(lb_screener_context_search, lb_screener_search_response_t, ctx_, market.c_str(), sid, has_sid, page, size);
}

void ScreenerContext::screener_indicators(AsyncCallback<ScreenerContext, std::string> callback) const {
  S_JSON(lb_screener_context_indicators, lb_screener_indicators_response_t, ctx_);
}

#undef S_JSON

} // namespace screener
} // namespace longbridge
