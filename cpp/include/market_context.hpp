#pragma once

#include "async_result.hpp"
#include "callback.hpp"
#include "config.hpp"
#include "types.hpp"

typedef struct lb_market_context_t lb_market_context_t;

namespace longbridge {
namespace market {

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

  /// Get top broker holdings; period: 0=rct_1,1=rct_5,2=rct_20,3=rct_60
  void broker_holding(const std::string& symbol, int32_t period,
                      AsyncCallback<MarketContext, BrokerHoldingTop> callback) const;

  /// Get full broker holding details
  void broker_holding_detail(const std::string& symbol,
                             AsyncCallback<MarketContext, BrokerHoldingDetail> callback) const;

  /// Get daily broker holding history
  void broker_holding_daily(const std::string& symbol, const std::string& broker_id,
                            AsyncCallback<MarketContext, BrokerHoldingDailyHistory> callback) const;

  /// Get A/H premium K-lines; period: 0-8
  void ah_premium(const std::string& symbol, int32_t period, uint32_t count,
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
};

} // namespace market
} // namespace longbridge
