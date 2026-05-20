#pragma once

#include "async_result.hpp"
#include "callback.hpp"
#include "config.hpp"
#include "types.hpp"
#include <optional>

typedef struct lb_fundamental_context_t lb_fundamental_context_t;

namespace longbridge {
namespace fundamental {

enum class FinancialReportKind
{
  IncomeStatement = 0,
  BalanceSheet    = 1,
  CashFlow        = 2,
  All             = 3,
};
enum class FinancialReportPeriod
{
  Annual        = 0,
  SemiAnnual    = 1,
  Q1            = 2,
  Q2            = 3,
  Q3            = 4,
  QuarterlyFull = 5,
};

/// Fundamental data context.
class FundamentalContext
{
private:
  const lb_fundamental_context_t* ctx_;

public:
  FundamentalContext();
  FundamentalContext(const lb_fundamental_context_t* ctx);
  FundamentalContext(const FundamentalContext& ctx);
  FundamentalContext(FundamentalContext&& ctx);
  ~FundamentalContext();
  FundamentalContext& operator=(const FundamentalContext& ctx);

  static FundamentalContext create(const Config& config);

  /// Get financial reports — list_json is a JSON string
  void financial_report(const std::string& symbol, FinancialReportKind kind, std::optional<FinancialReportPeriod> period,
                        AsyncCallback<FundamentalContext, FinancialReports> callback) const;

  /// Get analyst ratings
  void institution_rating(const std::string& symbol,
                          AsyncCallback<FundamentalContext, InstitutionRating> callback) const;

  /// Get historical analyst rating details
  void institution_rating_detail(const std::string& symbol,
                                 AsyncCallback<FundamentalContext, InstitutionRatingDetail> callback) const;

  /// Get dividend history
  void dividend(const std::string& symbol,
                AsyncCallback<FundamentalContext, DividendList> callback) const;

  /// Get detailed dividend information
  void dividend_detail(const std::string& symbol,
                       AsyncCallback<FundamentalContext, DividendList> callback) const;

  /// Get EPS forecasts
  void forecast_eps(const std::string& symbol,
                    AsyncCallback<FundamentalContext, ForecastEps> callback) const;

  /// Get valuation metrics
  void valuation(const std::string& symbol,
                 AsyncCallback<FundamentalContext, ValuationData> callback) const;

  /// Get historical valuation
  void valuation_history(const std::string& symbol,
                         AsyncCallback<FundamentalContext, ValuationHistoryResponse> callback) const;

  /// Get company overview
  void company(const std::string& symbol,
               AsyncCallback<FundamentalContext, CompanyOverview> callback) const;

  /// Get major shareholders
  void shareholder(const std::string& symbol,
                   AsyncCallback<FundamentalContext, ShareholderList> callback) const;

  /// Get fund and ETF holders
  void fund_holder(const std::string& symbol,
                   AsyncCallback<FundamentalContext, FundHolders> callback) const;

  /// Get corporate actions
  void corp_action(const std::string& symbol,
                   AsyncCallback<FundamentalContext, CorpActions> callback) const;

  /// Get investor relations data
  void invest_relation(const std::string& symbol,
                       AsyncCallback<FundamentalContext, InvestRelations> callback) const;

  /// Get operating metrics
  void operating(const std::string& symbol,
                 AsyncCallback<FundamentalContext, OperatingList> callback) const;

  /// Get consensus estimates
  void consensus(const std::string& symbol,
                 AsyncCallback<FundamentalContext, FinancialConsensus> callback) const;

  /// Get industry valuation
  void industry_valuation(const std::string& symbol,
                          AsyncCallback<FundamentalContext, IndustryValuationList> callback) const;

  /// Get industry valuation distribution
  void industry_valuation_dist(const std::string& symbol,
                               AsyncCallback<FundamentalContext, IndustryValuationDist> callback) const;

  /// Get executive info
  void executive(const std::string& symbol,
                 AsyncCallback<FundamentalContext, ExecutiveList> callback) const;

  /// Get buyback data
  void buyback(const std::string& symbol,
               AsyncCallback<FundamentalContext, BuybackData> callback) const;

  /// Get stock ratings
  void ratings(const std::string& symbol,
               AsyncCallback<FundamentalContext, StockRatings> callback) const;

  /// Get business segment breakdowns (latest snapshot)
  void business_segments(const std::string& symbol,
                         AsyncCallback<FundamentalContext, BusinessSegments> callback) const;

  /// Get historical business segment breakdowns.
  /// Pass nullptr for report/cate to omit them.
  void business_segments_history(const std::string& symbol,
                                 const char* report,
                                 const char* cate,
                                 AsyncCallback<FundamentalContext, BusinessSegmentsHistory> callback) const;

  /// Get historical institutional rating view time-series
  void institution_rating_views(const std::string& symbol,
                                AsyncCallback<FundamentalContext, InstitutionRatingViews> callback) const;

  /// Get industry rank for a market.
  /// indicator: "0"–"7"; sort_type: "0"=asc, "1"=desc
  void industry_rank(const std::string& market,
                     const std::string& indicator,
                     const std::string& sort_type,
                     uint32_t limit,
                     AsyncCallback<FundamentalContext, IndustryRankResponse> callback) const;

  /// Get the industry peer chain.
  /// Pass nullptr for industry_id to omit it.
  void industry_peers(const std::string& counter_id,
                      const std::string& market,
                      const char* industry_id,
                      AsyncCallback<FundamentalContext, IndustryPeersResponse> callback) const;

  /// Get a financial report snapshot.
  /// Pass nullptr for report/fiscal_period; pass 0 for fiscal_year to omit it.
  void financial_report_snapshot(const std::string& symbol,
                                 const char* report,
                                 int32_t fiscal_year,
                                 const char* fiscal_period,
                                 AsyncCallback<FundamentalContext, FinancialReportSnapshot> callback) const;
};

} // namespace fundamental
} // namespace longbridge
