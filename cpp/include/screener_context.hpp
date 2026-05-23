#pragma once

#include "async_result.hpp"
#include "callback.hpp"
#include "config.hpp"
#include <optional>
#include <string>

typedef struct lb_screener_context_t lb_screener_context_t;

namespace longbridge {
namespace screener {

/// Screener context — stock screener strategies, search, and indicators.
class ScreenerContext
{
public:
  ScreenerContext();
  explicit ScreenerContext(const lb_screener_context_t* ctx);
  ScreenerContext(const ScreenerContext& ctx);
  ScreenerContext(ScreenerContext&& ctx);
  ~ScreenerContext();
  ScreenerContext& operator=(const ScreenerContext& ctx);

  static ScreenerContext create(const Config& config);

  /// Get preset screener strategies for a given market (raw JSON string)
  void screener_recommend_strategies(const std::string& market,
                                     AsyncCallback<ScreenerContext, std::string> callback) const;

  /// Get the current user's saved screener strategies (raw JSON string)
  void screener_user_strategies(const std::string& market,
                                AsyncCallback<ScreenerContext, std::string> callback) const;

  /// Get detail for one screener strategy by ID (raw JSON string)
  void screener_strategy(int64_t id, AsyncCallback<ScreenerContext, std::string> callback) const;

  /// Search / screen securities using a strategy (raw JSON string)
  void screener_search(const std::string& market,
                       std::optional<int64_t> strategy_id,
                       uint32_t page,
                       uint32_t size,
                       AsyncCallback<ScreenerContext, std::string> callback) const;

  /// Get all available screener indicator definitions (raw JSON string)
  void screener_indicators(AsyncCallback<ScreenerContext, std::string> callback) const;

private:
  const lb_screener_context_t* ctx_;
};

} // namespace screener
} // namespace longbridge
