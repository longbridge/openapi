#include "fundamental_context.hpp"
#include "longbridge.h"
#include "convert.hpp"
#include <optional>


namespace longbridge {
namespace fundamental {

FundamentalContext::FundamentalContext() : ctx_(nullptr) {}
FundamentalContext::FundamentalContext(const lb_fundamental_context_t* ctx) { ctx_ = ctx; if (ctx_) lb_fundamental_context_retain(ctx_); }
FundamentalContext::FundamentalContext(const FundamentalContext& ctx) { ctx_ = ctx.ctx_; if (ctx_) lb_fundamental_context_retain(ctx_); }
FundamentalContext::FundamentalContext(FundamentalContext&& ctx) { ctx_ = ctx.ctx_; ctx.ctx_ = nullptr; }
FundamentalContext::~FundamentalContext() { if (ctx_) lb_fundamental_context_release(ctx_); }
FundamentalContext& FundamentalContext::operator=(const FundamentalContext& ctx) { ctx_ = ctx.ctx_; if (ctx_) lb_fundamental_context_retain(ctx_); return *this; }
FundamentalContext FundamentalContext::create(const Config& config) { auto* ptr = lb_fundamental_context_new(config); FundamentalContext ctx(ptr); if (ptr) lb_fundamental_context_release(ptr); return ctx; }

void FundamentalContext::financial_report(const std::string& symbol, FinancialReportKind kind, std::optional<FinancialReportPeriod> period, AsyncCallback<FundamentalContext, FinancialReports> callback) const {
  int32_t period_val = period.has_value() ? (int32_t)period.value() : -1;
  lb_fundamental_context_financial_report(ctx_, symbol.c_str(), (lb_financial_report_kind_t)kind, period_val,
    [](auto res) {
      auto cb = callback::get_async_callback<FundamentalContext, FinancialReports>(res->userdata);
      FundamentalContext fctx((const lb_fundamental_context_t*)res->ctx); Status status(res->error);
      if (status) { const lb_financial_reports_t* d = (const lb_financial_reports_t*)res->data; FinancialReports r{ d->list_json ? d->list_json : "" }; (*cb)(AsyncResult<FundamentalContext, FinancialReports>(fctx, std::move(status), &r)); }
      else { (*cb)(AsyncResult<FundamentalContext, FinancialReports>(fctx, std::move(status), nullptr)); }
    }, new AsyncCallback<FundamentalContext, FinancialReports>(callback));
}

// CType = actual C header type (lb_*_t), Resp = C++ return type
#define F_TYPED(Resp, CType, cfn, ...) cfn(__VA_ARGS__, [](auto res) { \
  auto cb = callback::get_async_callback<FundamentalContext,Resp>(res->userdata); \
  FundamentalContext fctx((const lb_fundamental_context_t*)res->ctx); Status status(res->error); \
  if(status){auto r=convert::convert((const CType*)res->data);(*cb)(AsyncResult<FundamentalContext,Resp>(fctx,std::move(status),&r));} \
  else{(*cb)(AsyncResult<FundamentalContext,Resp>(fctx,std::move(status),nullptr));} \
}, new AsyncCallback<FundamentalContext,Resp>(callback))

#define F_JSON(cfn, ...) cfn(__VA_ARGS__, [](auto res) { \
  auto cb = callback::get_async_callback<FundamentalContext,std::string>(res->userdata); \
  FundamentalContext fctx((const lb_fundamental_context_t*)res->ctx); Status status(res->error); \
  if(status){std::string j((const char*)res->data);(*cb)(AsyncResult<FundamentalContext,std::string>(fctx,std::move(status),&j));} \
  else{(*cb)(AsyncResult<FundamentalContext,std::string>(fctx,std::move(status),nullptr));} \
}, new AsyncCallback<FundamentalContext,std::string>(callback))

void FundamentalContext::institution_rating(const std::string& s, AsyncCallback<FundamentalContext, InstitutionRating> callback) const {
  F_TYPED(InstitutionRating, lb_institution_rating_t, lb_fundamental_context_institution_rating, ctx_, s.c_str());
}
void FundamentalContext::institution_rating_detail(const std::string& s, AsyncCallback<FundamentalContext, InstitutionRatingDetail> callback) const {
  F_TYPED(InstitutionRatingDetail, lb_institution_rating_detail_t, lb_fundamental_context_institution_rating_detail, ctx_, s.c_str());
}
void FundamentalContext::dividend(const std::string& s, AsyncCallback<FundamentalContext, DividendList> callback) const {
  F_TYPED(DividendList, lb_dividend_list_t, lb_fundamental_context_dividend, ctx_, s.c_str());
}
void FundamentalContext::dividend_detail(const std::string& s, AsyncCallback<FundamentalContext, DividendList> callback) const {
  F_TYPED(DividendList, lb_dividend_list_t, lb_fundamental_context_dividend_detail, ctx_, s.c_str());
}
void FundamentalContext::forecast_eps(const std::string& s, AsyncCallback<FundamentalContext, ForecastEps> callback) const {
  F_TYPED(ForecastEps, lb_forecast_eps_t, lb_fundamental_context_forecast_eps, ctx_, s.c_str());
}
void FundamentalContext::valuation(const std::string& s, AsyncCallback<FundamentalContext, ValuationData> callback) const {
  F_TYPED(ValuationData, lb_valuation_data_t, lb_fundamental_context_valuation, ctx_, s.c_str());
}
void FundamentalContext::valuation_history(const std::string& s, AsyncCallback<FundamentalContext, ValuationHistoryResponse> callback) const {
  F_TYPED(ValuationHistoryResponse, lb_valuation_history_response_t, lb_fundamental_context_valuation_history, ctx_, s.c_str());
}
void FundamentalContext::company(const std::string& s, AsyncCallback<FundamentalContext, CompanyOverview> callback) const {
  F_TYPED(CompanyOverview, lb_company_overview_t, lb_fundamental_context_company, ctx_, s.c_str());
}
void FundamentalContext::shareholder(const std::string& s, AsyncCallback<FundamentalContext, ShareholderList> callback) const {
  F_TYPED(ShareholderList, lb_shareholder_list_t, lb_fundamental_context_shareholder, ctx_, s.c_str());
}
void FundamentalContext::fund_holder(const std::string& s, AsyncCallback<FundamentalContext, FundHolders> callback) const {
  F_TYPED(FundHolders, lb_fund_holders_t, lb_fundamental_context_fund_holder, ctx_, s.c_str());
}
void FundamentalContext::corp_action(const std::string& s, AsyncCallback<FundamentalContext, CorpActions> callback) const {
  F_TYPED(CorpActions, lb_corp_actions_t, lb_fundamental_context_corp_action, ctx_, s.c_str());
}
void FundamentalContext::invest_relation(const std::string& s, AsyncCallback<FundamentalContext, InvestRelations> callback) const {
  F_TYPED(InvestRelations, lb_invest_relations_t, lb_fundamental_context_invest_relation, ctx_, s.c_str());
}
void FundamentalContext::operating(const std::string& s, AsyncCallback<FundamentalContext, OperatingList> callback) const {
  F_TYPED(OperatingList, lb_operating_list_t, lb_fundamental_context_operating, ctx_, s.c_str());
}
void FundamentalContext::consensus(const std::string& s, AsyncCallback<FundamentalContext, FinancialConsensus> callback) const {
  F_TYPED(FinancialConsensus, lb_financial_consensus_t, lb_fundamental_context_consensus, ctx_, s.c_str());
}
void FundamentalContext::industry_valuation(const std::string& s, AsyncCallback<FundamentalContext, IndustryValuationList> callback) const {
  F_TYPED(IndustryValuationList, lb_industry_valuation_list_t, lb_fundamental_context_industry_valuation, ctx_, s.c_str());
}
void FundamentalContext::industry_valuation_dist(const std::string& s, AsyncCallback<FundamentalContext, IndustryValuationDist> callback) const {
  F_TYPED(IndustryValuationDist, lb_industry_valuation_dist_t, lb_fundamental_context_industry_valuation_dist, ctx_, s.c_str());
}
void FundamentalContext::executive(const std::string& s, AsyncCallback<FundamentalContext, ExecutiveList> callback) const {
  F_TYPED(ExecutiveList, lb_executive_list_t, lb_fundamental_context_executive, ctx_, s.c_str());
}
void FundamentalContext::buyback(const std::string& s, AsyncCallback<FundamentalContext, BuybackData> callback) const {
  F_TYPED(BuybackData, lb_buyback_data_t, lb_fundamental_context_buyback, ctx_, s.c_str());
}
void FundamentalContext::ratings(const std::string& s, AsyncCallback<FundamentalContext, StockRatings> callback) const {
  F_TYPED(StockRatings, lb_stock_ratings_t, lb_fundamental_context_ratings, ctx_, s.c_str());
}

#undef F_TYPED
#undef F_JSON

} // namespace fundamental
} // namespace longbridge
