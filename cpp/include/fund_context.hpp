#pragma once

#include "async_result.hpp"
#include "callback.hpp"
#include "config.hpp"
#include "types.hpp"
#include <vector>

typedef struct CFundContext CFundContext;

namespace longbridge {
namespace fund {

/// Fund (mutual fund) channel context.
class FundContext
{
private:
  const CFundContext* ctx_;

public:
  FundContext();
  FundContext(const CFundContext* ctx);
  FundContext(const FundContext& ctx);
  FundContext(FundContext&& ctx);
  ~FundContext();

  FundContext& operator=(const FundContext& ctx);

  /// Create a FundContext from a Config.
  static FundContext create(const Config& config);

  // ── fund catalog / market data ─────────────────────────────────────────────

  /// Get the hot-selling fund list
  void hot_funds(
    AsyncCallback<FundContext, std::vector<HotFund>> callback) const;

  /// Get the fund list
  void funds(const std::optional<GetFundsOptions>& opts,
             AsyncCallback<FundContext, std::vector<FundBrief>> callback) const;

  /// Get the fund list filter options
  void filters(AsyncCallback<FundContext, FundFilters> callback) const;

  /// Get fund detail
  void detail(const std::string& counter_id,
              AsyncCallback<FundContext, FundDetail> callback) const;

  /// Get fund analysis (level 1)
  void analysis(const std::string& counter_id,
                const std::optional<GetFundAnalysisOptions>& opts,
                AsyncCallback<FundContext, FundAnalysis> callback) const;

  /// Get fund analysis detail (level 2)
  void analysis_detail(
    const std::string& counter_id,
    const std::optional<GetFundAnalysisOptions>& opts,
    AsyncCallback<FundContext, FundAnalysisDetail> callback) const;

  /// Get fund trend chart
  void trend(const std::string& counter_id,
             const std::optional<GetFundAnalysisOptions>& opts,
             AsyncCallback<FundContext, FundTrend> callback) const;

  /// Get fund annual returns
  void annual_returns(
    const std::string& counter_id,
    const std::optional<FundPageOptions>& opts,
    AsyncCallback<FundContext, std::vector<FundAnnualReturn>> callback) const;

  /// Get fund quarterly returns
  void quarterly_returns(
    const std::string& counter_id,
    const std::optional<FundPageOptions>& opts,
    AsyncCallback<FundContext, std::vector<FundQuarterlyReturn>> callback) const;

  /// Get fund performance figures
  void performance(
    const std::string& counter_id,
    AsyncCallback<FundContext, std::vector<FundPerformance>> callback) const;

  /// Get fund performance comparison
  void performance_comparison(
    const std::string& counter_id,
    const std::optional<GetFundAnalysisOptions>& opts,
    AsyncCallback<FundContext, FundPerformanceComparison> callback) const;

  /// Get fund latest net value
  void nav(const std::string& counter_id,
           AsyncCallback<FundContext, std::vector<FundNavValue>> callback) const;

  /// Get fund historical net value (paged)
  void nav_history(
    const std::string& counter_id,
    const std::optional<FundPageOptions>& opts,
    AsyncCallback<FundContext, std::vector<FundNavValue>> callback) const;

  /// Get fund historical net value by relative time range
  void nav_range(
    const std::string& counter_id,
    const std::optional<FundNavRangeOptions>& opts,
    AsyncCallback<FundContext, std::vector<FundNavValue>> callback) const;

  /// Get a fund's top-10 holdings
  void holdings(const std::string& counter_id,
                const std::optional<GetFundHoldingsOptions>& opts,
                AsyncCallback<FundContext, FundHoldings> callback) const;

  /// Get the stocks held by a fund (reverse lookup)
  void stock_holdings(
    const std::string& counter_id,
    const std::optional<GetFundStockHoldingsOptions>& opts,
    AsyncCallback<FundContext, std::vector<FundStockHolding>> callback) const;

  // ── user fund positions ────────────────────────────────────────────────────

  /// Get the user's fund positions overview
  void positions(const std::optional<GetFundPositionsOptions>& opts,
                 AsyncCallback<FundContext, FundPositions> callback) const;

  /// Get the user's single fund position detail
  void position(const std::string& counter_id,
                const std::optional<GetFundPositionOptions>& opts,
                AsyncCallback<FundContext, FundPositionDetail> callback) const;

  /// Get the performance figures of a held fund
  void position_performance(
    const std::string& counter_id,
    AsyncCallback<FundContext, std::vector<FundPositionPerformance>> callback)
    const;

  /// Get the cumulative-profit series of a held fund
  void position_profits(
    const std::string& counter_id,
    const std::optional<GetFundPositionProfitsOptions>& opts,
    AsyncCallback<FundContext, FundPositionProfits> callback) const;

  /// Get the net-value history of a held fund
  void position_nav(
    const std::string& counter_id,
    const std::optional<FundNavRangeOptions>& opts,
    AsyncCallback<FundContext, std::vector<FundPositionNav>> callback) const;

  /// Get the dividend records of a held fund
  void position_dividends(
    const std::string& counter_id,
    const std::optional<GetFundPositionDividendsOptions>& opts,
    AsyncCallback<FundContext, FundDividends> callback) const;

  // ── fund orders & trading ───────────────────────────────────────────────────

  /// Get the user's fund orders
  void orders(const std::optional<GetFundOrdersOptions>& opts,
              AsyncCallback<FundContext, std::vector<FundOrder>> callback) const;

  /// Get a fund order detail
  void order(int64_t order_id,
             AsyncCallback<FundContext, FundOrderDetail> callback) const;

  /// Get the user's fund transactions (cash-flow records)
  void transactions(
    const std::optional<GetFundTransactionsOptions>& opts,
    AsyncCallback<FundContext, std::vector<FundTransaction>> callback) const;

  /// Validate a fund order before submitting
  void validate_order(
    const ValidateFundOrderOptions& opts,
    AsyncCallback<FundContext, FundOrderValidation> callback) const;

  /// Submit a fund order (buy / sell)
  void submit_order(
    const SubmitFundOrderOptions& opts,
    AsyncCallback<FundContext, FundOrderSubmitResponse> callback) const;

  /// Cancel (withdraw) a fund order
  void cancel_order(int64_t order_id,
                    AsyncCallback<FundContext, void> callback) const;
};

} // namespace fund
} // namespace longbridge
