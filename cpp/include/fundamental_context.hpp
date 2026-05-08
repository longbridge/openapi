#pragma once

#include "async_result.hpp"
#include "callback.hpp"
#include "config.hpp"
#include "types.hpp"

typedef struct lb_fundamental_context_t lb_fundamental_context_t;

namespace longbridge {
namespace fundamental {

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
  void financial_report(const std::string& symbol, int32_t kind, int32_t period,
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
};

} // namespace fundamental
} // namespace longbridge
