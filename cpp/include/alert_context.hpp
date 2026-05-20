#pragma once
#include "async_result.hpp"
#include "callback.hpp"
#include "config.hpp"
#include "types.hpp"

typedef struct lb_alert_context_t lb_alert_context_t;

namespace longbridge {
namespace alert {

enum class AlertCondition
{
  PriceRise   = 0,
  PriceFall   = 1,
  PercentRise = 2,
  PercentFall = 3,
};
enum class AlertFrequency
{
  Daily     = 0,
  EveryTime = 1,
  Once      = 2,
};

/// Price alert management context.
class AlertContext {
private:
  const lb_alert_context_t* ctx_;

public:
  AlertContext();
  AlertContext(const lb_alert_context_t* ctx);
  AlertContext(const AlertContext& ctx);
  AlertContext(AlertContext&& ctx);
  ~AlertContext();
  AlertContext& operator=(const AlertContext& ctx);

  /// Create an AlertContext from a Config.
  static AlertContext create(const Config& config);

  /// List all price alerts grouped by security.
  void list(AsyncCallback<AlertContext, AlertList> callback) const;
  /// Add a price alert.
  void add(const std::string& symbol, AlertCondition condition,
           const std::string& trigger_value, AlertFrequency frequency,
           AsyncCallback<AlertContext, void> callback) const;
  /// Update (enable or disable) a price alert.
  /// Set item.enabled before calling to choose the new state.
  void update(const AlertItem& item,
              AsyncCallback<AlertContext, void> callback) const;
};

} // namespace alert
} // namespace longbridge
