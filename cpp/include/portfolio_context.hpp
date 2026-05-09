#pragma once
#include "async_result.hpp"
#include "callback.hpp"
#include "config.hpp"
#include "types.hpp"

typedef struct lb_portfolio_context_t lb_portfolio_context_t;

namespace longbridge {
namespace portfolio {

/// Asset class category.
enum class AssetType
{
  Unknown = 0,
  Stock   = 1,
  Fund    = 2,
  Crypto  = 3,
};

/// Trade flow direction.
enum class FlowDirection
{
  Unknown = 0,
  Buy     = 1,
  Sell    = 2,
};

/// Exchange rate for a currency pair.
struct ExchangeRate { double average_rate; std::string base_currency; double bid_rate; double offer_rate; std::string other_currency; };
/// Collection of exchange rates for supported currencies.
struct ExchangeRates { std::vector<ExchangeRate> exchanges; };

/// P&L summary for one asset category.
struct ProfitSummaryInfo { AssetType asset_type; std::string profit_max; std::string profit_max_name; std::string loss_max; std::string loss_max_name; };

/// P&L breakdown by asset type.
struct ProfitSummaryBreakdown {
  std::string stock; std::string fund; std::string crypto; std::string mmf; std::string other;
  std::string cumulative_transaction_amount; std::string trade_order_num; std::string trade_stock_num;
  int32_t ipo_hit; int32_t ipo_subscription; std::vector<ProfitSummaryInfo> summary_info;
};

/// Account-level P&L summary.
struct ProfitAnalysisSummary {
  std::string currency; std::string current_total_asset; std::string start_date; std::string end_date;
  std::string start_time; std::string end_time; std::string ending_asset_value;
  std::string initial_asset_value; std::string invest_amount; bool is_traded;
  std::string sum_profit; std::string sum_profit_rate; ProfitSummaryBreakdown profits;
};

/// P&L for one security.
struct ProfitAnalysisItem {
  std::string name; std::string market; bool is_holding; std::string profit; std::string profit_rate;
  int64_t clearance_times; AssetType item_type; std::string currency; std::string symbol;
  std::string holding_period; std::string security_code; std::string isin;
  std::string underlying_profit; std::string derivatives_profit; std::string order_profit;
};

/// Per-security P&L breakdown.
struct ProfitAnalysisSublist {
  std::string start; std::string end; std::string start_date; std::string end_date;
  std::string updated_at; std::string updated_date; std::vector<ProfitAnalysisItem> items;
};

/// Combined portfolio P&L analysis response.
struct ProfitAnalysis { ProfitAnalysisSummary summary; ProfitAnalysisSublist sublist; };

/// One security entry in a by-market P&L response.
struct ProfitAnalysisByMarketItem { std::string code; std::string name; std::string market; std::string profit; };

/// P&L grouped by market response.
struct ProfitAnalysisByMarket {
  std::string profit; bool has_more; std::vector<ProfitAnalysisByMarketItem> stock_items;
};

/// One profit-analysis flow record.
struct FlowItem {
  std::string executed_date; std::string executed_timestamp; std::string code; FlowDirection direction;
  std::string executed_quantity; std::string executed_price; std::string executed_cost; std::string describe;
};

/// Paginated list of profit-analysis flow records.
struct ProfitAnalysisFlows { std::vector<FlowItem> flows_list; bool has_more; };

/// One P&L detail line item (credit, debit, or fee).
struct ProfitDetailEntry { std::string describe; std::string amount; };

/// Detailed P&L breakdown for one asset class.
struct ProfitDetails {
  std::string holding_value; std::string profit; std::string cumulative_credited_amount;
  std::vector<ProfitDetailEntry> credited_details; std::string cumulative_debited_amount;
  std::vector<ProfitDetailEntry> debited_details; std::string cumulative_fee_amount;
  std::vector<ProfitDetailEntry> fee_details; std::string short_holding_value;
  std::string long_holding_value; std::string holding_value_at_beginning; std::string holding_value_at_ending;
};

/// Detailed P&L for one security.
struct ProfitAnalysisDetail {
  std::string profit; ProfitDetails underlying_details; ProfitDetails derivative_pnl_details;
  std::string name; std::string updated_at; std::string updated_date; std::string currency;
  int32_t default_tag; std::string start; std::string end; std::string start_date; std::string end_date;
};

/// Portfolio analytics context — exchange rates and P&L analysis.
class PortfolioContext {
private: const lb_portfolio_context_t* ctx_;
public:
  PortfolioContext(); PortfolioContext(const lb_portfolio_context_t* ctx); PortfolioContext(const PortfolioContext&); PortfolioContext(PortfolioContext&&); ~PortfolioContext(); PortfolioContext& operator=(const PortfolioContext&);
  /// Create a PortfolioContext from a Config.
  static PortfolioContext create(const Config& config);
  /// Get exchange rates for all supported currencies.
  void exchange_rate(AsyncCallback<PortfolioContext, ExchangeRates> callback) const;
  /// Get portfolio P&L analysis. start/end: optional "YYYY-MM-DD"; pass empty string for none.
  void profit_analysis(const std::string& start, const std::string& end, AsyncCallback<PortfolioContext, ProfitAnalysis> callback) const;
  /// Get P&L detail for a specific security. start/end: optional "YYYY-MM-DD"; pass empty string for none.
  void profit_analysis_detail(const std::string& symbol, const std::string& start, const std::string& end, AsyncCallback<PortfolioContext, ProfitAnalysisDetail> callback) const;
  /// Get P&L grouped by market. All filter params are optional; pass empty string for none.
  /// page is 1-based (default 1), size is page size (default 20).
  void profit_analysis_by_market(const std::string& market, const std::string& start,
                                 const std::string& end, const std::string& currency,
                                 int32_t page, int32_t size,
                                 AsyncCallback<PortfolioContext, ProfitAnalysisByMarket> callback) const;
  /// Get P&L flow records for a security. start/end: optional "YYYY-MM-DD"; pass empty string for none.
  /// page is 1-based, size is page size. derivative filters derivative flows.
  void profit_analysis_flows(const std::string& symbol, int32_t page, int32_t size,
                             bool derivative, const std::string& start, const std::string& end,
                             AsyncCallback<PortfolioContext, ProfitAnalysisFlows> callback) const;
};

} // namespace portfolio
} // namespace longbridge
