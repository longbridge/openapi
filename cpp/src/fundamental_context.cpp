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
void FundamentalContext::business_segments(const std::string& s, AsyncCallback<FundamentalContext, BusinessSegments> callback) const {
  F_TYPED(BusinessSegments, lb_business_segments_t, lb_fundamental_context_business_segments, ctx_, s.c_str());
}
void FundamentalContext::business_segments_history(const std::string& s, const char* report, const char* cate, AsyncCallback<FundamentalContext, BusinessSegmentsHistory> callback) const {
  F_TYPED(BusinessSegmentsHistory, lb_business_segments_history_t, lb_fundamental_context_business_segments_history, ctx_, s.c_str(), report, cate);
}
void FundamentalContext::institution_rating_views(const std::string& s, AsyncCallback<FundamentalContext, InstitutionRatingViews> callback) const {
  F_TYPED(InstitutionRatingViews, lb_institution_rating_views_t, lb_fundamental_context_institution_rating_views, ctx_, s.c_str());
}
void FundamentalContext::industry_rank(const std::string& market, const std::string& indicator, const std::string& sort_type, uint32_t limit, AsyncCallback<FundamentalContext, IndustryRankResponse> callback) const {
  F_TYPED(IndustryRankResponse, lb_industry_rank_response_t, lb_fundamental_context_industry_rank, ctx_, market.c_str(), indicator.c_str(), sort_type.c_str(), limit);
}
void FundamentalContext::industry_peers(const std::string& counter_id, const std::string& market, const char* industry_id, AsyncCallback<FundamentalContext, IndustryPeersResponse> callback) const {
  F_TYPED(IndustryPeersResponse, lb_industry_peers_response_t, lb_fundamental_context_industry_peers, ctx_, counter_id.c_str(), market.c_str(), industry_id);
}
void FundamentalContext::financial_report_snapshot(const std::string& s, const char* report, int32_t fiscal_year, const char* fiscal_period, AsyncCallback<FundamentalContext, FinancialReportSnapshot> callback) const {
  F_TYPED(FinancialReportSnapshot, lb_financial_report_snapshot_t, lb_fundamental_context_financial_report_snapshot, ctx_, s.c_str(), report, fiscal_year, fiscal_period);
}

#undef F_TYPED
#undef F_JSON

// ── New JSON-string APIs ──────────────────────────────────────────
// These return a struct with a single `data` field (JSON string).
// We extract the string and return it as std::string.
#define F_JSON_STRUCT(cfn, CType, ...) cfn(__VA_ARGS__, [](auto res) { \
  auto cb = callback::get_async_callback<FundamentalContext,std::string>(res->userdata); \
  FundamentalContext fctx((const lb_fundamental_context_t*)res->ctx); Status status(res->error); \
  if(status){const CType* d=(const CType*)res->data; std::string j(d->data ? d->data : ""); (*cb)(AsyncResult<FundamentalContext,std::string>(fctx,std::move(status),&j));} \
  else{(*cb)(AsyncResult<FundamentalContext,std::string>(fctx,std::move(status),nullptr));} \
}, new AsyncCallback<FundamentalContext,std::string>(callback))

void FundamentalContext::shareholder_top(const std::string& s, AsyncCallback<FundamentalContext, std::string> callback) const {
  F_JSON_STRUCT(lb_fundamental_context_shareholder_top, lb_shareholder_top_response_t, ctx_, s.c_str());
}

void FundamentalContext::shareholder_detail(const std::string& s, int64_t object_id, AsyncCallback<FundamentalContext, std::string> callback) const {
  F_JSON_STRUCT(lb_fundamental_context_shareholder_detail, lb_shareholder_detail_response_t, ctx_, s.c_str(), object_id);
}

void FundamentalContext::valuation_comparison(const std::string& s, const std::string& currency, const std::vector<std::string>* comparison_symbols, AsyncCallback<FundamentalContext, std::string> callback) const {
  std::vector<const char*> syms_ptrs;
  size_t num_syms = 0;
  const char** syms_data = nullptr;
  if (comparison_symbols) {
    for (const auto& sym : *comparison_symbols) syms_ptrs.push_back(sym.c_str());
    syms_data = syms_ptrs.empty() ? nullptr : syms_ptrs.data();
    num_syms = syms_ptrs.size();
  }
  F_JSON_STRUCT(lb_fundamental_context_valuation_comparison, lb_valuation_comparison_response_t, ctx_, s.c_str(), currency.c_str(), syms_data, num_syms);
}

#undef F_JSON_STRUCT

} // namespace fundamental
} // namespace longbridge
