#pragma once

#include "async_result.hpp"
#include "callback.hpp"
#include "config.hpp"
#include "types.hpp"
#include <string>
#include <vector>

typedef struct lb_market_context_t lb_market_context_t;

namespace longbridge {
namespace market {

enum class BrokerHoldingPeriod
{
  Rct1  = 0,
  Rct5  = 1,
  Rct20 = 2,
  Rct60 = 3,
};
enum class AhPremiumPeriod
{
  Min1  = 0,
  Min5  = 1,
  Min15 = 2,
  Min30 = 3,
  Min60 = 4,
  Day   = 5,
  Week  = 6,
  Month = 7,
  Year  = 8,
};

/// Market data context.
class MarketContext
{
private:
  const lb_market_context_t* ctx_;

public:
  MarketContext();
  MarketContext(const lb_market_context_t* ctx);
  MarketContext(const MarketContext& ctx);
  MarketContext(MarketContext&& ctx);
  ~MarketContext();
  MarketContext& operator=(const MarketContext& ctx);

  static MarketContext create(const Config& config);

  /// Get market trading status
  void market_status(AsyncCallback<MarketContext, MarketStatusResponse> callback) const;

  /// Get top broker holdings
  void broker_holding(const std::string& symbol, BrokerHoldingPeriod period,
                      AsyncCallback<MarketContext, BrokerHoldingTop> callback) const;

  /// Get full broker holding details
  void broker_holding_detail(const std::string& symbol,
                             AsyncCallback<MarketContext, BrokerHoldingDetail> callback) const;

  /// Get daily broker holding history
  void broker_holding_daily(const std::string& symbol, const std::string& broker_id,
                            AsyncCallback<MarketContext, BrokerHoldingDailyHistory> callback) const;

  /// Get A/H premium K-lines
  void ah_premium(const std::string& symbol, AhPremiumPeriod period, uint32_t count,
                  AsyncCallback<MarketContext, AhPremiumKlines> callback) const;

  /// Get A/H premium intraday
  void ah_premium_intraday(const std::string& symbol,
                           AsyncCallback<MarketContext, AhPremiumIntraday> callback) const;

  /// Get trade statistics
  void trade_stats(const std::string& symbol,
                   AsyncCallback<MarketContext, TradeStatsResponse> callback) const;

  /// Get market anomalies
  void anomaly(const std::string& market,
               AsyncCallback<MarketContext, AnomalyResponse> callback) const;

  /// Get index constituents
  void constituent(const std::string& symbol,
                   AsyncCallback<MarketContext, IndexConstituents> callback) const;

  /// Get top movers (stocks with unusual price movements) across one or more markets (raw JSON string)
  void top_movers(const std::vector<std::string>& markets,
                  uint32_t sort,
                  const std::string* date,
                  uint32_t limit,
                  AsyncCallback<MarketContext, std::string> callback) const;

  /// Get all available rank category keys and labels (raw JSON string)
  void rank_categories(AsyncCallback<MarketContext, std::string> callback) const;

  /// Get a ranked list of securities for the given category key (raw JSON string)
  void rank_list(const std::string& key,
                 bool need_article,
                 AsyncCallback<MarketContext, std::string> callback) const;
};

} // namespace market
} // namespace longbridge
