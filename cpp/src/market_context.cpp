#include "market_context.hpp"
#include "longbridge.h"
#include "convert.hpp"

extern "C" {
const lb_market_context_t* lb_market_context_new(const lb_config_t* config);
void lb_market_context_retain(const lb_market_context_t* ctx);
void lb_market_context_release(const lb_market_context_t* ctx);
void lb_market_context_market_status(const lb_market_context_t*, lb_async_callback_t, void*);
void lb_market_context_broker_holding(const lb_market_context_t*, const char*, lb_broker_holding_period_t, lb_async_callback_t, void*);
void lb_market_context_broker_holding_detail(const lb_market_context_t*, const char*, lb_async_callback_t, void*);
void lb_market_context_broker_holding_daily(const lb_market_context_t*, const char*, const char*, lb_async_callback_t, void*);
void lb_market_context_ah_premium(const lb_market_context_t*, const char*, lb_ah_premium_period_t, uint32_t, lb_async_callback_t, void*);
void lb_market_context_ah_premium_intraday(const lb_market_context_t*, const char*, lb_async_callback_t, void*);
void lb_market_context_trade_stats(const lb_market_context_t*, const char*, lb_async_callback_t, void*);
void lb_market_context_anomaly(const lb_market_context_t*, const char*, lb_async_callback_t, void*);
void lb_market_context_constituent(const lb_market_context_t*, const char*, lb_async_callback_t, void*);
}

namespace longbridge {
namespace market {

MarketContext::MarketContext() : ctx_(nullptr) {}
MarketContext::MarketContext(const lb_market_context_t* ctx) { ctx_ = ctx; if (ctx_) lb_market_context_retain(ctx_); }
MarketContext::MarketContext(const MarketContext& ctx) { ctx_ = ctx.ctx_; if (ctx_) lb_market_context_retain(ctx_); }
MarketContext::MarketContext(MarketContext&& ctx) { ctx_ = ctx.ctx_; ctx.ctx_ = nullptr; }
MarketContext::~MarketContext() { if (ctx_) lb_market_context_release(ctx_); }
MarketContext& MarketContext::operator=(const MarketContext& ctx) { ctx_ = ctx.ctx_; if (ctx_) lb_market_context_retain(ctx_); return *this; }

MarketContext MarketContext::create(const Config& config) {
  auto* ptr = lb_market_context_new(config);
  MarketContext ctx(ptr);
  if (ptr) lb_market_context_release(ptr);
  return ctx;
}

// CType is the actual C header type (lb_*_t), RespType is the C++ type.
#define TYPED_CB(CppCtx, RespType, CType, c_fn, ...)                          \
  c_fn(__VA_ARGS__,                                                            \
    [](auto res) {                                                             \
      auto cb = callback::get_async_callback<CppCtx, RespType>(res->userdata);\
      CppCtx fctx((const lb_market_context_t*)res->ctx);                      \
      Status status(res->error);                                               \
      if (status) {                                                            \
        auto r = convert::convert((const CType*)res->data);                   \
        (*cb)(AsyncResult<CppCtx, RespType>(fctx, std::move(status), &r));    \
      } else {                                                                 \
        (*cb)(AsyncResult<CppCtx, RespType>(fctx, std::move(status), nullptr));\
      }                                                                        \
    }, new AsyncCallback<CppCtx, RespType>(callback))

void MarketContext::market_status(AsyncCallback<MarketContext, MarketStatusResponse> callback) const {
  TYPED_CB(MarketContext, MarketStatusResponse, lb_market_status_response_t, lb_market_context_market_status, ctx_);
}
void MarketContext::broker_holding(const std::string& symbol, BrokerHoldingPeriod period, AsyncCallback<MarketContext, BrokerHoldingTop> callback) const {
  TYPED_CB(MarketContext, BrokerHoldingTop, lb_broker_holding_top_t, lb_market_context_broker_holding, ctx_, symbol.c_str(), (lb_broker_holding_period_t)period);
}
void MarketContext::broker_holding_detail(const std::string& symbol, AsyncCallback<MarketContext, BrokerHoldingDetail> callback) const {
  TYPED_CB(MarketContext, BrokerHoldingDetail, lb_broker_holding_detail_t, lb_market_context_broker_holding_detail, ctx_, symbol.c_str());
}
void MarketContext::broker_holding_daily(const std::string& symbol, const std::string& broker_id, AsyncCallback<MarketContext, BrokerHoldingDailyHistory> callback) const {
  TYPED_CB(MarketContext, BrokerHoldingDailyHistory, lb_broker_holding_daily_history_t, lb_market_context_broker_holding_daily, ctx_, symbol.c_str(), broker_id.c_str());
}
void MarketContext::ah_premium(const std::string& symbol, AhPremiumPeriod period, uint32_t count, AsyncCallback<MarketContext, AhPremiumKlines> callback) const {
  TYPED_CB(MarketContext, AhPremiumKlines, lb_ah_premium_klines_t, lb_market_context_ah_premium, ctx_, symbol.c_str(), (lb_ah_premium_period_t)period, count);
}
void MarketContext::ah_premium_intraday(const std::string& symbol, AsyncCallback<MarketContext, AhPremiumIntraday> callback) const {
  TYPED_CB(MarketContext, AhPremiumIntraday, lb_ah_premium_intraday_t, lb_market_context_ah_premium_intraday, ctx_, symbol.c_str());
}
void MarketContext::trade_stats(const std::string& symbol, AsyncCallback<MarketContext, TradeStatsResponse> callback) const {
  TYPED_CB(MarketContext, TradeStatsResponse, lb_trade_stats_response_t, lb_market_context_trade_stats, ctx_, symbol.c_str());
}
void MarketContext::anomaly(const std::string& market, AsyncCallback<MarketContext, AnomalyResponse> callback) const {
  TYPED_CB(MarketContext, AnomalyResponse, lb_anomaly_response_t, lb_market_context_anomaly, ctx_, market.c_str());
}
void MarketContext::constituent(const std::string& symbol, AsyncCallback<MarketContext, IndexConstituents> callback) const {
  TYPED_CB(MarketContext, IndexConstituents, lb_index_constituents_t, lb_market_context_constituent, ctx_, symbol.c_str());
}

#undef TYPED_CB

// ── New JSON-string API (rank_categories) ────────────────────────
#define M_JSON(cfn, CType, ...) cfn(__VA_ARGS__, [](auto res) { \
  auto cb = callback::get_async_callback<MarketContext,std::string>(res->userdata); \
  MarketContext mctx((const lb_market_context_t*)res->ctx); Status status(res->error); \
  if(status){const CType* d=(const CType*)res->data; std::string j(d->data ? d->data : ""); (*cb)(AsyncResult<MarketContext,std::string>(mctx,std::move(status),&j));} \
  else{(*cb)(AsyncResult<MarketContext,std::string>(mctx,std::move(status),nullptr));} \
}, new AsyncCallback<MarketContext,std::string>(callback))

void MarketContext::rank_categories(AsyncCallback<MarketContext, std::string> callback) const {
  M_JSON(lb_market_context_rank_categories, lb_rank_categories_response_t, ctx_);
}

#undef M_JSON

void MarketContext::top_movers(const std::vector<std::string>& markets, uint32_t sort, const std::string* date, uint32_t limit, AsyncCallback<MarketContext, TopMoversResponse> callback) const {
  std::vector<const char*> mptrs;
  for (const auto& m : markets) mptrs.push_back(m.c_str());
  const char* date_str = date ? date->c_str() : nullptr;
  lb_market_context_top_movers(ctx_, mptrs.data(), mptrs.size(), sort, date_str, limit,
    [](auto res) {
      auto cb = callback::get_async_callback<MarketContext, TopMoversResponse>(res->userdata);
      MarketContext mctx((const lb_market_context_t*)res->ctx); Status status(res->error);
      if (status) {
        auto r = convert::convert((const lb_top_movers_response_t*)res->data);
        (*cb)(AsyncResult<MarketContext, TopMoversResponse>(mctx, std::move(status), &r));
      } else {
        (*cb)(AsyncResult<MarketContext, TopMoversResponse>(mctx, std::move(status), nullptr));
      }
    }, new AsyncCallback<MarketContext, TopMoversResponse>(callback));
}

void MarketContext::rank_list(const std::string& key, bool need_article, AsyncCallback<MarketContext, RankListResponse> callback) const {
  lb_market_context_rank_list(ctx_, key.c_str(), need_article,
    [](auto res) {
      auto cb = callback::get_async_callback<MarketContext, RankListResponse>(res->userdata);
      MarketContext mctx((const lb_market_context_t*)res->ctx); Status status(res->error);
      if (status) {
        auto r = convert::convert((const lb_rank_list_response_t*)res->data);
        (*cb)(AsyncResult<MarketContext, RankListResponse>(mctx, std::move(status), &r));
      } else {
        (*cb)(AsyncResult<MarketContext, RankListResponse>(mctx, std::move(status), nullptr));
      }
    }, new AsyncCallback<MarketContext, RankListResponse>(callback));
}

} // namespace market
} // namespace longbridge
