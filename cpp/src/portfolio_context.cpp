#include "portfolio_context.hpp"
#include "longbridge.h"
#include "convert.hpp"

extern "C" {
const lb_portfolio_context_t* lb_portfolio_context_new(const lb_config_t* config);
void lb_portfolio_context_retain(const lb_portfolio_context_t* ctx);
void lb_portfolio_context_release(const lb_portfolio_context_t* ctx);
void lb_portfolio_context_exchange_rate(const lb_portfolio_context_t*, lb_async_callback_t, void*);
void lb_portfolio_context_profit_analysis(const lb_portfolio_context_t*, const char*, const char*, lb_async_callback_t, void*);
void lb_portfolio_context_profit_analysis_detail(const lb_portfolio_context_t*, const char*, const char*, const char*, lb_async_callback_t, void*);
void lb_portfolio_context_profit_analysis_by_market(const lb_portfolio_context_t*, const char*, const char*, const char*, const char*, int32_t, int32_t, lb_async_callback_t, void*);
void lb_portfolio_context_profit_analysis_flows(const lb_portfolio_context_t*, const char*, int32_t, int32_t, bool, const char*, const char*, lb_async_callback_t, void*);
}

namespace longbridge {
namespace portfolio {

PortfolioContext::PortfolioContext() : ctx_(nullptr) {}
PortfolioContext::PortfolioContext(const lb_portfolio_context_t* ctx) { ctx_ = ctx; if (ctx_) lb_portfolio_context_retain(ctx_); }
PortfolioContext::PortfolioContext(const PortfolioContext& ctx) { ctx_ = ctx.ctx_; if (ctx_) lb_portfolio_context_retain(ctx_); }
PortfolioContext::PortfolioContext(PortfolioContext&& ctx) { ctx_ = ctx.ctx_; ctx.ctx_ = nullptr; }
PortfolioContext::~PortfolioContext() { if (ctx_) lb_portfolio_context_release(ctx_); }
PortfolioContext& PortfolioContext::operator=(const PortfolioContext& ctx) { ctx_ = ctx.ctx_; if (ctx_) lb_portfolio_context_retain(ctx_); return *this; }
PortfolioContext PortfolioContext::create(const Config& config) { auto* ptr = lb_portfolio_context_new(config); PortfolioContext ctx(ptr); if (ptr) lb_portfolio_context_release(ptr); return ctx; }

void PortfolioContext::exchange_rate(AsyncCallback<PortfolioContext, ExchangeRates> callback) const {
  lb_portfolio_context_exchange_rate(ctx_,
    [](auto res) {
      auto cb = callback::get_async_callback<PortfolioContext, ExchangeRates>(res->userdata);
      PortfolioContext fctx((const lb_portfolio_context_t*)res->ctx);
      Status status(res->error);
      if (status) {
        auto* r = (const lb_exchange_rates_t*)res->data;
        ExchangeRates resp;
        for (size_t i = 0; i < r->num_exchanges; ++i) {
          const auto& e = r->exchanges[i];
          resp.exchanges.push_back({ e.average_rate, e.base_currency, e.bid_rate, e.offer_rate, e.other_currency });
        }
        (*cb)(AsyncResult<PortfolioContext, ExchangeRates>(fctx, std::move(status), &resp));
      } else { (*cb)(AsyncResult<PortfolioContext, ExchangeRates>(fctx, std::move(status), nullptr)); }
    }, new AsyncCallback<PortfolioContext, ExchangeRates>(callback));
}

#define PORTFOLIO_TYPED_CB(Resp, CType, c_fn, ...) c_fn(__VA_ARGS__, [](auto res) { \
  auto cb = callback::get_async_callback<PortfolioContext, Resp>(res->userdata); \
  PortfolioContext fctx((const lb_portfolio_context_t*)res->ctx); Status status(res->error); \
  if(status){auto r=convert::convert((const CType*)res->data);(*cb)(AsyncResult<PortfolioContext,Resp>(fctx,std::move(status),&r));} \
  else{(*cb)(AsyncResult<PortfolioContext,Resp>(fctx,std::move(status),nullptr));} \
}, new AsyncCallback<PortfolioContext,Resp>(callback))

void PortfolioContext::profit_analysis(const std::string& start, const std::string& end, AsyncCallback<PortfolioContext, ProfitAnalysis> callback) const {
  PORTFOLIO_TYPED_CB(ProfitAnalysis, lb_profit_analysis_t, lb_portfolio_context_profit_analysis, ctx_, start.empty()?nullptr:start.c_str(), end.empty()?nullptr:end.c_str());
}
void PortfolioContext::profit_analysis_detail(const std::string& symbol, const std::string& start, const std::string& end, AsyncCallback<PortfolioContext, ProfitAnalysisDetail> callback) const {
  PORTFOLIO_TYPED_CB(ProfitAnalysisDetail, lb_profit_analysis_detail_t, lb_portfolio_context_profit_analysis_detail, ctx_, symbol.c_str(), start.empty()?nullptr:start.c_str(), end.empty()?nullptr:end.c_str());
}
void PortfolioContext::profit_analysis_by_market(const std::string& market, const std::string& start,
                                                  const std::string& end, const std::string& currency,
                                                  int32_t page, int32_t size,
                                                  AsyncCallback<PortfolioContext, ProfitAnalysisByMarket> callback) const {
  PORTFOLIO_TYPED_CB(ProfitAnalysisByMarket, lb_profit_analysis_by_market_t, lb_portfolio_context_profit_analysis_by_market, ctx_,
    market.empty()?nullptr:market.c_str(),
    start.empty()?nullptr:start.c_str(),
    end.empty()?nullptr:end.c_str(),
    currency.empty()?nullptr:currency.c_str(),
    page, size);
}
void PortfolioContext::profit_analysis_flows(const std::string& symbol, int32_t page, int32_t size,
                                              bool derivative, const std::string& start,
                                              const std::string& end,
                                              AsyncCallback<PortfolioContext, ProfitAnalysisFlows> callback) const {
  PORTFOLIO_TYPED_CB(ProfitAnalysisFlows, lb_profit_analysis_flows_t, lb_portfolio_context_profit_analysis_flows, ctx_,
    symbol.c_str(), page, size, derivative,
    start.empty()?nullptr:start.c_str(),
    end.empty()?nullptr:end.c_str());
}
#undef PORTFOLIO_TYPED_CB

} // namespace portfolio
} // namespace longbridge
